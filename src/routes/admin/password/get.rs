use actix_web::{http::header::ContentType, web, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

use crate::authentication::UserId;

#[tracing::instrument(name = "Delivering change password form", skip(flash_messages))]
pub async fn change_password_form(
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
            <title>Change Password</title>
          </head>
          <body>
              {msg_html}
            <form action="/admin/password" method="post">
              <label>
                <input type="password" placeholder="Enter Password" name="password"/>
              </label>
              <button type="submit">Login</button>
            </form>
          </body>
        </html>"#
        )))
}
