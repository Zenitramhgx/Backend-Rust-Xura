use crate::models::credenciales::Credencial;
use crate::queries::credenciales::{
    INSERT_CREDENCIAL, GET_CREDENCIAL, GET_CREDENCIALES,
};
use sqlx::{mysql::MySqlPool};
use uuid::Uuid;
use bcrypt::{hash, DEFAULT_COST};

pub async fn insert_credencial(
    credencial: Credencial,
) -> Result<Credencial, Box<dyn std::error::Error>> {
    let pool = MySqlPool::connect(&dotenv::var("DATABASE_URL")?).await?;
    let id_credencial = Uuid::new_v4().to_string();
    let hashed_password = hash(&credencial.contrasena, DEFAULT_COST)?;

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
    curp: Option<&str>,
    correo: Option<&str>,
    celular: Option<&str>,
) -> Result<Option<Credencial>, Box<dyn std::error::Error>> {
    let celular_query = format!("%{}%", celular.unwrap_or_default());

    let result = sqlx::query_as::<_, Credencial>(GET_CREDENCIAL)
        .bind(curp.unwrap_or_default())
        .bind(correo.unwrap_or_default())
        .bind(celular_query)
        .fetch_optional(pool)
        .await?;

    Ok(result)
}

pub async fn get_credenciales(
    pool: &MySqlPool,
    filtros: Option<&str>,
    orden: Option<&str>,
    limite: Option<u32>,
    pagina: Option<u32>,
) -> Result<Vec<Credencial>, Box<dyn std::error::Error>> {
    let query = format!(
        "{} {} {} LIMIT ? OFFSET ?",
        GET_CREDENCIALES,
        filtros.unwrap_or(""),
        orden.unwrap_or("")
    );

    let offset = limite.unwrap_or(10) * pagina.unwrap_or(0);
    let credenciales = sqlx::query_as::<_, Credencial>(&query)
        .bind(limite.unwrap_or(10))
        .bind(offset)
        .fetch_all(pool)
        .await?;

    Ok(credenciales)
}
