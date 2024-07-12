use crate::{imports::*, utils::upload_struct};

pub async fn posts_handler(mut req: Request<AppState>) -> tide::Result {
    let session = req.session();
    let lang = session
        .get::<String>("lang")
        .unwrap_or_else(|| "GEO".into());

    let path = req.url().path().to_string();
    let posts = get_posts(&mut req, &path).await?;
    let state = req.state();
    let translations = state.translations.get(&lang);

    let mut content_url = state.content_url.clone();
    if state.config.enviorement == "local" {
        content_url = "".to_string();
    }

    let context = context! {
        "tr" => translations,
        "posts" => posts,
        "ver" => state.version,
        "route" => req.url().path(),
        "content_url" => content_url,
        "branch" => state.branch
    };

    let response = state
        .tera
        .render_response("components/posts.html", &context)?;

    Ok(response)
}

async fn get_posts(
    req: &mut Request<AppState>,
    url: &str,
) -> tide::Result<Vec<upload_struct::PostStruct>> {
    let query: &str;
    if url.contains("popular") {
        query = "SELECT * FROM posts ORDER BY views DESC LIMIT 4";
    } else {
        query = "SELECT * FROM posts ORDER BY time_posted DESC LIMIT 8";
    }

    let mut pg_conn = req.sqlx_conn::<Postgres>().await;
    let posts = sqlx::query_as::<_, upload_struct::PostStruct>(&query)
        .fetch_all(pg_conn.acquire().await?)
        .await?;
    Ok(posts)
}
