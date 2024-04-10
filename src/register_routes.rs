use crate::imports::*;
use crate::routes::*;
use crate::utils::*;

pub fn register_routes(app: &mut Server<AppState>) {
    app.at("/").get(home::home_handler);
    app.at("/login").get(login::login_handler);
    app.at("/login").post(login::login_post_handler);

    // API ROUTES
    app.at("/api/set_language/:lang")
        .post(language::set_language_handler);
}
