use crate::imports::*;

#[derive(Deserialize)]
pub struct LoginData {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    // sub: String,
    pub email: String,
    // uid: u64,
    pub exp: usize,
}
