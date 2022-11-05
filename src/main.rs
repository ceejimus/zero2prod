use std::net::TcpListener;

use secrecy::ExposeSecret;
use sqlx::PgPool;
use zero2prod::telemetry::{get_subscriber, init_subscriber};
use zero2prod::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    let configuration = get_configuration().expect("Failed to load config.");
    let connection_pool =
        // PgPool::connect(configuration.database.connection_string().expose_secret())
        PgPool::connect_lazy(configuration.database.connection_string().expose_secret()) // is this still needed
            // .await
            .expect("Failed to connect to Postgres.");
    let address = configuration.application.get_address();
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
