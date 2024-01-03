use tide::{Request, Response, StatusCode};
use crate::utils::translations;
use tide::sessions::Session;

pub async fn set_language_handler(mut req: tide::Request<tera::Tera>) -> tide::Result {
    let language = req.param("lang").unwrap().to_string();
    let session = req.session_mut();
    session.insert("lang", language)?;

    Ok(Response::new(StatusCode::Ok))
}

