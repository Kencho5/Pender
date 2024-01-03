use tera::{Tera};
use std::{fs};
use tide_tera::prelude::*;
use crate::utils::translations::load_translations;

pub async fn render_navbar(tera: &Tera, lang: &str, route: &str) -> tide::Result<String> {
    let navbar_content = fs::read_to_string("./front/templates/navbar.html").expect("Navbar Not Found");

    let translations = load_translations(&lang).await.unwrap();

    let navbar_context = context! {
        "navbar" => &navbar_content,
        "tr" => &translations, 
        "lang" => &lang,
        "route" => &route,
    };

    let rendered_navbar = tera.render("navbar.html", &navbar_context)?;
    Ok(rendered_navbar)
}

