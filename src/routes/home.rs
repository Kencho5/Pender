use crate::{imports::*, utils};

pub async fn home_handler(req: Request<AppState>) -> tide::Result {
    let context = utils::common::get_context(&req).await?;

    let state = req.state();
    let response = state.tera.render_response("home.html", &context)?;

    Ok(response)
}
