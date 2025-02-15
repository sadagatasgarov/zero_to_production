use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;
use actix_web::{web, HttpResponse};
use crate::FormData;


pub async fn subscribe(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
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
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        },
    }
}