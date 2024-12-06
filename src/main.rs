mod routes;
mod controllers;
mod services;
mod models;
mod queries;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Configura el nivel de logs para SQLx y Actix Web
    env::set_var("RUST_LOG", "sqlx=debug,actix_web=info");
    env_logger::init();

    println!("Servidor iniciado en http://localhost:3001");

    HttpServer::new(|| {
        App::new()
            .configure(routes::credenciales::config) // Rutas de credenciales
            .configure(routes::sesiones::config) // Rutas de sesiones
            .configure(routes::codigos::config)      // Rutas de códigos
    })
    .bind("localhost:3001")? // Dirección y puerto del servidor
    .run()
    .await
}
