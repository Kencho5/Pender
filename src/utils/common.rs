use crate::imports::*;

pub async fn get_common(req: &Request<AppState>) -> tide::Result<tera::Context> {
    let session = req.session();
    let lang = session.get::<String>("lang").unwrap_or("GE".into());

    let state = req.state();
    let translations = state.translations.get(&lang);

    let context = context! {
        "tr" => translations,
        "lang" => lang
    };

    Ok(context)
}
