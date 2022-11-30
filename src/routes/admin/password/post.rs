use actix_web::{web, HttpResponse};
use actix_web_flash_messages::FlashMessage;
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;

use crate::{
    authentication::{validate_credential, Credential, UserId},
    routes::utils::{e500, get_username, see_other},
};

#[derive(serde::Deserialize)]
pub struct FormData {
    current_password: Secret<String>,
    new_password: Secret<String>,
    new_password_check: Secret<String>,
}

pub async fn change_password(
    user_id: web::ReqData<UserId>,
    pool: web::Data<PgPool>,
    form: web::Form<FormData>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = user_id.into_inner();

    if form.new_password.expose_secret() != form.new_password_check.expose_secret() {
        FlashMessage::error(
            "You entered two different new passwords - the field values must match.",
        )
        .send();
        return Ok(see_other("/admin/password"));
    }

    let username = get_username(*user_id, &pool).await.map_err(e500)?;

    let credential = Credential {
        username,
        password: form.0.current_password,
    };

    if let Err(e) = validate_credential(credential, &pool).await {
        return match e {
            crate::authentication::AuthError::InvalidCredentials(_) => {
                FlashMessage::error("You entered an invalid password.").send();
                Ok(see_other("/admin/password"))
            }
            crate::authentication::AuthError::UnexpectedError(_) => Err(e500(e)),
        };
    };

    crate::authentication::change_password(*user_id, form.0.new_password, &pool)
        .await
        .map_err(e500)?;
    FlashMessage::info("Your password has been changed.").send();
    Ok(see_other("/admin/password"))
}
