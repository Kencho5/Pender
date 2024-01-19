use crate::imports::*;

pub async fn load_defaults(req: &Request<Tera>) -> tide::Result<(String, tera::Context)> {
    let session = req.session();
    let lang = session
        .get_raw("lang")
        .unwrap_or("ge".into())
        .replace("\"", "");
    let translations = translations::load_translations(&lang).await.unwrap();

    let context = context! {
        "tr" => &translations,
        "user" => "user",
        "lang" => &lang
    };

    Ok((lang, context))
}
