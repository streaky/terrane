//! Narrow bridges for Axum 0.8 shapes that Terrane cannot yet project directly.

use std::future::Future;

use axum::Router;
use axum::extract::WebSocketUpgrade;
use axum::extract::ws::{DefaultOnFailedUpgrade, WebSocket};
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

/// Await Axum's `IntoFuture` server value at the currently projectable concrete router boundary.
///
/// # Errors
///
/// Returns Axum's underlying I/O error when the server fails.
pub async fn serve_router(listener: TcpListener, router: Router) -> Result<(), std::io::Error> {
    axum::serve(listener, router).await
}
