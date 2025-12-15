---
title: "Deploy Edge Hive to Termux (Android) - Priority: CRITICAL"
labels:
  - infrastructure
  - termux
  - android
  - deployment
  - critical
assignees:
  - jules
---

## 🚨 CRITICAL PRIORITY

**Timeline:** 3 días
**Blocker para:** APK development (#FEAT_tauri-mobile-app)
**Estrategia:** Crawl → Walk → Run (validar en Termux antes de APK nativo)

---

## 📋 Context

Edge Hive debe ejecutarse primero en **Termux** (Android terminal emulator) para validar el stack completo antes de invertir 6 semanas en desarrollo de APK nativo.

**Análisis completo:** `docs/agent-docs/ANALYSIS_APK_VS_TERMUX.md`

**Razones para priorizar Termux:**
1. ✅ **3 días** de implementación vs 6 semanas APK
2. ✅ **80% código compartido** con APK final (Rust core, Tor, libp2p, SurrealDB)
3. ✅ **Validación real** de networking en Android sin permisos especiales
4. ✅ **Iteración rápida** para debugging (no rebuilds APK lentos)
5. ✅ **Comunidad activa** - Termux tiene 50M+ descargas en F-Droid/Play Store

---

## 🎯 Objectives

### 1. Cross-Compilation para Android (1 día)

**Target:** `aarch64-linux-android` (Android ARM64)

**Setup toolchain:**
```bash
# Instalar Android NDK
rustup target add aarch64-linux-android

# Configurar linker
# .cargo/config.toml
[target.aarch64-linux-android]
linker = "aarch64-linux-android-clang"
ar = "aarch64-linux-android-ar"

[env]
CC_aarch64_linux_android = "aarch64-linux-android-clang"
CXX_aarch64_linux_android = "aarch64-linux-android-clang++"
```

**Compilar:**
```bash
cargo build --release --target aarch64-linux-android
# Output: target/aarch64-linux-android/release/edge-hive
```

**Tareas:**
- [ ] Configurar Android NDK r27 o superior
- [ ] Actualizar `.cargo/config.toml` con Android target
- [ ] Probar build en CI (GitHub Actions)
- [ ] Verificar que el binario es ejecutable en Termux

---

### 2. Integración con Termux (1 día)

**Instalación script:**
```bash
#!/data/data/com.termux/files/usr/bin/bash
# scripts/install-termux.sh

set -e

echo "🔧 Installing Edge Hive on Termux..."

# 1. Update packages
pkg update -y
pkg upgrade -y

# 2. Install dependencies
pkg install -y \
    rust \
    clang \
    openssl \
    libsodium \
    libevent \
    zstd

# 3. Download latest release
VERSION="v0.1.0"
ARCH="aarch64"
URL="https://github.com/user/edge-hive/releases/download/${VERSION}/edge-hive-${ARCH}-linux-android"

curl -L -o /data/data/com.termux/files/usr/bin/edge-hive "$URL"
chmod +x /data/data/com.termux/files/usr/bin/edge-hive

# 4. Create config directory
mkdir -p ~/.config/edge-hive

# 5. Initialize database
edge-hive init

echo "✅ Edge Hive installed successfully!"
echo "Run: edge-hive start"
```

**Verificar dependencias:**
```bash
# En Termux:
pkg list-installed | grep -E "(rust|clang|openssl|libsodium)"
```

**Tareas:**
- [ ] Crear `scripts/install-termux.sh` con instalación automatizada
- [ ] Probar en dispositivo Android real (Termux F-Droid)
- [ ] Documentar errores comunes (permissions, paths, etc.)
- [ ] Crear script de desinstalación `scripts/uninstall-termux.sh`

---

### 3. Testing en Termux (1 día)

**Checklist de validación:**

#### **A. Compilación y Ejecución**
```bash
# En Termux:
edge-hive --version
# Expected: edge-hive 0.1.0

edge-hive start
# Expected: Server listening on 0.0.0.0:8080
```

- [ ] Binario ejecuta sin segfaults
- [ ] Help menu (`--help`) funciona
- [ ] Logging a stdout/stderr visible

---

#### **B. Networking - Tor**
```bash
# Test Tor bootstrap
edge-hive tor --test
# Expected: Tor circuit established (3 hops)
```

**Verificar:**
- [ ] Tor (arti) se conecta a la red Tor
- [ ] Onion Service v3 se crea (`http://abc123.onion`)
- [ ] `.onion` address es accesible desde otro dispositivo con Tor Browser

**Debugging común:**
```bash
# Tor logs
edge-hive logs --filter tor

# Expected output:
# [INFO] Bootstrapping Tor: 100%
# [INFO] HS published: http://abc123xyz.onion:8080
```

**Posibles errores:**
| Error | Causa | Solución |
|-------|-------|----------|
| `Failed to bootstrap Tor` | Network blocked | Usar Tor bridges |
| `Permission denied (onion service)` | Filesystem permissions | `chmod 700 ~/.tor/` |
| `No route to host` | IPv6 issues | Deshabilitar IPv6 en arti config |

---

#### **C. Networking - libp2p**
```bash
# Test local discovery (mDNS)
edge-hive discover --local
# Expected: Found peers: [node-pc-1, node-android-2]

# Test global DHT
edge-hive discover --global
# Expected: Connected to Kademlia DHT, 12 peers
```

**Verificar:**
- [ ] mDNS descubre peers en misma WiFi
- [ ] Kademlia DHT funciona (peer discovery global)
- [ ] QUIC transport funciona (UDP hole punching)
- [ ] Noise protocol encripta conexiones

**Debugging:**
```bash
# libp2p logs
edge-hive logs --filter libp2p

# Expected:
# [INFO] mDNS discovered peer: 12D3KooW...
# [INFO] Connected via QUIC: /ip4/192.168.1.10/udp/4001/quic-v1
```

---

#### **D. Database - SurrealDB**
```bash
# Test DB operations
edge-hive db query "CREATE users:test SET name = 'Jules'"
edge-hive db query "SELECT * FROM users"
# Expected: [{ id: users:test, name: "Jules" }]
```

**Verificar:**
- [ ] SurrealDB embedded inicia sin errores
- [ ] Queries SQL funcionan
- [ ] Datos persisten entre reinicios (RocksDB backend)
- [ ] Tamaño de DB < 100 MB inicial

**Archivos generados:**
```bash
ls -lh ~/.local/share/edge-hive/
# Expected:
# edge-hive.db/  (RocksDB directory)
```

---

#### **E. Performance Metrics**
```bash
# CPU usage
top -n 1 | grep edge-hive
# Expected: < 10% CPU idle, < 30% CPU active

# Memory usage
ps aux | grep edge-hive
# Expected: < 100 MB RAM

# Battery drain (after 1 hour)
termux-battery-status
# Expected: < 5% battery/hour (background mode)
```

**Benchmark:**
```bash
# File sync test
edge-hive sync ~/test-file.bin --peer 12D3KooW...
# Expected: >1 MB/s over libp2p, >500 KB/s over Tor
```

---

### 4. Background Execution (Termux Services)

**Problema:** Termux no ejecuta procesos en background cuando app está cerrada.

**Solución:** Termux:Boot + Wake Lock

**Setup:**
```bash
# 1. Instalar Termux:Boot (F-Droid)
pkg install termux-services

# 2. Crear servicio
mkdir -p ~/.termux/boot
cat > ~/.termux/boot/edge-hive.sh << 'EOF'
#!/data/data/com.termux/files/usr/bin/bash
termux-wake-lock
edge-hive start --daemon
EOF

chmod +x ~/.termux/boot/edge-hive.sh

# 3. Activar servicio
sv-enable edge-hive
```

**Verificar:**
- [ ] Edge Hive inicia al boot de Termux
- [ ] Wake lock previene Android de matar proceso
- [ ] Proceso sobrevive a cerrar Termux app
- [ ] Logs disponibles: `sv status edge-hive`

**Monitoreo:**
```bash
# Status del servicio
svlogtail edge-hive

# Reiniciar servicio
sv restart edge-hive

# Detener
sv stop edge-hive
```

---

### 5. Documentación de Usuario (0.5 días)

**Crear:** `docs/TERMUX_GUIDE.md`

**Contenido:**
```markdown
# Edge Hive en Termux (Android)

## Requisitos
- Android 7.0+ (ARM64)
- Termux (F-Droid version recomendada)
- 500 MB espacio libre
- Conexión a internet

## Instalación Rápida
bash <(curl -fsSL https://edgehive.dev/install-termux.sh)

## Uso Básico
# Iniciar servidor
edge-hive start

# Ver logs
edge-hive logs

# Detener
edge-hive stop

## Configuración
# Archivo: ~/.config/edge-hive/config.toml
[network]
tor_enabled = true
libp2p_port = 4001

[storage]
db_path = "~/.local/share/edge-hive/edge-hive.db"

## Troubleshooting
...
```

**Tareas:**
- [ ] Escribir guía completa de Termux
- [ ] Screenshots de instalación
- [ ] Sección de FAQ
- [ ] Actualizar README.md principal con sección Termux

---

## 📦 Deliverables

- [ ] **Binario compilado:** `edge-hive-aarch64-linux-android` en GitHub Releases
- [ ] **Install script:** `scripts/install-termux.sh` funcional
- [ ] **Tests pasados:** Tor, libp2p, SurrealDB funcionan en Termux
- [ ] **Background service:** Termux:Boot configuration
- [ ] **Documentación:** `docs/TERMUX_GUIDE.md` completa
- [ ] **CI/CD:** GitHub Actions compila Android target automáticamente

---

## 🎯 Definition of Done

- ✅ `cargo build --target aarch64-linux-android` compila sin errores
- ✅ Binario ejecuta en Termux (Android 12+ testeado)
- ✅ Tor bootstrap completa (onion service accesible)
- ✅ libp2p descubre peers (mDNS + Kademlia)
- ✅ SurrealDB persiste datos entre reinicios
- ✅ CPU < 30%, RAM < 100 MB, battery drain < 5%/hour
- ✅ Background execution funciona (Termux:Boot)
- ✅ Documentación publicada en `docs/TERMUX_GUIDE.md`
- ✅ Al menos 3 desarrolladores han validado instalación en sus Androids

---

## 🔗 Related Issues

- #0 Architecture foundation (completed)
- #NET_tor-integration - Depende de que Tor funcione en Android
- #NET_identity-system - Ed25519 debe funcionar en Android
- #FEAT_tauri-mobile-app - **BLOQUEADO** hasta que Termux deployment esté validado
- #FEAT_web-admin-ui - Web UI será accesible desde Termux via `localhost:8080`

---

## 📚 References

- [Termux Wiki](https://wiki.termux.com/)
- [Rust Android Guide](https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-21-rust-on-android.html)
- [Tor on Android](https://support.torproject.org/tormobile/tormobile-7/)
- [libp2p Mobile Guide](https://docs.libp2p.io/guides/mobile/)
- [SurrealDB Embedded](https://surrealdb.com/docs/surrealdb/integration/sdks/rust)

---

## 💬 Notes

**Jules:** Este es el issue MÁS CRÍTICO para validar la viabilidad de Edge Hive. Si Termux deployment falla, reconsiderar arquitectura completa.

**Prioridad:**
1. Tor funcionando (sin esto, no hay privacidad)
2. libp2p descubrimiento local (validar que mDNS funciona en Android)
3. SurrealDB embedded (probar performance en ARM)

**Fallback plan:** Si Termux resulta muy complejo (permisos, bugs), considerar:
- Docker en Termux (via `proot-distro`)
- UserLAnd app (Ubuntu on Android)
- Directamente saltar a APK nativo (pero pierde validación rápida)

**Timeline:** 3 días es agresivo. Si toma 5 días, está bien. El objetivo es aprender rápido.
