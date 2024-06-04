#[derive(serde::Deserialize)]
pub struct UploadForm {
    pub user_id: String,
    pub user_name: String,
    pub photos: Vec<String>,
    pub animal: String,
    pub breed: String,
    pub post_type: String,
    pub price: Option<String>,
    pub age_type: String,
    pub age: String,
    pub sex: String,
    pub phone: String,
    pub city: String,
    pub description: String,
}

#[derive(serde::Deserialize, serde::Serialize, sqlx::FromRow)]
pub struct PostStruct {
    pub id: String,
    pub user_id: String,
    pub user_name: String,
    pub animal: String,
    pub breed: String,
    pub post_type: String,
    pub price: Option<String>,
    pub age_type: String,
    pub age: String,
    pub sex: String,
    pub phone: String,
    pub city: String,
    pub description: String,
    pub photos: String,
}
