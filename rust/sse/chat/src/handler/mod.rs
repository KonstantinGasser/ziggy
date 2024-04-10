mod rendering;
mod template;

use rendering::Response;

use axum::{
    extract::{Form, Path, State},
    http::{
        header::{HeaderMap, HeaderValue, SET_COOKIE},
        StatusCode,
    },
    response::{
        sse::{Event, KeepAlive, Sse},
        IntoResponse, Redirect,
    },
};

use axum_extra::TypedHeader;
use headers::Cookie;

use futures::stream::Stream;
use tokio_stream::StreamExt as _;

use std::{convert::Infallible, time::Duration};

use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::chat;

pub async fn index(
    State(state): State<Arc<chat::State>>,
    TypedHeader(cookie): TypedHeader<Cookie>,
) -> impl IntoResponse {
    let user_handle = match cookie.get("user_handle") {
        Some(cookie) => Some(cookie.to_string()),
        None => None,
    };

    Response(template::Index {
        rooms: state.get_hangout_short(),
        online: state.get_online_users(),
        cookie_user_handle: user_handle,
    })
}

#[derive(Serialize, Deserialize)]
pub struct ClaimUserHandleReq {
    user_handle: String,
}
pub async fn claim_user_handle(
    State(state): State<Arc<chat::State>>,
    Form(req): Form<ClaimUserHandleReq>,
) -> impl IntoResponse {
    let Some(user_id) = state.claim_user_handle(&req.user_handle) else {
        return (
            StatusCode::BAD_REQUEST,
            format!("User Handle \"{}\" already exisits", &req.user_handle),
        )
            .into_response();
    };

    let mut headers = HeaderMap::new();
    headers.insert(
        SET_COOKIE,
        HeaderValue::from_str(&format!("user_handle={}", &user_id)).unwrap(),
    );

    (
        headers,
        rendering::Response(template::Index {
            rooms: state.get_hangout_short(),
            online: state.get_online_users(),
            cookie_user_handle: Some(req.user_handle),
        }),
    )
        .into_response()
}

#[derive(Serialize, Deserialize)]
pub struct CreateHangoutReq {
    pub hangout_name: String,
}

pub async fn create_hangout(
    State(state): State<Arc<chat::State>>,
    Form(req): Form<CreateHangoutReq>,
) -> impl IntoResponse {
    // TODO:: handle !None respone indicating that the hangout room already exisits
    let _ = state.create_hangout(&req.hangout_name);
    Response(template::HangoutList {
        rooms: state.get_hangout_short(),
    })
}

#[derive(Serialize, Deserialize)]
pub struct ConnectHangoutReq {}

pub async fn load_hangout(
    State(state): State<Arc<chat::State>>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    state.init_hangout(&name);

    rendering::Response(template::Chat {
        hangout_id: name,
        user_handle: "unkown".to_string(),
    })
}

pub async fn connect_to_hangout(
    State(state): State<Arc<chat::State>>,
    Path(hangout): Path<String>,
    TypedHeader(cookie): TypedHeader<Cookie>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let Some(cookie_user) = cookie.get("user_handle") else {
        // no cookie no access, redirect to Path("/")
        // however, function returns Sse<...> and not impl IntoResponse..
        panic!("No cookie found!");
    };
    let (rx, _) = state.connect_to_hangout(&hangout, &cookie_user).unwrap();

    let stream = tokio_stream::wrappers::BroadcastStream::new(rx);

    Sse::new(
        stream
            .map(|msg| match msg.unwrap() {
                chat::Message::ChatMessage(msg) => Event::default().event("message").data(msg),
                chat::Message::UserJoin(user_name) => {
                    Event::default().event("user_join").data(user_name)
                }
            })
            .map(Ok),
    )
    .keep_alive(
        KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}

#[derive(Serialize, Deserialize)]
pub struct SendReq {
    pub send_message: String,
    pub hangout_id: String,
    pub user_handle: String,
}

pub async fn send_message(
    State(state): State<Arc<chat::State>>,
    Form(req): Form<SendReq>,
) -> impl IntoResponse {
    state.broadcast_to_hangout(&req.hangout_id, &req.send_message);

    (StatusCode::OK, "message send to channel")
}
