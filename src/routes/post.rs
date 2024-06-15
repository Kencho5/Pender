use crate::{
    imports::*,
    utils::{self, upload_struct},
};
use sqlx::postgres::PgQueryResult;

pub async fn post_handler(mut req: Request<AppState>) -> tide::Result {
    let mut context = utils::common::get_context(&req).await?;

    let post_id = req.param("post_id").unwrap().to_string();

    let session = req.session_mut();
    let mut viewed_posts = session.get::<String>("viewed_posts").unwrap_or_default();

    if !viewed_posts.contains(&post_id) {
        viewed_posts.push_str(&post_id);
        session.insert("viewed_posts", viewed_posts)?;
        increment_views(&mut req, &post_id).await?;
    }

    match get_post(&req, post_id).await {
        Ok(post) => {
            context.insert("post", &post);
        }
        Err(_) => {
            context.insert("post", "error");
        }
    }

    let state = req.state();
    let response = state.tera.render_response("post.html", &context)?;

    Ok(response)
}

async fn get_post(
    req: &Request<AppState>,
    post_id: String,
) -> tide::Result<upload_struct::PostStruct> {
    let mut pg_conn = req.sqlx_conn::<Postgres>().await;

    let post = sqlx::query_as::<_, upload_struct::PostStruct>("SELECT * FROM posts WHERE id = $1")
        .bind(post_id)
        .fetch_one(pg_conn.acquire().await?)
        .await?;

    Ok(post)
}

async fn increment_views(
    req: &mut Request<AppState>,
    post_id: &String,
) -> Result<PgQueryResult, sqlx::Error> {
    let mut pg_conn = req.sqlx_conn::<Postgres>().await;
    let result = sqlx::query("UPDATE posts SET views = views + 1 WHERE id = $1")
        .bind(post_id)
        .execute(pg_conn.acquire().await?)
        .await?;
    Ok(result)
}
