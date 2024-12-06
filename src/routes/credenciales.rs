use actix_web::web;
use crate::controllers::credenciales::insert_credencial;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/credenciales")
            .route("/", web::post().to(insert_credencial))
    );
}
