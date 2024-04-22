use crate::{imports::*, utils};

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
    let mut response = Response::builder(200)
        .body(
            r#"
            <p class='success'>
                <i class="fa-solid fa-circle-check"></i>
                Code Sent!
            </p>
            "#,
        )
        .build();

    Ok(response)
}
