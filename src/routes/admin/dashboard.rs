use actix_web::{http::header::ContentType, web, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;
use tera::Tera;
use uuid::Uuid;

use crate::{
    routes::ServerError,
    session_state::TypedSession,
    utils::{e500, see_other},
};

pub async fn admin_dashboard(
    session: TypedSession,
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
) -> Result<HttpResponse, actix_web::Error> {
    let username = if let Some(user_id) = session.get_user_id().map_err(e500)? {
        get_username(user_id, &pool).await.map_err(e500)?
    } else {
        return Ok(see_other("/login"));
    };

    let mut context = tera::Context::new();

    context.insert("username", &username);

    let template = tera
        .render("admin-dashboard.html", &context)
        .map_err(|e| ServerError::RenderError(e.into()))?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(template))
}

pub async fn get_username(user_id: Uuid, pool: &PgPool) -> Result<String, anyhow::Error> {
    let row = sqlx::query!(
        r#"
        SELECT username
        FROM users
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_one(pool)
    .await
    .context("Failed to perform a query to retrieve a username.")?;

    Ok(row.username)
}
