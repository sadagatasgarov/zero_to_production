use once_cell::sync::Lazy;
use sqlx::Executor;
use sqlx::{Connection, PgConnection, PgPool};
use zero2prod::telemetry::{get_subscriber, init_subscriber};
use std::net::TcpListener;
use uuid::Uuid;
use zero2prod::configuration::get_configuration;
use zero2prod::{configuration::DatabaseSettings, startup::run};

static TRACING: Lazy<()> = Lazy::new(||{
    let subscriber = get_subscriber("test".into(), "debug".into());
    init_subscriber(subscriber);
});

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

// fn drop() {
//     tokio::spawn(async move {
//         let mut connection = PgConnection::connect(&connection_string)
//             .await
//             .expect("Failed to connect to Postgres");
//         sqlx::query(&format!(
//             r#"DROP DATABASE "{}" WITH (FORCE);"#,
//             database_name
//         ))
//         .execute(&mut connection)
//         .await
//         .expect("Failed to drop database");
//     });
// }

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);    
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind address");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let mut configuration = get_configuration().expect("Failed to read configuration");
    configuration.database.database_name = Uuid::new_v4().to_string();

    let connection_pool = configure_database(&configuration.database).await;

    let server = run(listener, connection_pool.clone()).expect("failed to bind address");

    let _ = tokio::spawn(server);

    TestApp {
        address,
        db_pool: connection_pool,
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect Postgres");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create Database");

    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect postgres test");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed migrate database");

    connection_pool
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let body = "name=Sadagat%20Asgarov&email=sadagatasgarov%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscriptions");

    assert_eq!(saved.email, "sadagatasgarov@gmail.com");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=Sadagat%20Asgarov", "missing the email"),
        ("email=sadagat.asgarov@gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execcute request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad request whwn the payload was {}",
            error_message
        )
    }
}
