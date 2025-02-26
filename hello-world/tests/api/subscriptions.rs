use crate::helpers::spawn_app;

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app = spawn_app().await;
    let body = "name=Sadagat%20Asgarov&email=sadagatasgarov%40gmail.com";

    let response = app.post_subscriptions(body.into()).await;

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscriptions");

    assert_eq!(saved.email, "sadagatasgarov@gmail.com");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app = spawn_app().await;
    let test_cases = vec![
        ("name=Sadagat%20Asgarov", "missing the email"),
        ("email=sadagat.asgarov@gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = app.post_subscriptions(invalid_body.into()).await;

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad request whwn the payload was {}",
            error_message
        )
    }
}

#[tokio::test]
// Renamed!
async fn subscribe_returns_a_400_when_fields_are_present_but_invalid() {
    // Arrange
    let app = spawn_app().await;
    let test_cases = vec![
        ("name=&email=sadagat.asgarov%40gmail.com", "empty name"),
        ("name=Sadagat&email=", "empty email"),
        (
            "name=Sadagat&email=definitely-not-an-email",
            "invalid email",
        ),
    ];

    for (invalid_body, description) in test_cases {
        // Act
        let response = app.post_subscriptions(invalid_body.into()).await;

        assert_eq!(
            // Not 200 anymore!
            400,
            response.status().as_u16(),
            "The API did not return a 400 Bad Request when the payload was {}.",
            description
        );
    }
}
