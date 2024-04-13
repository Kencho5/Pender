use crate::{imports::*, utils};

pub async fn register_handler(req: Request<AppState>) -> tide::Result {
    let mut context = utils::common::get_context(&req).await.unwrap();
    render_register_page(&req, &mut context).await
}

async fn render_register_page(
    req: &Request<AppState>,
    context: &mut tera::Context,
) -> tide::Result {
    let state = req.state();
    let response = state.tera.render_response("register.html", context)?;
    Ok(response)
}

pub async fn register_post_handler(mut req: Request<AppState>) -> tide::Result {
    let user: auth_struct::RegisterData = req.body_form().await?;
    let mut pg_conn = req.sqlx_conn::<Postgres>().await;

    let check_existing = sqlx::query("SELECT id FROM users WHERE email = $1")
        .bind(&user.email)
        .fetch_one(pg_conn.acquire().await?)
        .await;

    match check_existing {
        Ok(_) => {
            let mut context = utils::common::get_context(&req).await.unwrap();
            context.insert("error", "in_use");
            return Ok(render_register_page(&req, &mut context).await?);
        }
        Err(_) => {}
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
            let mut context = utils::common::get_context(&req).await.unwrap();
            context.insert("error", "fill_form");
            return Ok(render_register_page(&req, &mut context).await?);
        }
    }

    Ok(tide::Redirect::new("/register").into())
}
