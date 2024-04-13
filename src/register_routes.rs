use crate::imports::*;
use crate::routes::*;
use crate::utils::*;

pub fn register_routes(app: &mut Server<AppState>) {
    app.at("/").get(home::home_handler);

    app.at("/login")
        .get(login::login_handler)
        .post(login::login_post_handler);

    app.at("/register")
        .get(register::register_handler)
        .post(register::register_post_handler);

    // API ROUTES
    app.at("/api/set_language/:lang")
        .post(language::set_language_handler);

    app.at("/api/get-cities").get(cities::get_cities);
}
