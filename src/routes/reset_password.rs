use crate::routes::login;
use crate::{imports::*, utils};
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use rand::Rng;

pub async fn reset_password_handler(req: Request<AppState>) -> tide::Result {
    let context = utils::common::get_context(&req).await?;

    let state = req.state();
    let response = state
        .tera
        .render_response("reset_password.html", &context)?;

    Ok(response)
}

pub async fn reset_post_handler(mut req: Request<AppState>) -> tide::Result {
    let user: auth_struct::ResetData = req.body_form().await?;
    let mut response = Response::builder(200).build();

    if login::find_user(&mut req, &user.email).await.is_err() {
        response.set_body(r#"<p class='error'>Email not found</p>"#);
        return Ok(response);
    }

    let mut rng = rand::thread_rng();
    let code = rng.gen_range(1000..9999);

    let email_html = req
        .state()
        .tera
        .render("components/email.html", &context! { "code" => code })?;

    let email = Message::builder()
        .from("Pender <support@pender.ge>".parse().unwrap())
        .to(user.email.parse().unwrap())
        .subject("Password Reset")
        .header(ContentType::TEXT_HTML)
        .body(email_html)
        .unwrap();

    let config = &req.state().config;
    let creds = Credentials::new(
        config.smtp.username.to_owned(),
        config.smtp.password.to_owned(),
    );

    let mailer = SmtpTransport::relay("email-smtp.eu-central-1.amazonaws.com")
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => {
            let session = req.session_mut();
            session.insert("code", code.to_string())?;
            session.insert("email", user.email)?;

            response.set_body(r#"<p class='success'>Code Sent!</p>"#);
            Ok(response)
        }
        Err(_) => {
            response.set_body(r#"<p class='error'>Couldn't send code</p>"#);
            Ok(response)
        }
    }
}

pub async fn reset_code_handler(mut req: Request<AppState>) -> tide::Result {
    let user: auth_struct::ResetCodeData = req.body_form().await?;
    let mut response = Response::builder(200).build();

    let session = req.session();
    let code = session.get::<String>("code").unwrap();
    let email = session.get::<String>("email").unwrap();

    if code == user.code {
        let password_hash = bcrypt::hash(&user.password)?;
        let mut pg_conn = req.sqlx_conn::<Postgres>().await;

        sqlx::query("UPDATE users SET password = $1 WHERE email = $2")
            .bind(password_hash)
            .bind(email)
            .execute(pg_conn.acquire().await?)
            .await?;

        response.set_body(r#"<p class='success'>Password reset!</p>"#);
        response.insert_header("HX-Location", "/login");
    } else {
        response.set_body(r#"<p class='error'>Wrong code!</p>"#);
    }

    Ok(response)
}
