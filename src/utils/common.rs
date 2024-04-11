use crate::imports::*;

pub async fn get_context(req: &Request<AppState>) -> tide::Result<tera::Context> {
    let session = req.session();
    let lang = session.get::<String>("lang").unwrap_or("GE".into());

    let state = req.state();
    let translations = state.translations.get(&lang);

    let context = context! {
        "tr" => translations,
        "lang" => lang
    };

    Ok(context)
}

pub async fn get_claims(req: &Request<AppState>) -> tide::Result<BTreeMap<String, String>> {
    let session = req.session();
    let jwt = session
        .get::<String>("_jwt")
        .unwrap_or("not found".to_string());

    let secret = std::env::var("TIDE_SECRET")?;
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes())?;

    let claims: BTreeMap<String, String> = jwt.verify_with_key(&key)?;

    Ok(claims)
}
