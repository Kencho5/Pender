use crate::imports::*;

pub async fn posts_handler(req: Request<AppState>) -> tide::Result {
    let session = req.session();
    let lang = session.get::<String>("lang").unwrap_or("GEO".into());

    let state = req.state();
    let translations = state.translations.get(&lang);

    let context = context! {
        "tr" => translations,
        // "posts" => get_posts()
    };

    let state = req.state();
    let response = state
        .tera
        .render_response("components/posts.html", &context)?;

    Ok(response)
}

// async fn get_posts(req: &mut Request<AppState>) -> tide::Result {
//     let mut pg_conn = req.sqlx_conn::<Postgres>().await;
//     let user = sqlx::query_as::<_, auth_struct::UserStruct>("SELECT * FROM posts WHERE")
//         .fetch_one(pg_conn.acquire().await?)
//         .await?;
//     Ok(user)
// }
