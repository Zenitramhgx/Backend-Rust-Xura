use crate::models::{codigos::Codigo, credenciales::Credencial};
use crate::queries::codigos::{GET_CODIGO, GET_CODIGOS, DELETE_CODIGO, INSERT_CODIGO, VALIDAR_CODIGO, CONFIRMAR_CODIGO};
use crate::services::credenciales::get_credencial;
use sqlx::mysql::MySqlPool;
use sqlx::Row;
use std::error::Error;

pub async fn get_codigo(pool: &MySqlPool, id_codigo: &str) -> Result<Option<Codigo>, Box<dyn Error>> {
    let row = sqlx::query_as::<_, Codigo>(GET_CODIGO)
        .bind(id_codigo)
        .fetch_optional(pool)
        .await?;
    Ok(row)
}

pub async fn get_codigos(
    pool: &MySqlPool,
    filtros: Option<&str>,
    orden: Option<&str>,
    limite: Option<u32>,
    pagina: Option<u32>,
) -> Result<Vec<Codigo>, Box<dyn Error>> {
    let mut query = GET_CODIGOS.to_string();

    if let Some(filter) = filtros {
        query = format!("{} WHERE {}", query, filter);
    }

    if let Some(order) = orden {
        query = format!("{} ORDER BY {}", query, order);
    }

    if let Some(limit) = limite {
        query = format!("{} LIMIT {}", query, limit);
    }

    if let Some(page) = pagina {
        query = format!("{} OFFSET {}", query, (page - 1) * limite.unwrap_or(10));
    }

    let rows = sqlx::query_as::<_, Codigo>(&query)
        .fetch_all(pool)
        .await?;
    Ok(rows)
}

pub async fn delete_codigo(pool: &MySqlPool, id_credencial: &str) -> Result<u64, Box<dyn Error>> {
    let result = sqlx::query(DELETE_CODIGO)
        .bind(id_credencial)
        .execute(pool)
        .await?;
    Ok(result.rows_affected())
}

pub async fn insert_codigo(
    pool: &MySqlPool,
    id_credencial: &str,
    tipo: &str,
    medio: &str,
) -> Result<Codigo, Box<dyn Error>> {
    let _data = get_credencial(pool, id_credencial).await?;
    let row = sqlx::query_as::<_, Codigo>(INSERT_CODIGO)
        .bind(id_credencial)
        .bind(tipo)
        .bind(medio)
        .fetch_one(pool)
        .await?;

    // Si necesitas lógica adicional para manejar "Correo" o "Celular", agrégala aquí.
    Ok(row)
}

pub async fn validar_codigo(
    pool: &MySqlPool,
    id_credencial: &str,
    clave: &str,
    medio: &str,
    tipo: &str,
) -> Result<Codigo, Box<dyn Error>> {
    let row = sqlx::query_as::<_, Codigo>(VALIDAR_CODIGO)
        .bind(id_credencial)
        .bind(clave)
        .bind(medio)
        .bind(tipo)
        .fetch_one(pool)
        .await?;

    if row.estado == "Pendiente" {
        sqlx::query(CONFIRMAR_CODIGO)
            .bind(&row.id_codigo)
            .execute(pool)
            .await?;
    }

    Ok(row)
}
