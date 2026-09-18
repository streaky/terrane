
use axum::Router;
use axum::extract::WebSocketUpgrade;
use axum::extract::ws::{DefaultOnFailedUpgrade, WebSocket};
use axum::response::{IntoResponse, Response};
use tokio::net::TcpListener;

pub struct UpgradeResponse(Response);

impl IntoResponse for UpgradeResponse {
    fn into_response(self) -> Response {
        self.0
    }
}

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

pub async fn serve_router(listener: TcpListener, router: Router) -> Result<(), std::io::Error> {
    axum::serve(listener, router).await
}
