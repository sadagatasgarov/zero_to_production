use crate::FormData;
use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();

    let request_span = tracing::info_span!(
        "Adding a new subscriber.",
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name
    );

    let _request_span_guard = request_span.enter();

    let query_span = tracing::info_span!("saving new subscriber details in the database");


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
    .instrument(query_span)
    .await;

    match netice {
        Ok(_) => {
            log::info!("New subscriber details heve been saved");

            HttpResponse::Ok().finish()
        }

        Err(e) => {
            log::error!("Failed to execute query: {:?}",e);
            println!("failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }


}
