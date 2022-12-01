use actix_web::{http::header::ContentType, web, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

use crate::authentication::UserId;

#[tracing::instrument(name = "Delivering publish newsletter form", skip(flash_messages))]
pub async fn publish_newsletter_form(
    _: web::ReqData<UserId>,
    flash_messages: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    let mut msg_html = String::new();
    for m in flash_messages.iter() {
        writeln!(msg_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!DOCTYPE html>
        <html lang="en">
          <head>
            <meta http-equiv="content-type" content="text/html; charset=utf-8" />
            <title>Post Newsletter</title>
          </head>
          <body>
            {msg_html}
            <form action="/admin/newsletters" method="post">
              <label>Title
                <input type="text" placeholder="New Newsletter" name="title"/>
              </label>
              <label>Text Content
                <input type="text" placeholder="Content" name="text_content"/>
              </label>
              <label>HTML Content
                <input type="text" placeholder="Content" name="html_content"/>
              </label>
              <button type="submit">Post</button>
            </form>
          </body>
        </html>"#
        )))
}
