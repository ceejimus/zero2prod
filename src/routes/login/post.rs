use actix_session::Session;
use actix_web::{error::InternalError, web, HttpResponse};

use actix_web_flash_messages::FlashMessage;
use reqwest::header::LOCATION;
use secrecy::Secret;
use sqlx::PgPool;

use crate::{
    authentication::{validate_credential, Credential},
    routes::utils::error_chain_fmt,
};

#[derive(serde::Deserialize)]
pub struct FormData {
    username: String,
    password: Secret<String>,
}

#[derive(thiserror::Error)]
pub enum LoginError {
    #[error("Authentication Failed")]
    AuthError(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

// impl ResponseError for LoginError {
//     fn status_code(&self) -> reqwest::StatusCode {
//         // match self {
//         // LoginError::AuthError(_) => reqwest::StatusCode::UNAUTHORIZED,
//         // LoginError::UnexpectedError(_) => reqwest::StatusCode::INTERNAL_SERVER_ERROR,
//         // }
//         reqwest::StatusCode::SEE_OTHER
//     }

//     fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {}
// }

#[tracing::instrument(name = "Processing login request", skip(form, pool, session))]
pub async fn login(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
    session: Session,
) -> Result<HttpResponse, InternalError<LoginError>> {
    let credential = Credential {
        username: form.0.username,
        password: form.0.password,
    };

    match validate_credential(credential, &pool).await {
        Ok(user_id) => {
            tracing::Span::current().record("user_id", &tracing::field::display(&user_id));

            session.renew();
            session
                .insert("user_id", user_id)
                .map_err(|e| login_redirect(LoginError::UnexpectedError(e.into())))?;

            Ok(HttpResponse::SeeOther()
                .insert_header((LOCATION, "/admin/dashboard"))
                .finish())
        }
        Err(e) => {
            let e = match e {
                crate::authentication::AuthError::InvalidCredentials(_) => {
                    LoginError::AuthError(e.into())
                }
                crate::authentication::AuthError::UnexpectedError(_) => {
                    LoginError::UnexpectedError(e.into())
                }
            };

            Err(login_redirect(e))
        }
    }
}

fn login_redirect(e: LoginError) -> InternalError<LoginError> {
    FlashMessage::error(e.to_string()).send();

    let response = HttpResponse::SeeOther()
        .insert_header((LOCATION, "/login"))
        .finish();
    InternalError::from_response(e, response)
}
