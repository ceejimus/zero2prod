use crate::helpers::{assert_is_redirect_to, spawn_app};

#[tokio::test]
async fn root_redirects_to_admin_dashboard_if_logged_in() {
    // Arrange
    let app = spawn_app().await;

    app.login().await;

    // Act
    let response = app.get_root().await;
    assert_is_redirect_to(&response, "/admin/dashboard");
}

#[tokio::test]
async fn you_must_be_logged_in_to_access_the_admin_dashboard() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app.get_admin_dashboard().await;

    // Assert
    assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn logout_clears_session_state() {
    // Arrange
    let app = spawn_app().await;

    app.login().await;

    // Act 1
    let html_page = app.get_admin_dashboard_html().await;
    assert!(html_page.contains(&format!("Welcome {}", app.test_user.username)));

    // Act 2
    let response = app.post_logout().await;
    assert_is_redirect_to(&response, "/login");

    // Act 3
    let html_page = app.get_login_html().await;
    assert!(html_page.contains("<p><i>You have successfully logged out.</i></p>"));

    // Act 4
    let response = app.get_admin_dashboard().await;
    assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn admin_dashboard_includes_link_to_submit_newsletter() {
    // Arrange
    let app = spawn_app().await;
    app.login().await;

    // Act
    let html_page = app.get_admin_dashboard_html().await;
    assert!(html_page.contains("href=\"/admin/newsletters\""));
}
