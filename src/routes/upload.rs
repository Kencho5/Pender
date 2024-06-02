use crate::imports::*;
use crate::utils::{self, cities::get_city, common::logged_in, upload_struct};
extern crate rusoto_core;
extern crate rusoto_s3;

use rusoto_s3::{PutObjectRequest, S3Client, S3};

pub async fn upload_handler(req: Request<AppState>) -> tide::Result {
    let context = utils::common::get_context(&req).await?;
    if !logged_in(&context).await? {
        return Ok(tide::Redirect::see_other("/login").into());
    }

    let state = req.state();
    let response = state
        .tera
        .render_response("components/upload.html", &context)?;

    Ok(response)
}

pub async fn upload_post_handler(mut req: Request<AppState>) -> tide::Result {
    let post_id = Uuid::new_v4();
    let mut response = Response::builder(200).build();

    let mut form_data: upload_struct::UploadForm = req.body_json().await?;

    let region = rusoto_core::Region::EuCentral1;
    let s3_client = S3Client::new(region);
    for (index, photo) in form_data.photos.iter().enumerate() {
        s3_client
            .put_object(PutObjectRequest {
                bucket: String::from("pender-assets"),
                key: format!("post-images/{}/{}.jpg", post_id, index),
                body: Some(image_base64::from_base64(photo.to_string()).into()),
                ..Default::default()
            })
            .await?;
    }

    form_data.city = get_city().await.unwrap()["GEO"][form_data.city].to_string();

    let mut pg_conn = req.sqlx_conn::<Postgres>().await;
    if !insert_post(&mut pg_conn, post_id, &form_data).await {
        response.set_body(
            r#"
            Failed to upload
            "#,
        );
        return Ok(response);
    }

    response.set_body(json!({
        "post_id": post_id.to_string()
    }));
    Ok(response)
}

async fn insert_post(
    pg_conn: &mut sqlx::PgConnection,
    post_id: Uuid,
    post: &upload_struct::UploadForm,
) -> bool {
    let insert_result = sqlx::query(
        "INSERT INTO posts(id, user_id, animal, breed, post_type, price, age_type, age, sex, phone, city, description) VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)",
    )
    .bind(post_id.to_string())
    .bind(&post.user_id)
    .bind(&post.animal)
    .bind(&post.breed)
    .bind(&post.post_type)
    .bind(&post.price)
    .bind(&post.age_type)
    .bind(&post.age)
    .bind(&post.sex)
    .bind(&post.phone)
    .bind(&post.city)
    .bind(&post.description)
    .execute(pg_conn)
    .await;

    insert_result.is_ok()
}
