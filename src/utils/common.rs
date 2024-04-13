use crate::imports::*;

pub async fn get_context(req: &Request<AppState>) -> tide::Result<tera::Context> {
    let session = req.session();
    let lang = session.get::<String>("lang").unwrap_or("GE".into());

    let state = req.state();
    let translations = state.translations.get(&lang);

    let context = context! {
        "tr" => translations,
        "lang" => lang,
        "route" => req.url().path(),
        "claims" => json!(get_claims(req).await?)
    };

    Ok(context)
}

pub async fn get_claims(req: &Request<AppState>) -> tide::Result<BTreeMap<String, String>> {
    let jwt = match req.cookie("_jwt") {
        Some(jwt) => jwt,
        None => return Ok(BTreeMap::new()),
    };

    let key: Hmac<Sha256> = Hmac::new_from_slice(req.state().config.tide_secret.as_bytes())?;

    let claims: BTreeMap<String, String> = jwt.value().verify_with_key(&key)?;

    Ok(claims)
}
