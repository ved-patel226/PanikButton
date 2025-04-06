pub mod essentials {
    pub mod email;
}

use axum::{ http::StatusCode, Router };
use std::net::SocketAddr;
use tower_http::services::ServeFile;
use std::collections::HashMap;
use axum::{ extract::Query, response::IntoResponse };
use base64::{ engine::general_purpose::STANDARD as decode, Engine };
use essentials::email;
use std::env;

async fn submit_handler(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let student_id = params
        .get("studentId")
        .cloned()
        .unwrap_or_else(|| "".to_string());
    let problem = params
        .get("problem")
        .cloned()
        .unwrap_or_else(|| "".to_string());

    let student_id_decoded = decode.decode(student_id).expect("Error decoding studentId");
    let problem_decoded = decode.decode(problem).expect("Error decoding problem");

    // Convert the decoded bytes to a String
    let student_id_str = String::from_utf8(student_id_decoded).expect(
        "Error converting decoded studentId to String"
    );
    let problem_str = String::from_utf8(problem_decoded).expect(
        "Error converting decoded problem to String"
    );

    println!("studentId: {}, problem: {}", student_id_str, problem_str);

    let target_email = env::var("TARGET_EMAIL").expect("TARGET_EMAIL not set");
    let subject = format!("PANIKBUTTON - {}'s help is needed!", student_id_str);
    let body = format!("Here is their problem: \n{}", problem_str);

    email::send_email(&target_email, &subject, &body).expect("Err sending email");

    axum::response::Html(
        std::fs::read_to_string("assets/html/success.html").expect("Failed to read success.html")
    )
}

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

        .route(
            "/email",
            axum::routing
                ::get_service(ServeFile::new("assets/html/email.html"))
                .handle_error(|_| async { StatusCode::INTERNAL_SERVER_ERROR })
        )

        .route("/submit", axum::routing::get(submit_handler))

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
