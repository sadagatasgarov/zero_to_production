use crate::FormData;
use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();

    tracing::info!(
        "request_id {} - Adding '{}' '{}' as a new subscriber.",
        request_id,
        form.email,
        form.name
    );

    tracing::info!("request_id {} - saving new subscriber details in the database", request_id);


    let netice = sqlx::query!(
        r#"
            INSERT INTO subscriptions (id, email, name, subscribed_at) 
            VALUES ($1, $2, $3, $4);
        "#,
        request_id,
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .await;

    match netice {
        Ok(_) => {
            log::info!("request_id {}- New subscriber details heve been saved", request_id);

            HttpResponse::Ok().finish()
        }

        Err(e) => {
            log::error!("request_id {} - Failed to execute query: {:?}", request_id, e);
            println!("failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
