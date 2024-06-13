use crate::imports::*;
use std::env;
use std::fs::read_to_string;
use std::process::Command;
use tide::http::mime;

pub async fn analytics_handler(_req: Request<AppState>) -> tide::Result {
    env::set_var("HOME", env::current_dir().unwrap());

    let command = "zcat /var/log/nginx/access.log*.gz ; cat /var/log/nginx/access.log* | goaccess -o ./report.html --log-format=COMBINED";

    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .spawn()
        .expect("Failed to execute command");
    println!("{:?}", output);

    let response = Response::builder(200)
        .body(read_to_string("./report.html")?)
        .content_type(mime::HTML)
        .build();

    Ok(response)
}
