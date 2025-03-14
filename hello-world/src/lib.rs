pub mod configuration;
pub mod domain;

pub mod email_client;
mod routes;

pub mod startup;
pub mod telemetry;

pub mod authentication;

pub mod session_state;

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}
