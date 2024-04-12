use crate::imports::*;
use std::fs;

pub async fn get_cities(req: tide::Request<AppState>) -> tide::Result {
    let session = req.session();
    let lang = session.get::<String>("lang").unwrap_or("GE".into());

    let file_path = "./public/translations/cities.json";
    let file = fs::read_to_string(&file_path).expect("Not Found");
    let cities: serde_json::Value = serde_json::from_str(&file)?;

    let mut html_content = String::new();
    if let Some(ge_cities) = cities[lang].as_object() {
        for (key, value) in ge_cities {
            html_content.push_str(&format!(
                "<p class='{}'>{}</p>",
                key,
                value.to_string().trim_matches('"')
            ));
        }
    }

    let response = Response::builder(200).body(html_content).build();
    Ok(response)
}
