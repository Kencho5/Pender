use crate::imports::*;

pub async fn logout_handler(_req: Request<AppState>) -> tide::Result {
    let mut response = Response::new(StatusCode::Ok);
    response.remove_cookie(Cookie::named("_jwt"));
    response.insert_header("HX-Redirect", "/login");

    Ok(response)
}
