use axum::{Router, routing::{post, get, patch}, http::Method, Extension, middleware};

pub mod routes;

use routes::*;
use tower_http::cors::{CorsLayer, Any};

use crate::middlewares;




#[derive(Clone)]
pub struct SharedData {
    message: String
}




pub fn create_routes() -> Router<()> {

    let shared_data = SharedData {message: "katto rani!".to_string()};



    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);
    Router::new()
        .route("/custom-header", get(read_custom_header))
        .route_layer(middleware::from_fn(middlewares::custom_header::read_custom_header_middleware))
        
        .route("/auth", get(auth_route))
        .route_layer(middleware::from_fn(middlewares::auth::auth_user))
        
        .route("/", get(hello_world))
        .route("/json", post(extract_json))
        .route("/path/:age/:name", get(handle_path))
        .route("/query", get(handle_query))
        .route("/header", get(extract_user_agent_header))
        .route("/header", post(extract_content_type_header))
        .route("/header", patch(extract_custom_header))
        .route("/shared_data", get(extract_shared_data))
        .route("/custom-201", get(custom_201_response))
        .route("/get_json", get(get_json))
        .route("/json-by-extractor", get(json_by_extractor))

        .layer(Extension(shared_data))
        .layer(cors)
}   
