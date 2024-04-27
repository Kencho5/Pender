use crate::{
    imports::*,
    utils::{self, upload_struct},
};

pub async fn post_handler(req: Request<AppState>) -> tide::Result {
    let mut context = utils::common::get_context(&req).await?;

    let post_id = req.param("post_id").unwrap().to_string();

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

pub async fn get_post(
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
