use actix_web::HttpResponse;
use actix_web_flash_messages::FlashMessage;

use crate::{
    routes::utils::{e500, see_other},
    session_state::TypedSession,
};

#[tracing::instrument(name = "Logging out", skip(session))]
pub async fn logout(session: TypedSession) -> Result<HttpResponse, actix_web::Error> {
    if session.get_user_id().map_err(e500)?.is_some() {
        session.logout();
        FlashMessage::info("You have successfully logged out.").send();
    }

    Ok(see_other("/login"))
}
