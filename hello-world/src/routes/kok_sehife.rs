use actix_web::{HttpResponse, Responder};
use reqwest::StatusCode;

pub async fn kok_sehife() -> impl Responder {
    HttpResponse::with_body(StatusCode::ACCEPTED, "Salam Qurban")
}
