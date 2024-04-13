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
        sqlx::query_as::<_, auth_struct::RegisterData>("SELECT * FROM users WHERE email = $1")
            .bind(&user.email)
            .fetch_one(pg_conn.acquire().await?)
            .await
    };

    match find_user {
        Ok(_) => {
            if unix::verify(user.password, &find_user.unwrap().password) {
                let key: Hmac<Sha256> =
                    Hmac::new_from_slice(req.state().config.tide_secret.as_bytes())?;

                let mut claims = BTreeMap::new();
                claims.insert("email", user.email);

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
