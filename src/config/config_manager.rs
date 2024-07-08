use serde::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub database: DatabaseConfig,
    pub smtp: SmtpConfig,
    pub tide_secret: String,
    pub port: String,
    pub aws: AwsConfig,
    pub enviorement: String,
}

#[derive(Deserialize, Clone)]
pub struct DatabaseConfig {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub db_name: String,
}

#[derive(Deserialize, Clone)]
pub struct SmtpConfig {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Clone)]
pub struct AwsConfig {
    pub access_key_id: String,
    pub secret_access_key: String,
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let mut file = File::open("src/config/config.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let config: Config = serde_json::from_str(&contents)?;

    Ok(config)
}
