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

// ✅ AGGIUNGI QUESTO IMPORT
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    // Carica .env (va creato localmente, ma non commit­tare)
    dotenv().ok();

    use axum::http::{HeaderValue, Method};
    let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:1420".parse::<HeaderValue>().unwrap(),
            "tauri://localhost".parse::<HeaderValue>().unwrap(),
            "capacitor://localhost".parse::<HeaderValue>().unwrap(),
        ])
        .allow_methods([Method::GET, Method::OPTIONS])
        .allow_headers(Any);

    // Definisci il router
    let app = Router::new()
        .route("/player/{tag}", get(get_player))
        .route("/player/{tag}/battlelog", get(get_battlelog))
        .route("/clubs/{tag}", get(get_club))
        .route("/brawlers", get(get_brawlers))
        .route("/brawlers/{id}", get(get_brawler_by_id))
        .route("/events", get(get_events))
        .route("/rankings/{code}/players", get(get_country_rankings_players))
        .route("/rankings/{code}/clubs", get(get_country_rankings_clubs))

        // ✅ APPICA IL CORS QUI
        .layer(cors);

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
async fn get_battlelog(Path(tag): Path<String>) -> impl IntoResponse {
    let token =
        std::env::var("BRAWLSTARS_TOKEN").expect("Token mancante");
    let url = format!(
        "https://api.brawlstars.com/v1/players/%23{}/battlelog",
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
async fn get_club(Path(tag): Path<String>) -> impl IntoResponse {
    let token =
        std::env::var("BRAWLSTARS_TOKEN").expect("Token mancante");
    let url = format!(
        "https://api.brawlstars.com/v1/clubs/%23{}",
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
// /brawlers
async fn get_brawlers() -> impl IntoResponse {
    let token = std::env::var("BRAWLSTARS_TOKEN").expect("Token mancante");
    let url = "https://api.brawlstars.com/v1/brawlers";

    let client = Client::new();
    match client.get(url).bearer_auth(token).send().await {
        Ok(resp) if resp.status().is_success() => {
            let body = resp.text().await.unwrap_or_default();
            (StatusCode::OK, body)
        }
        Ok(resp) => {
            let status = resp.status();
            (status, format!("Errore Brawl Stars: {}", status))
        }
        Err(err) => (
            StatusCode::BAD_GATEWAY,
            format!("Errore reqwest: {}", err),
        ),
    }
}

// /brawlers/:id
async fn get_brawler_by_id(Path(id): Path<u32>) -> impl IntoResponse {
    let token = std::env::var("BRAWLSTARS_TOKEN").expect("Token mancante");
    let url = format!("https://api.brawlstars.com/v1/brawlers/{}", id);

    let client = Client::new();
    match client.get(&url).bearer_auth(token).send().await {
        Ok(resp) if resp.status().is_success() => {
            let body = resp.text().await.unwrap_or_default();
            (StatusCode::OK, body)
        }
        Ok(resp) => {
            let status = resp.status();
            (status, format!("Errore Brawl Stars: {}", status))
        }
        Err(err) => (
            StatusCode::BAD_GATEWAY,
            format!("Errore reqwest: {}", err),
        ),
    }
}

// /events
async fn get_events() -> impl IntoResponse {
    let token = std::env::var("BRAWLSTARS_TOKEN").expect("Token mancante");
    let url = "https://api.brawlstars.com/v1/events/rotation";

    let client = Client::new();
    match client.get(url).bearer_auth(token).send().await {
        Ok(resp) if resp.status().is_success() => {
            let body = resp.text().await.unwrap_or_default();
            (StatusCode::OK, body)
        }
        Ok(resp) => {
            let status = resp.status();
            (status, format!("Errore Brawl Stars: {}", status))
        }
        Err(err) => (
            StatusCode::BAD_GATEWAY,
            format!("Errore reqwest: {}", err),
        ),
    }
}

// /rankings/:country_code/players
async fn get_country_rankings_players(Path(code): Path<String>) -> impl IntoResponse {
    let token = std::env::var("BRAWLSTARS_TOKEN").expect("Token mancante");
    let url = format!("https://api.brawlstars.com/v1/rankings/{}/players", code);

    let client = Client::new();
    match client.get(&url).bearer_auth(token).send().await {
        Ok(resp) if resp.status().is_success() => {
            let body = resp.text().await.unwrap_or_default();
            (StatusCode::OK, body)
        }
        Ok(resp) => {
            let status = resp.status();
            (status, format!("Errore Brawl Stars: {}", status))
        }
        Err(err) => (
            StatusCode::BAD_GATEWAY,
            format!("Errore reqwest: {}", err),
        ),
    }
}

// /rankings/:country_code/clubs
async fn get_country_rankings_clubs(Path(code): Path<String>) -> impl IntoResponse {
    let token = std::env::var("BRAWLSTARS_TOKEN").expect("Token mancante");
    let url = format!("https://api.brawlstars.com/v1/rankings/{}/clubs", code);

    let client = Client::new();
    match client.get(&url).bearer_auth(token).send().await {
        Ok(resp) if resp.status().is_success() => {
            let body = resp.text().await.unwrap_or_default();
            (StatusCode::OK, body)
        }
        Ok(resp) => {
            let status = resp.status();
            (status, format!("Errore Brawl Stars: {}", status))
        }
        Err(err) => (
            StatusCode::BAD_GATEWAY,
            format!("Errore reqwest: {}", err),
        ),
    }
}
