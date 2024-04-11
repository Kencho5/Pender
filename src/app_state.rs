use crate::config::config_manager::Config;
use crate::imports::Tera;

#[derive(Clone, Debug)]
pub struct AppState {
    pub tera: Tera,
    pub translations: serde_json::Value,
    pub config: Config,
}
