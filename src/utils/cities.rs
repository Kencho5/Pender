use crate::imports::*;

pub async fn get_cities(mut req: tide::Request<AppState>) -> tide::Result {
    Ok(Response::new(StatusCode::Ok))
}
