# Test OAuth2 Authentication System
# This script tests the MCP OAuth2 endpoints

param(
    [string]$BaseUrl = "http://localhost:8080",
    [string]$DockerNode = "http://172.20.0.10:8080"
)

Write-Host "🧪 Testing Edge Hive OAuth2 Authentication System" -ForegroundColor Cyan
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Cyan
Write-Host ""

$ErrorActionPreference = "Continue"

function Test-Endpoint {
    param(
        [string]$Name,
        [string]$Url,
        [string]$Method = "GET",
        [hashtable]$Headers = @{},
        [object]$Body = $null
    )

    Write-Host "📍 Testing: $Name" -ForegroundColor Yellow
    Write-Host "   URL: $Url"
    Write-Host "   Method: $Method"

    try {
        $params = @{
            Uri = $Url
            Method = $Method
            Headers = $Headers
            ContentType = "application/json"
        }

        if ($Body) {
            $params.Body = ($Body | ConvertTo-Json)
        }

        $response = Invoke-RestMethod @params
        Write-Host "   ✅ Success" -ForegroundColor Green
        return $response
    } catch {
        Write-Host "   ❌ Failed: $($_.Exception.Message)" -ForegroundColor Red
        return $null
    }
}

# Test 1: Health Check
Write-Host ""
Write-Host "Test 1: Health Check" -ForegroundColor Magenta
$health = Test-Endpoint -Name "Health" -Url "$BaseUrl/health"
if ($health) {
    Write-Host "   Status: $($health.status)"
    Write-Host "   Version: $($health.version)"
}

# Test 2: Create OAuth2 Client
Write-Host ""
Write-Host "Test 2: Create OAuth2 Client" -ForegroundColor Magenta
$clientBody = @{
    name = "test-vscode-client"
    scopes = @("mcp:read", "mcp:call", "mcp:resources")
}
$client = Test-Endpoint -Name "Create Client" -Url "$BaseUrl/mcp/auth/clients" -Method "POST" -Body $clientBody

if ($client) {
    Write-Host "   Client ID: $($client.client_id)"
    Write-Host "   Client Secret: $($client.client_secret)"
    Write-Host "   Scopes: $($client.scopes -join ', ')"

    # Save credentials for next tests
    $global:ClientId = $client.client_id
    $global:ClientSecret = $client.client_secret
}

# Test 3: Get Access Token (OAuth2 Client Credentials Flow)
Write-Host ""
Write-Host "Test 3: OAuth2 Token Request (Client Credentials)" -ForegroundColor Magenta
if ($global:ClientId -and $global:ClientSecret) {
    $tokenBody = @{
        grant_type = "client_credentials"
        client_id = $global:ClientId
        client_secret = $global:ClientSecret
        scope = "mcp:read mcp:call"
    }

    $token = Test-Endpoint -Name "Get Token" -Url "$BaseUrl/mcp/auth/token" -Method "POST" -Body $tokenBody

    if ($token) {
        Write-Host "   Access Token: $($token.access_token.Substring(0, 30))..."
        Write-Host "   Token Type: $($token.token_type)"
        Write-Host "   Expires In: $($token.expires_in) seconds"
        Write-Host "   Scope: $($token.scope)"

        $global:AccessToken = $token.access_token
    }
} else {
    Write-Host "   ⏩ Skipped (no client credentials)" -ForegroundColor Yellow
}

# Test 4: List OAuth2 Clients
Write-Host ""
Write-Host "Test 4: List OAuth2 Clients" -ForegroundColor Magenta
$clients = Test-Endpoint -Name "List Clients" -Url "$BaseUrl/mcp/auth/clients"

if ($clients) {
    Write-Host "   Total clients: $($clients.clients.Count)"
    foreach ($c in $clients.clients) {
        Write-Host "   - $($c.name) ($($c.client_id)) - Scopes: $($c.scopes -join ', ')"
    }
}

# Test 5: Authenticated Request (using Bearer token)
Write-Host ""
Write-Host "Test 5: Authenticated Request (Bearer Token)" -ForegroundColor Magenta
if ($global:AccessToken) {
    $authHeaders = @{
        Authorization = "Bearer $global:AccessToken"
    }

    $nodeInfo = Test-Endpoint -Name "Get Node Info" -Url "$BaseUrl/api/v1/node" -Headers $authHeaders

    if ($nodeInfo) {
        Write-Host "   Node: $($nodeInfo.name)"
        Write-Host "   Peer ID: $($nodeInfo.peer_id)"
    }
} else {
    Write-Host "   ⏩ Skipped (no access token)" -ForegroundColor Yellow
}

