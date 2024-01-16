use tera::Tera;
use tide::{Request, Response, StatusCode};

pub async fn api_upload_handler(req: Request<Tera>) -> tide::Result {
    let response = Response::new(StatusCode::Ok);

    Ok(response)
}
