use actix_web::web;
use crate::controllers::codigos::{get_codigo, get_codigos, delete_codigo, insert_codigo, validar_codigo};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/codigos")
            .route("/{id_codigo}", web::get().to(get_codigo)) // Obtiene un código por ID
            .route("/", web::get().to(get_codigos)) // Obtiene una lista de códigos
            .route("/{id_credencial}", web::delete().to(delete_codigo)) // Elimina un código por ID de credencial
            .route("/{id_credencial}", web::post().to(insert_codigo)) // Inserta un nuevo código
            .route("/{id_credencial}/{codigo}", web::post().to(validar_codigo)) // Valida un código
    );
}
