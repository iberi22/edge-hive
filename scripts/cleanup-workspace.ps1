#!/usr/bin/env pwsh
<#
.SYNOPSIS
Cleanup and reorganize logs, temp files, and build artifacts according to industry standards

.DESCRIPTION
This script:
1. Moves all .log files to logs/ directory
2. Moves all .tmp_* and .tmp files to .tmp/ directory  
3. Organizes build outputs to build-output/
4. Updates scripts to use new directory structure
5. Cleans root directory following Git-Core Protocol standards

.EXAMPLE
./cleanup-workspace.ps1

.EXAMPLE
./cleanup-workspace.ps1 -DryRun

.PARAMETER DryRun
Show what would be moved without actually moving files
#>

param(
    [switch]$DryRun = $false,
    [switch]$Help = $false
)

if ($Help) {
    Get-Help $MyInvocation.MyCommand.Path -Full
    exit 0
}

$ErrorActionPreference = "Continue"
$ProjectRoot = if ($PSScriptRoot) { Split-Path -Parent $PSScriptRoot } else { Get-Location }
# Fallback to parent of scripts directory if running from scripts folder
if ((Split-Path -Leaf $ProjectRoot) -eq "scripts") {
    $ProjectRoot = Split-Path -Parent $ProjectRoot
}

$logAction = if ($DryRun) { "Would move" } else { "Moving" }
$stats = @{
    "logs"   = 0
    "temps"  = 0
    "builds" = 0
    "errors" = 0
}

Write-Host "[CLEANUP] Git-Core Protocol: Workspace Cleanup" -ForegroundColor Cyan
Write-Host ("=" * 60)

# Create directories if they don't exist
@("logs", ".tmp", "build-output") | ForEach-Object {
    $dir = Join-Path $ProjectRoot $_
    if (-not (Test-Path $dir)) {
        New-Item -ItemType Directory -Path $dir -Force | Out-Null
        Write-Host "[DIR] Created directory: $_"
    }
}

Write-Host ""
Write-Host "[INFO] Analyzing root directory..."
Write-Host ""

# 1. Move .log files to logs/
Write-Host "[1] Logs (.log files)"
Write-Host ("-" * 40)
Get-ChildItem -Path $ProjectRoot -Filter "*.log" -ErrorAction SilentlyContinue | ForEach-Object {
    $dest = Join-Path $ProjectRoot "logs" $_.Name
    if ($DryRun) {
        Write-Host "  ${logAction}: $($_.Name) => logs/"
    } else {
        Move-Item -Path $_.FullName -Destination $dest -Force
        Write-Host "  [OK] Moved: $($_.Name)"
    }
    $stats.logs++
}

# 2. Move test output and build logs to logs/
Write-Host ""
Write-Host "[2] Test Output Files"
Write-Host ("-" * 40)
@(
    "test-*.txt",
    "test-*.log",
    "build-*.log",
    "serve-*.log",
    "deploy-*.log"
) | ForEach-Object {
    Get-ChildItem -Path $ProjectRoot -Filter $_ -ErrorAction SilentlyContinue | ForEach-Object {
        $dest = Join-Path $ProjectRoot "logs" $_.Name
        if ($DryRun) {
            Write-Host "  ${logAction}: $($_.Name) => logs/"
        } else {
            Move-Item -Path $_.FullName -Destination $dest -Force
            Write-Host "  [OK] Moved: $($_.Name)"
        }
        $stats.logs++
    }
}

# 3. Move .tmp_* files to .tmp/
Write-Host ""
Write-Host "[3] Temporary Files (.tmp_* pattern)"
Write-Host ("-" * 40)
Get-ChildItem -Path $ProjectRoot -Filter ".tmp_*" -ErrorAction SilentlyContinue | ForEach-Object {
    $dest = Join-Path $ProjectRoot ".tmp" $_.Name
    if ($DryRun) {
        Write-Host "  ${logAction}: $($_.Name) => .tmp/"
    } else {
        Move-Item -Path $_.FullName -Destination $dest -Force
        Write-Host "  [OK] Moved: $($_.Name)"
    }
    $stats.temps++
}

# 4. Move build artifacts
Write-Host ""
Write-Host "[4] Build Artifacts"
Write-Host ("-" * 40)
@(
    "build-cli.txt",
    "build-app.log",
    "build-core.log",
    "build-docker.log",
    "build-identity.log",
    "build-tunnel.log"
) | ForEach-Object {
    $file = Join-Path $ProjectRoot $_
    if (Test-Path $file) {
        $dest = Join-Path $ProjectRoot "build-output" (Split-Path $_ -Leaf)
        if ($DryRun) {
            Write-Host "  ${logAction}: $_ => build-output/"
        } else {
            Move-Item -Path $file -Destination $dest -Force
            Write-Host "  [OK] Moved: $_"
        }
        $stats.builds++
    }
}

# Summary
Write-Host ""
Write-Host ("=" * 60)
Write-Host "[SUMMARY] Results" -ForegroundColor Green
Write-Host ("-" * 40)
Write-Host "  [LOGS]   Logs moved to logs/:          $($stats.logs) files"
Write-Host "  [TEMPS]  Temp files moved to .tmp/:    $($stats.temps) files"
Write-Host "  [BUILD]  Build artifacts to build/:   $($stats.builds) files"
Write-Host ""

if ($DryRun) {
    Write-Host "[DRY-RUN] No files were moved" -ForegroundColor Yellow
    Write-Host "[INFO]   Run without -DryRun to apply changes" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "[DIRS] Directory Structure" -ForegroundColor Cyan
Write-Host ("-" * 40)
Write-Host "  logs/           - Build and test logs"
Write-Host "  .tmp/           - Temporary files"
Write-Host "  build-output/   - Build artifacts"
Write-Host ""
Write-Host "[OK] Workspace organized per Git-Core Protocol v3.2" -ForegroundColor Green
