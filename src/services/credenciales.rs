use crate::models::credenciales::Credencial;
use crate::queries::credenciales::{INSERT_CREDENCIAL, GET_CREDENCIAL, GET_CREDENCIALES, DELETE_CREDENCIAL, UPDATE_CREDENCIAL};
use sqlx::{mysql::MySqlPool};
use uuid::Uuid;
use bcrypt::{hash, DEFAULT_COST, verify};

pub async fn insert_credencial(
    credencial: Credencial,
) -> Result<Credencial, Box<dyn std::error::Error>> {
    let pool = MySqlPool::connect(&dotenv::var("DATABASE_URL")?).await?;
    let id_credencial = Uuid::new_v4().to_string();
    let salt = DEFAULT_COST;
    let hashed_password = hash(&credencial.contrasena, salt)?;

    sqlx::query(INSERT_CREDENCIAL)
        .bind(&id_credencial)
        .bind(&credencial.curp)
        .bind(&credencial.nombre)
        .bind(&credencial.primer_apellido)
        .bind(&credencial.segundo_apellido)
        .bind(&credencial.fecha_nacimiento)
        .bind(&credencial.estado_nacimiento)
        .bind(&credencial.correo)
        .bind(&credencial.celular)
        .bind(&hashed_password)
        .bind(&credencial.tipo)
        .execute(&pool)
        .await?;

    Ok(credencial)
}

pub async fn get_credencial(
    pool: &MySqlPool,
    id_credencial: &str,
) -> Result<Option<Credencial>, Box<dyn std::error::Error>> {
    let row = sqlx::query_as::<_, Credencial>(GET_CREDENCIAL)
        .bind(id_credencial)
        .fetch_optional(pool)
        .await?;

    Ok(row)
}

pub async fn get_credenciales(
    pool: &MySqlPool,
    filtros: Option<&str>,
    orden: Option<&str>,
    limite: Option<u32>,
    pagina: Option<u32>,
) -> Result<Vec<Credencial>, Box<dyn std::error::Error>> {
    let query = format!(
        "{} {} {}",
        GET_CREDENCIALES,
        filtros.unwrap_or(""),
        orden.unwrap_or("")
    );

    let offset = limite.unwrap_or(10) * pagina.unwrap_or(0);
    let credenciales = sqlx::query_as::<_, Credencial>(&query)
        .bind(offset)
        .bind(limite.unwrap_or(10))
        .fetch_all(pool)
        .await?;

    Ok(credenciales)
}

pub async fn delete_credencial(
    pool: &MySqlPool,
    id_credencial: &str,
) -> Result<u64, Box<dyn std::error::Error>> {
    let result = sqlx::query(DELETE_CREDENCIAL)
        .bind(id_credencial)
        .execute(pool)
        .await?;

    Ok(result.rows_affected())
}

pub async fn update_credencial(
    pool: &MySqlPool,
    id_credencial: &str,
    curp: &str,
    correo: &str,
    celular: &str,
    contrasena: &str,
    tipo: &str,
) -> Result<Option<Credencial>, Box<dyn std::error::Error>> {
    let hashed_password = if contrasena != "N1nguna" {
        hash(contrasena, DEFAULT_COST)?
    } else {
        contrasena.to_string()
    };

    let updated_credencial = sqlx::query_as::<_, Credencial>(UPDATE_CREDENCIAL)
        .bind(id_credencial)
        .bind(curp)
        .bind(correo)
        .bind(celular)
        .bind(hashed_password)
        .bind(tipo)
        .fetch_optional(pool)
        .await?;

    Ok(updated_credencial)
}
