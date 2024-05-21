use crate::routes::login::generate_token;
use crate::{
    imports::*,
    utils::{self, common::logged_in},
};

pub async fn profile_handler(req: Request<AppState>) -> tide::Result {
    let context = utils::common::get_context(&req).await?;

    if !logged_in(&context).await? {
        return Ok(tide::Redirect::see_other("/login").into());
    }

    let state = req.state();
    let response = state.tera.render_response("profile.html", &context)?;

    Ok(response)
}

pub async fn edit_profile(mut req: Request<AppState>) -> tide::Result {
    let session = req.session();
    let lang = session.get::<String>("lang").unwrap_or("GEO".into());

    let user: auth_struct::UserStruct = req.body_form().await?;
    let mut response = Response::builder(200)
        .body(
            r#"
                <i class="fa-solid fa-circle-check"></i>
                Saved!
            "#,
        )
        .build();

    if let Err(_) = edit_details(&mut req, &user).await {
        response.set_body(
            r#"
                <i class="fa-solid fa-circle-exclamation"></i>
                Failed To save.
            "#,
        );
        return Ok(response);
    }

    if let Some(token) = generate_token(&req.state().config, &user, lang).await? {
        response.insert_cookie(
            Cookie::build("_jwt", token)
                .max_age(time::Duration::days(7))
                .same_site(tide::http::cookies::SameSite::Lax)
                .path("/")
                .finish(),
        );
        response.set_body(
            r#"
                <i class="fa-solid fa-circle-check"></i>
                Saved!
            "#,
        );
    }

    Ok(response)
}

async fn edit_details(
    req: &mut Request<AppState>,
    user: &auth_struct::UserStruct,
) -> tide::Result<sqlx::postgres::PgQueryResult> {
    let mut pg_conn = req.sqlx_conn::<Postgres>().await;
    let user =
        sqlx::query("UPDATE users SET name = $1, email = $2, phone = $3, city = $4, facebook = $5 WHERE id = $6")
            .bind(&user.name)
            .bind(&user.email)
            .bind(&user.phone)
            .bind(&user.city)
            .bind(&user.facebook)
            .bind(&user.id)
            .execute(pg_conn.acquire().await?)
            .await?;

    Ok(user)
}
