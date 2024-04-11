use serde::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
    pub tide_secret: String,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub db_name: String,
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let mut file = File::open("src/config/config.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let config: Config = serde_json::from_str(&contents)?;

    Ok(config)
}
