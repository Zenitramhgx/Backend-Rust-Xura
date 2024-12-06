use actix_web::{web, HttpResponse};
use crate::models::credenciales::Credencial;
use crate::services::credenciales;

pub async fn insert_credencial(
    req_body: web::Json<Credencial>,
) -> HttpResponse {
    let credencial = req_body.into_inner();

    match credenciales::insert_credencial(credencial).await {
        Ok(response) => HttpResponse::Created().json(response),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
