use crate::imports::*;

pub async fn login_handler(req: Request<Tera>) -> tide::Result {
    let (_lang, context) = load_defaults(&req).await?;
    let tera = req.state();
    let response = tera.render_response("login.html", &context)?;
    Ok(response)
}
