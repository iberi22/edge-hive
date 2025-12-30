# Edge Hive PowerShell Script: Sync Issues to GitHub
# Usage: ./scripts/sync-issues.ps1

param(
    [switch]$DryRun,
    [string]$Filter = "*"
)

$ErrorActionPreference = "Stop"

Write-Host "🐝 Edge Hive Issue Sync" -ForegroundColor Yellow
Write-Host "========================" -ForegroundColor Yellow
Write-Host ""

# Check gh CLI
if (-not (Get-Command gh -ErrorAction SilentlyContinue)) {
    Write-Host "ERROR: GitHub CLI (gh) not found" -ForegroundColor Red
    Write-Host "   Install: winget install GitHub.cli" -ForegroundColor Gray
    exit 1
}

# Check auth
$authStatus = gh auth status 2>&1
if ($LASTEXITCODE -ne 0) {
    Write-Host "ERROR: Not authenticated with GitHub" -ForegroundColor Red
    Write-Host "   Run: gh auth login" -ForegroundColor Gray
    exit 1
}

$issuesDir = Join-Path $PSScriptRoot (Join-Path ".." (Join-Path ".github" "issues"))
$files = Get-ChildItem -Path $issuesDir -Filter "$Filter.md"

Write-Host "INFO: Found $($files.Count) issue files" -ForegroundColor Cyan
Write-Host ""

foreach ($file in $files) {
    $content = Get-Content $file.FullName -Raw

    # Parse frontmatter
    if ($content -match "(?s)^---\r?\n(.+?)\r?\n---") {
        $frontmatter = $matches[1]
        $body = $content -replace "(?s)^---\r?\n.+?\r?\n---\r?\n", ""

        # Extract title
        if ($frontmatter -match 'title:\s*(?:["'']?)([^"''\r\n]+)(?:["'']?)') {
            $title = $matches[1]
        } else {
            Write-Host "WARN: Skipping $($file.Name): No title found" -ForegroundColor Yellow
            continue
        }

        # Extract labels
        $labels = @()
        if ($frontmatter -match '(?s)labels:\s*\r?\n((?:\s*-\s*.+\r?\n?)+)') {
            $labelsBlock = $matches[1]
            $labels = $labelsBlock -split "`n" | ForEach-Object {
                $_ -replace "^\s*-\s*", "" | ForEach-Object { $_.Trim() }
            } | Where-Object { $_ }
        }

        Write-Host "INFO: $title" -ForegroundColor Cyan
        Write-Host "   Labels: $($labels -join ', ')" -ForegroundColor Gray

        if ($DryRun) {
            Write-Host "   [DRY RUN] Would create/update issue" -ForegroundColor Yellow
            continue
        }

        # Check if issue exists
        $searchQuery = "in:title `"$title`""
        $existingIssue = gh issue list --search $searchQuery --json number,title 2>&1 | ConvertFrom-Json

        if ($existingIssue -and $existingIssue.Count -gt 0) {
            $issueNum = $existingIssue[0].number
            Write-Host "   Updating Issue $issueNum..." -ForegroundColor Gray

            $labelArgs = if ($labels) { "--add-label `"$($labels -join ',')`"" } else { "" }
            gh issue edit $issueNum --body $body $labelArgs

            Write-Host "   SUCCESS: Updated Issue $issueNum" -ForegroundColor Green
        } else {
            Write-Host "   Creating new issue..." -ForegroundColor Gray

            $labelArgs = if ($labels) { "--label `"$($labels -join ',')`"" } else { "" }
            $result = gh issue create --title $title --body $body $labelArgs 2>&1

            if ($result -match "#(\d+)") {
                Write-Host "   SUCCESS: Created Issue $($matches[1])" -ForegroundColor Green
            } else {
                Write-Host "   SUCCESS: Created" -ForegroundColor Green
            }
        }
    }
}

Write-Host ""
Write-Host "SUCCESS: Sync complete!" -ForegroundColor Green
