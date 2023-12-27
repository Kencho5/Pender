use tide::Request;
use tide::prelude::*;
use std::fs;
use tide::Response;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();

    app.at("/").get(home);
    app.at("/api/test").post(test);

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

async fn home(_req: Request<()>) -> tide::Result {
    let html = fs::read_to_string("templates/index.html").expect("HTML Not Found");

    Ok(Response::builder(200)
        .body(html)
        .content_type("text/html")
        .build())
}

async fn test(_req: Request<()>) -> tide::Result {
    Ok(Response::builder(200)
        .body("asd")
        .build())
}
