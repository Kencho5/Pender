use crate::{
    imports::*,
    utils::{self},
};

pub async fn steps_handler(req: Request<AppState>) -> tide::Result {
    let context = utils::common::get_context(&req).await?;
    let step = req.param("step").unwrap().to_string();

    let state = req.state();
    let html = format!("components/upload_steps/upload_step{}.html", step);
    let response = state.tera.render_response(&html, &context)?;

    Ok(response)
}
