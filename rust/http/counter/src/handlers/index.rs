use askama::Template;
use axum::extract::Extension;
use axum::http::HeaderValue;
use axum::{http::StatusCode, response::IntoResponse};

use crate::counter::app;
use crate::handlers::html_render;

#[derive(Template)]
#[template(path = "base.html")]
pub struct BaseTemplate {
    view_counter: usize,
}

#[derive(Template)]
#[template(path = "counter.html")]
pub struct CounterTemplate {
    view_counter: usize,
    error: Option<String>,
}

#[derive(Template)]
#[template(path = "error.html")]
pub struct ErrorTemplate {
    error: String,
}

pub async fn home(Extension(state): axum::extract::Extension<app::App>) -> impl IntoResponse {
    let count = state.get_count();
    html_render::TemplateResponse(BaseTemplate {
        view_counter: count,
    })
}
pub async fn increment(Extension(state): axum::extract::Extension<app::App>) -> impl IntoResponse {
    let new_count = state.increment();
    html_render::TemplateResponse(CounterTemplate {
        view_counter: new_count,
        error: None,
    })
    .into_response()
}

pub async fn decrement(Extension(state): axum::extract::Extension<app::App>) -> impl IntoResponse {
    let Some(new_count) = state.decrement() else {
        let mut resp = html_render::TemplateResponse(ErrorTemplate {
            error: "negative count not allowed".to_owned(),
        })
        .into_response();

        resp.headers_mut()
            .insert("HX-Retarget", HeaderValue::from_static("#error"));
        return resp;
    };

    html_render::TemplateResponse(CounterTemplate {
        view_counter: new_count,
        error: None,
    })
    .into_response()
}
