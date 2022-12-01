use actix_web::{http::header::ContentType, HttpResponse};

use crate::{
    routes::{e500, see_other},
    session_state::TypedSession,
};

pub async fn home(session: TypedSession) -> Result<HttpResponse, actix_web::Error> {
    match session.get_user_id().map_err(e500)? {
        Some(_) => Ok(see_other("/admin/dashboard")),
        None => Ok(HttpResponse::Ok()
            .content_type(ContentType::html())
            .body(include_str!("home.html"))),
    }
}
