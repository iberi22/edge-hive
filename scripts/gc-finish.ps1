<#
.SYNOPSIS
    Master script to finish a session according to Git-Core Protocol.
.DESCRIPTION
    1. Synchronizes local issues with GitHub.
    2. Cleans up local .md files for closed issues.
    3. Stages all changes.
    4. Performs atomic commits.
    5. Pushes and creates/updates a PR.
    6. Generates an AI Report.
#>

param(
    [switch]$SkipPush,
    [string]$PrBranch = "master"
)

$ErrorActionPreference = "Continue" # Don't stop on minor errors like "no issues to sync"

Write-Host "🚀 Finishing session..." -ForegroundColor Cyan

# 1. Sync issues
Write-Host "📋 Synchronizing issues..." -ForegroundColor Blue
pwsh -File ./scripts/sync-issues.ps1

# 2. Clean closed issues
Write-Host "🧹 Cleaning up closed issues..." -ForegroundColor Blue
pwsh -File ./scripts/clean-closed-issues.ps1

# 3. Stage changes
Write-Host "📦 Staging all changes..." -ForegroundColor Blue
git add .

# 4. Atomic commits
Write-Host "⚛️  Performing atomic commits..." -ForegroundColor Blue
pwsh -File ./scripts/git-atomize.ps1 -Auto

# 5. Push and PR (if not skipped)
if (-not $SkipPush) {
    Write-Host "📤 Pushing changes..." -ForegroundColor Blue
    $currentBranch = git rev-parse --abbrev-ref HEAD
    git push origin $currentBranch

    Write-Host "🔗 Creating/Updating PR..." -ForegroundColor Blue
    $prResult = gh pr create --fill 2>&1
    if ($prResult -match "pull request already exists") {
        Write-Host "ℹ️  PR already exists." -ForegroundColor Cyan
    }

    # Get PR number
    $prNumber = gh pr view --json number --template '{{.number}}'

    # 6. AI Report
    if ($prNumber) {
        Write-Host "🤖 Generating AI Report for PR #$prNumber..." -ForegroundColor Blue
        pwsh -File ./scripts/ai-report.ps1 -PrNumber $prNumber
    } else {
        Write-Host "🤖 Generating AI Report for last commits..." -ForegroundColor Blue
        pwsh -File ./scripts/ai-report.ps1
    }
}

Write-Host "✅ Session finished successfully!" -ForegroundColor Green
