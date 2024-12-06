use actix_web::{web, HttpResponse, HttpRequest};
use serde::{Deserialize};
use crate::services::sesiones;
use lazy_static::lazy_static;
use std::env;

// Variable global para almacenar la API Key
lazy_static! {
    static ref API_KEY: String = env::var("X_API_KEY").unwrap_or_default();
}

// Función para verificar la API Key
fn check_api_key(req: &HttpRequest) -> bool {
    req.headers().get("api_key").map_or(false, |v| v == API_KEY.as_str())
}

// Estructuras de datos para solicitudes
#[derive(Deserialize, Debug)]
pub struct SesionRequest {
    curp: Option<String>,
    correo: Option<String>,
    celular: Option<String>,
    contrasena: Option<String>,
}


// Obtiene una sesión
pub async fn get_sesion(
    pool: web::Data<sqlx::MySqlPool>,
    req: HttpRequest,
    body: web::Json<SesionRequest>,
) -> HttpResponse {
    if !check_api_key(&req) {
        return HttpResponse::Unauthorized().json(serde_json::json!({ "code": 401, "message": "Falta api-key" }));
    }

    if body.curp.is_none() && body.correo.is_none() && body.celular.is_none() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "code": 400,
            "message": "Se requiere al menos un campo: curp, correo o celular"
        }));
    }

    log::info!("Recibida solicitud en /sesiones con body: {:?}", body);

match sesiones::get_sesion(
    &pool,
    body.curp.as_deref(),
    body.correo.as_deref(),
    body.celular.as_deref(),
    body.contrasena.as_deref(),
)
.await
{
    Ok(response) => {
        log::info!("Respuesta obtenida: {:?}", response);
        HttpResponse::Ok().json(response)
    }
    Err(err) => {
        log::error!("Error al obtener sesión: {:?}", err);
        HttpResponse::InternalServerError().json(serde_json::json!({ "message": err.to_string() }))
    }
    }
}

// Valida una credencial
pub async fn get_validacion(
    pool: web::Data<sqlx::MySqlPool>,
    req: HttpRequest,
    body: web::Json<SesionRequest>,
) -> HttpResponse {
    if !check_api_key(&req) {
        return HttpResponse::Unauthorized().json(serde_json::json!({ "code": 401, "message": "Falta api-key" }));
    }

    if body.curp.is_none() && body.correo.is_none() && body.celular.is_none() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "code": 400,
            "message": "Se requiere al menos un campo: curp, correo o celular"
        }));
    }

    match sesiones::get_validacion(
        &pool,
        body.curp.as_deref(),
        body.correo.as_deref(),
        body.celular.as_deref(),
    )
    .await
    {
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
    if !check_api_key(&req) {
        return HttpResponse::Unauthorized().json(serde_json::json!({ "code": 401, "message": "Falta api-key" }));
    }

    if body.curp.is_none() && body.correo.is_none() && body.celular.is_none() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "code": 400,
            "message": "Se requiere al menos un campo: curp, correo o celular"
        }));
    }

    match sesiones::get_autenticacion(
        &pool,
        body.curp.as_deref(),
        body.correo.as_deref(),
        body.celular.as_deref(),
    )
    .await
    {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({ "message": err.to_string() })),
    }
}
