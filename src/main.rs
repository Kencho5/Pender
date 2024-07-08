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
        content_url: format!("https://d19qt7p7fni613.cloudfront.net/{}", branch),
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

    app.at("/assets").serve_dir("./public/assets/")?;
    app.at("/static").serve_dir("./public/static/")?;
    app.at("/post-images")
        .serve_dir("/var/uploads/post-images/")?;

    register_routes::register_routes(&mut app);

    app.listen(format!("127.0.0.1:{}", config.port)).await?;
    Ok(())
}
