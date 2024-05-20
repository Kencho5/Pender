use crate::imports::*;
use std::fs;

#[derive(Deserialize)]
struct Search {
    city_dummy: String,
}

pub async fn get_cities(req: tide::Request<AppState>) -> tide::Result {
    let session = req.session();
    let lang = session.get::<String>("lang").unwrap_or("GE".into());

    let file_path = "./public/translations/cities.json";
    let file = fs::read_to_string(&file_path).expect("Not Found");
    let json: serde_json::Value = serde_json::from_str(&file)?;

    let mut input: Search = req.query()?;
    input.city_dummy.make_ascii_lowercase();

    let mut result = match_cities(&json[lang].as_object().unwrap(), &input)?;
    if result.is_empty() {
        result = match_cities(&json["EN"].as_object().unwrap(), &input)?;
    }

    let response = Response::builder(200).body(result).build();

    Ok(response)
}

fn match_cities(
    cities: &serde_json::Map<String, serde_json::Value>,
    input: &Search,
) -> tide::Result<String> {
    let mut result = String::new();
    let mut count = 0;

    for (key, value) in cities {
        let city_name = value.as_str().unwrap();
        if city_name.to_lowercase().contains(&input.city_dummy) {
            result.push_str(format!("<p class='{}'>{}</p>", key, city_name).as_str());

            count += 1;
            if count >= 3 {
                break;
            }
        }
    }
    Ok(result)
}
