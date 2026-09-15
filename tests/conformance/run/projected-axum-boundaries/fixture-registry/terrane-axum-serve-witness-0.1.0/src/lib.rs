pub async fn verify_serve(router: axum::Router) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("loopback bind must succeed");
    let server = axum::serve(listener, router);
    drop(server);
}
