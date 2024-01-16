use crate::components::navbar::render_navbar;
use crate::utils::translations::load_translations;
use tera::Tera;
use tide::Request;
use tide_tera::prelude::*;

pub async fn load_defaults(req: &Request<Tera>) -> tide::Result<(String, tera::Context)> {
    let tera = req.state();
    let route = req.url().path();

    let session = req.session();
    let lang = session
        .get_raw("lang")
        .unwrap_or("ge".into())
        .replace("\"", "");
    let translations = load_translations(&lang).await.unwrap();

    let rendered_navbar = render_navbar(&tera, &lang, &route, &translations, &req).await?;

    let context = context! {
        "navbar" => &rendered_navbar,
        "tr" => &translations,
    };

    Ok((lang, context))
}
