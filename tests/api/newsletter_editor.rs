use crate::helpers::{assert_is_redirect_to, spawn_app};

#[actix_web::test]
async fn unauthenticated_requests_are_rejected() {
    let app = spawn_app().await;

    let response = app.get_newsletter_editor().await;
    assert_is_redirect_to(&response, "/login");
}

#[actix_web::test]
async fn newsletter_editor_must_contain_necessary_fields() {
    let app = spawn_app().await;
    app.test_user.login(&app).await;

    let form_fields = vec![
        "Newsletter title",
        "Newsletter text content",
        "Newsletter HTML content",
    ];

    let html_page = app.get_newsletter_editor_html().await;
    for field in form_fields {
        assert!(html_page.contains(field));
    }
}
