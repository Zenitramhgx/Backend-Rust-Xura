use actix_web::{web, HttpResponse, HttpRequest};
use serde::{Deserialize};
use crate::services::codigos;

#[derive(Deserialize)]
pub struct InsertCodigoRequest {
    tipo: String,
    medio: String,
}

#[derive(Deserialize)]
pub struct ValidarCodigoRequest {
    medio: String,
    tipo: String,
}

// Obtiene un código por ID
pub async fn get_codigo(
    pool: web::Data<sqlx::MySqlPool>,
    path: web::Path<String>,
) -> HttpResponse {
    let id_codigo = path.into_inner();

    match codigos::get_codigo(&pool, &id_codigo).await {
        Ok(Some(codigo)) => HttpResponse::Ok().json(codigo),
        Ok(None) => HttpResponse::NoContent().finish(),
        Err(err) => HttpResponse::InternalServerError().json({
            serde_json::json!({
                "code": 500,
                "message": err.to_string(),
            })
        }),
    }
}

// Obtiene una lista de códigos con filtros opcionales
pub async fn get_codigos(
    pool: web::Data<sqlx::MySqlPool>,
    params: web::Query<std::collections::HashMap<String, String>>,
) -> HttpResponse {
    let filtros = params.get("filtros").cloned();
    let orden = params.get("orden").cloned();
    let limite = params.get("limite").and_then(|s| s.parse::<u32>().ok());
    let pagina = params.get("pagina").and_then(|s| s.parse::<u32>().ok());

    match codigos::get_codigos(&pool, filtros.as_deref(), orden.as_deref(), limite, pagina).await {
        Ok(codigos) if !codigos.is_empty() => HttpResponse::Ok().json(codigos),
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(err) => HttpResponse::InternalServerError().json({
            serde_json::json!({
                "code": 500,
                "message": err.to_string(),
            })
        }),
    }
}

// Elimina un código por ID de credencial
pub async fn delete_codigo(
    pool: web::Data<sqlx::MySqlPool>,
    path: web::Path<String>,
) -> HttpResponse {
    let id_credencial = path.into_inner();

    match codigos::delete_codigo(&pool, &id_credencial).await {
        Ok(affected_rows) => HttpResponse::NoContent().json({
            serde_json::json!({ "affectedRows": affected_rows })
        }),
        Err(err) => HttpResponse::InternalServerError().json({
            serde_json::json!({
                "code": 500,
                "message": err.to_string(),
            })
        }),
    }
}

// Inserta un nuevo código
pub async fn insert_codigo(
    pool: web::Data<sqlx::MySqlPool>,
    path: web::Path<String>,
    req: HttpRequest, // Cambiado a HttpRequest
    body: web::Json<InsertCodigoRequest>,
) -> HttpResponse {
    let id_credencial = path.into_inner();
    let x_api_key = req
        .headers()
        .get("api_key")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if x_api_key.is_empty() || x_api_key != std::env::var("X_API_KEY").unwrap_or_default() {
        return HttpResponse::Unauthorized().json({
            serde_json::json!({
                "code": 401,
                "message": "Falta api-key o no es válida",
            })
        });
    }

    match codigos::insert_codigo(&pool, &id_credencial, &body.tipo, &body.medio).await {
        Ok(codigo) => HttpResponse::Created().json({
            serde_json::json!({
                "idCredencial": codigo.id_credencial,
                "medio": codigo.medio,
                "tipo": codigo.tipo,
            })
        }),
        Err(err) => HttpResponse::InternalServerError().json({
            serde_json::json!({
                "code": 500,
                "message": err.to_string(),
            })
        }),
    }
}

// Valida un código
pub async fn validar_codigo(
    pool: web::Data<sqlx::MySqlPool>,
    path: web::Path<(String, String)>,
    req: HttpRequest, // Cambiado a HttpRequest
    body: web::Json<ValidarCodigoRequest>,
) -> HttpResponse {
    let (id_credencial, codigo) = path.into_inner();
    let x_api_key = req
        .headers()
        .get("api_key")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if x_api_key.is_empty() || x_api_key != std::env::var("X_API_KEY").unwrap_or_default() {
        return HttpResponse::Unauthorized().json({
            serde_json::json!({
                "code": 401,
                "message": "Falta api-key o no es válida",
            })
        });
    }

    match codigos::validar_codigo(&pool, &id_credencial, &codigo, &body.medio, &body.tipo).await {
        Ok(_) => HttpResponse::Ok().json({
            serde_json::json!({
                "estatus": "OK",
            })
        }),
        Err(err) => HttpResponse::InternalServerError().json({
            serde_json::json!({
                "code": 500,
                "message": err.to_string(),
            })
        }),
    }
}
