#[macro_use]
extern crate rocket;
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;
use std::fs;

#[get("/")]
fn index() -> Template {
    let navbar = fs::read_to_string("../front/navbar.html.hbs").expect("Not Found");
    let context = ("navbar", navbar);
    Template::render("home", &context)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/static", FileServer::from("../front/static"))
        .attach(Template::fairing())
}
