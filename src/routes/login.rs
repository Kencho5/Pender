use crate::{imports::*, utils};

pub async fn login_handler(req: Request<AppState>) -> tide::Result {
    let context = utils::common::get_common(&req).await.unwrap();

    let state = req.state();
    let response = state.tera.render_response("login.html", &context)?;

    Ok(response)
}

pub async fn login_post_handler(mut req: Request<AppState>) -> tide::Result {
    let user: form_struct::LoginData = req.body_form().await?;
    println!("{:?}", user.email);

    Ok(tide::Redirect::new("/login").into())
}
