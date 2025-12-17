# Edge Hive Android App

Native Android app (Jetpack Compose) para controlar tu nodo Edge Hive desde tu dispositivo móvil.

## 🚀 Features

- ✅ OAuth2 automático (crea cliente si no existe)
- ✅ Ver estado del nodo (health, status)
- ✅ Listar edge functions
- ✅ Ejecutar edge functions
- ✅ Crear nuevas edge functions vía MCP
- ✅ Material Design 3 (Material You)
- ✅ Dark mode support

## 📱 Requisitos

- Android Studio Hedgehog (2023.1.1) o superior
- Android SDK 24+ (Android 7.0)
- Kotlin 1.9.22+
- Gradle 8.2+

## 🛠️ Setup

### 1. Clonar el proyecto

Ya está en `mobile/android/EdgeHiveApp/`

### 2. Abrir en Android Studio

1. Abre Android Studio
2. File → Open → Selecciona carpeta `EdgeHiveApp`
3. Espera a que Gradle sincronice

### 3. Configurar URL del backend

Por defecto, la app apunta a `http://10.0.2.2:8080` (emulador Android = localhost).

**Para dispositivo real:**

Edita `MainActivity.kt` línea 22:

```kotlin
edgeHiveClient = EdgeHiveClient(
    baseUrl = "http://192.168.1.100:8080", // Cambia a tu IP local
    clientId = "",
    clientSecret = ""
)
```

**Para usar con Cloudflare Tunnel:**

```kotlin
edgeHiveClient = EdgeHiveClient(
    baseUrl = "https://tu-subdominio.trycloudflare.com",
    clientId = "",
    clientSecret = ""
)
```

### 4. Compilar

```bash
# Desde Android Studio:
# Build → Make Project (Ctrl+F9)

# O desde terminal:
cd mobile/android/EdgeHiveApp
./gradlew assembleDebug
```

### 5. Instalar en dispositivo/emulador

```bash
# Vía Android Studio:
# Run → Run 'app' (Shift+F10)

# O vía adb:
adb install app/build/outputs/apk/debug/app-debug.apk
```

## 📖 Uso

### Emulador Android

1. Asegúrate de que tu servidor Edge Hive esté corriendo en `localhost:8080`
2. La app usará automáticamente `10.0.2.2:8080` (el localhost del host)
3. Lanza la app

### Dispositivo físico

1. Encuentra tu IP local: `ipconfig` (Windows) o `ip addr` (Linux)
2. Cambia `baseUrl` en `MainActivity.kt` a tu IP (ej: `http://192.168.1.100:8080`)
3. Asegúrate de que firewall permita conexiones en puerto 8080
4. Instala y ejecuta

### Navegación

**Pantallas:**

- **Status** 📊 - Ver salud del nodo + status detallado
- **Functions** 🔧 - Listar y ejecutar edge functions
- **Create** ➕ - Crear nuevas edge functions con template JSON

**Flujo básico:**

1. Abre la app → Status se carga automáticamente
2. Tab "Functions" → lista funciones existentes → tap para ejecutar
3. Tab "Create" → escribe nombre + JSON template → "Create Function"

## 🔐 OAuth2

La app crea automáticamente un cliente OAuth2 si `clientId` está vacío.

**Para usar credenciales pre-creadas:**

```bash
# En tu servidor VPS, crea un cliente:
docker exec edge-hive-node1 edge-hive auth client create --name android-app

# Obtienes: client_id y client_secret
# Cópialos a MainActivity.kt:
edgeHiveClient = EdgeHiveClient(
    baseUrl = "...",
    clientId = "cli_abc123...",
    clientSecret = "sec_xyz789..."
)
```

## 🧪 Testing

```bash
# Unit tests
./gradlew test

# Instrumented tests (en emulador/dispositivo)
./gradlew connectedAndroidTest
```

## 📦 Build Release

```bash
# Generar APK firmado (release)
./gradlew assembleRelease

# APK estará en:
# app/build/outputs/apk/release/app-release.apk
```

**Para firmar:**

1. Genera keystore: `keytool -genkey -v -keystore edge-hive.keystore ...`
2. Configura en `app/build.gradle.kts`:

   ```kotlin
   signingConfigs {
       create("release") {
           storeFile = file("../edge-hive.keystore")
           storePassword = "..."
           keyAlias = "edge-hive"
           keyPassword = "..."
       }
   }
   ```

## 🎨 Customización

### Cambiar colores (Material Theme)

Edita `app/src/main/java/com/edgehive/app/ui/theme/Color.kt`:

```kotlin
val md_theme_light_primary = Color(0xFF006A6A) // Cambia aquí
```

### Agregar nuevas pantallas

1. Crea composable en `MainActivity.kt` o nuevo archivo
2. Agrega tab en `NavigationBar`
3. Agrega case en `when (selectedTab)`

## 🐛 Troubleshooting

### "Failed to connect"

- Verifica que el servidor esté corriendo (`docker ps`)
- Verifica que la URL sea correcta (emulador usa `10.0.2.2`, no `localhost`)
- Verifica firewall

### "Cleartext HTTP traffic not permitted"

Ya está habilitado en `AndroidManifest.xml` con `android:usesCleartextTraffic="true"`.
Para producción, usa HTTPS.

### Gradle sync failed

- Actualiza Android Studio
- File → Invalidate Caches → Restart
- Borra carpeta `.gradle` y vuelve a sincronizar

## 📚 Dependencies

- **Jetpack Compose** - UI declarativa
- **Material 3** - Material Design components
- **OkHttp** - HTTP client
- **Kotlinx Serialization** - JSON parsing
- **Coroutines** - Async/await

## 🔗 Links útiles

- [Jetpack Compose](https://developer.android.com/jetpack/compose)
- [Material 3](https://m3.material.io/)
- [OkHttp](https://square.github.io/okhttp/)
