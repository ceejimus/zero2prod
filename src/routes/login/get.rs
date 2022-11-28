use actix_web::{http::header::ContentType, HttpResponse};
use actix_web_flash_messages::{IncomingFlashMessages, Level};

pub async fn login_form(flash_messages: IncomingFlashMessages) -> HttpResponse {
    let error_html = match flash_messages
        .iter()
        .filter(|m| m.level() == Level::Error)
        .map(|m| format!("<p><i>{}</i></p>", m.content()))
        .next()
    {
        Some(err) => err,
        None => String::new(),
    };

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
            {error_html}
            <form action="/login" method="post">
              <label
                ><input type="text" placeholder="Enter Username" name="username"
              /></label>
              <label
                ><input type="password" placeholder="Enter Password" name="password"
              /></label>
              <button type="submit">Login</button>
            </form>
          </body>
        </html>"#
        ))
}
