use crate::imports::*;
use std::process::Command;

pub async fn analytics_handler(req: Request<AppState>) -> tide::Result {
    Command::new("sudo")
        .arg("goaccess")
        .args(&[
            "/var/log/nginx/access.log",
            "-o",
            "/tmp/report.html",
            "--log-format=COMBINED",
        ])
        .output()
        .expect("Failed to execute goaccess command");

    let state = req.state();
    let response = state
        .tera
        .render_response("/tmp/report.html", &context! {})?;

    Ok(response)
}
