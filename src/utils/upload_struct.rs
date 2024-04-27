#[derive(serde::Deserialize)]
pub struct UploadForm {
    pub photos: Vec<String>,
    pub animal: String,
    pub breed: String,
    pub post_type: String,
    pub price: String,
    pub age_years: String,
    pub age_months: String,
    pub gender: String,
    pub phone: String,
    pub description: String,
}

#[derive(serde::Deserialize, serde::Serialize, sqlx::FromRow)]
pub struct PostStruct {
    pub animal: String,
    pub breed: String,
    pub post_type: String,
    pub price: String,
    pub age_years: String,
    pub age_months: String,
    pub gender: String,
    pub phone: String,
    pub description: String,
}
