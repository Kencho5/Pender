use actix_web::{web, App, HttpServer, HttpResponse, Result};
use std::fs;

async fn index() -> Result<HttpResponse> {
    // Read the contents of the HTML file
    let html_content = fs::read_to_string("front/home.html")
        .map_err(|e| {
            println!("Error reading file: {}", e);
            actix_web::error::ErrorInternalServerError("Failed to read file")
        })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(html_content))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
