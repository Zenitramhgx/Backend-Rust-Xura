use actix_web::web;
use crate::controllers::sesiones::{get_sesion, delete_sesion, get_validacion, get_autenticacion, set_password};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/sesiones")
            .route("/", web::post().to(get_sesion))
            .route("/", web::delete().to(delete_sesion))
            .route("/is-valid", web::post().to(get_validacion))
            .route("/is-auth", web::post().to(get_autenticacion))
            .route("/{id_credencial}/set-contrasena", web::patch().to(set_password)),
    );
}
