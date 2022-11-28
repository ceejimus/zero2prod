use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use once_cell::sync::Lazy;
use reqwest::Url;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use wiremock::MockServer;
use zero2prod::{
    configuration::{get_configuration, DatabaseSettings},
    startup::{get_connection_pool, Application},
    telemetry::{get_subscriber, init_subscriber},
};

static TRACING: Lazy<()> = Lazy::new(|| {
    let subscriber_name = String::from("zero2prod_integration_tests");
    let default_filter_level = String::from("debug");

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

pub struct ConfirmationLinks {
    pub html: reqwest::Url,
    pub plain_text: reqwest::Url,
}

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
    pub email_server: MockServer,
    pub port: u16,
    pub test_user: TestUser,
    pub api_client: reqwest::Client,
}

impl TestApp {
    pub async fn post_login<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.api_client
            .post(&format!("{}/login", self.address))
            .form(body)
            .send()
            .await
            .expect("Failed to send post.")
    }

    pub async fn get_login_html(&self) -> String {
        self.api_client
            .get(&format!("{}/login", &self.address))
            .send()
            .await
            .expect("Failed to make get request")
            .text()
            .await
            .unwrap()
    }

    pub async fn post_subscriptions(&self, body: String) -> reqwest::Response {
        self.api_client
            .post(&format!("{}/subscriptions", &self.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to send post.")
    }

    pub async fn get_confirmation_links(
        &self,
        email_request: &wiremock::Request,
    ) -> ConfirmationLinks {
        let body: serde_json::Value = serde_json::from_slice(&email_request.body).unwrap();
        // let body: SendEmailRequest = serde_json::from_slice(&email_request.body).unwrap();

        let get_link = |s: &str| {
            let links: Vec<_> = linkify::LinkFinder::new()
                .links(s)
                .filter(|l| *l.kind() == linkify::LinkKind::Url)
                .collect();
            links[0].as_str().to_string()
        };

        let html_confirmation_link = &get_link(body["HtmlBody"].as_str().unwrap());
        let text_confirmation_link = &get_link(body["TextBody"].as_str().unwrap());

        let mut html = Url::parse(html_confirmation_link).unwrap();
        let mut plain_text = Url::parse(text_confirmation_link).unwrap();

        html.set_port(Some(self.port))
            .expect("Failed to set URL port.");
        plain_text
            .set_port(Some(self.port))
            .expect("Failed to set URL port.");

        ConfirmationLinks { html, plain_text }
    }

    pub async fn get_admin_dashboard(&self) -> String {
        let response = self
            .api_client
            .get(&format!("{}/admin/dashboard", &self.address))
            .send()
            .await
            .expect("Failed to send get.");

        assert_eq!(response.status().as_u16(), 200);

        response.text().await.unwrap()
    }

    pub async fn post_newsletters(&self, body: serde_json::Value) -> reqwest::Response {
        self.api_client
            .post(&format!("{}/newsletters", &self.address))
            // .basic_auth(test_user.0, Some(test_user.1))
            .basic_auth(&self.test_user.username, Some(&self.test_user.password))
            .json(&body)
            .send()
            .await
            .expect("Failed to send post.")
    }
}

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let email_server = MockServer::start().await;

    let configuration = {
        let mut configuration = get_configuration().expect("Failed to load config.");
        configuration.database.database_name = Uuid::new_v4().to_string();
        configuration.application.port = 0;
        configuration.email_client.base_url = email_server.uri();
        configuration
    };

    configure_database(&configuration.database).await;

    let db_pool = get_connection_pool(&configuration.database);

    let test_user = TestUser::generate();
    test_user.store(&db_pool).await;

    let application = Application::build(configuration)
        .await
        .expect("Failed to build application");
    let port = application.port();
    // I don't like this very much but w/e
    let address = format!("http://127.0.0.1:{}", port);

    let _ = tokio::spawn(application.run_until_stopped());

    let api_client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .cookie_store(true)
        .build()
        .unwrap();

    TestApp {
        address,
        // db_pool: get_connection_pool(&configuration.database),
        db_pool,
        email_server,
        port,
        test_user,
        api_client,
    }
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database");

    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate database");

    connection_pool
}

pub struct TestUser {
    pub user_id: Uuid,
    pub username: String,
    pub password: String,
}

impl TestUser {
    pub fn generate() -> Self {
        Self {
            user_id: uuid::Uuid::new_v4(),
            username: uuid::Uuid::new_v4().to_string(),
            password: uuid::Uuid::new_v4().to_string(),
        }
    }

    async fn store(&self, pool: &PgPool) {
        // let argon2_params =
        // Params::new(15000, 2, 1, None).expect("Failed to create Argon2 params.");
        // let hasher = Argon2::new(
        //     argon2::Algorithm::Argon2id,
        //     argon2::Version::V0x13,
        //     argon2_params,
        // );

        let salt = SaltString::generate(rand::thread_rng());
        let password_hash = Argon2::default()
            .hash_password(self.password.as_bytes(), &salt)
            .expect("Failed to generate password hash")
            .to_string();

        sqlx::query!(
            r#"
            INSERT INTO users (user_id, username, password_hash)
            VALUES ($1, $2, $3)
            "#,
            self.user_id,
            self.username,
            password_hash,
        )
        .execute(pool)
        .await
        .expect("Failed to insert test user into database.");
    }
}

pub fn assert_is_redirect_to(response: &reqwest::Response, location: &str) {
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(response.headers().get("Location").unwrap(), location);
}
