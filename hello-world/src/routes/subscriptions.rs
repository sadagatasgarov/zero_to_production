use crate::FormData;
use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    log::info!("saving new subscriber details in the database");
    let netice = sqlx::query!(
        r#"
            INSERT INTO subscriptions (id, email, name, subscribed_at) 
            VALUES ($1, $2, $3, $4);
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .await;

    match netice {
        Ok(_) => {
            log::info!("New subscriber details heve been saved");
            HttpResponse::Ok().finish()
        }

        Err(e) => {
            println!("failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
