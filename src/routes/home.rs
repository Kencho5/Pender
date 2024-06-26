use crate::{
    imports::*,
    utils::{self},
};

pub async fn home_handler(mut req: Request<AppState>) -> tide::Result {
    let mut context = utils::common::get_context(&req).await?;
    context.insert("count", &get_posts_count(&mut req).await?);

    let state = req.state();
    let response = state.tera.render_response("home.html", &context)?;

    Ok(response)
}

async fn get_posts_count(req: &mut Request<AppState>) -> tide::Result<i64> {
    let mut pg_conn = req.sqlx_conn::<Postgres>().await;
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM POSTS")
        .fetch_one(pg_conn.acquire().await?)
        .await?;

    Ok(count.0)
}
