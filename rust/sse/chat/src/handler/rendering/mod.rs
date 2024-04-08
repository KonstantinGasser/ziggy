use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

pub struct Response<T>(pub T);

impl<T> IntoResponse for Response<T>
where
    T: Template,
{
    fn into_response(self) -> axum::response::Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("unable to parse template.\nError: {err}"),
            )
                .into_response(),
        }
    }
}
