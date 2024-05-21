use crate::config::config_manager;
use crate::{imports::*, utils};

pub async fn login_handler(req: Request<AppState>) -> tide::Result {
    let context = utils::common::get_context(&req).await.unwrap();

    let state = req.state();
    let response = state.tera.render_response("login.html", &context)?;

    Ok(response)
}

pub async fn login_post_handler(mut req: Request<AppState>) -> tide::Result {
    let session = req.session();
    let lang = session.get::<String>("lang").unwrap_or("GEO".into());

    let user: auth_struct::LoginData = req.body_form().await?;
    let mut response = Response::builder(200)
        .body(
            r#"
            <p class='success'>
                <i class="fa-solid fa-circle-check"></i>
                Logged in!
            </p>
            "#,
        )
        .build();

    let find_user_result = find_user(&mut req, &user.email).await;

    match find_user_result {
        Ok(user_db) => {
            if unix::verify(user.password, &user_db.password) {
                if let Some(token) = generate_token(&req.state().config, &user_db, lang).await? {
                    response.insert_cookie(
                        Cookie::build("_jwt", token)
                            .max_age(time::Duration::days(7))
                            .same_site(tide::http::cookies::SameSite::Lax)
                            .path("/")
                            .finish(),
                    );

                    return Ok(response);
                }
            }
            response.set_body(
                r#"
                <p class='error'>
                    <i class="fa-solid fa-circle-exclamation"></i>
                    Wrong password
                </p>
                "#,
            );
            return Ok(response);
        }
        Err(_) => {
            response.set_body(
                r#"
                <p class='error'>
                    <i class="fa-solid fa-circle-exclamation"></i>
                    Email address not found
                </p>
                "#,
            );
            return Ok(response);
        }
    }
}

pub async fn find_user(
    req: &mut Request<AppState>,
    email: &str,
) -> tide::Result<auth_struct::UserStruct> {
    let mut pg_conn = req.sqlx_conn::<Postgres>().await;
    let user = sqlx::query_as::<_, auth_struct::UserStruct>("SELECT * FROM users WHERE email = $1")
        .bind(email)
        .fetch_one(pg_conn.acquire().await?)
        .await?;
    Ok(user)
}

pub async fn generate_token(
    config: &config_manager::Config,
    user: &auth_struct::UserStruct,
    lang: String,
) -> tide::Result<Option<String>> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(config.tide_secret.as_bytes())?;
    let city = utils::translations::load_translations("cities")
        .await
        .unwrap()[lang]
        .get(&user.city)
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();

    let mut claims = BTreeMap::new();
    claims.insert("id", &user.id);
    claims.insert("email", &user.email);
    claims.insert("name", &user.name);
    claims.insert("phone", &user.phone);
    claims.insert("city", &city);
    claims.insert("city_value", &user.city);
    claims.insert("facebook", &user.facebook);

    let token = claims.sign_with_key(&key)?;
    Ok(Some(token))
}
