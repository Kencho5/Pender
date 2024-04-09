use crate::imports::*;
use crate::routes::*;
use crate::utils::*;

pub fn register_routes(app: &mut Server<AppState>) {
    app.at("/").get(home::home_handler);
    app.at("/api/set_language/:lang")
        .post(language::set_language_handler);
}
