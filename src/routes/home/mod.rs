use actix_web::{
    http::{header::ContentType, StatusCode},
    web, HttpResponse, ResponseError,
};
use tera::{Context, Tera};

use super::error_chain_fmt;

pub async fn home(tera: web::Data<Tera>) -> Result<HttpResponse, ServerError> {
    let template = tera
        .render("home.tera.html", &Context::new())
        .map_err(|e| ServerError::RenderError(e.into()))?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(template))
}

#[derive(thiserror::Error)]
pub enum ServerError {
    #[error("Failed to render template")]
    RenderError(#[source] anyhow::Error),
}

impl std::fmt::Debug for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for ServerError {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
