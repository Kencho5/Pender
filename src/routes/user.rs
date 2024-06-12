use crate::{
    imports::*,
    utils::{self},
};

pub async fn user_handler(req: Request<AppState>) -> tide::Result {
    let user_id = req.param("user_id").unwrap().to_string();

    let context = utils::common::get_context(&req).await?;

    let state = req.state();
    let response = state.tera.render_response("user.html", &context)?;

    Ok(response)
}
