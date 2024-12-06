# **Backend Rust Xura**

## **Estructura del Proyecto**

```plaintext
src/
├── controllers/
│   ├── credenciales.rs
│   ├── mod.rs
├── models/
│   ├── credenciales.rs
│   ├── mod.rs
├── queries/
│   ├── credenciales.rs
│   ├── mod.rs
├── routes/
│   ├── credenciales.rs
│   ├── mod.rs
├── services/
│   ├── credenciales.rs
│   ├── mod.rs
├── main.rs
```

## **Orden de Implementación**

Sigue este orden para estructurar y desarrollar las diferentes capas del proyecto:

1. **`queries`**:
   - Define los procedimientos almacenados y consultas SQL para interactuar con la base de datos.

2. **`services`**:
   - Implementa la lógica de negocio y conexión a la base de datos, utilizando los queries definidos.

3. **`controllers`**:
   - Define las funciones que procesan las solicitudes HTTP. Aquí se valida la entrada y se interactúa con los servicios.

4. **`models`**:
   - Define las estructuras de datos utilizadas en todo el proyecto, como las respuestas y las solicitudes.

5. **`routes`**:
   - Configura las rutas de la API y enlázalas con los controladores correspondientes.

6. **`main`**:
   - Configura y arranca el servidor HTTP, y registra las rutas definidas.


## **Descripción de los Archivos**

### **`queries/credenciales.rs`**
Define los queries SQL y procedimientos almacenados:
- **`INSERT_CREDENCIAL`**: Procedimiento almacenado que inserta una nueva credencial en la base de datos.

---

### **`services/credenciales.rs`**
Contiene la lógica de negocio:
- **`insert_credencial`**:
  - Inserta una nueva credencial en la base de datos.
  - **Programación funcional**: Utiliza combinadores como `?` y `Ok()` para manejar resultados y errores.

---

### **`controllers/credenciales.rs`**
Procesa las solicitudes HTTP relacionadas con credenciales:
- **`validate_api_key`**:
  - Valida la API Key en las cabeceras de la solicitud.
  - **Programación funcional**: Devuelve un `Result` y utiliza combinadores como `Ok()` y `Err()`.

- **`validate_credencial_data`**:
  - Valida que los datos de la credencial sean correctos.
  - **Programación funcional**: Similar a la validación de la API Key, asegura la consistencia de los datos.

- **`insert_credencial`**:
  - Procesa la solicitud para insertar una nueva credencial y responde con los datos insertados.

---

### **`models/credenciales.rs`**
Define la estructura de las credenciales:
- Representa los datos en Rust con `struct Credencial`.
- Usa `derive` para habilitar características como `Debug`, `Serialize`, y `Deserialize`.

---

### **`routes/credenciales.rs`**
Configura la ruta **`/credenciales`**:
- Registra el controlador **`insert_credencial`**.

---

### **`main.rs`**
Configura y arranca el servidor:
- Define las configuraciones globales, como el registro de logs.
- Asocia las rutas de la API con sus controladores.


# **Cómo Ejecutar el Proyecto**

## **1. Instalar Dependencias**
Ejecuta el siguiente comando en el directorio del proyecto para instalar las dependencias:
```bash
cargo build
```

## **2. Ejecutar el Servidor**
```bash
cargo run
```

## **3. Configurar Variables de Entorno**
Crea un archivo .env en la raíz del proyecto con el siguiente contenido:
```plaintext
DATABASE_URL=mysql://usuario:contraseña@localhost/db_name
X_API_KEY=tu_api_key_aqui
```