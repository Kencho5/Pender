use tide::{Request, Response, StatusCode};
use tera::Tera;
use tide_tera::prelude::*;
use std::fs;
use tide::sessions::{Session, MemoryStore};
use serde_json::Value;
use std::error::Error;

mod routes {
    pub mod home;
}

mod utils {
    pub mod translations;
    pub mod language;
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    // tide::log::start();

    let mut tera = Tera::new("./front/templates/**/*")?;
    tera.autoescape_on(vec!["html"]);

    let mut app = tide::with_state(tera);
    app.with(tide::sessions::SessionMiddleware::new(MemoryStore::new(), b"we recommend you use std::env::var(\"TIDE_SECRET\").unwrap().as_bytes() instead of a fixed value"));

    app.at("/assets").serve_dir("./front/assets/")?;
    app.at("/static").serve_dir("./front/static/")?;

    app.at("/").get(routes::home::home_handler);

    app.at("/set_language/:lang").get(utils::language::set_language_handler);


    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

