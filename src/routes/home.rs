use tide::{Request, Response};
use tera::{Context, Tera};
use std::{fs, error::Error};
use tide_tera::prelude::*;
use crate::utils::translations::load_translations;
use crate::components::navbar::render_navbar;

pub async fn home_handler(req: Request<Tera>) -> tide::Result {
    let tera = req.state();
    let route = req.url().path();

    let session = req.session();
    let lang = session.get_raw("lang").unwrap_or("ge".into()).replace("\"", "");
    let translations = load_translations(&lang).await.unwrap();

    let rendered_navbar = render_navbar(&tera, &lang, &route).await?;

    let mut context = context! {
        "navbar" => &rendered_navbar,
        "tr" => &translations, 
    };

    let response = tera.render_response("home.html", &context)?;
    Ok(response)
}
