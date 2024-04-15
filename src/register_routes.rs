use crate::imports::*;
use crate::routes::*;
use crate::utils::*;

pub fn register_routes(app: &mut Server<AppState>) {
    app.at("/")
        .with(GovernorMiddleware::per_hour(240).unwrap())
        .get(home::home_handler);

    app.at("/login")
        .get(login::login_handler)
        .with(GovernorMiddleware::per_minute(5).unwrap())
        .post(login::login_post_handler);

    app.at("/register")
        .get(register::register_handler)
        .with(GovernorMiddleware::per_minute(5).unwrap())
        .post(register::register_post_handler);

    app.at("/profile")
        .with(GovernorMiddleware::per_hour(120).unwrap())
        .get(profile::profile_handler);

    // API ROUTES
    app.at("/api/set_language/:lang")
        .with(GovernorMiddleware::per_minute(20).unwrap())
        .post(language::set_language_handler);

    app.at("/api/get-cities")
        .with(GovernorMiddleware::per_minute(5).unwrap())
        .get(cities::get_cities);
}
