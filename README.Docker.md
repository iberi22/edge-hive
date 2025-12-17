# 🐋 Edge Hive Docker Deployment

Guía rápida para desplegar Edge Hive usando Docker en Windows.

## 📋 Prerequisitos

1. **Docker Desktop para Windows** instalado y corriendo
   - Descarga: <https://docs.docker.com/desktop/install/windows-install/>
   - Requiere WSL 2 (recomendado) o Hyper-V
   - 4GB RAM mínimo

2. **Verificar instalación:**

```powershell
docker --version
docker-compose --version
```

## 🚀 Despliegue Rápido

### Opción 1: Docker Compose (Recomendado)

```powershell
# Construir y ejecutar
docker-compose up -d

# Ver logs
docker-compose logs -f

# Detener
docker-compose down

# Reconstruir después de cambios
docker-compose up -d --build
```

### Opción 2: Docker CLI

```powershell
# Construir imagen
docker build -t edge-hive:latest .

# Ejecutar contenedor
docker run -d \
  --name edge-hive \
  -p 8080:8080 \
  -v edge-hive-data:/app/data \
  edge-hive:latest

# Ver logs
docker logs -f edge-hive

# Detener
docker stop edge-hive

# Eliminar
docker rm edge-hive
```

## 🌐 Acceso

Una vez desplegado, accede a:

- **Frontend:** <http://localhost:8080>
- **API Health:** <http://localhost:8080/health>
- **API Gateway:** <http://localhost:8080/api/v1/>

## 🔧 Configuración

### Variables de Entorno

Edita `docker-compose.yml` para cambiar configuraciones:

```yaml
environment:
  - RUST_LOG=debug  # Nivel de logs: trace, debug, info, warn, error
  - EDGE_HIVE_PORT=8080
```

### Puertos

Cambiar el puerto externo (mantén 8080 interno):

```yaml
ports:
  - "3000:8080"  # Acceso en http://localhost:3000
```

### Volúmenes Persistentes

Los datos se guardan automáticamente en el volumen `edge-hive-data`.

Ubicación en Windows:

```
\\wsl$\docker-desktop-data\version-pack-data\community\docker\volumes\
```

## 🐛 Troubleshooting

### El puerto 8080 ya está en uso

```powershell
# Ver qué proceso usa el puerto
netstat -ano | findstr :8080

# Matar el proceso (reemplaza PID)
taskkill /PID <PID> /F

# O cambiar el puerto en docker-compose.yml
```

### El contenedor no inicia

```powershell
# Ver logs detallados
docker-compose logs edge-hive

# Ver estado
docker-compose ps

# Reconstruir desde cero
docker-compose down -v
docker-compose up -d --build
```

### Error de memoria

Docker Desktop → Settings → Resources → Aumentar memoria a 4GB+

### WSL 2 no disponible

```powershell
# Verificar WSL
wsl --version

# Actualizar WSL
wsl --update

# Instalar WSL 2
wsl --install
```

## 📊 Monitoreo

```powershell
# Ver uso de recursos
docker stats edge-hive

# Ver procesos dentro del contenedor
docker top edge-hive

# Ejecutar comando dentro del contenedor
docker exec -it edge-hive /bin/bash
```

## 🔄 Actualización

```powershell
# Pull últimos cambios
git pull

# Reconstruir imagen
docker-compose up -d --build

# Limpiar imágenes antiguas
docker image prune -f
```

## 🧹 Limpieza

```powershell
# Detener y eliminar contenedores
docker-compose down

# Eliminar también volúmenes (⚠️ borra datos)
docker-compose down -v

# Limpiar sistema completo
docker system prune -a --volumes
```

## 🔐 Producción

Para producción, considera:

1. **HTTPS:** Agregar reverse proxy (nginx/traefik)
2. **Secrets:** Usar Docker secrets en vez de env vars
3. **Backups:** Respaldar volumen `edge-hive-data`
4. **Monitoring:** Prometheus + Grafana
5. **Logs:** Enviar a servicio externo (ELK, Loki)

## 📚 Referencias

- [Docker Desktop Windows](https://docs.docker.com/desktop/install/windows-install/)
- [Docker Compose](https://docs.docker.com/compose/)
- [Rust Docker Best Practices](https://docs.docker.com/language/rust/)
