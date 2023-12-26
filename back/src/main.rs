use tide::{Request, Response, StatusCode, security::CorsMiddleware};
use serde_json::json;

async fn test_handler(_: Request<()>) -> tide::Result {
    let json_response = json!({
        "message": "This is a test JSON response from /api/test route"
    });

    Ok(Response::builder(StatusCode::Ok)
        .body(json_response)
        .content_type(tide::http::mime::JSON)
        .build())
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();

    // Configure CORS middleware
    let cors = CorsMiddleware::new()
        .allow_methods("GET, POST, OPTIONS".parse::<tide::http::headers::HeaderValue>().unwrap())
        .allow_origin(tide::security::Origin::from("*")) // Allow all origins
        .allow_credentials(false);

    // Add the CORS middleware to the application
    app.with(cors);

    app.at("/api/test").post(test_handler);

    app.listen("127.0.0.1:3000").await?;
    Ok(())
}
