$ErrorActionPreference = "Stop"

$ExePath = "target/release/edge-hive.exe"
if (!(Test-Path $ExePath)) {
    Write-Host "⚠️  Executable not found. Building..."
    cargo build --quiet --release -p edge-hive-core
}

Write-Host "🧪 Testing MCP Server..."

$Request = '{"jsonrpc": "2.0", "method": "initialize", "params": {}, "id": 1}'
$ProcessInfo = New-Object System.Diagnostics.ProcessStartInfo
$ProcessInfo.FileName = $ExePath
$ProcessInfo.Arguments = "mcp"
$ProcessInfo.RedirectStandardInput = $true
$ProcessInfo.RedirectStandardOutput = $true
$ProcessInfo.UseShellExecute = $false
$ProcessInfo.CreateNoWindow = $true

$Process = New-Object System.Diagnostics.Process
$Process.StartInfo = $ProcessInfo
try {
    $Process.Start() | Out-Null
} catch {
    Write-Error "Failed to start process: $_"
    exit 1
}

$Process.StandardInput.WriteLine($Request)
$Process.StandardInput.Flush()

# Read response (blocking with timeout simulation logic if needed, but keeping simple)
$Response = $Process.StandardOutput.ReadLine()

Write-Host "⬅️  Sent: $Request"
Write-Host "➡️  Received: $Response"

$Json = $Response | ConvertFrom-Json

if ($Json.result.serverInfo.name -eq "edge-hive-mcp") {
    Write-Host "✅ MCP Integration Test Passed!" -ForegroundColor Green
} else {
    Write-Host "❌ MCP Test Failed" -ForegroundColor Red
    Write-Host "Expected server name 'edge-hive-mcp', got '$($Json.result.serverInfo.name)'"
    exit 1
}

# Clean up
try {
    if (!$Process.HasExited) {
        $Process.Kill()
    }
} catch {}
