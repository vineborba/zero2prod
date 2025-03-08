use actix_web::{http::header::ContentType, web, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;
use tera::Tera;
use uuid::Uuid;

use crate::{authentication::UserId, routes::ServerError, utils::e500};

pub async fn admin_dashboard(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    user_id: web::ReqData<UserId>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = user_id.into_inner();
    let username = get_username(*user_id, &pool).await.map_err(e500)?;

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
