pub const INSERT_CREDENCIAL: &str = r#"
    INSERT INTO Credenciales 
    (idCredencial, curp, nombre, primerApellido, segundoApellido, fechaNacimiento, estadoNacimiento, correo, celular, contrasena, tipo)
    VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);
"#;

pub const GET_CREDENCIAL: &str = r#"
    SELECT * FROM Credenciales WHERE idCredencial = ?;
"#;

pub const GET_CREDENCIALES: &str = r#"
    SELECT c.idCredencial, c.curp, c.nombre, c.primerApellido, c.segundoApellido, c.fechaNacimiento, c.estadoNacimiento,
    c.correo, c.celular, c.tipo, c.estado,
    GROUP_CONCAT(DISTINCT e.nombre SEPARATOR ',') AS etiquetas,
    GROUP_CONCAT(DISTINCT r.nombre SEPARATOR ',') AS roles,
    GROUP_CONCAT(DISTINCT g.nombre SEPARATOR ',') AS grupos
    FROM Credenciales c
    LEFT JOIN Etiquetas e ON c.idCredencial = e.idCredencial AND e.estado != 'Inactivo'
    LEFT JOIN Perfiles p ON c.idCredencial = p.idCredencial AND p.estado != 'Inactivo'
    LEFT JOIN Roles r ON p.idRol = r.idRol AND r.estado != 'Inactivo'
    LEFT JOIN Miembros m ON c.idCredencial = m.idCredencial AND m.estado != 'Inactivo'
    LEFT JOIN Grupos g ON m.idGrupo = g.idGrupo AND g.estado != 'Inactivo'
    GROUP BY c.idCredencial;
"#;

pub const DELETE_CREDENCIAL: &str = r#"
    UPDATE Credenciales SET estado = "Inactivo" WHERE idCredencial = ?;
"#;

pub const UPDATE_CREDENCIAL: &str = r#"
    CALL proc_update_credenciales(?, ?, ?, ?, ?, ?);
"#;
