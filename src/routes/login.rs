use crate::{imports::*, utils};

pub async fn login_handler(req: Request<AppState>) -> tide::Result {
    let context = utils::common::get_context(&req).await.unwrap();

    let state = req.state();
    let response = state.tera.render_response("login.html", &context)?;

    Ok(response)
}

pub async fn login_post_handler(mut req: Request<AppState>) -> tide::Result {
    let user: auth_struct::LoginData = req.body_form().await?;

    let secret = std::env::var("TIDE_SECRET")?;
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes())?;

    let mut claims = BTreeMap::new();
    claims.insert("email", user.email);

    let token = claims.sign_with_key(&key)?;

    let session = req.session_mut();
    session.insert("_jwt", token)?;

    Ok(tide::Redirect::new("/").into())
}
