use crate::{
    imports::*,
    utils::{self},
};

pub async fn user_handler(mut req: Request<AppState>) -> tide::Result {
    let user_id = req.param("user_id").unwrap().to_string();
    let mut user = get_user(&mut req, &user_id).await?;
    let posts = get_user_posts(&mut req, &user_id).await?;

    let mut context = utils::common::get_context(&req).await?;
    let session = req.session();
    let lang = session.get::<String>("lang").unwrap_or("GEO".into());

    let translations = utils::translations::load_translations("cities")
        .await
        .unwrap();
    let city = &translations[lang].get(&user.city);

    user.city = city.unwrap().as_str().unwrap().to_string();
    context.insert("user", &user);
    context.insert("posts", &posts);

    let state = req.state();
    let response = state.tera.render_response("user.html", &context)?;

    Ok(response)
}

async fn get_user(
    req: &mut Request<AppState>,
    user_id: &String,
) -> tide::Result<auth_struct::UserPub> {
    let mut pg_conn = req.sqlx_conn::<Postgres>().await;
    let user = sqlx::query_as::<_, auth_struct::UserPub>(
        "SELECT name, phone, city, facebook FROM users where id = $1",
    )
    .bind(user_id)
    .fetch_one(pg_conn.acquire().await?)
    .await?;
    Ok(user)
}

async fn get_user_posts(
    req: &mut Request<AppState>,
    user_id: &String,
) -> tide::Result<auth_struct::UserPub> {
    let mut pg_conn = req.sqlx_conn::<Postgres>().await;
    let posts = sqlx::query_as::<_, auth_struct::UserPub>("SELECT * FROM posts where user_id = $1")
        .bind(user_id)
        .fetch_one(pg_conn.acquire().await?)
        .await?;
    Ok(posts)
}
