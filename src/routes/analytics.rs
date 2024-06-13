use crate::imports::*;
use std::fs::read_to_string;
use std::process::Command;
use tide::http::mime;

pub async fn analytics_handler(_req: Request<AppState>) -> tide::Result {
    let output = Command::new("sudo")
        .arg("goaccess")
        .arg("-c")
        .args(&[
            r#"(zcat /var/log/nginx/access.log*.gz ; cat /var/log/nginx/access.log*) | goaccess -o report.html --log-format=COMBINED"#,
        ])
        .output()
        .expect("Failed to execute goaccess command");

    println!("{:?}", output);

    let response = Response::builder(200)
        .body(read_to_string("./report.html")?)
        .content_type(mime::HTML)
        .build();

    Ok(response)
}
