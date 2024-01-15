use tera::Tera;
use tide::Server;

use crate::api::*;
use crate::routes::*;
use crate::utils::*;

pub fn register_routes(app: &mut Server<Tera>) {
    app.at("/").get(home::home_handler);
    app.at("/profile").get(profile::profile_handler);
    app.at("/search").get(search::search_handler);
    app.at("/upload").get(upload::upload_handler);

    // API ROUTES
    app.at("/api/upload").post(api_upload::api_upload_handler);

    app.at("/api/set_language/:lang")
        .post(language::set_language_handler);
}
