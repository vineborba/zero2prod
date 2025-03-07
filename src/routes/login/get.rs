use actix_web::{
    cookie::{time::Duration, Cookie},
    http::header::ContentType,
    web, HttpResponse,
};
use actix_web_flash_messages::{IncomingFlashMessages, Level};
use tera::{Context, Tera};

use crate::routes::ServerError;

pub async fn login_form(
    flash_messages: IncomingFlashMessages,
    tera: web::Data<Tera>,
) -> Result<HttpResponse, ServerError> {
    let mut error_html: Option<String> = None;

    for m in flash_messages.iter().filter(|m| m.level() == Level::Error) {
        error_html = Some(m.content().into());
    }

    let mut context = Context::new();

    context.insert("error_html", &error_html);

    let template = tera
        .render("login.tera.html", &context)
        .map_err(|e| ServerError::RenderError(e.into()))?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .cookie(Cookie::build("_flash", "").max_age(Duration::ZERO).finish())
        .body(template))
}
