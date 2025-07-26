use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};
use dotenv::dotenv;
use reqwest::Client;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Carica .env (va creato localmente, ma non commit­tare)
    dotenv().ok();

    // Definisci il router
    let app = Router::new()
        .route("/player/{tag}", get(get_player));

    // Bind TCP all’interfaccia 0.0.0.0:8080
    let addr: SocketAddr = "0.0.0.0:8080".parse().unwrap();
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Fallito il bind sulla porta 8080");

    println!("Server in ascolto su {}", listener.local_addr().unwrap());

    // Avvia il server: axum::serve gira sopra hyper internamente
    axum::serve(listener, app)
        .await
        .unwrap();
}

// Handler per /player/:tag
async fn get_player(Path(tag): Path<String>) -> impl IntoResponse {
    let token =
        std::env::var("BRAWLSTARS_TOKEN").expect("Token mancante");
    let url = format!(
        "https://api.brawlstars.com/v1/players/%23{}",
        tag
    );

    let client = Client::new();
    match client.get(&url).bearer_auth(token).send().await {
        Ok(resp) if resp.status().is_success() => {
            let body = resp.text().await.unwrap_or_default();
            (StatusCode::OK, body)
        }
        Ok(resp) => {
            let status = resp.status();
            (
                status,
                format!("Errore Brawl Stars: {}", status),
            )
        }
        Err(err) => (
            StatusCode::BAD_GATEWAY,
            format!("Errore reqwest: {}", err),
        ),
    }
}
