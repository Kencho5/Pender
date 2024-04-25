use crate::imports::*;
use crate::utils::{self, upload_struct};

use std::fs::File;
use std::io::Write;

pub async fn upload_handler(req: Request<AppState>) -> tide::Result {
    let context = utils::common::get_context(&req).await?;

    let state = req.state();
    let response = state
        .tera
        .render_response("components/upload.html", &context)?;

    Ok(response)
}

pub async fn upload_post_handler(mut req: Request<AppState>) -> tide::Result {
    let post_id = Uuid::new_v4();
    let mut response = Response::builder(200).build();

    let form_data: upload_struct::UploadForm = req.body_json().await?;

    for (index, photo) in form_data.photos.iter().enumerate() {
        println!("{}", index);
        if let Err(error) = save_images(post_id, photo, index) {
            println!("{}", error);

            response.set_body(
                r#"<p class='error'><i class="fa-solid fa-circle-exclamation"></i>Failed to upload photos</p>"#,
            );
            return Ok(response);
        }
    }

    response.set_body("success");
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
