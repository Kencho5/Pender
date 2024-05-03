use crate::{imports::*, utils};

pub async fn search_handler(req: Request<AppState>) -> tide::Result {
    let context = utils::common::get_context(&req).await?;

    let state = req.state();
    let response = state.tera.render_response("search.html", &context)?;

    Ok(response)
}
