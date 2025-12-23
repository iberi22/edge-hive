---
title: "Implementar Edge Function para validación de login"
labels:
  - enhancement
  - edge-functions
  - wasm
  - jules
assignees: []
priority: medium
---

## Descripción
Crear una Edge Function de ejemplo en WASM que valide credenciales de login, demostrando la capacidad de ejecutar lógica custom en el edge.

## Contexto
Las Edge Functions permiten a los usuarios deployar lógica de validación custom sin modificar el core del sistema. Este issue implementa:
1. Una función de validación de login como ejemplo
2. Host functions para acceso a BD desde WASM
3. Integración con el handler de login

## Archivos a Modificar
- `crates/edge-hive-wasm/src/runtime.rs`
- `crates/edge-hive-wasm/src/lib.rs`
- `crates/edge-hive-api/src/handlers/edge.rs`
- Nuevo: `examples/edge-functions/login-validator/`

## Dependencias
- **Requiere:** `FEAT_login-handler-real` completado

## Implementación Requerida

### 1. Definir interfaz de host functions

```rust
// En runtime.rs
use wasmtime::*;

pub struct HostFunctions {
    db: Arc<DatabaseService>,
}

impl HostFunctions {
    pub fn add_to_linker(linker: &mut Linker<HostState>) -> Result<()> {
        // Función para consultar BD
        linker.func_wrap("edge_hive", "db_query", |caller: Caller<'_, HostState>, sql_ptr: i32, sql_len: i32| -> i32 {
            // Leer string SQL de memoria WASM
            // Ejecutar query
            // Escribir resultado en memoria WASM
            // Retornar puntero a resultado
            0
        })?;

        // Función para logging
        linker.func_wrap("edge_hive", "log", |caller: Caller<'_, HostState>, level: i32, msg_ptr: i32, msg_len: i32| {
            // Leer mensaje de memoria WASM
            // Loggear con tracing
        })?;

        Ok(())
    }
}
```

### 2. Crear ejemplo de Edge Function en Rust

```rust
// examples/edge-functions/login-validator/src/lib.rs
#![no_std]

extern "C" {
    fn db_query(sql_ptr: *const u8, sql_len: i32) -> i32;
    fn log(level: i32, msg_ptr: *const u8, msg_len: i32);
}

#[no_mangle]
pub extern "C" fn validate_login(email_ptr: *const u8, email_len: i32) -> i32 {
    // 1. Leer email de memoria
    // 2. Verificar reglas custom (ej: solo dominios permitidos)
    // 3. Retornar 1 si válido, 0 si no

    // Ejemplo: solo permitir emails de @mycompany.com
    let email = unsafe {
        core::slice::from_raw_parts(email_ptr, email_len as usize)
    };

    if email.ends_with(b"@mycompany.com") {
        1
    } else {
        0
    }
}
```

### 3. Compilar y testear

```bash
# Compilar a WASM
cd examples/edge-functions/login-validator
cargo build --target wasm32-unknown-unknown --release

# Copiar a directorio de funciones
cp target/wasm32-unknown-unknown/release/login_validator.wasm \
   ~/.edge-hive/edge-functions/
```

### 4. Integrar con login handler

```rust
// En handlers/auth.rs
pub async fn login(/* ... */) -> Result</* ... */> {
    // Antes de validar password, ejecutar edge function si existe
    if let Some(validator) = state.wasm_runtime.get_function("login-validator") {
        let result = validator.call("validate_login", &[email.as_ptr(), email.len()])?;
        if result == 0 {
            return Err((StatusCode::FORBIDDEN, Json(ErrorResponse {
                error: "Login blocked by custom rule".into()
            })));
        }
    }

    // Continuar con login normal...
}
```

## Criterios de Aceptación
- [ ] Host functions definidas e implementadas
- [ ] Ejemplo `login-validator` compila a WASM
- [ ] Edge function se ejecuta durante login
- [ ] Ejemplo demuestra validación custom
- [ ] Documentación de cómo crear edge functions

## Tests Requeridos
```rust
#[tokio::test]
async fn test_edge_function_execution() {
    // Cargar WASM, ejecutar función, verificar resultado
}

#[tokio::test]
async fn test_login_with_edge_validation() {
    // Verificar que edge function bloquea emails no permitidos
}
```

## Notas
Esta es una feature más avanzada. Prioriza las issues de BD y auth primero.
