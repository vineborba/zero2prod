use actix_web::{http::header::ContentType, web, HttpResponse};
use actix_web_flash_messages::{IncomingFlashMessages, Level};
use tera::{Context, Tera};

use crate::{
    routes::ServerError,
    session_state::TypedSession,
    utils::{e500, see_other},
};

pub async fn change_password_form(
    tera: web::Data<Tera>,
    session: TypedSession,
    flash_messages: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    if session.get_user_id().map_err(e500)?.is_none() {
        return Ok(see_other("/login"));
    }
    let mut message_html: Option<String> = None;

    for m in flash_messages.iter().filter(|m| m.level() == Level::Error) {
        message_html = Some(m.content().into());
    }

    let mut context = Context::new();

    context.insert("message_html", &message_html);

    let template = tera
        .render("change-password.html", &context)
        .map_err(|e| ServerError::RenderError(e.into()))?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(template))
}
