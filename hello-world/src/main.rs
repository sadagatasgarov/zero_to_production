use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::email_client::EmailClient;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    //env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration");

    // let connection = PgConnection::connect(&configuration.database.connection_string())
    //     .await
    //     .expect("Failed to connect to Postgres.");

    let connextion_pool = PgPool::connect_lazy_with(configuration.database.with_db());

    // Build an `EmailClient` using `configuration`
    let sender_email = configuration
        .email_client
        .sender()
        .expect("Invalid sender email address.");
    let email_client = EmailClient::new(configuration.email_client.base_url, sender_email);

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );

    let listener = TcpListener::bind(address)?;
    println!("{:?}", listener);
    run(listener, connextion_pool, email_client)?.await
}
