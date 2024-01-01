use tide::{Request, Response, StatusCode};
use tera::Tera;
use tide_tera::prelude::*;
use std::fs;
use tide::sessions::{Session, MemoryStore};
use serde_json::Value;
use std::error::Error;

#[async_std::main]
async fn main() -> tide::Result<()> {
    // tide::log::start();

    let mut tera = Tera::new("./front/templates/**/*")?;
    tera.autoescape_on(vec!["html"]);

    let mut app = tide::with_state(tera);
    app.with(tide::sessions::SessionMiddleware::new(MemoryStore::new(), b"we recommend you use std::env::var(\"TIDE_SECRET\").unwrap().as_bytes() instead of a fixed value"));

    app.at("/assets").serve_dir("./front/assets/")?;
    app.at("/static").serve_dir("./front/static/")?;

    app.at("/").get(|req: tide::Request<Tera>| async move {
        let tera = req.state();
        let navbar = fs::read_to_string("./front/templates/navbar.html").expect("Not Found");

        let session = req.session();
        let lang = session.get_raw("lang").unwrap_or("en".into());
        let translations = load_translations(&lang).await.unwrap();

        let context = context! {
            "navbar" => &navbar,
            "tr" => &translations, 
        };

        tera.render_response("home.html", &context)
    });

    app.at("/set_language/:lang").get(set_language);


    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

async fn set_language(mut req: Request<Tera>) -> tide::Result {
    let language = req.param("lang").unwrap().to_string();
    let session = req.session_mut();
    session.insert("lang", language)?;

    Ok(Response::new(StatusCode::Ok))
}


async fn load_translations(lang: &str) -> Result<serde_json::Value, Box<dyn Error>> {
    let file_path = format!("./front/assets/translations/{}.json", lang.replace("\"", ""));
    let file = fs::read_to_string(&file_path).expect("Not Found");
    let json: serde_json::Value = serde_json::from_str(&file)?;

    Ok(json)
}

