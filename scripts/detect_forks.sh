#!/bin/bash
# LuxiEdge Fork and Usage Detector
# Run this script to check for unauthorized usage and forks

set -e

REPO="RegularJoe-CEO/LuxiEdge"
REPORT_FILE="fork_detection_report_$(date +%Y%m%d_%H%M%S).txt"

echo "╔══════════════════════════════════════════════════════════╗"
echo "║         LuxiEdge Fork & Usage Detection Tool            ║"
echo "╠══════════════════════════════════════════════════════════╣"
echo "║                                                          ║"
echo "║  Checking for forks, clones, and unauthorized usage     ║"
echo "║  Report will be saved to: $REPORT_FILE                  ║"
echo "║                                                          ║"
echo "╚══════════════════════════════════════════════════════════╝"
echo ""

# Create report header
cat > "$REPORT_FILE" << EOF
LuxiEdge Fork & Usage Detection Report
Generated: $(date)
Repository: https://github.com/$REPO

═══════════════════════════════════════════════════════════════

EOF

echo "Step 1: Checking GitHub forks..."
echo "═══════════════════════════════════════════════════════════" >> "$REPORT_FILE"
echo "GITHUB FORKS" >> "$REPORT_FILE"
echo "═══════════════════════════════════════════════════════════" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

if command -v gh &> /dev/null; then
    # Using GitHub CLI
    echo "Using GitHub CLI (gh)..."
    FORKS=$(gh api repos/$REPO/forks --paginate)
    FORK_COUNT=$(echo "$FORKS" | jq '. | length')
    
    echo "Found $FORK_COUNT public forks" | tee -a "$REPORT_FILE"
    echo "" >> "$REPORT_FILE"
    
    if [ "$FORK_COUNT" -gt 0 ]; then
        echo "$FORKS" | jq -r '.[] | "Fork: \(.full_name)\n  Owner: \(.owner.login)\n  Created: \(.created_at)\n  Stars: \(.stargazers_count)\n  Updated: \(.updated_at)\n  URL: \(.html_url)\n"' >> "$REPORT_FILE"
    else
        echo "No public forks found." >> "$REPORT_FILE"
    fi
else
    # Fallback to curl
    echo "Using curl (install 'gh' for better results)..."
    FORKS=$(curl -s "https://api.github.com/repos/$REPO/forks?per_page=100")
    FORK_COUNT=$(echo "$FORKS" | jq '. | length')
    
    echo "Found $FORK_COUNT public forks" | tee -a "$REPORT_FILE"
    echo "" >> "$REPORT_FILE"
    
    if [ "$FORK_COUNT" -gt 0 ]; then
        echo "$FORKS" | jq -r '.[] | "Fork: \(.full_name)\n  Owner: \(.owner.login)\n  Created: \(.created_at)\n  URL: \(.html_url)\n"' >> "$REPORT_FILE"
    else
        echo "No public forks found." >> "$REPORT_FILE"
    fi
fi

echo "" >> "$REPORT_FILE"
echo "Step 2: Checking traffic stats (requires repo access)..."
echo "═══════════════════════════════════════════════════════════" >> "$REPORT_FILE"
echo "REPOSITORY TRAFFIC" >> "$REPORT_FILE"
echo "═══════════════════════════════════════════════════════════" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

if command -v gh &> /dev/null; then
    # Clone stats
    CLONES=$(gh api repos/$REPO/traffic/clones 2>/dev/null || echo '{"count": 0, "uniques": 0}')
    echo "Clone Statistics:" >> "$REPORT_FILE"
    echo "$CLONES" | jq -r '"  Total clones: \(.count)\n  Unique cloners: \(.uniques)"' >> "$REPORT_FILE"
    
    # Views stats
    VIEWS=$(gh api repos/$REPO/traffic/views 2>/dev/null || echo '{"count": 0, "uniques": 0}')
    echo "" >> "$REPORT_FILE"
    echo "View Statistics:" >> "$REPORT_FILE"
    echo "$VIEWS" | jq -r '"  Total views: \(.count)\n  Unique visitors: \(.uniques)"' >> "$REPORT_FILE"
    
    # Popular content
    PATHS=$(gh api repos/$REPO/traffic/popular/paths 2>/dev/null || echo '[]')
    echo "" >> "$REPORT_FILE"
    echo "Most Viewed Files:" >> "$REPORT_FILE"
    echo "$PATHS" | jq -r '.[] | "  \(.path): \(.count) views"' >> "$REPORT_FILE"
