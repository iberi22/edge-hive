$closed = @(
    "53", "52", "51", "50", "46", "42", "41", "40", "31", "30", "29", "28", "21", "20", "19", "17", "16", "12"
)

$files = Get-ChildItem -Path .github/issues/*.md
foreach ($file in $files) {
    $content = Get-Content $file.FullName -Raw
    # Try to find issue number in frontmatter if added by sync
    if ($content -match "github_issue:\s*(\d+)") {
        $num = $matches[1]
        if ($closed -contains $num) {
            Remove-Item $file.FullName -Force
            Write-Host "Deleted $file because issue #$num is closed."
        }
    }
}

# Manual cleanup by name for those without numbers
$toManualDelete = @(
    "MVP_frontend-bindings.md",
    "MVP_e2e-testing.md",
    "INFRA_termux-installer.md",
    "DEPLOY_docker-compose.md",
    "MVP_vpn-wireguard.md",
    "FEAT_cloud-provisioning.md",
    "FEAT_stripe-billing.md",
    "FEAT_server-control.md",
    "TASK_e2e-testing.md",
    "FEAT_surrealdb-integration.md",
    "FEAT_libp2p-discovery.md",
    "FEAT_tor-core-integration.md",
    "CORE_workspace-setup.md",
    "NET_node-discovery.md",
    "NET_tor-integration.md",
    "INFRA_termux-deployment.md",
    "DEPLOY_termux-installer.md",
    "DATA_surrealdb-integration.md"
)

foreach ($name in $toManualDelete) {
    if (Test-Path ".github/issues/$name") {
        Remove-Item ".github/issues/$name" -Force
        Write-Host "Manually deleted .github/issues/$name"
    }
}
