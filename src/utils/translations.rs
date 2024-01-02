use std::{error::Error, fs};
use serde_json::Value;

pub async fn load_translations(lang: &str) -> Result<serde_json::Value, Box<dyn Error>> {
    let file_path = format!("./front/assets/translations/{}.json", lang.replace("\"", ""));
    let file = fs::read_to_string(&file_path).expect("Not Found");
    let json: serde_json::Value = serde_json::from_str(&file)?;

    Ok(json)
}

