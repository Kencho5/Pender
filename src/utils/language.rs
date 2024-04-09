use crate::imports::*;

pub async fn set_language_handler(mut req: tide::Request<AppState>) -> tide::Result {
    let language = req.param("lang").unwrap().to_string();
    let session = req.session_mut();
    session.insert("lang", language)?;

    Ok(Response::new(StatusCode::Ok))
}
