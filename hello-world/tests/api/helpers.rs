use once_cell::sync::Lazy;
use sqlx::Executor;
use sqlx::{Connection, PgConnection, PgPool};
use uuid::Uuid;
use zero2prod::configuration::get_configuration;
use zero2prod::configuration::DatabaseSettings;
use zero2prod::startup::get_connection_pool;
use zero2prod::startup::Application;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

use wiremock::MockServer;

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "debug".to_string();
    let subscriber_name = "test".to_string();

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    };
});

pub struct TestApp {
    pub port: u16,
    pub address: String,
    pub db_pool: PgPool,
    pub email_server: MockServer,
}

impl TestApp {
    pub async fn post_subscriptions(&self, body: String) -> reqwest::Response {
        reqwest::Client::new()
            .post(&format!("{}/subscriptions", &self.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }
}

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let email_server = MockServer::start().await;

    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration.");
        // Use a different database for each test case
        c.database.database_name = Uuid::new_v4().to_string();
        // Use a random OS port
        c.application.port = 0;
        c.email_client.base_url = email_server.uri();
        c
    };

    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build application.");

    // Create and migrate the database
    configure_database(&configuration.database).await;

    let application_port = application.port();
    let address = format!("http://127.0.0.1:{}", application.port());
    let _ = tokio::spawn(application.run_until_stopped());

    TestApp {
        // How do we get these?
        address: address,
        db_pool: get_connection_pool(&configuration.database),
        email_server,
        port: application_port,
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect Postgres");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create Database");

    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect postgres test");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed migrate database");

    connection_pool
}
