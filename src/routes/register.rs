use crate::{imports::*, utils};

pub async fn register_handler(req: Request<AppState>) -> tide::Result {
    let context = utils::common::get_context(&req).await.unwrap();

    let state = req.state();
    let response = state.tera.render_response("register.html", &context)?;
    Ok(response)
}

pub async fn register_post_handler(mut req: Request<AppState>) -> tide::Result {
    let user: auth_struct::RegisterData = req.body_form().await?;
    let mut pg_conn = req.sqlx_conn::<Postgres>().await;
    let mut response = Response::builder(200)
        .body("<p class='success'>Account Created!</p>")
        .build();

    let check_existing = sqlx::query("SELECT id FROM users WHERE email = $1")
        .bind(&user.email)
        .fetch_one(pg_conn.acquire().await?)
        .await;

    match check_existing {
        Ok(_) => {
            response.set_body("<p class='error'>Email address already in use</p>");
            return Ok(response);
        }
        Err(_) => {}
    }

    if user.password.is_empty() {
        response.set_body("<p class='error'>Fill in the form</p>");
        return Ok(response);
    }

    let pass_hash = bcrypt::hash(user.password).unwrap();
    let user_id = Uuid::new_v4();

    let registration_result = sqlx::query(
        "INSERT INTO users(id, email, name, phone, city, password) VALUES($1, $2, $3, $4, $5, $6)",
    )
    .bind(user_id.to_string())
    .bind(user.email)
    .bind(user.name)
    .bind(user.phone)
    .bind(user.city)
    .bind(pass_hash)
    .execute(pg_conn.acquire().await?)
    .await;

    match registration_result {
        Ok(_) => {}
        Err(_) => {
            response.set_body("<p class='error'>Fill in the form</p>");
            return Ok(response);
        }
    }

    Ok(response)
}
