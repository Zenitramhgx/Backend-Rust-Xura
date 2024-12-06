use actix_web::{web, HttpResponse, HttpRequest};
use serde::{Deserialize, Serialize};
use crate::services::sesiones;
use std::env;

#[derive(Deserialize)]
pub struct SesionRequest {
    curp: Option<String>,
    correo: Option<String>,
    celular: Option<String>,
    contrasena: Option<String>,
}

#[derive(Deserialize)]
pub struct PasswordRequest {
    curp: String,
    correo: String,
    celular: String,
    contrasena: String,
}

// Obtiene una sesión
pub async fn get_sesion(
    pool: web::Data<sqlx::MySqlPool>,
    req: HttpRequest,
    body: web::Json<SesionRequest>,
) -> HttpResponse {
    let x_api_key = req.headers().get("api_key").and_then(|v| v.to_str().ok());
    if x_api_key != Some(&env::var("X_API_KEY").unwrap_or_default()) {
        return HttpResponse::Unauthorized().json(serde_json::json!({ "code": 401, "message": "Falta api-key" }));
    }

    match sesiones::get_sesion(&pool, &body.curp, &body.correo, &body.celular, body.contrasena.as_deref()).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({ "message": err.to_string() })),
    }
}

// Valida una credencial
pub async fn get_validacion(
    pool: web::Data<sqlx::MySqlPool>,
    req: HttpRequest,
    body: web::Json<SesionRequest>,
) -> HttpResponse {
    let x_api_key = req.headers().get("api_key").and_then(|v| v.to_str().ok());
    if x_api_key != Some(&env::var("X_API_KEY").unwrap_or_default()) {
        return HttpResponse::Unauthorized().json(serde_json::json!({ "code": 401, "message": "Falta api-key" }));
    }

    match sesiones::get_validacion(&pool, &body.curp, &body.correo, &body.celular).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({ "message": err.to_string() })),
    }
}

// Autentica una credencial
pub async fn get_autenticacion(
    pool: web::Data<sqlx::MySqlPool>,
    req: HttpRequest,
    body: web::Json<SesionRequest>,
) -> HttpResponse {
    let x_api_key = req.headers().get("api_key").and_then(|v| v.to_str().ok());
    if x_api_key != Some(&env::var("X_API_KEY").unwrap_or_default()) {
        return HttpResponse::Unauthorized().json(serde_json::json!({ "code": 401, "message": "Falta api-key" }));
    }

    match sesiones::get_autenticacion(&pool, &body.curp, &body.correo, &body.celular).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({ "message": err.to_string() })),
    }
}

// Elimina una sesión
pub async fn delete_sesion(
    pool: web::Data<sqlx::MySqlPool>,
    path: web::Path<String>,
) -> HttpResponse {
    let id_credencial = path.into_inner();
    match sesiones::delete_sesion(&pool, &id_credencial).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({ "message": err.to_string() })),
    }
}

// Establece una nueva contraseña
pub async fn set_password(
    pool: web::Data<sqlx::MySqlPool>,
    req: HttpRequest,
    path: web::Path<String>,
    body: web::Json<PasswordRequest>,
) -> HttpResponse {
    let id_credencial = path.into_inner();
    let x_api_key = req.headers().get("api_key").and_then(|v| v.to_str().ok());
    if x_api_key != Some(&env::var("X_API_KEY").unwrap_or_default()) {
        return HttpResponse::Unauthorized().json(serde_json::json!({ "code": 401, "message": "Falta api-key" }));
    }

    match sesiones::set_password(&pool, &id_credencial, &body.curp, &body.correo, &body.celular, &body.contrasena).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(err) => HttpResponse::InternalServerError().json({ serde_json::json!({ "message": err.to_string() }) }),
    }     
}
