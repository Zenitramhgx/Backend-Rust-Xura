pub const GET_CODIGOS: &str = r#"
    SELECT * FROM Codigos;
"#;

pub const GET_CODIGO: &str = r#"
    SELECT * FROM Codigos WHERE idCodigo = ?;
"#;

pub const DELETE_CODIGO: &str = r#"
    CALL proc_delete_codigos(?);
"#;

pub const INSERT_CODIGO: &str = r#"
    CALL proc_insert_codigos(?, ?, ?);
"#;

// pub const UPDATE_CODIGO: &str = r#"
//     CALL proc_update_codigos(?, ?);
// "#;

pub const VALIDAR_CODIGO: &str = r#"
    SELECT * FROM Codigos 
    WHERE idCredencial = ? AND clave = ? AND medio = ? AND tipo = ?;
"#;

pub const CONFIRMAR_CODIGO: &str = r#"
    UPDATE Codigos SET estado = 'Confirmado' WHERE idCodigo = ?;
"#;
