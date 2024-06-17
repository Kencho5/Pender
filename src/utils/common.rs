use crate::imports::*;
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn get_context(req: &Request<AppState>) -> tide::Result<tera::Context> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let session = req.session();
    let lang = session.get::<String>("lang").unwrap_or("GEO".into());

    let state = req.state();
    let translations = state.translations.get(&lang);

    let context = context! {
        "tr" => translations,
        "lang" => lang,
        "route" => req.url().path(),
        "claims" => get_claims(req).await?,
        "now" => now
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

pub async fn logged_in(context: &tera::Context) -> tide::Result<bool> {
    if let Some(claims) = context.get("claims") {
        if claims.is_object() && claims.as_object().unwrap().is_empty() {
            return Ok(false);
        }
    }

    Ok(true)
}
