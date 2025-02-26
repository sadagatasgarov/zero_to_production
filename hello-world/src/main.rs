use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::email_client::EmailClient;
use zero2prod::startup::run;
use zero2prod::startup::Application;
use zero2prod::telemetry::{get_subscriber, init_subscriber};
#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    //env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration");

    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;

    Ok(())
}
