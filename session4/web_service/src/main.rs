use axum::response::Html;
use axum::{
    routing::{get, post},
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(say_hello_html_file_with_tokio_file))
        .route("/json", get(hello_json))
        .route("/post", post(hello_post));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

use serde::Serialize;

#[derive(Serialize)]
struct HelloJson {
    message: String,
}

async fn hello_post() -> Html<String> {
    let result = Html("Hello from post 🦄".to_string());
    result
}

async fn hello_json() -> axum::Json<HelloJson> {
    let message = HelloJson {
        message: "Hi from JSON 🦄".to_string(),
    };
    let result = axum::Json(message);
    result
}

async fn say_hello_html_file_with_tokio_file() -> Html<String> {
    let path = std::path::Path::new("src/hello.html");
    let content = tokio::fs::read_to_string(path).await.unwrap();

    Html(content)
}

// async fn say_hello_html_file() -> Html<&'static str> {
//     const HTML_CONTENT: &str = include_str!("hello.html");
//     Html(HTML_CONTENT)
// }

// async fn say_hello_html() -> Html<&'static str> {
//     Html("<h1 style='color:red;'>Hello World Axum ! 🦀</h1>")
// }

// async fn say_hello_text() -> &'static str {
//     "Hello World Axum ! 🦀"
// }
