# Edge Hive Telegram Bot

Bot de Telegram para controlar tu nodo Edge Hive remotamente.

## 🚀 Instalación

```bash
# Instalar dependencias
pip install -r requirements.txt

# Configurar credenciales
export TELEGRAM_BOT_TOKEN="tu_token_de_botfather"
export EDGE_HIVE_URL="http://localhost:8080"

# Opcional: crear cliente OAuth2 manualmente
# edge-hive auth client create --name telegram-bot
# export EDGE_HIVE_CLIENT_ID="cli_..."
# export EDGE_HIVE_CLIENT_SECRET="..."

# Ejecutar bot
python bot.py
```

## 📱 Uso

### Comandos disponibles

- `/start` - Menu principal con botones interactivos
- `/status` - Estado del nodo VPS
- `/list` - Listar edge functions
- `/create <name> <json>` - Crear edge function
- `/run <name> <payload>` - Ejecutar edge function

### Ejemplos

```
# Crear función
/create saludar {"mensaje": "Hola desde Telegram"}

# Ejecutar función
/run saludar {"usuario": "Alice"}

# Ver estado
/status
```

## 🔧 Características

- ✅ OAuth2 automático (crea cliente si no existe)
- ✅ Botones inline para navegación rápida
- ✅ Crear/listar/ejecutar edge functions
- ✅ Check de salud del nodo
- ✅ Soporte para MCP tools

## 🐳 Uso con Docker

Si tu nodo Edge Hive está en Docker (localhost:8080):

```bash
export EDGE_HIVE_URL="http://localhost:8080"
python bot.py
```

Si usaste Cloudflare Tunnel:

```bash
export EDGE_HIVE_URL="https://tu-subdominio.trycloudflare.com"
python bot.py
```

## 📖 Obtener token de Telegram

1. Habla con [@BotFather](https://t.me/botfather)
2. Envía `/newbot`
3. Sigue las instrucciones
4. Copia el token HTTP API

## 🔐 Seguridad

- El bot usa OAuth2 client credentials
- Si no existen, las crea automáticamente
- Guarda `EDGE_HIVE_CLIENT_ID` y `EDGE_HIVE_CLIENT_SECRET` de forma segura
- Para producción, usa HTTPS con certificados válidos
