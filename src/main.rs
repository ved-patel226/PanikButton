pub mod panikbutton {
    pub mod email;
}

use axum::{ http::StatusCode, Router };
use std::net::SocketAddr;
use tower_http::services::ServeFile;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    // Build our application router
    let app = Router::new()
        .route(
            "/",
            axum::routing
                ::get_service(ServeFile::new("assets/html/index.html"))
                .handle_error(|_| async { StatusCode::INTERNAL_SERVER_ERROR })
        )
        .nest_service(
            "/css",
            axum::routing
                ::get_service(tower_http::services::ServeDir::new("assets/css"))
                .handle_error(|_| async { StatusCode::INTERNAL_SERVER_ERROR })
        );

    // Set up the server address
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);

    // Setup TCP listener
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    // Serve
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
