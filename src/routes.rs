use axum::extract::Path;
use axum::http::StatusCode;

pub async fn index() -> &'static str {
    "Welcome to imoog"
}

pub async fn deliver_file(Path(identifier): Path<String>) -> StatusCode {
    println!("{}", identifier);
    StatusCode::OK
}
