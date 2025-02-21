use crate::{
    domain::{NewSubscriber, SubscriberName},
    FormData,
};
use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

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
    let new_subscriber = NewSubscriber {
        email: form.0.email,
        name: SubscriberName::parse(form.0.name),
    };

    // if !is_valid_name(&form.name) {
    //     HttpResponse::BadRequest().finish();
    // }

    match insert_subscriber(&pool, &new_subscriber).await {
        Ok(_) => HttpResponse::Ok().finish(),

        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

// pub fn is_valid_name(s: &str) -> bool {
//     let empty_or_whitspace = s.trim().is_empty();
//     let is_too_long = s.graphemes(true).count() > 256;
//     let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];

//     let contain_forbidden_characters = s.chars().any(|x|{
//         forbidden_characters.contains(&x)
//     });

//     !(empty_or_whitspace || is_too_long || contain_forbidden_characters)
// }

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(new_subscriber, pool)
)]
pub async fn insert_subscriber(
    pool: &PgPool,
    new_subscriber: &NewSubscriber,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
            INSERT INTO subscriptions (id, email, name, subscribed_at) 
            VALUES ($1, $2, $3, $4);
        "#,
        Uuid::new_v4(),
        new_subscriber.email,
        new_subscriber.name.inner_ref(),
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
