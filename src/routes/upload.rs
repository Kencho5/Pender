use crate::config::config_manager::Config;
use crate::imports::*;
use crate::utils::{self, cities::get_city, common::logged_in, upload_struct};
use image_base64::from_base64;
use std::env;
use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};

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
    let config = &req.state().config;

    fs::create_dir(format!("/var/uploads/post-images/{}", post_id))?;

    for (index, photo) in form_data.photos.iter().enumerate() {
        let image_data = from_base64(photo.to_string());
        let input_path = format!("/var/uploads/post-images/{}/{}.jpg", post_id, index);
        let scale_filter = "scale=iw*0.8:ih*0.8";

        let ffmpeg = Command::new("ffmpeg")
            .args(&["-i", "pipe:0", "-vf", scale_filter, &input_path])
            .stdin(Stdio::piped())
            .spawn();

        if let Some(mut stdin) = ffmpeg.unwrap().stdin.take() {
            stdin.write_all(&image_data)?;
        }
    }
    upload_images(&post_id, config).await?;

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

async fn upload_images(post_id: &String, config: &Config) -> tide::Result<()> {
    env::set_var("AWS_ACCESS_KEY_ID", &config.aws.access_key_id);
    env::set_var("AWS_SECRET_ACCESS_KEY", &config.aws.secret_access_key);

    let output = Command::new("aws")
        .arg("s3")
        .arg("sync")
        .arg(format!("/var/uploads/post-images/{}", post_id))
        .arg(format!("s3://pender-assets/post-images/{}", post_id))
        .arg("--cache-control")
        .arg("max-age=31536000")
        .output()
        .expect("Failed to execute script");
    println!("{:?}", output);

    Ok(())
}
