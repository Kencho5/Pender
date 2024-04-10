use crate::imports::*;

#[derive(Deserialize)]
pub struct LoginData {
    pub email: String,
    pub password: String,
}
