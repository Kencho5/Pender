use crate::{imports::*, utils};

pub async fn home_handler(req: Request<AppState>) -> tide::Result {
    let context = utils::common::get_common(&req).await.unwrap();

    let state = req.state();
    let response = state.tera.render_response("home.html", &context)?;

    Ok(response)
}
