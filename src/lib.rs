use std::net::SocketAddr;

use axum::Server;

mod routes;
mod middlewares;

use routes::create_routes;

pub async fn run() {
    let app = create_routes();
    
    let socket_addr = SocketAddr::from(([0,0,0,0], 3000));
    Server::bind(&socket_addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}
