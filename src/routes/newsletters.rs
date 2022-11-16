use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct BodyData {
    title: String,
    content: Content,
}

#[derive(serde::Deserialize)]
pub struct Content {
    html: String,
    text: String,
}

#[tracing::instrument(
    name = "Publishing new newsletter",
    skip(_body),
    fields(
        newsletter_title = %_body.title,
        newsletter_content_html = %_body.content.html,
        newsletter_content_text = %_body.content.text,
    ),
)]
pub async fn publish_newsletter(_body: web::Json<BodyData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
