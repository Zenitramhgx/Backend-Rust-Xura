use actix_web::web;
use crate::controllers::credenciales::{insert_credencial, get_credencial, get_credenciales};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/credenciales")
            .route("/", web::post().to(insert_credencial))
            .route("/", web::get().to(get_credenciales))
            .route("/{idCredencial}", web::get().to(get_credencial)),
    );
}
