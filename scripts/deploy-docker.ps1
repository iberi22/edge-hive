#!/usr/bin/env pwsh
# Edge Hive Docker Deployment Script para Windows
# Uso: .\deploy-docker.ps1 [-Mode dev|prod|multinode] [-Build] [-Logs] [-Stop] [-Clean]

param(
    [ValidateSet("dev", "prod", "multinode")]
    [string]$Mode = "dev",
    [switch]$Build,
    [switch]$Logs,
    [switch]$Stop,
    [switch]$Clean,
    [switch]$Help
)

$ErrorActionPreference = "Stop"

function Show-Help {
    Write-Host @"
🐋 Edge Hive Docker Deployment Tool

USO:
  .\deploy-docker.ps1 [OPCIONES]

OPCIONES:
  -Mode <dev|prod|multinode>  Modo de despliegue (default: dev)
  -Build                      Construir imagen antes de iniciar
  -Logs                       Ver logs en tiempo real
  -Stop                       Detener contenedores
  -Clean                      Limpiar contenedores y volúmenes
  -Help                       Mostrar esta ayuda

EJEMPLOS:
  # Despliegue rápido desarrollo
  .\deploy-docker.ps1

  # Construir y desplegar
  .\deploy-docker.ps1 -Build

  # Ver logs
  .\deploy-docker.ps1 -Logs

  # Detener
  .\deploy-docker.ps1 -Stop

  # Limpiar todo
  .\deploy-docker.ps1 -Clean

  # Multi-nodo para testing
  .\deploy-docker.ps1 -Mode multinode -Build

MODOS:
  dev        - Single node con frontend (docker-compose.dev.yml)
  prod       - Producción optimizado
  multinode  - 3 nodos para testing distribuido (docker-compose.yml)
"@
    exit 0
}

function Test-DockerRunning {
    try {
        docker info | Out-Null
        return $true
    } catch {
        Write-Host "❌ Docker no está corriendo. Inicia Docker Desktop primero." -ForegroundColor Red
        Write-Host "   Descarga: https://docs.docker.com/desktop/install/windows-install/" -ForegroundColor Yellow
        exit 1
    }
}

function Get-ComposeFile {
    param([string]$Mode)

    switch ($Mode) {
        "dev" { return "docker-compose.dev.yml" }
        "multinode" { return "docker-compose.yml" }
        "prod" { return "docker-compose.yml" }
        default { return "docker-compose.dev.yml" }
    }
}

function Start-EdgeHive {
    param(
        [string]$ComposeFile,
        [bool]$BuildFirst
    )

    Write-Host "🚀 Iniciando Edge Hive..." -ForegroundColor Cyan

    if ($BuildFirst) {
        Write-Host "🔨 Construyendo imagen Docker..." -ForegroundColor Yellow
        docker-compose -f $ComposeFile build --no-cache
        if ($LASTEXITCODE -ne 0) {
            Write-Host "❌ Error al construir imagen" -ForegroundColor Red
            exit 1
        }
    }

    docker-compose -f $ComposeFile up -d

    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Edge Hive desplegado exitosamente!" -ForegroundColor Green
        Write-Host ""
        Write-Host "📊 Acceso:" -ForegroundColor Cyan

        if ($ComposeFile -eq "docker-compose.dev.yml") {
            Write-Host "   Frontend:  http://localhost:8080" -ForegroundColor White
            Write-Host "   Health:    http://localhost:8080/health" -ForegroundColor White
            Write-Host "   API:       http://localhost:8080/api/v1/" -ForegroundColor White
        } else {
            Write-Host "   Node 1:    http://localhost:8080" -ForegroundColor White
            Write-Host "   Node 2:    http://localhost:8081" -ForegroundColor White
            Write-Host "   Node 3:    http://localhost:8082" -ForegroundColor White
        }

        Write-Host ""
        Write-Host "📝 Ver logs:   docker-compose -f $ComposeFile logs -f" -ForegroundColor Gray
        Write-Host "🛑 Detener:    docker-compose -f $ComposeFile down" -ForegroundColor Gray
    } else {
        Write-Host "❌ Error al desplegar" -ForegroundColor Red
        exit 1
    }
}

function Show-Logs {
    param([string]$ComposeFile)

    Write-Host "📝 Mostrando logs (Ctrl+C para salir)..." -ForegroundColor Cyan
    docker-compose -f $ComposeFile logs -f --tail=100
}

function Stop-EdgeHive {
    param([string]$ComposeFile)

    Write-Host "🛑 Deteniendo Edge Hive..." -ForegroundColor Yellow
    docker-compose -f $ComposeFile down

    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Edge Hive detenido" -ForegroundColor Green
    }
}

function Clean-EdgeHive {
    param([string]$ComposeFile)

    Write-Host "⚠️  ADVERTENCIA: Esto eliminará contenedores Y datos persistentes" -ForegroundColor Yellow
    $confirm = Read-Host "¿Continuar? (y/N)"

    if ($confirm -eq "y" -or $confirm -eq "Y") {
        Write-Host "🧹 Limpiando..." -ForegroundColor Yellow
        docker-compose -f $ComposeFile down -v

        # Limpiar imágenes huérfanas
        docker image prune -f

        Write-Host "✅ Limpieza completada" -ForegroundColor Green
    } else {
        Write-Host "❌ Cancelado" -ForegroundColor Red
    }
}

# MAIN
if ($Help) {
    Show-Help
}

Write-Host "🐋 Edge Hive Docker Deployment" -ForegroundColor Cyan
Write-Host "================================" -ForegroundColor Cyan
Write-Host ""

# Verificar Docker
Test-DockerRunning

$composeFile = Get-ComposeFile -Mode $Mode

# Ejecutar acción
if ($Clean) {
    Clean-EdgeHive -ComposeFile $composeFile
} elseif ($Stop) {
    Stop-EdgeHive -ComposeFile $composeFile
} elseif ($Logs) {
    Show-Logs -ComposeFile $composeFile
} else {
    Start-EdgeHive -ComposeFile $composeFile -BuildFirst $Build
}
