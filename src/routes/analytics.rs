use crate::imports::*;
use std::fs::read_to_string;
use std::process::Command;
use tide::http::mime;

pub async fn analytics_handler(_req: Request<AppState>) -> tide::Result {
    Command::new("bash")
        .arg("./report.sh")
        .output()
        .expect("Failed to execute script");

    let response = Response::builder(200)
        .body(read_to_string("./report.html")?)
        .content_type(mime::HTML)
        .build();

    Ok(response)
}
