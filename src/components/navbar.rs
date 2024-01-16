use serde_json::Value;
use tera::Tera;
use tide::Request;
use tide_tera::prelude::*;

pub async fn render_navbar(
    tera: &Tera,
    lang: &str,
    route: &str,
    translations: &Value,
    req: &Request<Tera>,
) -> tide::Result<String> {
    let session = req.session();
    let user = session.get_raw("user");

    let navbar_context = context! {
        "tr" => &translations,
        "lang" => &lang,
        "route" => &route,
        "user" => &user
    };

    let rendered_navbar = tera.render("components/navbar.html", &navbar_context)?;
    Ok(rendered_navbar)
}
