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
    with_sse: bool,
    cycles: usize,
    alive_cells: usize,
}

#[derive(Template)]
#[template(path = "grid.html")]
struct GridTemplate {
    state: Vec<Vec<Option<()>>>,
    cycles: usize,
    alive_cells: usize,
}

#[derive(Template)]
#[template(path = "grid_response.html")]
struct GridResponseTemplate {
    state: Vec<Vec<Option<()>>>,
    cycles: usize,
    alive_cells: usize,
}

pub async fn index(state: Extension<Arc<Mutex<conway::game::Game>>>) -> impl IntoResponse {
    let game = state.lock().unwrap();

    TemplateResponse(IndexTemplate {
        state: game.state.clone(),
        with_sse: false,
        cycles: game.cycles,
        alive_cells: game.alive_cells,
    })
    .into_response()
}

pub async fn next_cycle(state: Extension<Arc<Mutex<conway::game::Game>>>) -> impl IntoResponse {
    let mut game = state.lock().unwrap();
    let tmp = game.next_cycle();

    game.state = tmp.state;
    game.cycles = tmp.cycles;
    game.alive_cells = tmp.alive_cells;

    TemplateResponse(GridResponseTemplate {
        state: game.state.clone(),
        cycles: game.cycles,
        alive_cells: game.alive_cells,
    })
    .into_response()
}

pub async fn reset(state: Extension<Arc<Mutex<conway::game::Game>>>) -> impl IntoResponse {
    let mut game = state.lock().unwrap();

    game.reset();
    TemplateResponse(GridTemplate {
        state: game.state.clone(),
        cycles: game.cycles,
        alive_cells: game.alive_cells,
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
    let mut game = state.lock().unwrap();

    let i = swaps.i;
    let j = swaps.j;

    game.flip(i, j);

    // TODO: figure out why we cannot return a template even if the parent template is passing in
    // the required variables. Error message is angry that i,j are not available...
    match game.state[i][j] {
        Some(_) =>  format!("<div id=\"cell-{i}-{j}\" hx-get=\"/flip?i={i}&j={j}\" hx-target=\"#cell-{i}-{j}\" hx-swap=\"outerHTML\"
                style=\"background-color: #000000; width: 20px; height: 20px; border: 1px solid #c4c4c4; cursor: pointer;\"></div>"),
        None =>     format!("<div id=\"cell-{i}-{j}\" hx-get=\"/flip?i={i}&j={j}\" hx-target=\"#cell-{i}-{j}\" hx-swap=\"outerHTML\"style=\"background-color: #ffffff; width: 20px; height: 20px; border: 1px solid #c4c4c4; cursor: pointer;\"></div>")

    }
}

pub async fn index_with_sse(state: Extension<Arc<Mutex<conway::game::Game>>>) -> impl IntoResponse {
    let game = state.lock().unwrap();

    TemplateResponse(IndexTemplate {
        state: game.state.clone(),
        with_sse: true,
        cycles: game.cycles,
        alive_cells: game.alive_cells,
    })
    .into_response()
}

// NOTE: have a look at this issue (https://github.com/tokio-rs/axum/issues/2150)
// it is discussing how to work with channels to trigger an event of the sse stream
// instead of have a periodical send event. Further this allows to separate the state
// by NOT moving inside the stream closure from the stream thus allowing multiple
// subscribers to share the same view/state.
pub async fn stream_cycle(
    state: Extension<Arc<Mutex<conway::game::Game>>>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = stream::repeat_with(move || {
        let mut game = state.lock().unwrap();
        let tmp = game.next_cycle();

        game.state = tmp.state;
        game.cycles = tmp.cycles;
        game.alive_cells = tmp.alive_cells;
        Event::default().data(
            GridResponseTemplate {
                state: game.state.clone(),
                cycles: game.cycles,
                alive_cells: game.alive_cells,
            }
            .render()
            .unwrap(),
        )
    })
    .map(Ok)
    .throttle(Duration::from_millis(250));

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
