use actix_web::{web, HttpResponse, HttpRequest};
use crate::models::credenciales::Credencial;
use crate::services::credenciales;
use serde_json::json;

// Valida la API Key
fn validate_api_key(headers: &HttpRequest) -> Result<(), HttpResponse> {
    let api_key = headers
        .headers()
        .get("api_key")
        .and_then(|v| v.to_str().ok());
    if api_key != Some(std::env::var("X_API_KEY").unwrap_or_default().as_str()) {
        Err(HttpResponse::Unauthorized().json(json!({
            "code": 401,
            "message": "Falta api-key!",
        })))
    } else {
        Ok(())
    }
}

// Valida los datos de la credencial
fn validate_credencial_data(credencial: &Credencial) -> Result<(), HttpResponse> {
    if credencial.curp.trim().is_empty() || credencial.nombre.trim().is_empty() {
        Err(HttpResponse::BadRequest().json(json!({
            "code": 400,
            "message": "Curp y nombre son campos obligatorios!",
        })))
    } else if credencial.correo.trim().is_empty() || credencial.celular.trim().is_empty() {
        Err(HttpResponse::BadRequest().json(json!({
            "code": 400,
            "message": "Correo y celular son campos obligatorios!",
        })))
    } else {
        Ok(())
    }
}

// Inserta una nueva credencial
pub async fn insert_credencial(
    req_body: web::Json<Credencial>,
    headers: HttpRequest,
) -> HttpResponse {
    if let Err(err) = validate_api_key(&headers) {
        return err;
    }

    let credencial = req_body.into_inner();

    if let Err(err) = validate_credencial_data(&credencial) {
        return err;
    }

    match credenciales::insert_credencial(credencial).await {
        Ok(response) => HttpResponse::Created().json(json!({
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
