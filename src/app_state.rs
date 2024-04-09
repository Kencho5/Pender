use crate::imports::Tera;

#[derive(Clone, Debug)]
pub struct AppState {
    pub tera: Tera,
    pub translations: serde_json::Value,
}
