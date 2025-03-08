use actix_web::{
    cookie::{time::Duration, Cookie},
    http::header::ContentType,
    web, HttpResponse,
};
use actix_web_flash_messages::IncomingFlashMessages;
use tera::{Context, Tera};

use crate::routes::ServerError;

pub async fn login_form(
    flash_messages: IncomingFlashMessages,
    tera: web::Data<Tera>,
) -> Result<HttpResponse, ServerError> {
    let mut message_html: Option<String> = None;

    for m in flash_messages.iter() {
        message_html = Some(m.content().into());
    }

    let mut context = Context::new();

    context.insert("message_html", &message_html);

    let template = tera
        .render("login.html", &context)
        .map_err(|e| ServerError::RenderError(e.into()))?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .cookie(Cookie::build("_flash", "").max_age(Duration::ZERO).finish())
        .body(template))
}
