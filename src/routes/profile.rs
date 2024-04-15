use crate::{
    imports::*,
    utils::{self, common::logged_in},
};

pub async fn profile_handler(req: Request<AppState>) -> tide::Result {
    let context = utils::common::get_context(&req).await?;

    if !logged_in(&context).await? {
        return Ok(tide::Redirect::see_other("/login").into());
    }

    let state = req.state();
    let response = state.tera.render_response("profile.html", &context)?;

    Ok(response)
}
