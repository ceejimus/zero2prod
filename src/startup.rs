use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::web;
use actix_web::App;
use actix_web::HttpServer;
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;

use crate::routes::*;
// use crate::routes::health_check::*;
// use crate::routes::subscriptions::*;

pub fn run(listener: TcpListener, db_pool: PgPool) -> std::io::Result<Server> {
    let db_pool = web::Data::new(db_pool); // this is just a fancy Arc
                                           // HttpServer handles all transport-level concerns (port binding, TLS, connections, etc.)
    let server = HttpServer::new(move || {
        // App handles logic (routing, request handling, etc.)
        App::new()
            .wrap(TracingLogger::default())
            // .route("/", Route::new().guard(Guard::get()).to(_))
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_pool.clone())
    })
    // .bind(address)? // we can have the server create a listener for us
    .listen(listener)?
    .run();

    Ok(server)
}
