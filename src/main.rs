use tera::Tera;
use tide::sessions::MemoryStore;

mod register_routes;

mod routes {
    pub mod home;
    pub mod login;
    pub mod profile;
    pub mod register;
    pub mod search;
    pub mod upload;
}

mod api {
    pub mod api_upload;
}

mod utils {
    pub mod common;
    pub mod language;
    pub mod translations;
}

mod components {}

#[async_std::main]
async fn main() -> tide::Result<()> {
    // tide::log::start();

    let mut tera = Tera::new("./front/templates/**/*")?;
    tera.autoescape_on(vec!["html"]);

    let mut app = tide::with_state(tera);
    let secret_key =
        std::env::var("TIDE_SECRET").expect("TIDE_SECRET environment variable not set");
    app.with(tide::sessions::SessionMiddleware::new(
        MemoryStore::new(),
        secret_key.as_bytes(),
    ));

    app.at("/assets").serve_dir("./front/assets/")?;
    app.at("/static").serve_dir("./front/static/")?;

    register_routes::register_routes(&mut app);

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
