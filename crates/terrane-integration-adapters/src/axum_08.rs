//! Narrow bridges for Axum 0.8 shapes that Terrane cannot yet project directly.

use std::future::Future;

use axum::Router;
use axum::extract::WebSocketUpgrade;
use axum::extract::ws::{DefaultOnFailedUpgrade, Message, WebSocket};
use axum::response::{IntoResponse, Response};
use tokio::net::TcpListener;

/// Concrete response returned by a WebSocket upgrade handler.
pub struct UpgradeResponse(Response);

impl IntoResponse for UpgradeResponse {
    fn into_response(self) -> Response {
        self.0
    }
}

/// Turn Axum's closed response alias into a concrete handler result.
pub fn upgrade<F, Fut>(
    request: WebSocketUpgrade<DefaultOnFailedUpgrade>,
    callback: F,
) -> UpgradeResponse
where
    F: FnOnce(WebSocket) -> Fut + Send + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
    UpgradeResponse(request.on_upgrade(callback))
}

/// Receive the next text message, ignoring non-data control messages.
///
/// # Errors
///
/// Returns Axum's receive error when the WebSocket stream fails.
pub async fn receive_text(socket: &mut WebSocket) -> Result<Option<String>, axum::Error> {
    loop {
        match socket.recv().await.transpose()? {
            Some(Message::Text(text)) => return Ok(Some(text.to_string())),
            Some(Message::Close(_)) | None => return Ok(None),
            Some(_) => {}
        }
    }
}

/// Construct Axum's data-carrying text message variant.
#[must_use]
pub fn text_message(text: String) -> Message {
    Message::Text(text.into())
}

/// Construct Axum's close message variant without a close payload.
#[must_use]
pub fn close_message() -> Message {
    Message::Close(None)
}

/// Bind the Tokio listener required by `axum::serve` without exposing its unprojectable static API.
///
/// # Errors
///
/// Returns the operating-system bind error when the address cannot be listened on.
pub async fn bind_listener(address: String) -> Result<TcpListener, std::io::Error> {
    TcpListener::bind(address).await
}

/// Await Axum's `IntoFuture` server value at the currently projectable concrete router boundary.
///
/// # Errors
///
/// Returns Axum's underlying I/O error when the server fails.
pub async fn serve_router(listener: TcpListener, router: Router) -> Result<(), std::io::Error> {
    axum::serve(listener, router).await
}
