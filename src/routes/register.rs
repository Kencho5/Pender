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

    let result = sqlx::query("INSERT INTO users(email, password) VALUES($1, $2)")
        .bind(user.email)
        .bind(user.password)
        .execute(pg_conn.acquire().await?)
        .await;

    match result {
        Ok(_) => println!("Row inserted"),
        Err(e) => println!("Error inserting row: {}", e),
    }

    Ok(tide::Redirect::new("/register").into())
}
