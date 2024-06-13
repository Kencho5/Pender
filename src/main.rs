mod app_state;
mod config;
mod imports;
mod register_routes;
mod routes;
mod utils;
use crate::imports::*;
use std::fs;

#[async_std::main]
async fn main() -> tide::Result<()> {
    // tide::log::start();
    let config = config::config_manager::load_config().expect("Config Error.");

    let mut tera = Tera::new("./templates/**/*")?;

    let css_dir = "./public/static/css/";
    for entry in fs::read_dir(css_dir)? {
        let path = entry?.path();

        let css_content = fs::read_to_string(&path)?;
        let relative_path = path.strip_prefix(css_dir)?.to_string_lossy().to_string();

        tera.add_raw_templates(vec![(relative_path, &css_content)])?;
    }

    tera.autoescape_on(vec!["html"]);
    let translations = utils::translations::load_translations("translations")
        .await
        .unwrap();

    let mut app = tide::with_state(AppState {
        tera,
        translations,
        config: config.clone(),
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
    app.at("/fonts").serve_dir("./public/fonts/")?;
    app.at("/var/uploads/post-images")
        .serve_dir("/var/uploads/post-images/")?;

    register_routes::register_routes(&mut app);

    app.listen(format!("127.0.0.1:{}", config.port)).await?;
    Ok(())
}
