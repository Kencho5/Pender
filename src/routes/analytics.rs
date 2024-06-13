use crate::imports::*;
use std::fs::read_to_string;
use std::process::Command;
use tide::http::mime;

pub async fn analytics_handler(_req: Request<AppState>) -> tide::Result {
    Command::new("goaccess")
        .arg("/var/log/nginx/access.log*")
        .arg("-o")
        .arg("report.html")
        .arg("--log-format=COMBINED")
        .arg("--load-from-disk")
        .output()
        .expect("Failed to run command");

    let response = Response::builder(200)
        .body(read_to_string("./report.html")?)
        .content_type(mime::HTML)
        .build();

    Ok(response)
}
