<#
.SYNOPSIS
    Clean closed GitHub issues from local .github/issues folder

.DESCRIPTION
    Checks GitHub for closed issues and removes corresponding local .md files

.EXAMPLE
    ./clean-closed-issues.ps1
    ./clean-closed-issues.ps1 -DryRun
#>

param(
    [switch]$DryRun
)

$ErrorActionPreference = "Stop"
$IssuesDir = ".github/issues"

function Write-Success { param($msg) Write-Host "[OK] $msg" -ForegroundColor Green }
function Write-Info { param($msg) Write-Host "[INFO] $msg" -ForegroundColor Cyan }
function Write-Warn { param($msg) Write-Host "[WARN] $msg" -ForegroundColor Yellow }

# Get closed issues from GitHub
Write-Info "Fetching closed issues from GitHub..."
$closedJson = gh issue list --state closed --limit 100 --json number 2>$null
if (-not $closedJson) {
    Write-Warn "Could not fetch closed issues from GitHub"
    exit 1
}

$closedIssues = ($closedJson | ConvertFrom-Json).number
Write-Info "Found $($closedIssues.Count) closed issues"

# Scan local issue files
$deleted = 0
$files = Get-ChildItem -Path $IssuesDir -Filter "*.md" -ErrorAction SilentlyContinue |
         Where-Object { $_.Name -notmatch "^_" -and $_.Name -ne ".gitkeep" }

foreach ($file in $files) {
    $content = Get-Content $file.FullName -Raw -Encoding UTF8 -ErrorAction SilentlyContinue

    # Check if file has github_issue field
    if ($content -match "github_issue:\s*(\d+)") {
        $issueNum = [int]$matches[1]

        if ($closedIssues -contains $issueNum) {
            if ($DryRun) {
                Write-Warn "[DRY-RUN] Would delete: $($file.Name) (issue #$issueNum is closed)"
            } else {
                Remove-Item $file.FullName -Force
                Write-Success "Deleted: $($file.Name) (issue #$issueNum is closed)"
            }
            $deleted++
        }
    }
}

# Also check by title matching for files without github_issue field
$openIssues = gh issue list --state open --limit 200 --json title 2>$null | ConvertFrom-Json

foreach ($file in $files) {
    $content = Get-Content $file.FullName -Raw -Encoding UTF8 -ErrorAction SilentlyContinue

    # Skip if already has github_issue
    if ($content -match "github_issue:") { continue }

    # Extract title from frontmatter
    if ($content -match 'title:\s*"?([^"\n]+)"?') {
        $localTitle = $matches[1].Trim()

        # Check if this title exists in open issues
        $matchingOpen = $openIssues | Where-Object { $_.title -eq $localTitle }

        if (-not $matchingOpen) {
            # Check if it was ever created and is now closed
            $closedMatch = gh issue list --state closed --search "$localTitle" --json number,title 2>$null | ConvertFrom-Json

            if ($closedMatch -and $closedMatch.Count -gt 0) {
                $matchedIssue = $closedMatch | Where-Object { $_.title -eq $localTitle }
                if ($matchedIssue) {
                    if ($DryRun) {
                        Write-Warn "[DRY-RUN] Would delete: $($file.Name) (matches closed issue: $localTitle)"
                    } else {
                        Remove-Item $file.FullName -Force
                        Write-Success "Deleted: $($file.Name) (matches closed issue)"
                    }
                    $deleted++
                }
            }
        }
    }
}

if ($deleted -eq 0) {
    Write-Success "No closed issues to clean up"
} elseif ($DryRun) {
    Write-Info "Would delete $deleted files. Run without -DryRun to execute."
} else {
    Write-Success "Cleaned $deleted closed issues"
}
