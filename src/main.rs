mod app_state;
mod config;
mod imports;
mod register_routes;
mod routes;
mod utils;
use crate::imports::*;
use std::fs::File;
use std::io::Read;
use std::process::Command;
use std::str;

#[async_std::main]
async fn main() -> tide::Result<()> {
    // tide::log::start();
    let config = config::config_manager::load_config().expect("Config Error.");
    let output = Command::new("git")
        .args(&["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .expect("Failed to execute command");
    let branch = str::from_utf8(&output.stdout).unwrap().trim();

    let mut tera = Tera::new("./templates/**/*")?;

    tera.autoescape_on(vec!["html"]);
    let translations = utils::translations::load_translations("translations")
        .await
        .unwrap();

    Command::new("bash")
        .arg("./scripts/gen_version.sh")
        .output()
        .expect("Failed to run Bash script");

    let mut file = File::open("./scripts/version.txt")?;
    let mut version = String::new();
    file.read_to_string(&mut version)?;

    let mut app = tide::with_state(AppState {
        tera,
        translations,
        config: config.clone(),
        version,
        branch: branch.to_string(),
    });

    app.with(tide::sessions::SessionMiddleware::new(
        MemoryStore::new(),
        config.tide_secret.as_bytes(),
    ));

    let connection_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.database.username,
        config.database.password,
        config.database.host,
        config.database.port,
        config.database.db_name
    );
    app.with(SQLxMiddleware::<Postgres>::new(&connection_url).await?);

    if config.enviorement == "local" {
        // Serve static files locally during development
        app.at("/assets").serve_dir("./public/assets/")?;
        app.at("/static").serve_dir("./public/static/")?;
        app.at("/post-images")
            .serve_dir("/var/uploads/post-images/")?;
    } else {
        // Serve static files from CloudFront URLs in production
        app.at("/assets/*").get(cloudfront_redirect);
        app.at("/static/*").get(cloudfront_redirect);
        app.at("/post-images/*").get(cloudfront_redirect);
    }

    register_routes::register_routes(&mut app);

    app.listen(format!("127.0.0.1:{}", config.port)).await?;
    Ok(())
}

async fn cloudfront_redirect(req: Request<AppState>) -> tide::Result {
    let path = req.url().path();
    let mut branch = format!("/{}", req.state().branch.clone());
    if path.contains("post-images") {
        branch = "".to_string();
    }

    let cloudfront_url = format!("https://d19qt7p7fni613.cloudfront.net{}", branch);
    let redirect_url = format!("{}{}", cloudfront_url, path);
    // println!("{:?}", redirect_url);

    let mut response = Response::new(302);
    response.insert_header("Location", redirect_url);

    Ok(response)
}
