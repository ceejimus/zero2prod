use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::{PgConnection, PgPool};
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub(crate) struct FormData {
    pub email: String,
    pub name: String,
}

pub(crate) async fn subscribe(
    form: web::Form<FormData>,
    db_pool: web::Data<PgPool>,
) -> HttpResponse {
    match sqlx::query!(
        r#"
    INSERT INTO subscriptions (id, email, name, subscribed_at)
    VALUES ($1, $2, $3, $4)
    "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(db_pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError()
        }
    }
    .finish()
}
