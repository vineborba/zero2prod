use actix_web::{http::header::ContentType, web, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;
use tera::{Context, Tera};

use crate::routes::ServerError;

pub async fn newsletter_editor(
    tera: web::Data<Tera>,
    flash_messages: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    let mut message_html: Option<String> = None;

    for m in flash_messages.iter() {
        message_html = Some(m.content().into());
    }

    let mut context = Context::new();

    context.insert("message_html", &message_html);

    let template = tera
        .render("newsletter-editor.html", &context)
        .map_err(|e| ServerError::RenderError(e.into()))?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(template))
}
