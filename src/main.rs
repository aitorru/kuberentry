use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> Result<impl IntoResponse, (StatusCode, String)> {
    #[cfg(debug_assertions)]
    let html_contents = match std::fs::read_to_string("src/assets/index.html") {
        Ok(contents) => contents,
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read file".to_string(),
            ))
        }
    };

    #[cfg(not(debug_assertions))]
    let html_contents = include_str!("assets/index.html");

    Ok(axum::response::Html(html_contents))
}
