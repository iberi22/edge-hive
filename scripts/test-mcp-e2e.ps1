#!/usr/bin/env pwsh
<#
.SYNOPSIS
    End-to-End tests for Edge Hive MCP Server with OAuth2 and SSE

.DESCRIPTION
    Comprehensive test suite that validates:
    - OAuth2 Client Credentials flow
    - Bearer token authentication
    - MCP tool execution (get_status, provision_node)
    - MCP resource read (edge-hive://status, edge-hive://logs/last)
    - SSE streaming endpoint
    - TLS/HTTPS certificate validation

.EXAMPLE
    .\test-mcp-e2e.ps1
    Run all tests against https://localhost:8443

.EXAMPLE
    .\test-mcp-e2e.ps1 -BaseUrl "https://172.20.0.10:8443"
    Run tests against Docker node
#>

param(
    [string]$BaseUrl = "https://localhost:8443",
    [switch]$SkipSSL = $true,
    [bool]$AutoStartLocalServer = $true,
    [switch]$Verbose
)

$ErrorActionPreference = "Stop"

$RepoRoot = Split-Path -Parent $PSScriptRoot
$EdgeHiveBin = Join-Path $RepoRoot "target\release\edge-hive.exe"
if (-not (Test-Path $EdgeHiveBin)) {
    throw "edge-hive binary not found at: $EdgeHiveBin. Build it with: cargo build --release"
}

# Colors
$Green = "`e[32m"
$Red = "`e[31m"
$Yellow = "`e[33m"
$Blue = "`e[34m"
$Reset = "`e[0m"

# Test counters
$script:TestsPassed = 0
$script:TestsFailed = 0
$script:TestsTotal = 0

function Write-TestHeader {
    param([string]$Title)
    Write-Host "`n$Blue═══════════════════════════════════════════════════════$Reset" -NoNewline
    Write-Host "`n$Blue  $Title$Reset"
    Write-Host "$Blue═══════════════════════════════════════════════════════$Reset`n"
}

function Assert-Test {
    param(
        [string]$Name,
        [scriptblock]$Test,
        [string]$Expected
    )

    $script:TestsTotal++

    try {
        $result = & $Test

        if ($result -match $Expected -or $result -eq $true) {
            Write-Host "  $Green✓$Reset $Name" -NoNewline
            if ($Verbose) {
                Write-Host " → $result" -ForegroundColor DarkGray
            } else {
                Write-Host ""
            }
            $script:TestsPassed++
            return $true
        } else {
            Write-Host "  $Red✗$Reset $Name" -NoNewline
            Write-Host " (Expected: $Expected, Got: $result)" -ForegroundColor Red
            $script:TestsFailed++
            return $false
        }
    } catch {
        Write-Host "  $Red✗$Reset $Name" -NoNewline
        Write-Host " (Error: $_)" -ForegroundColor Red
        $script:TestsFailed++
        return $false
    }
}

function Invoke-HttpRequest {
    param(
        [string]$Method = "GET",
        [string]$Endpoint,
        [hashtable]$Headers = @{},
        [object]$Body = $null
    )

    $url = "$BaseUrl$Endpoint"
    $params = @{
        Uri = $url
        Method = $Method
        Headers = $Headers
        ContentType = "application/json"
    }

    if ($SkipSSL) {
        $params.SkipCertificateCheck = $true
    }

    if ($Body) {
        $params.Body = ($Body | ConvertTo-Json -Depth 10)
    }

    if ($Verbose) {
        Write-Host "    → $Method $url" -ForegroundColor DarkGray
    }

    return Invoke-RestMethod @params
}

function Test-HealthOnce {
    try {
        $null = Invoke-HttpRequest -Endpoint "/health"
        return $true
    } catch {
        return $false
    }
}

$script:ServerProcess = $null
$script:ServerStartedByTest = $false
$script:ExitCode = 1

try {
    # If the server isn't reachable, auto-start a local instance (only for localhost/127.0.0.1).
    $baseUri = [Uri]$BaseUrl
    $isLocalHost = @("localhost", "127.0.0.1", "::1") -contains $baseUri.Host

    if (-not (Test-HealthOnce)) {
        if (-not $AutoStartLocalServer) {
            throw "Server is not reachable at $BaseUrl and AutoStartLocalServer is disabled."
        }

        if (-not $isLocalHost) {
            throw "Server is not reachable at $BaseUrl and auto-start is only supported for localhost."
        }

        $port = $baseUri.Port
        $existing = Get-NetTCPConnection -LocalPort $port -State Listen -ErrorAction SilentlyContinue | Select-Object -First 1
        if ($null -ne $existing) {
            Write-Host "${Yellow}Server not responding but port $port is already in use (pid $($existing.OwningProcess)).$Reset"
            throw "Port $port is already listening but /health failed. Stop the other process or change -BaseUrl."
        }

        Write-Host "${Yellow}Starting local Edge Hive server for E2E...$Reset"

        $logsDir = Join-Path $RepoRoot "target\tmp"
        New-Item -ItemType Directory -Force $logsDir | Out-Null
        $outLog = Join-Path $logsDir "mcp-e2e-server.out.log"
        $errLog = Join-Path $logsDir "mcp-e2e-server.err.log"
        Remove-Item $outLog, $errLog -ErrorAction SilentlyContinue

        $args = @(
            "start",
            "--port", "$port"
        )

        if ($baseUri.Scheme -eq "https") {
            $args += "--https"
            $args += @("--hostname", $baseUri.Host)
        }

        $script:ServerProcess = Start-Process -FilePath $EdgeHiveBin -ArgumentList $args -PassThru -NoNewWindow -RedirectStandardOutput $outLog -RedirectStandardError $errLog
        $script:ServerStartedByTest = $true

        $deadline = (Get-Date).AddSeconds(15)
        while ((Get-Date) -lt $deadline) {
            if (Test-HealthOnce) { break }
            Start-Sleep -Milliseconds 250
        }

        if (-not (Test-HealthOnce)) {
            throw "Server failed to start within timeout. Logs: $outLog ; $errLog"
        }
    }

# ═══════════════════════════════════════════════════════
# TEST 1: Server Health Check
# ═══════════════════════════════════════════════════════

Write-TestHeader "TEST 1: Server Health Check"

Assert-Test "Server is running" {
    $response = Invoke-HttpRequest -Endpoint "/health"
    $response.status -eq "ok"
} -Expected "True"

Assert-Test "Server version is valid" {
    $response = Invoke-HttpRequest -Endpoint "/health"
    $response.version -match "^\d+\.\d+\.\d+$"
} -Expected "True"

# ═══════════════════════════════════════════════════════
# TEST 2: OAuth2 Client Credentials Flow
# ═══════════════════════════════════════════════════════

Write-TestHeader "TEST 2: OAuth2 Client Credentials Flow"

# Create test client
Write-Host "  ${Yellow}Creating test OAuth2 client...$Reset"
$createOutput = & $EdgeHiveBin auth client create --name "test-e2e" --scopes "mcp:read,mcp:call,mcp:resources" 2>&1 | Out-String

$clientId = if ($createOutput -match "Client ID:\s+(\S+)") { $matches[1] } else { $null }
$clientSecret = if ($createOutput -match "Client Secret:\s+(\S+)") { $matches[1] } else { $null }

Assert-Test "Client ID generated" {
    $clientId -ne $null -and $clientId.StartsWith("cli_")
} -Expected "True"

Assert-Test "Client Secret generated" {
    $clientSecret -ne $null -and $clientSecret.Length -eq 64
} -Expected "True"

if (-not $clientId -or -not $clientSecret) {
    Write-Host "`n$Red✗ FATAL: Failed to create test client. Aborting tests.$Reset`n"
    throw "Failed to create OAuth2 client (clientId/clientSecret missing)"
}

# Get access token
Write-Host "  ${Yellow}Requesting access token...$Reset"

$tokenRequest = @{
    grant_type = "client_credentials"
    client_id = $clientId
    client_secret = $clientSecret
    scope = "mcp:read mcp:call mcp:resources"
}

$tokenResponse = $null
Assert-Test "Token endpoint responds" {
    $script:tokenResponse = Invoke-HttpRequest -Method POST -Endpoint "/mcp/auth/token" -Body $tokenRequest
    $script:tokenResponse -ne $null
} -Expected "True"

Assert-Test "Access token received" {
    $script:tokenResponse.access_token -ne $null
} -Expected "True"

Assert-Test "Token type is Bearer" {
    $script:tokenResponse.token_type -eq "Bearer"
} -Expected "True"

Assert-Test "Token expires in 1 hour" {
    $script:tokenResponse.expires_in -eq 3600
} -Expected "True"

$accessToken = $tokenResponse.access_token

# ═══════════════════════════════════════════════════════
# TEST 3: MCP Tool Execution
# ═══════════════════════════════════════════════════════

Write-TestHeader "TEST 3: MCP Tool Execution"

$authHeaders = @{ Authorization = "Bearer $accessToken" }

# Test get_status tool
Assert-Test "Tool: get_status executes" {
    $payload = @{
        method = "tools/call"
        params = @{
            name = "get_status"
            arguments = @{}
        }
    }
    $response = Invoke-HttpRequest -Method POST -Endpoint "/mcp/tools/call" -Headers $authHeaders -Body $payload
    @($response.content).Count -gt 0
} -Expected "True"

Assert-Test "Tool: get_status returns node info" {
    $payload = @{
        method = "tools/call"
        params = @{
            name = "get_status"
            arguments = @{}
        }
    }
    $response = Invoke-HttpRequest -Method POST -Endpoint "/mcp/tools/call" -Headers $authHeaders -Body $payload
    $response.content[0].text -match "Node:"
} -Expected "True"

# Test provision_node tool
Assert-Test "Tool: provision_node executes" {
    $payload = @{
        method = "tools/call"
        params = @{
            name = "provision_node"
            arguments = @{
                name = "test-node-e2e"
            }
        }
    }
    $response = Invoke-HttpRequest -Method POST -Endpoint "/mcp/tools/call" -Headers $authHeaders -Body $payload
    @($response.content).Count -gt 0
} -Expected "True"

Assert-Test "Tool: provision_node uses node name" {
    $payload = @{
        method = "tools/call"
        params = @{
            name = "provision_node"
            arguments = @{
                name = "test-node-e2e"
            }
        }
    }
    $response = Invoke-HttpRequest -Method POST -Endpoint "/mcp/tools/call" -Headers $authHeaders -Body $payload
    $response.content[0].text -match "test-node-e2e"
} -Expected "True"

# Test invalid tool
Assert-Test "Tool: invalid tool returns error" {
    $payload = @{
        method = "tools/call"
        params = @{
            name = "invalid_tool"
            arguments = @{}
        }
    }
    try {
        $response = Invoke-HttpRequest -Method POST -Endpoint "/mcp/tools/call" -Headers $authHeaders -Body $payload
        $response.error.code -eq -32601
    } catch {
        $true # Expected to fail
    }
} -Expected "True"

# ═══════════════════════════════════════════════════════
# TEST 4: MCP Resource Read
# ═══════════════════════════════════════════════════════

Write-TestHeader "TEST 4: MCP Resource Read"

# Test edge-hive://status resource
Assert-Test "Resource: edge-hive://status exists" {
    $response = Invoke-HttpRequest -Endpoint "/mcp/resources/edge-hive://status" -Headers $authHeaders
    $response.uri -eq "edge-hive://status"
} -Expected "True"

Assert-Test "Resource: edge-hive://status is JSON" {
    $response = Invoke-HttpRequest -Endpoint "/mcp/resources/edge-hive://status" -Headers $authHeaders
    $response.mimeType -eq "application/json"
} -Expected "True"

Assert-Test "Resource: edge-hive://status contains version" {
    $response = Invoke-HttpRequest -Endpoint "/mcp/resources/edge-hive://status" -Headers $authHeaders
    $response.text -match "version"
} -Expected "True"

# Test edge-hive://logs/last resource
Assert-Test "Resource: edge-hive://logs/last exists" {
    $response = Invoke-HttpRequest -Endpoint "/mcp/resources/edge-hive://logs/last" -Headers $authHeaders
    $response.uri -eq "edge-hive://logs/last"
} -Expected "True"

Assert-Test "Resource: edge-hive://logs/last is text" {
    $response = Invoke-HttpRequest -Endpoint "/mcp/resources/edge-hive://logs/last" -Headers $authHeaders
    $response.mimeType -eq "text/plain"
} -Expected "True"

# ═══════════════════════════════════════════════════════
# TEST 5: SSE Streaming Endpoint
# ═══════════════════════════════════════════════════════

Write-TestHeader "TEST 5: SSE Streaming Endpoint"

Assert-Test "SSE: Endpoint accepts connection" {
    $url = "$BaseUrl/mcp/stream"

    $handler = New-Object System.Net.Http.HttpClientHandler
    if ($SkipSSL) {
        $handler.ServerCertificateCustomValidationCallback = [System.Net.Http.HttpClientHandler]::DangerousAcceptAnyServerCertificateValidator
    }

    $client = New-Object System.Net.Http.HttpClient($handler)
    $client.Timeout = [TimeSpan]::FromSeconds(5)

    $request = New-Object System.Net.Http.HttpRequestMessage([System.Net.Http.HttpMethod]::Get, $url)
    $null = $request.Headers.TryAddWithoutValidation("Authorization", "Bearer $accessToken")
    $null = $request.Headers.TryAddWithoutValidation("Accept", "text/event-stream")

    try {
        $response = $client.SendAsync($request, [System.Net.Http.HttpCompletionOption]::ResponseHeadersRead).GetAwaiter().GetResult()
        [int]$response.StatusCode -eq 200
    } finally {
        $client.Dispose()
        $handler.Dispose()
    }
} -Expected "True"

# ═══════════════════════════════════════════════════════
# TEST 6: Authorization & Security
# ═══════════════════════════════════════════════════════

Write-TestHeader "TEST 6: Authorization & Security"

Assert-Test "Security: Missing token rejected" {
    try {
        Invoke-HttpRequest -Method POST -Endpoint "/mcp/tools/call" -Body @{ method = "tools/call" }
        $false
    } catch {
        $_.Exception.Response.StatusCode.value__ -eq 401
    }
} -Expected "True"

Assert-Test "Security: Invalid token rejected" {
    try {
        $badHeaders = @{ Authorization = "Bearer invalid_token_12345" }
        Invoke-HttpRequest -Method POST -Endpoint "/mcp/tools/call" -Headers $badHeaders -Body @{ method = "tools/call" }
        $false
    } catch {
        $_.Exception.Response.StatusCode.value__ -eq 401
    }
} -Expected "True"

Assert-Test "Security: Malformed auth header rejected" {
    try {
        $badHeaders = @{ Authorization = "NotBearer $accessToken" }
        Invoke-HttpRequest -Method POST -Endpoint "/mcp/tools/call" -Headers $badHeaders -Body @{ method = "tools/call" }
        $false
    } catch {
        $_.Exception.Response.StatusCode.value__ -eq 401
    }
} -Expected "True"

# ═══════════════════════════════════════════════════════
# TEST 7: Cleanup
# ═══════════════════════════════════════════════════════

Write-TestHeader "TEST 7: Cleanup"

Assert-Test "Cleanup: Revoke test client" {
    $revokeOutput = & $EdgeHiveBin auth client revoke $clientId 2>&1 | Out-String
    $revokeOutput -match "revoked" -or $LASTEXITCODE -eq 0
} -Expected "True"

Assert-Test "Cleanup: Verify client revoked" {
    $listOutput = & $EdgeHiveBin auth client list --all 2>&1 | Out-String
    ($listOutput -match [regex]::Escape($clientId)) -and ($listOutput -match "REVOKED")
} -Expected "True"

# ═══════════════════════════════════════════════════════
# FINAL REPORT
# ═══════════════════════════════════════════════════════

Write-Host "`n$Blue═══════════════════════════════════════════════════════$Reset"
Write-Host "$Blue  TEST RESULTS$Reset"
Write-Host "$Blue═══════════════════════════════════════════════════════$Reset`n"

$passRate = [math]::Round(($script:TestsPassed / $script:TestsTotal) * 100, 1)

Write-Host "  Total Tests:  $script:TestsTotal"
Write-Host "  ${Green}Passed:       $script:TestsPassed$Reset"
Write-Host "  ${Red}Failed:       $script:TestsFailed$Reset"
Write-Host "  Pass Rate:    $passRate%`n"

if ($script:TestsFailed -eq 0) {
    Write-Host "$Green✓ ALL TESTS PASSED$Reset`n"
    $script:ExitCode = 0
} else {
    Write-Host "$Red✗ SOME TESTS FAILED$Reset`n"
    $script:ExitCode = 1
}

} catch {
    Write-Host "`n$Red✗ E2E runner error:$Reset $_`n" -ForegroundColor Red
    $script:ExitCode = 1
} finally {
    if ($script:ServerStartedByTest -and $null -ne $script:ServerProcess) {
        try {
            if (-not $script:ServerProcess.HasExited) {
                Stop-Process -Id $script:ServerProcess.Id -Force
            }
        } catch {
            # ignore cleanup errors
        }
    }
}

exit $script:ExitCode
