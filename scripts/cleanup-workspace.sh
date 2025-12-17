#!/bin/bash
# Git-Core Protocol: Workspace Cleanup
# Reorganizes logs, temp files, and build artifacts according to industry standards

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Options
DRY_RUN=false
HELP=false

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --dry-run|-d) DRY_RUN=true; shift ;;
        --help|-h) HELP=true; shift ;;
        *) echo "Unknown option: $1"; exit 1 ;;
    esac
done

if [ "$HELP" = true ]; then
    cat << EOF
Usage: ./cleanup-workspace.sh [OPTIONS]

Cleanup and reorganize logs, temp files, and build artifacts

Options:
  --dry-run, -d    Show what would be moved without moving
  --help, -h       Show this help message

Examples:
  ./cleanup-workspace.sh              # Perform cleanup
  ./cleanup-workspace.sh --dry-run    # Preview changes

EOF
    exit 0
fi

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
LOG_ACTION=$([ "$DRY_RUN" = true ] && echo "Would move" || echo "Moving")

logs=0
temps=0
builds=0

echo -e "${CYAN}🧹 Git-Core Protocol: Workspace Cleanup${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Create directories if they don't exist
for dir in logs .tmp build-output; do
    if [ ! -d "$PROJECT_ROOT/$dir" ]; then
        mkdir -p "$PROJECT_ROOT/$dir"
        echo "📁 Created directory: $dir"
    fi
done

echo ""
echo "📋 Analyzing root directory..."
echo ""

# 1. Move .log files to logs/
echo "1️⃣  Logs (.log files)"
echo "──────────────────────────────────────────"
for file in "$PROJECT_ROOT"/*.log; do
    [ -f "$file" ] || continue
    basename=$(basename "$file")
    if [ "$DRY_RUN" = true ]; then
        echo "  $LOG_ACTION: $basename → logs/"
    else
        mv "$file" "$PROJECT_ROOT/logs/$basename"
        echo "  ✅ Moved: $basename"
    fi
    ((logs++))
done

# 2. Move test output files
echo ""
echo "2️⃣  Test Output Files"
echo "──────────────────────────────────────────"
for pattern in "test-*.txt" "test-*.log" "build-*.log" "serve-*.log" "deploy-*.log"; do
    for file in "$PROJECT_ROOT"/$pattern; do
        [ -f "$file" ] || continue
        basename=$(basename "$file")
        if [ "$DRY_RUN" = true ]; then
            echo "  $LOG_ACTION: $basename → logs/"
        else
            mv "$file" "$PROJECT_ROOT/logs/$basename"
            echo "  ✅ Moved: $basename"
        fi
        ((logs++))
    done
done

# 3. Move .tmp_* files to .tmp/
echo ""
echo "3️⃣  Temporary Files (.tmp_* pattern)"
echo "──────────────────────────────────────────"
for file in "$PROJECT_ROOT"/.tmp_*; do
    [ -f "$file" ] || continue
    basename=$(basename "$file")
    if [ "$DRY_RUN" = true ]; then
        echo "  $LOG_ACTION: $basename → .tmp/"
    else
        mv "$file" "$PROJECT_ROOT/.tmp/$basename"
        echo "  ✅ Moved: $basename"
    fi
    ((temps++))
done

# 4. Move build artifacts
echo ""
echo "4️⃣  Build Artifacts"
echo "──────────────────────────────────────────"
for filename in "build-cli.txt" "build-app.log" "build-core.log" "build-docker.log" "build-identity.log" "build-tunnel.log"; do
    file="$PROJECT_ROOT/$filename"
    if [ -f "$file" ]; then
        if [ "$DRY_RUN" = true ]; then
            echo "  $LOG_ACTION: $filename → build-output/"
        else
            mv "$file" "$PROJECT_ROOT/build-output/$filename"
            echo "  ✅ Moved: $filename"
        fi
        ((builds++))
    fi
done

# Summary
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${GREEN}📊 Summary${NC}"
echo "──────────────────────────────────────────"
echo "  📝 Logs moved to logs/:          $logs files"
echo "  📌 Temp files moved to .tmp/:    $temps files"
echo "  🔨 Build artifacts to build/:   $builds files"
echo ""

if [ "$DRY_RUN" = true ]; then
    echo -e "${YELLOW}🔍 DRY RUN MODE - No files were moved${NC}"
    echo -e "${YELLOW}   Run without --dry-run to apply changes${NC}"
fi

echo ""
echo -e "${CYAN}📖 Directory Structure${NC}"
echo "──────────────────────────────────────────"
echo "  logs/           - Build & test logs (.log, .txt)"
echo "  .tmp/           - Temporary files (scripts, analysis)"
echo "  build-output/   - Build artifacts & binaries"
echo ""
echo -e "${GREEN}✅ Workspace organized according to Git-Core Protocol v3.2${NC}"