# Test 6: Invalid Token Test
Write-Host ""
Write-Host "Test 6: Invalid Token Test (should fail)" -ForegroundColor Magenta
$invalidHeaders = @{
    Authorization = "Bearer invalid_token_12345"
}
$invalid = Test-Endpoint -Name "Invalid Token" -Url "$BaseUrl/api/v1/node" -Headers $invalidHeaders

# Test 7: Token Expiration (decode JWT)
Write-Host ""
Write-Host "Test 7: JWT Token Inspection" -ForegroundColor Magenta
if ($global:AccessToken) {
    try {
        $parts = $global:AccessToken.Split('.')
        $payload = $parts[1]

        # Add padding if needed
        $padding = 4 - ($payload.Length % 4)
        if ($padding -ne 4) {
            $payload += "=" * $padding
        }

        $decoded = [System.Text.Encoding]::UTF8.GetString([Convert]::FromBase64String($payload))
        $claims = $decoded | ConvertFrom-Json

        Write-Host "   Subject (client_id): $($claims.sub)"
        Write-Host "   Issuer: $($claims.iss)"
        Write-Host "   Audience: $($claims.aud)"
        Write-Host "   Scopes: $($claims.scopes -join ', ')"
        Write-Host "   Expires: $(([DateTimeOffset]::FromUnixTimeSeconds($claims.exp)).ToString('yyyy-MM-dd HH:mm:ss'))"
        Write-Host "   Issued At: $(([DateTimeOffset]::FromUnixTimeSeconds($claims.iat)).ToString('yyyy-MM-dd HH:mm:ss'))"
    } catch {
        Write-Host "   ❌ Failed to decode JWT: $($_.Exception.Message)" -ForegroundColor Red
    }
} else {
    Write-Host "   ⏩ Skipped (no access token)" -ForegroundColor Yellow
}

# Test 8: Revoke Client
Write-Host ""
Write-Host "Test 8: Revoke OAuth2 Client" -ForegroundColor Magenta
if ($global:ClientId) {
    $null = Test-Endpoint -Name "Revoke Client" -Url "$BaseUrl/mcp/auth/clients/$global:ClientId" -Method "DELETE"

    # Try to use revoked credentials
    Write-Host "   Verifying revocation..."
    $tokenBody = @{
        grant_type = "client_credentials"
        client_id = $global:ClientId
        client_secret = $global:ClientSecret
    }

    $revokedTest = Test-Endpoint -Name "Revoked Client Test" -Url "$BaseUrl/mcp/auth/token" -Method "POST" -Body $tokenBody

    if ($null -eq $revokedTest) {
        Write-Host "   ✅ Revocation confirmed (token request failed as expected)" -ForegroundColor Green
    } else {
        Write-Host "   ⚠️  Warning: Revoked client still got a token" -ForegroundColor Yellow
    }
} else {
    Write-Host "   ⏩ Skipped (no client to revoke)" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Cyan
Write-Host "✅ OAuth2 Testing Complete" -ForegroundColor Green
Write-Host ""

# Summary
Write-Host "📊 Summary:" -ForegroundColor Cyan
Write-Host "   Health Check: $(if($health) { '✅ Pass' } else { '❌ Fail' })"
Write-Host "   Client Creation: $(if($client) { '✅ Pass' } else { '❌ Fail' })"
Write-Host "   Token Generation: $(if($token) { '✅ Pass' } else { '❌ Fail' })"
Write-Host "   Bearer Auth: $(if($nodeInfo) { '✅ Pass' } else { '❌ Fail' })"
Write-Host "   Client Revocation: $(if($null -eq $revokedTest) { '✅ Pass' } else { '❌ Fail' })"
Write-Host ""

# Docker test
if ($DockerNode -and (Test-Connection -ComputerName 172.20.0.10 -Count 1 -Quiet 2>$null)) {
    Write-Host "🐳 Testing Docker Node at $DockerNode" -ForegroundColor Cyan
    $dockerHealth = Test-Endpoint -Name "Docker Health" -Url "$DockerNode/health"
    if ($dockerHealth) {
        Write-Host "   ✅ Docker node is accessible" -ForegroundColor Green
    }
}
