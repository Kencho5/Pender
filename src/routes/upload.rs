use crate::imports::*;
use crate::utils::{self, cities::get_city, common::logged_in, upload_struct};
use std::fs;
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
    let post_id = generate_id();
    let mut response = Response::builder(200).build();
    let mut form_data: upload_struct::UploadForm = req.body_json().await?;

    for (index, photo) in form_data.photos.iter().enumerate() {
        let save_result = save_images(&post_id, photo, index).await;
        if let Err(_) = save_result {
            response.set_body(json!({
                "error": "Failed to upload photos"
            }));
            return Ok(response);
        }

        let input_path = format!("/var/uploads/post-images/{}/{}.jpg", post_id, index);
        let scale_filter = match fs::metadata(&input_path)?.len() / 1024 {
            size if size <= 500 => "scale=iw*0.8:ih*0.8",
            _ => "scale=iw*0.5:ih*0.5",
        };

        let output = Command::new("ffmpeg")
            .args(&["-y", "-i", &input_path, "-vf", scale_filter, &input_path])
            .output()
            .expect("Failed to execute ffmpeg command");

        if !output.status.success() {
            eprintln!("ffmpeg command failed with output: {:?}", output);
        }
    }
    upload_images(&post_id).await?;

    let city_data = get_city().await.unwrap();
    form_data.city = city_data["GEO"][form_data.city].to_string();

    let mut pg_conn = req.sqlx_conn::<Postgres>().await;
    let insert_result = insert_post(&mut pg_conn, &post_id, &form_data).await;
    if !insert_result {
        response.set_body("Failed to upload");
        return Ok(response);
    }

    response.set_body(json!({
        "post_id": post_id.to_string()
    }));

    Ok(response)
}

async fn save_images(post_id: &String, photo: &String, index: usize) -> Result<(), std::io::Error> {
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
    post_id: &String,
    post: &upload_struct::UploadForm,
) -> bool {
    let insert_result = sqlx::query(
        "INSERT INTO posts(id, user_id, user_name, animal, breed, post_type, price, age_type, age, sex, phone, city, description, photos) VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)",
    )
    .bind(&post_id)
    .bind(&post.user_id)
    .bind(&post.user_name)
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
    .bind(&post.photos.len().to_string())
    .execute(pg_conn)
    .await;

    insert_result.is_ok()
}

async fn upload_images(post_id: &String) -> tide::Result<()> {
    Command::new("aws")
        .arg("s3")
        .arg("cp")
        .arg(format!("/var/uploads/post-images/{}", post_id))
        .arg(format!("s3://pender-assets/post-images/{}", post_id))
        .arg("--recursive")
        .output()
        .expect("Failed to execute script");

    Ok(())
}
