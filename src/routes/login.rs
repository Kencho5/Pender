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

    let find_user = {
        let mut pg_conn = req.sqlx_conn::<Postgres>().await;
        sqlx::query_as::<_, auth_struct::UserStruct>("SELECT * FROM users WHERE email = $1")
            .bind(&user.email)
            .fetch_one(pg_conn.acquire().await?)
            .await
    };

    match find_user {
        Ok(_) => {
            let user_db = find_user.unwrap();

            if unix::verify(user.password, &user_db.password) {
                let key: Hmac<Sha256> =
                    Hmac::new_from_slice(req.state().config.tide_secret.as_bytes())?;

                let mut claims = BTreeMap::new();
                claims.insert("id", user_db.id);
                claims.insert("email", user_db.email);
                claims.insert("name", user_db.name);
                claims.insert("phone", user_db.phone);
                claims.insert("city", user_db.city);

                let token = claims.sign_with_key(&key)?;
                response.insert_header("Set-Cookie", format!("_jwt={}", token));

                response.set_body("<p class='success'>Logged in!</p>");
                return Ok(response);
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
