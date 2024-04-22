use crate::{imports::*, utils};

pub async fn reset_password_handler(req: Request<AppState>) -> tide::Result {
    let context = utils::common::get_context(&req).await?;

    let state = req.state();
    let response = state
        .tera
        .render_response("reset_password.html", &context)?;

    Ok(response)
}
