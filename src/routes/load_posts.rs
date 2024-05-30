use crate::{imports::*, utils::upload_struct};

pub async fn posts_handler(mut req: Request<AppState>) -> tide::Result {
    let session = req.session();
    let lang = session
        .get::<String>("lang")
        .unwrap_or_else(|| "GEO".into());

    let posts = get_posts(&mut req).await?;
    let state = req.state();
    let translations = state.translations.get(&lang);

    let context = context! {
        "tr" => translations,
        "posts" => posts,
    };

    let response = state
        .tera
        .render_response("components/posts.html", &context)?;

    Ok(response)
}

async fn get_posts(req: &mut Request<AppState>) -> tide::Result<Vec<upload_struct::PostStruct>> {
    let mut pg_conn = req.sqlx_conn::<Postgres>().await;
    let posts = sqlx::query_as::<_, upload_struct::PostStruct>(
        "SELECT * FROM posts ORDER BY time_posted DESC LIMIT 4",
    )
    .fetch_all(pg_conn.acquire().await?)
    .await?;
    Ok(posts)
}
