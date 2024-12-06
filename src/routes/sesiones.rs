use actix_web::web;
use crate::controllers::sesiones::{get_sesion, get_validacion, get_autenticacion};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/sesiones")
            .route("/", web::post().to(get_sesion))
            .route("/is-valid", web::post().to(get_validacion))
            .route("/is-auth", web::post().to(get_autenticacion))
    );
}
