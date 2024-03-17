use askama::Template;
use axum::response::IntoResponse;

use crate::handlers::html_render;

#[derive(Template)]
#[template(path = "counter.html")]
pub struct HtmlTest {}

pub async fn handle() -> impl IntoResponse {
    let template = HtmlTest {};
    html_render::TemplateResponse(template)
}
