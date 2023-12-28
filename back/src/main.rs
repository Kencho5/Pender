use tide::Request;
use tide::Response;
use tera::Tera;
use tide_tera::prelude::*;
use std::fs;

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();

    let mut tera = Tera::new("./front/templates/**/*")?;
    tera.autoescape_on(vec!["html"]);

    let mut app = tide::with_state(tera);

    app.at("/assets").serve_dir("./front/assets/")?;
    app.at("/static").serve_dir("./front/static/")?;

    app.at("/").get(|req: tide::Request<Tera>| async move {
        let tera = req.state();
        let navbar = fs::read_to_string("./front/templates/navbar.html").expect("Not Found");

        tera.render_response("home.html", &context! { "navbar" => &navbar})
    });

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
