use crate::utils::common::load_defaults;
use tera::Tera;
use tide::Request;
use tide_tera::prelude::*;

pub async fn search_handler(req: Request<Tera>) -> tide::Result {
    let (_lang, context) = load_defaults(&req).await?;
    let tera = req.state();
    let response = tera.render_response("search.html", &context)?;
    Ok(response)
}
