use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use axum::{
    Json, Router,
    body::Bytes,
    extract::Request,
    http::{Method, Uri},
    response::IntoResponse,
    routing::any,
};
use clap::Parser;
use serde_json::{Value, json};

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value_t = 7890)]
    port: u16,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let app = Router::new().route("/{*path}", any(handle_request));
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), args.port);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("listening on http://{addr}");
    axum::serve(listener, app).await.unwrap();
}

async fn handle_request(request: Request) -> impl IntoResponse {
    let method = request.method().clone();
    let uri = request.uri().clone();
    let (parts, body) = request.into_parts();
    let body = axum::body::to_bytes(body, usize::MAX).await.unwrap();

    log_request(&method, &uri, &body);

    if parts.uri.path() == "/v1/licenses/activate" {
        return Json(json!({
            "activated": true,
            "meta": {
                "store_id": 125728,
                "product_id": 372705,
            },
            "instance": {
                "id": "some-instance-id",
            },
        }));
    }

    Json(json!({ "ok": true }))
}

fn log_request(method: &Method, uri: &Uri, body: &Bytes) {
    println!("method: {method}");
    println!("uri: {uri}");

    if body.is_empty() {
        println!("payload: <empty>");
        return;
    }

    match serde_json::from_slice::<Value>(body) {
        Ok(payload) => {
            println!("payload:");
            println!("{}", serde_json::to_string_pretty(&payload).unwrap());
        }
        Err(_) => {
            println!("payload: {}", String::from_utf8_lossy(body));
        }
    }
}
