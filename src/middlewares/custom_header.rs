use axum::{http::{Request, StatusCode}, middleware::Next, response::Response};

use crate::routes::routes::HeaderMessage;

pub async fn read_custom_header_middleware<T>(mut req: Request<T>, next: Next<T>) -> Result<Response, StatusCode> {

    let custome_header = req.headers()
        .get("custom-header").cloned();

    match custome_header {
        Some(value) => {
            req.extensions_mut().insert(HeaderMessage(value.to_str().unwrap().to_string()));
            Ok(next.run(req).await)
        },
        None => Err(StatusCode::BAD_REQUEST)
    }
}