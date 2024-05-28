use crate::imports::*;

pub async fn posts_handler(req: Request<AppState>) -> tide::Result {
    let session = req.session();
    let lang = session.get::<String>("lang").unwrap_or("GEO".into());

    let state = req.state();
    let translations = state.translations.get(&lang);

    let context = context! {
        "tr" => translations,
    };

    let state = req.state();
    let response = state
        .tera
        .render_response("components/posts.html", &context)?;

    Ok(response)
}
