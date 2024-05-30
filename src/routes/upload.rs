use crate::imports::*;
use crate::utils::{self, cities::get_city, common::logged_in, upload_struct};
use std::fs::File;
use std::io::Write;
use std::process::Command;

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

    for (index, photo) in form_data.photos.iter().enumerate() {
        if let Err(_) = save_images(post_id, photo, index) {
            response.set_body(json!({
                "error": r#"Failed to upload photos"#
            }));
            return Ok(response);
        }

        if index == form_data.photos.len() - 1 {
            let input_path = format!("/var/uploads/post-images/{}/0.jpg", post_id);
            let output_path = format!("/var/uploads/post-images/{}/mini.jpg", post_id);
            let scale_filter = "scale=iw*0.3:ih*0.3";

            let output = Command::new("ffmpeg")
                .args(&["-i", &input_path, "-vf", scale_filter, &output_path])
                .output()
                .expect("Failed to execute ffmpeg command");

            if !output.status.success() {
                eprintln!("ffmpeg command failed with output: {:?}", output);
            }
        }
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

fn save_images(post_id: Uuid, photo: &String, index: usize) -> Result<(), std::io::Error> {
    let image = image_base64::from_base64(photo.to_string());

    let post_path = format!("/var/uploads/post-images/{}/", post_id);
    let img_path = format!("{}{}.jpg", post_path, index);

    std::fs::create_dir_all(&post_path)?;

    let mut file = File::create(&img_path)?;
    file.write_all(&image)?;

    Ok(())
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
