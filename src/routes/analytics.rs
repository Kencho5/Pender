use crate::imports::*;
use std::fs::read_to_string;
use std::process::Command;
use tide::http::mime;

pub async fn analytics_handler(_req: Request<AppState>) -> tide::Result {
    Command::new("sudo")
        .arg("goaccess")
        .arg("`ls")
        .arg("/var/log/nginx/access.log*")
        .arg("|")
        .arg("grep")
        .arg("-v")
        .arg("'.gz'")
        .arg("`")
        .arg("-a")
        .arg("-o")
        .arg("report.html")
        .arg("--log-format=COMBINED")
        .output()
        .expect("Failed to execute command");

    let response = Response::builder(200)
        .body(read_to_string("./report.html")?)
        .content_type(mime::HTML)
        .build();

    Ok(response)
}
