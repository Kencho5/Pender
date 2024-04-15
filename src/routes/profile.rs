use crate::{imports::*, utils};

pub async fn profile_handler(req: Request<AppState>) -> tide::Result {
    let context = utils::common::get_context(&req).await?;
    // if !context.get("claims").is_some() {
    //     println!("here");
    // }
    println!("{:?}", context.get("claims"));

    let state = req.state();
    let response = state.tera.render_response("profile.html", &context)?;

    Ok(response)
}
