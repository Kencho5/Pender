use crate::imports::*;
use std::fs::read_to_string;
use std::process::Command;
use tide::http::mime;

pub async fn analytics_handler(_req: Request<AppState>) -> tide::Result {
    let now = chrono::Utc::now();
    let seven_days_ago = now - chrono::Duration::days(7);

    Command::new("sudo")
        .arg("goaccess")
        .args(&[
            "/var/log/nginx/access.log",
            "-o",
            "./report.html",
            "--log-format=COMBINED",
            "--date-from=&",
            &seven_days_ago.format("%Y-%m-%d").to_string(),
            "--date-to=&",
            &now.format("%Y-%m-%d").to_string(),
        ])
        .output()
        .expect("Failed to execute command");

    let response = Response::builder(200)
        .body(read_to_string("./report.html")?)
        .content_type(mime::HTML)
        .build();

    Ok(response)
}
