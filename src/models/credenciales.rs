use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Credencial {
    pub curp: String,
    pub nombre: String,
    pub primer_apellido: String,
    pub segundo_apellido: String,
    pub fecha_nacimiento: String,
    pub estado_nacimiento: String,
    pub correo: String,
    pub celular: String,
    pub contrasena: String,
    pub tipo: String,
}

