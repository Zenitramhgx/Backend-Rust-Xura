use sqlx::{mysql::MySqlPool, Error};
use crate::queries::sesiones::{GET_CREDENCIAL};
use serde_json::json;
use serde::Serialize;

#[derive(sqlx::FromRow, Serialize, Debug)]
pub struct Credencial {
    id_credencial: String,
    correo: String,
    celular: String,
}

pub async fn get_sesion(
    pool: &MySqlPool,
    curp: Option<&str>,
    correo: Option<&str>,
    celular: Option<&str>,
    _contrasena: Option<&str>,
) -> Result<serde_json::Value, Error> {
    let celular_query = celular.map_or("%%".to_string(), |c| format!("%{}%", c));

    log::info!("Ejecutando query con curp: {:?}, correo: {:?}, celular: {:?}", curp, correo, celular);

    let query = sqlx::query_as::<_, Credencial>(GET_CREDENCIAL)
        .bind(curp.unwrap_or_default())
        .bind(correo.unwrap_or_default())
        .bind(celular_query);

    let result = query.fetch_optional(pool).await;

    log::info!("Resultado de la query: {:?}", result);

    match result {
        Ok(Some(record)) => Ok(json!({
            "statusCode": 0,
            "idCredencial": record.id_credencial,
            "correo": record.correo,
            "celular": record.celular
        })),
        Ok(None) => Ok(json!({
            "statusCode": 204,
            "message": "No content"
        })),
        Err(err) => {
            log::error!("Error al ejecutar query: {:?}", err);
            Err(err)
        }
    }
}

pub async fn get_validacion(
    pool: &MySqlPool,
    curp: Option<&str>,
    correo: Option<&str>,
    celular: Option<&str>,
) -> Result<serde_json::Value, Error> {
    get_sesion(pool, curp, correo, celular, None).await
}

pub async fn get_autenticacion(
    pool: &MySqlPool,
    curp: Option<&str>,
    correo: Option<&str>,
    celular: Option<&str>,
) -> Result<serde_json::Value, Error> {
    get_sesion(pool, curp, correo, celular, None).await
}
