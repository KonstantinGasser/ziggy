use askama::Template;
use serde::{de, Deserialize, Deserializer};
use std::sync::{Arc, Mutex};

use axum::{
    extract::{Extension, Query},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};

use axum::response::sse::{Event, KeepAlive, Sse};
use futures_util::stream::{self, Stream};
use std::{convert::Infallible, time::Duration};
use tokio_stream::StreamExt as _;

use crate::conway;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    state: Vec<Vec<Option<()>>>,
}

#[derive(Template)]
#[template(path = "grid.html")]
struct GridTemplate {
    state: Vec<Vec<Option<()>>>,
}

pub async fn index(state: Extension<Arc<Mutex<conway::game::Game>>>) -> impl IntoResponse {
    let state = state.lock().unwrap();

    TemplateResponse(IndexTemplate {
        state: state.0.clone(),
    })
    .into_response()
}

pub async fn next_cycle(state: Extension<Arc<Mutex<conway::game::Game>>>) -> impl IntoResponse {
    let mut state = state.lock().unwrap();
    state.0 = state.next_cycle().0;

    TemplateResponse(GridTemplate {
        state: state.0.clone(),
    })
    .into_response()
}

pub async fn reset(state: Extension<Arc<Mutex<conway::game::Game>>>) -> impl IntoResponse {
    let mut state = state.lock().unwrap();

    state.reset();
    TemplateResponse(GridTemplate {
        state: state.0.clone(),
    })
    .into_response()
}

#[derive(Debug, Deserialize)]
pub struct SwitchOptions {
    i: usize,
    j: usize,
}

pub async fn flip(
    Query(swaps): Query<SwitchOptions>,
    state: Extension<Arc<Mutex<conway::game::Game>>>,
) -> impl IntoResponse {
    let mut state = state.lock().unwrap();

    let i = swaps.i;
    let j = swaps.j;

    state.flip(i, j);

    // TODO: figure out why we cannot return a template even if the parent template is passing in
    // the required variables. Error message is angry that i,j are not available...
    match state.0[i][j] {
        Some(_) =>  format!("<div id=\"cell-{i}-{j}\" hx-get=\"/flip?i={i}&j={{j}}\" hx-target=\"#cell-{i}-{j}\" hx-swap=\"outerHTML\"
                style=\"background-color: #000000; width: 20px; height: 20px; border: 1px solid #c4c4c4; cursor: pointer;\"></div>"),
        None =>     format!("<div id=\"cell-{i}-{j}\" hx-get=\"/flip?i={i}&j={{j}}\" hx-target=\"#cell-{i}-{j}\" hx-swap=\"outerHTML\"style=\"background-color: #ffffff; width: 20px; height: 20px; border: 1px solid #c4c4c4; cursor: pointer;\"></div>")

    }
}

pub async fn stream_cycle(
    state: Extension<Arc<Mutex<conway::game::Game>>>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let mut state = state.lock().unwrap().clone();
    let stream = stream::repeat_with(move || {
        state = state.next_cycle();
        Event::default().data(
            GridTemplate {
                state: state.0.clone(),
            }
            .render()
            .unwrap(),
        )
    })
    .map(Ok)
    .throttle(Duration::from_secs(1));

    Sse::new(stream).keep_alive(KeepAlive::default())
}

struct TemplateResponse<T>(pub T);

impl<T> IntoResponse for TemplateResponse<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unable to parse template. Error: {err}"),
            )
                .into_response(),
        }
    }
}
