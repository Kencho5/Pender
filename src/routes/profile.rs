use crate::components::navbar::render_navbar;
use crate::utils::translations::load_translations;
use tera::Tera;
use tide::Request;
use tide_tera::prelude::*;

pub async fn profile_handler(req: Request<Tera>) -> tide::Result {
    let tera = req.state();
    let route = req.url().path();

    let session = req.session();
    let lang = session
        .get_raw("lang")
        .unwrap_or("ge".into())
        .replace("\"", "");
    let translations = load_translations(&lang).await.unwrap();

    let rendered_navbar = render_navbar(&tera, &lang, &route, &translations).await?;

    let context = context! {
        "navbar" => &rendered_navbar,
        "tr" => &translations,
    };

    let response = tera.render_response("profile.html", &context)?;
    Ok(response)
}
