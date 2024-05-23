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
        .body(
            r#"
            <p class='success'>
                Account created!
            </p>
            "#,
        )
        .build();

    if email_already_exists(&mut pg_conn, &user.email).await {
        response.set_body(
            r#"
            <p class='error'>
                Email already in use
            </p>
            "#,
        );
        return Ok(response);
    }

    if user.password.is_empty() {
        response.set_body(
            r#"
            <p class='error'>
                Fill in the form
            </p>
            "#,
        );
        return Ok(response);
    }

    let user_id = Uuid::new_v4();

    if !register_user(&mut pg_conn, user_id, &user).await {
        response.set_body(
            r#"
            <p class='error'>
                Failed to register user
            </p>
            "#,
        );
        return Ok(response);
    }

    Ok(response)
}

async fn email_already_exists(pg_conn: &mut sqlx::PgConnection, email: &str) -> bool {
    sqlx::query("SELECT id FROM users WHERE email = $1")
        .bind(email)
        .fetch_optional(pg_conn)
        .await
        .expect("Email fetching error")
        .is_some()
}

async fn register_user(
    pg_conn: &mut sqlx::PgConnection,
    user_id: Uuid,
    user: &auth_struct::RegisterData,
) -> bool {
    let pass_hash = bcrypt::hash(&user.password).unwrap();
    let registration_result = sqlx::query(
        "INSERT INTO users(id, email, name, phone, city, facebook, password) VALUES($1, $2, $3, $4, $5, $6, $7)",
    )
    .bind(user_id.to_string())
    .bind(&user.email)
    .bind(&user.name)
    .bind(&user.phone)
    .bind(&user.city)
    .bind(&user.facebook)
    .bind(&pass_hash)
    .execute(pg_conn)
    .await;

    registration_result.is_ok()
}
