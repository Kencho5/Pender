use crate::imports::*;
use std::env;
use std::fs::read_to_string;
use std::path::Path;
use std::process::Command;
use tide::http::mime;

pub async fn analytics_handler(_req: Request<AppState>) -> tide::Result {
    let script_path = "./report.sh";

    // Get the current directory path
    let current_dir = env::current_dir().expect("Failed to get current directory");

    // Set the HOME environment variable to the current directory
    env::set_var(
        "HOME",
        current_dir
            .to_str()
            .expect("Failed to convert current directory path"),
    );

    // Execute the script using the `sh` command
    let output = Command::new("sh")
        .arg(script_path)
        .output()
        .expect("Failed to execute script");

    // Print the output of the script if needed
    println!("Script output: {}", String::from_utf8_lossy(&output.stdout));

    let response = Response::builder(200)
        .body(read_to_string("./report.html")?)
        .content_type(mime::HTML)
        .build();

    Ok(response)
}
