use std::net::TcpListener;
use sqlx::PgPool;
use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use tracing::subscriber::set_global_default;

pub fn get_subscriber(
    name: String,
    env_filter: String
) -> impl Subscriber +Send + Sync {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new(name, std::io::stdout);

    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync ) {
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to get subscriber");
}


#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    //env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let subscriber = get_subscriber("zero2prod".into(), "info".into());
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration");

    // let connection = PgConnection::connect(&configuration.database.connection_string())
    //     .await
    //     .expect("Failed to connect to Postgres.");

    let connextion_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    let address = format!("127.0.0.1:{}", configuration.application_port);

    let listener = TcpListener::bind(address)?;
    println!("{:?}", listener);
    run(listener, connextion_pool)?.await
}
