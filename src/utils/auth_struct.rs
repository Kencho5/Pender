use crate::imports::*;

#[derive(Deserialize)]
pub struct LoginData {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, sqlx::FromRow)]
pub struct RegisterData {
    pub email: String,
    pub name: String,
    pub phone: String,
    pub city: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    // sub: String,
    pub email: String,
    // uid: u64,
    pub exp: usize,
}
