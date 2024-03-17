use askama::Template;
use axum::extract::Extension;
use axum::{http::StatusCode, response::IntoResponse};

use crate::counter::app;
use crate::handlers::html_render;

#[derive(Template)]
#[template(path = "counter.html")]
pub struct HtmlCounter {
    view_counter: usize,
}

pub async fn get_count(Extension(state): axum::extract::Extension<app::App>) -> impl IntoResponse {
    let count = state.get_count();
    html_render::TemplateResponse(HtmlCounter {
        view_counter: count,
    })
    // (StatusCode::OK, format!("Count is: {count}"))
}
pub async fn increment(Extension(state): axum::extract::Extension<app::App>) -> impl IntoResponse {
    let new_count = state.increment();
    html_render::TemplateResponse(HtmlCounter {
        view_counter: new_count,
    })
    // (StatusCode::OK, format!("New count is: {new_count}"))
}

pub async fn decrement(Extension(state): axum::extract::Extension<app::App>) -> impl IntoResponse {
    let new_count = state.decrement();
    html_render::TemplateResponse(HtmlCounter {
        view_counter: new_count,
    })
    // (StatusCode::OK, format!("New count is: {new_count}"))
}
