use crate::imports::*;
use std::env;
use std::fs::read;
use std::process::Command;
use tide::http::mime;

pub async fn analytics_handler(_req: Request<AppState>) -> tide::Result {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let current_dir_str = current_dir.to_string_lossy();
    env::set_var("HOME", &*current_dir_str);

    let output = Command::new("bash")
        .arg("scripts/report.sh")
        .output()
        .expect("Failed to execute script");
    println!("{:?}", output);

    match read("./report.html") {
        Ok(body) => {
            let response = Response::builder(200)
                .body(body)
                .content_type(mime::HTML)
                .build();
            Ok(response)
        }
        Err(err) => {
            tide::log::error!("Failed to read report.html: {:?}", err);
            Ok(Response::builder(500).body("Internal Server Error").build())
        }
    }
}
