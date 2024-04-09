use crate::imports::*;

pub async fn home_handler(req: Request<AppState>) -> tide::Result {
    let session = req.session();
    let lang = session.get::<String>("lang").unwrap_or("GE".into());

    let state = req.state();
    let translations = &state.translations.get(&lang);

    let response = state.tera.render_response(
        "home.html",
        &context! {
            "tr" => translations,
            "lang" => lang
        },
    )?;
    Ok(response)
}
