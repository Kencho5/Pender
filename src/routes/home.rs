use crate::imports::*;

pub async fn home_handler(req: Request<Tera>) -> tide::Result {
    let (_lang, context) = load_defaults(&req).await?;
    let tera = req.state();
    let response = tera.render_response("home.html", &context)?;
    Ok(response)
}
