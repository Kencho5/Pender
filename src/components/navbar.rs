use tera::{Tera};
use tide_tera::prelude::*;
use serde_json::Value;

pub async fn render_navbar(tera: &Tera, lang: &str, route: &str, translations: &Value) -> tide::Result<String> {
    let navbar_context = context! {
        "tr" => &translations, 
        "lang" => &lang,
        "route" => &route,
    };

    let rendered_navbar = tera.render("components/navbar.html", &navbar_context)?;
    Ok(rendered_navbar)
}

