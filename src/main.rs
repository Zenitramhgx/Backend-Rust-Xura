mod routes;
mod controllers;
mod services;
mod models;
mod queries;

use actix_web::{App, HttpServer};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .configure(routes::credenciales::config) // Rutas de credenciales
            .configure(routes::codigos::config)      // Rutas de códigos
    })
    .bind("127.0.0.1:8080")? // Dirección y puerto del servidor
    .run()
    .await
}
