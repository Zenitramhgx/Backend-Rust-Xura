use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Codigo {
    #[sqlx(rename = "id_codigo_db")]
    pub id_codigo: String,
    pub id_credencial: String,
    pub clave: String,
    pub medio: String,
    pub tipo: String,
    pub estado: String,
}