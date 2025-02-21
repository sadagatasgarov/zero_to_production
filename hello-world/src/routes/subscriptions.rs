use crate::FormData;
use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;
use unicode_segmentation::UnicodeSegmentation;

#[
    tracing::instrument(
        name = "Adding a new subscriberlllllllllllllllllllllllllllllllll",
        skip(form, pool),
        fields(
            subscriber_email = %form.email,
            subscriber_name = %form.name
        )

    )
]
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    if !is_valid_name(&form.name) {
        HttpResponse::BadRequest().finish();
    }


    match insert_subscriber(&pool, &form).await {
        Ok(_) => HttpResponse::Ok().finish(),

        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub fn is_valid_name(s: &str) -> bool {
    let empty_or_whitspace = s.trim().is_empty();
    let is_too_long = s.graphemes(true).count() > 256;
    let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];

    let contain_forbidden_characters = s.chars().any(|x|{
        forbidden_characters.contains(&x)
    });

    !(empty_or_whitspace || is_too_long || contain_forbidden_characters)
}




#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(form, pool)
)]
pub async fn insert_subscriber(pool: &PgPool, form: &FormData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
            INSERT INTO subscriptions (id, email, name, subscribed_at) 
            VALUES ($1, $2, $3, $4);
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed execute query: {:?}", e);
        e
    })?;
    Ok(())
}
