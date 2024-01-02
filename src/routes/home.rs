use tide::{Request, Response};
use tera::Tera;
use std::{fs, error::Error};
use tide_tera::prelude::*;
use crate::utils::translations::load_translations;

pub async fn home_handler(req: tide::Request<Tera>) -> tide::Result {
    let tera = req.state();
    let navbar = fs::read_to_string("./front/templates/navbar.html").expect("Not Found");

    let session = req.session();
    let lang = session.get_raw("lang").unwrap_or("en".into());
    let translations = load_translations(&lang).await.unwrap();

    let context = context! {
        "navbar" => &navbar,
        "tr" => &translations, 
    };

    tera.render_response("home.html", &context)
}
