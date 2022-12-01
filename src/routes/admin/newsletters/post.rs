use actix_web::{web, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;

use crate::{
    authentication::UserId, domain::SubscriberEmail, email_client::EmailClient, routes::e500,
};

#[derive(serde::Deserialize)]
pub struct BodyData {
    title: String,
    html_content: String,
    text_content: String,
}

struct ConfirmedSubscriber {
    email: SubscriberEmail,
}

#[tracing::instrument(name = "Publishing newsletter", skip(form, pool, email_client))]
pub async fn publish_newsletter(
    user_id: web::ReqData<UserId>,
    form: web::Form<BodyData>,
    pool: web::Data<PgPool>,
    email_client: web::Data<EmailClient>,
) -> Result<HttpResponse, actix_web::Error> {
    // let user_id = *(user_id.into_inner());
    // tracing::Span::current().record("user_id", &tracing::field::display(&user_id));

    let confirmed_subscribers = get_confirmed_subscribers(&pool).await.map_err(e500)?;

    for subscriber in confirmed_subscribers {
        match subscriber {
            Ok(subscriber) => {
                tracing::info!("Sending email to {}", &subscriber.email);
                email_client
                    .send_email(
                        &subscriber.email,
                        &form.title,
                        &form.html_content,
                        &form.text_content,
                    )
                    .await
                    .with_context(|| {
                        format!("Failed to send newsletter email to {}", &subscriber.email)
                    })
                    .map_err(e500)?;
            }
            Err(error) => {
                tracing::warn!(
                    error.cause_chain = ?error,
                    "Skipping a confirmed subscriber. \
                    Their stored contact details are invalid."
                )
            }
        }
    }

    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(name = "Fetching confirmed subscribers", skip(), fields())]
async fn get_confirmed_subscribers(
    pool: &PgPool,
) -> Result<Vec<Result<ConfirmedSubscriber, anyhow::Error>>, anyhow::Error> {
    let confirmed_subscribers = sqlx::query!(
        r#"
        SELECT email
        FROM subscriptions
        WHERE status = 'confirmed'
        "#
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|r| match SubscriberEmail::parse(r.email) {
        Ok(email) => Ok(ConfirmedSubscriber { email }),
        Err(error) => Err(anyhow::anyhow!(error)),
    })
    .collect();
    Ok(confirmed_subscribers)
}
