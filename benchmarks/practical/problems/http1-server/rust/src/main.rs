// Adapted from https://github.com/hanabi1224/Programming-Language-Benchmarks/blob/6aead3598958a3b310dc7ed3d33cc33c6f27382e/bench/algorithm/http-server/1.rs
// Changes: bind an OS-assigned port before starting clients; retain the upstream HTTP/1 workload.
use axum::routing::post;
use hyper::{client::HttpConnector, StatusCode};
use tokio::sync::mpsc::{self, Sender};

static HOST: &str = "127.0.0.1";
lazy_static::lazy_static! {
    static ref H1_CLIENT: hyper::Client<HttpConnector> = h1_client();
}

fn main() -> anyhow::Result<()> {
    let n = std::env::args_os()
        .nth(1)
        .and_then(|s| s.into_string().ok())
        .and_then(|s| s.parse().ok())
        .unwrap_or(10);
    tokio_main(n)?;
    std::process::exit(0);
}

#[tokio::main]
async fn tokio_main(n: usize) -> anyhow::Result<()> {
    let app = axum::Router::new().route("/api", post(handler));
    let server = axum::Server::bind(&format!("{}:0", HOST).parse()?);
    let port = server.local_addr().port();
    tokio::spawn(server.serve(app.into_make_service()));
    let (sender, mut receiver) = mpsc::channel::<usize>(n);
    let mut sum = 0;
    let api = format!("http://{}:{}/api", HOST, port);
    for i in 1..=n {
        tokio::spawn(send_with_retry(api.clone(), i, sender.clone()));
    }
    for _ in 0..n {
        if let Some(v) = receiver.recv().await {
            sum += v;
        }
    }
    println!("{}", sum);
    Ok(())
}

async fn send_with_retry(api: String, value: usize, sender: Sender<usize>) -> anyhow::Result<()> {
    loop {
        if let Ok(r) = send_once(&api, value).await {
            sender.send(r).await?;
            break;
        }
    }
    Ok(())
}

async fn send_once(api: &str, value: usize) -> anyhow::Result<usize> {
    let payload = Payload { value };
    let resp = H1_CLIENT
        .request(hyper::Request::post(api).body(serde_json::to_string(&payload)?.into())?)
        .await?;
    if resp.status().is_success() {
        let bytes = hyper::body::to_bytes(resp.into_body()).await?;
        let text = String::from_utf8_lossy(&bytes[..]);
        Ok(text.parse::<usize>()?)
    } else {
        anyhow::bail!("{}", resp.status())
    }
}

async fn handler(body: axum::extract::RawBody) -> (StatusCode, String) {
    match handler_inner(body).await {
        Ok(b) => (StatusCode::OK, b),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}

async fn handler_inner(body: axum::extract::RawBody) -> anyhow::Result<String> {
    let bytes = hyper::body::to_bytes(body.0).await?;
    let json: Payload = serde_json::from_slice(&bytes[..])?;
    Ok(format!("{}", json.value))
}

fn h1_client() -> hyper::Client<HttpConnector> {
    hyper::Client::builder().build_http()
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Payload {
    pub value: usize,
}
