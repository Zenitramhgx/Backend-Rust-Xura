pub const GET_CREDENCIAL: &str = "SELECT * FROM Credenciales WHERE curp = ? OR correo = ? OR celular LIKE ?";
pub const DELETE_SESION: &str = "DELETE FROM Codigos WHERE idCredencial = ? AND tipo = 'Autenticación'";
pub const UPDATE_CONTRASENA: &str = "CALL proc_updatePass_credenciales(?, ?);";