else
    echo "Traffic stats require GitHub CLI (gh) with authentication." >> "$REPORT_FILE"
fi

echo "" >> "$REPORT_FILE"
echo "Step 3: Searching for code copies online..."
echo "═══════════════════════════════════════════════════════════" >> "$REPORT_FILE"
echo "ONLINE CODE SEARCH" >> "$REPORT_FILE"
echo "═══════════════════════════════════════════════════════════" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

# List of distinctive code patterns to search for
SEARCH_PATTERNS=(
    "simd_eval_over_x_inplace"
    "LicenseRef-Luxi-Business-1.0"
    "luxi_eval::lexer::tokenize"
    "RegularJoe-CEO"
)

echo "Distinctive code patterns found in public search:" >> "$REPORT_FILE"
echo "(Manual verification required - open these URLs in browser)" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

for pattern in "${SEARCH_PATTERNS[@]}"; do
    ENCODED_PATTERN=$(echo "$pattern" | sed 's/ /%20/g')
    GITHUB_SEARCH="https://github.com/search?type=code&q=${ENCODED_PATTERN}"
    GOOGLE_SEARCH="https://www.google.com/search?q=\"${ENCODED_PATTERN}\""
    
    echo "Pattern: $pattern" >> "$REPORT_FILE"
    echo "  GitHub: $GITHUB_SEARCH" >> "$REPORT_FILE"
    echo "  Google: $GOOGLE_SEARCH" >> "$REPORT_FILE"
    echo "" >> "$REPORT_FILE"
done

echo "" >> "$REPORT_FILE"
echo "Step 4: Checking for similar project names..."
echo "═══════════════════════════════════════════════════════════" >> "$REPORT_FILE"
echo "SIMILAR PROJECTS" >> "$REPORT_FILE"
echo "═══════════════════════════════════════════════════════════" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

# Search GitHub for similar names
SIMILAR_NAMES=("luxi" "luxiedge" "edge-eval" "simd-eval")

for name in "${SIMILAR_NAMES[@]}"; do
    echo "Searching for: $name" >> "$REPORT_FILE"
    echo "  https://github.com/search?q=${name}&type=repositories" >> "$REPORT_FILE"
    echo "" >> "$REPORT_FILE"
done

echo "" >> "$REPORT_FILE"
echo "Step 5: Checking for Stripe-related forks..."
echo "═══════════════════════════════════════════════════════════" >> "$REPORT_FILE"
echo "STRIPE INTEGRATION DETECTION" >> "$REPORT_FILE"
echo "═══════════════════════════════════════════════════════════" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

echo "Google search suggestions:" >> "$REPORT_FILE"
echo "  https://www.google.com/search?q=\"luxi+edge\"+stripe" >> "$REPORT_FILE"
echo "  https://www.google.com/search?q=\"luxiedge\"+payment" >> "$REPORT_FILE"
echo "  https://www.google.com/search?q=site:github.com+luxi+stripe" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

echo "" >> "$REPORT_FILE"
echo "═══════════════════════════════════════════════════════════" >> "$REPORT_FILE"
echo "SUMMARY" >> "$REPORT_FILE"
echo "═══════════════════════════════════════════════════════════" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

cat >> "$REPORT_FILE" << EOF
Total Public Forks: $FORK_COUNT

Next Steps:
1. Review each fork listed above
2. Check the manual search URLs for code copies
3. Investigate any suspicious forks or copies
4. Contact fork owners about licensing if commercial use detected

For detailed analysis, see:
- SECURITY_ASSESSMENT_REPORT.md
- SECURITY_RECOMMENDATIONS.md

Questions or Concerns?
Contact: e@ewaller.com

═══════════════════════════════════════════════════════════════
END OF REPORT
═══════════════════════════════════════════════════════════════
EOF

echo ""
echo "✓ Detection complete!"
echo "✓ Report saved to: $REPORT_FILE"
echo ""
echo "Key Findings:"
echo "  - Public forks: $FORK_COUNT"
echo "  - Manual searches required: See report for URLs"
echo ""
echo "Please review the report file and follow up on any suspicious activity."
echo ""

# Open report in default editor (optional)
if command -v less &> /dev/null; then
    echo "Press Enter to view the report (or Ctrl+C to exit)..."
    read
    less "$REPORT_FILE"
fi
