use actix_web::{web, HttpResponse};
use crate::models::credenciales::Credencial;
use crate::services::credenciales;
use serde_json::json;
use sqlx::MySqlPool;

// Obtiene una credencial por ID
pub async fn get_credencial(
    pool: web::Data<MySqlPool>,
    id_credencial: web::Path<String>,
) -> HttpResponse {
    match credenciales::get_credencial(&pool, Some(&id_credencial), None, None).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({ "message": err.to_string() })),
    }       
}

// Obtiene una lista de credenciales con filtros
pub async fn get_credenciales(
    pool: web::Data<MySqlPool>,
    req_body: web::Json<serde_json::Value>,
) -> HttpResponse {
    let filtros = req_body.get("filtros").and_then(|v| v.as_str());
    let orden = req_body.get("orden").and_then(|v| v.as_str());
    let limite = req_body.get("limite").and_then(|v| v.as_u64()).map(|v| v as u32);
    let pagina = req_body.get("pagina").and_then(|v| v.as_u64()).map(|v| v as u32);

    match credenciales::get_credenciales(&pool, filtros, orden, limite, pagina).await {
        Ok(response) if !response.is_empty() => HttpResponse::Ok().json(response),
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(err) => HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": err.to_string(),
        })),
    }
}

// Inserta una nueva credencial
pub async fn insert_credencial(
    req_body: web::Json<Credencial>,
    headers: actix_web::HttpRequest,
) -> HttpResponse {
    let api_key = headers.headers().get("api_key").and_then(|v| v.to_str().ok());
    if api_key != Some(std::env::var("X_API_KEY").unwrap_or_default().as_str()) {
        return HttpResponse::Unauthorized().json(json!({
            "code": 401,
            "message": "Falta api-key!",
        }));
    }

    let credencial = req_body.into_inner();
    match credenciales::insert_credencial(credencial).await {
        Ok(response) => HttpResponse::Created().json(json!({
            "idCredencial": response.id_credencial,
            "curp": response.curp,
            "nombre": response.nombre,
            "primerApellido": response.primer_apellido,
            "segundoApellido": response.segundo_apellido,
            "correo": response.correo,
            "celular": response.celular,
            "tipo": response.tipo,
        })),
        Err(err) => HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": err.to_string(),
        })),
    }
}
