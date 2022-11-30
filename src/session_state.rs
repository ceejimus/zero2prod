use std::future::{ready, Ready};

use actix_session::{Session, SessionExt};
use actix_web::FromRequest;
use anyhow::Context;
use uuid::Uuid;

pub struct TypedSession(Session);

impl TypedSession {
    const USER_ID_KEY: &'static str = "user_id";

    pub fn renew(&self) {
        self.0.renew();
    }

    pub fn insert_user(&self, user_id: Uuid) -> Result<(), anyhow::Error> {
        self.0
            .insert(Self::USER_ID_KEY, user_id)
            .context("Failed to update session state.")
    }

    pub fn get_user_id(&self) -> Result<Option<Uuid>, anyhow::Error> {
        self.0
            .get(Self::USER_ID_KEY)
            .context("Failed to retrieve session state")
    }

    pub fn logout(&self) {
        self.0.purge()
    }
}

impl FromRequest for TypedSession {
    type Error = <Session as FromRequest>::Error;

    type Future = Ready<Result<TypedSession, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        ready(Ok(TypedSession(req.get_session())))
    }
}
