use crate::imports::*;
use crate::routes::*;
use crate::utils::*;

pub fn register_routes(app: &mut Server<AppState>) {
    app.at("/")
        .with(GovernorMiddleware::per_hour(240).unwrap())
        .get(home::home_handler);

    app.at("/profile")
        .with(GovernorMiddleware::per_hour(240).unwrap())
        .get(profile::profile_handler);

    app.at("/upload")
        .get(upload::upload_handler)
        .with(GovernorMiddleware::per_minute(5).unwrap())
        .post(upload::upload_post_handler);

    // AUTH ROUTES
    app.at("/login")
        .get(login::login_handler)
        .with(GovernorMiddleware::per_minute(5).unwrap())
        .post(login::login_post_handler);

    app.at("/register")
        .get(register::register_handler)
        .with(GovernorMiddleware::per_minute(5).unwrap())
        .post(register::register_post_handler);

    app.at("/reset-password")
        .get(reset_password::reset_password_handler)
        .with(GovernorMiddleware::per_minute(240).unwrap());

    app.at("/logout")
        .post(logout::logout_handler)
        .with(GovernorMiddleware::per_hour(120).unwrap());
    //////////////

    // API ROUTES
    app.at("/api/set_language/:lang")
        .with(GovernorMiddleware::per_minute(20).unwrap())
        .post(language::set_language_handler);

    app.at("/api/get-cities")
        .with(GovernorMiddleware::per_minute(100).unwrap())
        .get(cities::get_cities);

    app.at("/api/edit-profile")
        .with(GovernorMiddleware::per_minute(5).unwrap())
        .post(profile::edit_profile);

    app.at("/api/reset-password")
        .with(GovernorMiddleware::per_minute(15).unwrap())
        .post(reset_password::reset_post_handler);

    app.at("/api/confirm-code")
        .with(GovernorMiddleware::per_minute(15).unwrap())
        .post(reset_password::reset_code_handler);
}
