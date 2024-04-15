use crate::config::config_manager;
use crate::{imports::*, utils};

pub async fn login_handler(req: Request<AppState>) -> tide::Result {
    let context = utils::common::get_context(&req).await.unwrap();

    let state = req.state();
    let response = state.tera.render_response("login.html", &context)?;

    Ok(response)
}

pub async fn login_post_handler(mut req: Request<AppState>) -> tide::Result {
    let user: auth_struct::LoginData = req.body_form().await?;
    let mut response = Response::builder(200)
        .body("<p class='success'>Account Created!</p>")
        .build();

    let find_user_result = find_user(&mut req, &user.email).await;

    match find_user_result {
        Ok(user_db) => {
            if unix::verify(user.password, &user_db.password) {
                if let Some(token) = generate_token(&req.state().config, &user_db).await? {
                    response.insert_header("Set-Cookie", format!("_jwt={}", token));
                    response.set_body("<p class='success'>Logged in!</p>");
                    return Ok(response);
                }
            }
            response.set_body("<p class='error'>Wrong password</p>");
            return Ok(response);
        }
        Err(_) => {
            response.set_body("<p class='error'>Email address not found</p>");
            return Ok(response);
        }
    }
}

async fn find_user(
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

async fn generate_token(
    config: &config_manager::Config,
    user: &auth_struct::UserStruct,
) -> tide::Result<Option<String>> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(config.tide_secret.as_bytes())?;

    let mut claims = BTreeMap::new();
    claims.insert("id", &user.id);
    claims.insert("email", &user.email);
    claims.insert("name", &user.name);
    claims.insert("phone", &user.phone);
    claims.insert("city", &user.city);

    let token = claims.sign_with_key(&key)?;
    Ok(Some(token))
}
