use uuid::Uuid;

use crate::helpers::{assert_is_redirect_to, spawn_app};

#[tokio::test]
async fn you_must_be_logged_in_to_see_the_change_password_form() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app.get_change_password().await;

    // Assert
    assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn you_must_be_logged_in_to_change_your_password() {
    // Arrange
    let app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();

    // Act
    let change_password_body = serde_json::json!({
        "current_password": Uuid::new_v4().to_string(),
        "new_password": &new_password,
        "new_password_check": &new_password,
    });

    let response = app.post_change_password(&change_password_body).await;

    // Assert
    assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn new_password_fields_must_match() {
    // Arrange
    let app = spawn_app().await;

    let login_body = serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password,
    });

    app.post_login(&login_body).await;

    let new_password_1 = Uuid::new_v4().to_string();
    let new_password_2 = Uuid::new_v4().to_string();

    // Act 1
    let change_password_body = serde_json::json!({
        "current_password": Uuid::new_v4().to_string(),
        "new_password": &new_password_1,
        "new_password_check": &new_password_2,
    });

    let response = app.post_change_password(&change_password_body).await;

    // Assert 1
    assert_is_redirect_to(&response, "/admin/password");

    // Act 2
    let html = app.get_change_password_html().await;

    assert!(html.contains(
        "<p><i>You entered two different new passwords - the field values must match.</i></p>"
    ))
}

#[tokio::test]
async fn current_password_must_be_valid() {
    // Arrange
    let app = spawn_app().await;

    let login_body = serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password,
    });

    app.post_login(&login_body).await;

    let new_password = Uuid::new_v4().to_string();

    // Act 1
    let change_password_body = serde_json::json!({
        "current_password": Uuid::new_v4().to_string(),
        "new_password": &new_password,
        "new_password_check": &new_password,
    });

    let response = app.post_change_password(&change_password_body).await;
    assert_is_redirect_to(&response, "/admin/password");

    // Act 2
    let html = app.get_change_password_html().await;
    assert!(html.contains("<p><i>You entered an invalid password.</i></p>"))
}

#[tokio::test]
async fn changing_password_works() {
    // Arrange
    let app = spawn_app().await;

    let login_body = serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password,
    });

    let response = app.post_login(&login_body).await;
    assert_is_redirect_to(&response, "/admin/dashboard");

    let new_password = Uuid::new_v4().to_string();

    // Act 1
    let change_password_body = serde_json::json!({
        "current_password": &app.test_user.password,
        "new_password": &new_password,
        "new_password_check": &new_password,
    });

    let response = app.post_change_password(&change_password_body).await;
    assert_is_redirect_to(&response, "/admin/password");

    // Act 2
    let html = app.get_change_password_html().await;
    assert!(html.contains("<p><i>Your password has been changed.</i></p>"));

    // Act 3
    let response = app.post_logout().await;
    assert_is_redirect_to(&response, "/login");

    // Act 4
    let html_page = app.get_login_html().await;
    assert!(html_page.contains(r#"<p><i>You have successfully logged out.</i></p>"#));

    // Act 5
    let response = app.get_admin_dashboard().await;
    assert_is_redirect_to(&response, "/login");

    // Act 6
    let login_body = serde_json::json!({
        "username": &app.test_user.username,
        "password": &new_password,
    });
    let response = app.post_login(&login_body).await;
    assert_is_redirect_to(&response, "/admin/dashboard");
}
