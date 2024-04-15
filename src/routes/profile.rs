use crate::{imports::*, utils};

pub async fn profile_handler(req: Request<AppState>) -> tide::Result {
    let context = utils::common::get_context(&req).await?;

    if let Some(claims) = context.get("claims") {
        if claims.is_object() && claims.as_object().unwrap().is_empty() {
            return Ok(tide::Redirect::see_other("/login").into());
        }
    }

    let state = req.state();
    let response = state.tera.render_response("profile.html", &context)?;

    Ok(response)
}
