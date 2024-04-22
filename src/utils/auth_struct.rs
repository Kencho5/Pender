use crate::imports::*;

#[derive(Deserialize)]
pub struct LoginData {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct ResetData {
    pub email: String,
}

#[derive(Deserialize)]
pub struct ResetCodeData {
    pub code: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterData {
    pub email: String,
    pub name: String,
    pub phone: String,
    pub city: String,
    pub facebook: String,
    pub password: String,
}

#[derive(Deserialize, sqlx::FromRow)]
pub struct UserStruct {
    pub id: String,
    pub email: String,
    pub name: String,
    pub phone: String,
    pub city: String,
    pub facebook: String,
    pub password: String,
}
