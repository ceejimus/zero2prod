use anyhow::Context;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;

use crate::telemetry::spawn_blocking_with_tracing;

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials.")]
    InvalidCredentials(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}
pub struct Credential {
    pub username: String,
    pub password: Secret<String>,
}

#[tracing::instrument(name = "Validating user credential.", skip(credential, pool))]
pub async fn validate_credential(
    credential: Credential,
    pool: &PgPool,
) -> Result<uuid::Uuid, AuthError> {
    let mut user_id = None;
    let mut expected_password_hash = Secret::new(
        "$argon2id\
        $v=19\
        $m=4096,t=3,p=1\
        $FgMACLYouoP8Z5A3h7Ak2g\
        $NtAv9R1eorR2bns060FnMldSvdKReUmf3koNNeIwOaQ"
            .to_string(),
    );

    if let Some((stored_user_id, stored_password_hash)) =
        get_stored_credentials(&credential.username, pool)
            .await
            .map_err(AuthError::UnexpectedError)?
    {
        user_id = Some(stored_user_id);
        expected_password_hash = stored_password_hash;
    }

    spawn_blocking_with_tracing(move || {
        verify_password_hash(expected_password_hash, credential.password)
    })
    .await
    .context("Failed to spawn blocking task.")??;

    user_id.ok_or_else(|| AuthError::InvalidCredentials(anyhow::anyhow!("Unknown username")))
}

#[tracing::instrument(
    name = "Verifying password hash",
    skip(expected_password_hash, provided_password)
)]
fn verify_password_hash(
    expected_password_hash: Secret<String>,
    provided_password: Secret<String>,
) -> Result<(), AuthError> {
    let expected_password_hash = PasswordHash::new(expected_password_hash.expose_secret())
        .context("Failed to parse PHC string format.")?;

    Argon2::default()
        .verify_password(
            provided_password.expose_secret().as_bytes(),
            &expected_password_hash,
        )
        .context("Failed to verify password")
        .map_err(AuthError::InvalidCredentials)
}

#[tracing::instrument(name = "Retrieving stored credential", skip(username, pool))]
async fn get_stored_credentials(
    username: &str,
    pool: &PgPool,
) -> Result<Option<(uuid::Uuid, Secret<String>)>, anyhow::Error> {
    let row: Option<_> = sqlx::query!(
        r#"
        SELECT user_id, password_hash
        FROM users
        WHERE username = $1
        "#,
        username,
    )
    .fetch_optional(pool)
    .await
    .context("Failed query authentication credential.")?
    .map(|row| (row.user_id, Secret::new(row.password_hash)));

    Ok(row)
}
