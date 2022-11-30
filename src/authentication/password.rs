use anyhow::Context;
use argon2::{
    password_hash::SaltString, Algorithm, Argon2, Params, PasswordHash, PasswordHasher,
    PasswordVerifier, Version,
};
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;
use uuid::Uuid;

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

#[tracing::instrument(name = "Changing password", skip())]
pub async fn change_password(
    user_id: Uuid,
    password: Secret<String>,
    pool: &PgPool,
) -> Result<(), anyhow::Error> {
    let password_hash = spawn_blocking_with_tracing(move || compute_password_hash(password))
        .await?
        .context("Feild to hash password.")?;

    sqlx::query!(
        r#"
        UPDATE users
        SET password_hash = $1
        WHERE user_id = $2
        "#,
        password_hash.expose_secret(),
        user_id,
    )
    .execute(pool)
    .await
    .expect("Failed to set new password in database.");

    Ok(())
}

fn compute_password_hash(password: Secret<String>) -> Result<Secret<String>, anyhow::Error> {
    let salt = SaltString::generate(rand::thread_rng());
    let password_hash = make_argon()
        .hash_password(password.expose_secret().as_bytes(), &salt)
        .expect("Failed to generate password hash")
        .to_string();
    Ok(Secret::new(password_hash))
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

    make_argon()
        .verify_password(
            provided_password.expose_secret().as_bytes(),
            &expected_password_hash,
        )
        .context("Failed to verify password")
        .map_err(AuthError::InvalidCredentials)
}

fn make_argon() -> Argon2<'static> {
    Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(15000, 2, 1, None).unwrap(),
    )
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
