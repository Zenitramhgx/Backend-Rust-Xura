use sqlx::MySqlPool;
use std::error::Error;

pub async fn get_sesion(
    _pool: &MySqlPool,
    _curp: &Option<String>,
    _correo: &Option<String>,
    _celular: &Option<String>,
    _contrasena: Option<&str>,
) -> Result<String, Box<dyn Error>> {
    Ok("Sesión válida".to_string())
}
