use crate::{imports::*, utils};

pub async fn home_handler(req: Request<AppState>) -> tide::Result {
    let context = utils::common::get_common(&req).await.unwrap();

    let session = req.session();
    let jwt = session
        .get::<String>("_jwt")
        .unwrap_or("not found".to_string());

    let secret = std::env::var("TIDE_SECRET")?;
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes())?;

    let claims: BTreeMap<String, String> = jwt.verify_with_key(&key)?;

    let state = req.state();
    let response = state.tera.render_response("home.html", &context)?;

    Ok(response)
}
