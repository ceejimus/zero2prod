use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::web;
use actix_web::App;
use actix_web::HttpServer;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;

use crate::configuration::DatabaseSettings;
use crate::configuration::Settings;
use crate::email_client::EmailClient;
use crate::routes::*;

pub struct Application {
    port: u16,
    server: Server,
}

pub struct ApplicationBaseUrl(pub String);

impl Application {
    pub async fn build(configuration: &Settings) -> Result<Self, std::io::Error> {
        let sender_email = configuration
            .email_client
            .sender()
            .expect("Invalid sender email address.");
        let connection_pool = get_connection_pool(&configuration.database);
        // interesting subtlety - because the EmailClientSettings authorization_token's type doesn't implement Copy (Secret)
        // whenever we move the value somewhere it results in a "Partial Move" see: https://doc.rust-lang.org/rust-by-example/scope/move/partial_move.html
        // meaning we can access unmoved values but not the moved member of the struct OR the struct as a whole
        // since our `timeout()` method takes &self, we need to make sure we call it BEFORE we partially move it
        let timeout = configuration.email_client.timeout();
        let email_client = EmailClient::new(
            configuration.email_client.base_url.clone(),
            sender_email,
            configuration.email_client.authorization_token.clone(),
            timeout,
        );

        let address = configuration.application.get_address();
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();

        let server = run(
            listener,
            connection_pool,
            email_client,
            configuration.application.base_url.clone(),
        )?;

        Ok(Application { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        // PgPool::connect(configuration.database.connection_string().expose_secret())
        .connect_lazy_with(configuration.with_db())
}

pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
    email_client: EmailClient,
    base_url: String,
) -> std::io::Result<Server> {
    let db_pool = web::Data::new(db_pool); // this is just a fancy Arc
    let email_client = web::Data::new(email_client);
    let application_base_url = web::Data::new(ApplicationBaseUrl(base_url));
    // HttpServer handles all transport-level concerns (port binding, TLS, connections, etc.)
    let server = HttpServer::new(move || {
        // App handles logic (routing, request handling, etc.)
        App::new()
            .wrap(TracingLogger::default())
            // .route("/", Route::new().guard(Guard::get()).to(_))
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .route(
                "/subscriptions/confirm",
                web::get().to(confirm_subscription),
            )
            .route("/newsletters", web::post().to(publish_newsletter))
            .app_data(db_pool.clone())
            .app_data(email_client.clone())
            .app_data(application_base_url.clone())
    })
    // .bind(address)? // we can have the server create a listener for us
    .listen(listener)?
    .run();

    Ok(server)
}
