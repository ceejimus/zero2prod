use wiremock::{
    matchers::{method, path},
    Mock, ResponseTemplate,
};

use crate::helpers::{assert_is_redirect_to, spawn_app, ConfirmationLinks, TestApp};

async fn create_unconfirmed_subscriber(app: &TestApp) -> ConfirmationLinks {
    let body = "name=Ursula&email=ursula_le_guin%40gmail.com";

    let _mock_guard = Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .named("Create unconfirmed subscriber")
        // see https://docs.rs/wiremock/0.5.15/wiremock/struct.Mock.html#method.mount_as_scoped
        .mount_as_scoped(&app.email_server)
        .await;

    app.post_subscriptions(body.into()).await;

    let email_request = &app
        .email_server
        .received_requests()
        .await
        .unwrap()
        .pop()
        .unwrap();

    app.get_confirmation_links(email_request).await
}

async fn create_confirmed_subscriber(app: &TestApp) {
    let confirmation_links = create_unconfirmed_subscriber(app).await;

    reqwest::get(confirmation_links.html.to_string())
        .await
        .unwrap()
        .error_for_status()
        .unwrap();
}

#[tokio::test]
async fn must_login_to_post_newsletter() {
    // Arrange
    let app = spawn_app().await;
    create_confirmed_subscriber(&app).await;

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(0)
        .named("Post new newsletter")
        .mount(&app.email_server)
        .await;

    // Act
    let newsletter_request_body = serde_json::json!({
        "title": "New Title",
        "text_content": "Newsletter body as plain text",
        "html_content": "<p>Newsletter body as HTML</p>"
    });

    let response = app.post_newsletters(&newsletter_request_body).await;
    assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn newsletters_are_not_delivered_to_unconfirmed_subscribers() {
    // Arrange
    let app = spawn_app().await;

    create_unconfirmed_subscriber(&app).await;

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(0)
        .named("Post new newsletter")
        .mount(&app.email_server)
        .await;

    app.login().await;

    // Act
    let newsletter_request_body = serde_json::json!({
        "title": "New Title",
        "text_content": "Newsletter body as plain text",
        "html_content": "<p>Newsletter body as HTML</p>"
    });

    let response = app.post_newsletters(&newsletter_request_body).await;

    // Assert
    assert_eq!(response.status().as_u16(), 200);
    // mock assert on drop
}

#[tokio::test]
async fn newsletters_are_delivered_to_confirmed_subscribers() {
    // Arrange
    let app = spawn_app().await;

    create_confirmed_subscriber(&app).await;

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .named("Post new newsletter")
        .mount(&app.email_server)
        .await;

    app.login().await;

    // Act
    let newsletter_request_body = serde_json::json!({
        "title": "New Title",
        "text_content": "Newsletter body as plain text",
        "html_content": "<p>Newsletter body as HTML</p>"
    });

    let response = app.post_newsletters(&newsletter_request_body).await;

    // Assert
    assert_eq!(response.status().as_u16(), 200)
    // mock will assert new email
}

#[tokio::test]
async fn newsletters_returns_400_for_invalid_data() {
    // Arrange
    let app = spawn_app().await;

    app.login().await;

    let test_cases = vec![
        (
            serde_json::json!({
                "text_content": "Newsletter body as plain text",
                "html_content": "<p>Newsletter body as HTML</p>"
            }),
            "missing title",
        ),
        (
            serde_json::json!({
                "title": "New newsletter"
            }),
            "missing content",
        ),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = app.post_newsletters(&invalid_body).await;

        // Assert
        assert_eq!(
            response.status().as_u16(),
            400,
            "The API did not fail with 400 Bad REquest when the payload was {}",
            error_message
        )
    }
}
