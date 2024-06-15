use crate::imports::*;
use sqlx::postgres::PgQueryResult;

pub async fn delete_handler(mut req: tide::Request<AppState>) -> tide::Result {
    let post_id = req.param("post_id").unwrap().to_string();
    let mut response = Response::new(StatusCode::Ok);
    response.insert_header("HX-Redirect", "/profile?tab=posts-selector");

    delete_post(&mut req, &post_id).await?;

    Ok(response)
}

async fn delete_post(
    req: &mut Request<AppState>,
    post_id: &String,
) -> Result<PgQueryResult, sqlx::Error> {
    let mut pg_conn = req.sqlx_conn::<Postgres>().await;
    let result = sqlx::query("DELETE FROM posts WHERE id = $1")
        .bind(post_id)
        .execute(pg_conn.acquire().await?)
        .await?;
    Ok(result)
}
