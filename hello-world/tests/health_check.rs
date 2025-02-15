
use sqlx::{Connection, PgConnection, PgPool};
use zero2prod::startup::run;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool
}


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
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind address");
    let port = listener.local_addr().unwrap().port();
    let address= format!("http://127.0.0.1:{}", port);

    let configuration = get_configuration().expect("Failed to read configuration");

    let connection_pool = PgPool::connect(
        &configuration.database.connection_string()
    )
    .await
    .expect("Failet to connet Postgres");

    let server = run(listener, connection_pool.clone()).expect("failed to bind address");

    let _ = tokio::spawn(server);

  

    TestApp{
        address,
        db_pool: connection_pool
    }
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

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",).fetch_one(&app.db_pool).await.expect("Failed to fetch saved subscriptions");

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
