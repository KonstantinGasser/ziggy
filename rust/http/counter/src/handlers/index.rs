use askama::Template;
use axum::extract::Extension;
use axum::{http::StatusCode, response::IntoResponse};

use crate::counter::app;
use crate::handlers::html_render;

#[derive(Template)]
#[template(path = "counter.html")]
pub struct HtmlCounter {
    view_counter: usize,
    error: Option<String>,
}

pub async fn get_count(Extension(state): axum::extract::Extension<app::App>) -> impl IntoResponse {
    let count = state.get_count();
    html_render::TemplateResponse(HtmlCounter {
        view_counter: count,
        error: None,
    })
}
pub async fn increment(Extension(state): axum::extract::Extension<app::App>) -> impl IntoResponse {
    let new_count = state.increment();
    format!("<p class=\"text-lg font-bold text-center m-2\">Count {new_count}</p>")
}

pub async fn decrement(Extension(state): axum::extract::Extension<app::App>) -> impl IntoResponse {
    let Some(new_count) = state.decrement() else {
        return html_render::TemplateResponse(HtmlCounter {
            view_counter: state.get_count(),
            error: Some("negative count not allowed".to_owned()),
        })
        .into_response();
    };

    // html_render::TemplateResponse(HtmlCounter {
    //     view_counter: new_count,
    //     error: None,
    // })
    // .into_response()
    format!("<p class=\"text-lg font-bold text-center m-2\">Count {new_count}</p>").into_response()
}
