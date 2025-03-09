use actix_web::{http::header::ContentType, web, HttpResponse};
use tera::{Context, Tera};

use crate::routes::ServerError;

pub async fn newsletter_editor(tera: web::Data<Tera>) -> Result<HttpResponse, actix_web::Error> {
    let context = Context::new();

    let template = tera
        .render("newsletter-editor.html", &context)
        .map_err(|e| ServerError::RenderError(e.into()))?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(template))
}
