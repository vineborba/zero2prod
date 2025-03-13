use crate::helpers::{assert_is_redirect_to, spawn_app};

#[actix_web::test]
async fn you_must_be_logged_in_to_access_the_admin_dashboard() {
    let app = spawn_app().await;

    let response = app.get_admin_dashboard().await;

    assert_is_redirect_to(&response, "/login");
}

#[actix_web::test]
async fn logout_clears_session_state() {
    let app = spawn_app().await;
    app.test_user.login(&app).await;

    let html_page = app.get_admin_dashboard_html().await;
    assert!(html_page.contains(&format!("Welcome {}", app.test_user.username)));

    let response = app.post_logout().await;
    assert_is_redirect_to(&response, "/login");

    let html_page = app.get_login_html().await;
    assert!(html_page.contains(r#"<p><i>You have successfully logged out.</i></p>"#));

    let response = app.get_admin_dashboard().await;
    assert_is_redirect_to(&response, "/login");
}

#[actix_web::test]
async fn dashboard_must_contain_actions() {
    let app = spawn_app().await;

    let actions = vec!["Change password", "Logout", "Send newsletter issue"];
    app.test_user.login(&app).await;

    let html_page = app.get_admin_dashboard_html().await;

    assert!(html_page.contains(&format!("Welcome {}", app.test_user.username)));

    for action in actions {
        assert!(html_page.contains(action));
    }
}
