use tide::Request;
use tide::Response;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();

    app.at("/assets").serve_dir("../front/assets/")?;
    app.at("/static").serve_dir("../front/static/")?;

    app.at("/").serve_file("../front/home.html")?;
    app.at("/api/navbar").serve_file("../front/navbar.html")?;

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

// async fn navbar(_req: Request<()>) -> tide::Result {
//     Ok(Response::builder(200)
//         .body("asd")
//         .build())
// }
