use std::net::TcpListener;

use sqlx::postgres::PgPoolOptions;
use zero2prod::email_client::EmailClient;
use zero2prod::telemetry::{get_subscriber, init_subscriber};
use zero2prod::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to load config.");

    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        // PgPool::connect(configuration.database.connection_string().expose_secret())
        .connect_lazy_with(configuration.database.with_db());

    let sender_email = configuration
        .email_client
        .sender()
        .expect("Invalid sender email address.");
    // interesting subtlety - because the EmailClientSettings authorization_token's type doesn't implement Copy (Secret)
    // whenever we move the value somewhere it results in a "Partial Move" see: https://doc.rust-lang.org/rust-by-example/scope/move/partial_move.html
    // meaning we can access unmoved values but not the moved member of the struct OR the struct as a whole
    // since our `timeout()` method takes &self, we need to make sure we call it BEFORE we partially move it
    let timeout = configuration.email_client.timeout();
    let email_client = EmailClient::new(
        configuration.email_client.base_url,
        sender_email,
        configuration.email_client.authorization_token,
        timeout,
    );

    let address = configuration.application.get_address();
    let listener = TcpListener::bind(address)?;

    run(listener, connection_pool, email_client)?.await
}
