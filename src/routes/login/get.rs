use actix_web::{http::header::ContentType, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

#[tracing::instrument(name = "Delivering login form", skip(flash_messages))]
pub async fn login_form(flash_messages: IncomingFlashMessages) -> HttpResponse {
    let mut msg_html = String::new();
    for m in flash_messages.iter() {
        writeln!(msg_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!DOCTYPE html>
        <html lang="en">
          <head>
            <meta http-equiv="content-type" content="text/html; charset=utf-8" />
            <title>Home</title>
          </head>
          <body>
            {msg_html}
            <form action="/login" method="post">
              <label>
                Username:
                <input type="text" placeholder="Enter Username" name="username"/>
              </label>
              <label>
                Password:
                <input type="password" placeholder="Enter Password" name="password"/>
              </label>
              <button type="submit">Login</button>
            </form>
          </body>
        </html>"#
        ))
}
