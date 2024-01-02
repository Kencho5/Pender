use tide::{Request, Response};
use tera::{Context, Tera};
use std::{fs, error::Error};
use tide_tera::prelude::*;
use crate::utils::translations::load_translations;

pub async fn home_handler(req: Request<Tera>) -> tide::Result {
    let tera = req.state();
    let navbar_content = fs::read_to_string("./front/templates/navbar.html").expect("Navbar Not Found");
    let route = req.url().path();

    let session = req.session();
    let lang = session.get_raw("lang").unwrap_or("ge".into()).replace("\"", "");
    let translations = load_translations(&lang).await.unwrap();

    let mut navbar_context = context! {
        "navbar" => &navbar_content,
        "tr" => &translations, 
        "lang" => &lang,
        "route" => &route
    };

    let rendered_navbar = tera.render("navbar.html", &navbar_context)?;

    let mut context = context! {
        "navbar" => &rendered_navbar,
        "tr" => &translations, 
    };

    let response = tera.render_response("home.html", &context)?;
    Ok(response)
}
