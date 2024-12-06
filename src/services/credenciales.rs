use crate::models::credenciales::Credencial;
use crate::queries::credenciales::INSERT_CREDENCIAL;
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
