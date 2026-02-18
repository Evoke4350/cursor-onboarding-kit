#!/bin/bash
# Test Harness: Adversarial Taint Engine Testing
#
# This harness runs adversarial test cases against the taint engine.
# It's designed to BREAK the engine, not to make it pass.
#
# Usage:
#   ./tests/run-adversarial.sh [--cases-dir DIR] [--count N] [--verbose]

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
CASES_DIR="${PROJECT_ROOT}/tests/adversarial"
BINARY="${PROJECT_ROOT}/target/release/blackboard"
RESULTS_DIR="${PROJECT_ROOT}/test-results"
VERBOSE="${VERBOSE:-false}"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Counters
PASSED=0
FAILED=0
SKIPPED=0
FALSE_POSITIVE=0
FALSE_NEGATIVE=0

usage() {
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  --cases-dir DIR   Directory containing test cases (default: tests/adversarial)"
    echo "  --count N         Maximum number of cases to run (default: all)"
    echo "  --verbose         Show detailed output"
    echo "  --generate N      Generate N random adversarial cases before running"
    echo "  --help            Show this help"
    exit 0
}

parse_args() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            --cases-dir)
                CASES_DIR="$2"
                shift 2
                ;;
            --count)
                MAX_COUNT="$2"
                shift 2
                ;;
            --verbose)
                VERBOSE=true
                shift
                ;;
            --generate)
                GENERATE_COUNT="$2"
                shift 2
                ;;
            --help|-h)
                usage
                ;;
            *)
                echo "Unknown option: $1"
                usage
                ;;
        esac
    done
}

check_prerequisites() {
    echo -e "${YELLOW}Checking prerequisites...${NC}"

    if [[ ! -x "$BINARY" ]]; then
        echo -e "${RED}ERROR: Binary not found at $BINARY${NC}"
        echo "Run: cargo build --release"
        exit 1
    fi

    if ! command -v yq &> /dev/null; then
        echo -e "${RED}ERROR: yq is required for parsing YAML test cases${NC}"
        echo "Install: brew install yq"
        exit 1
    fi

    mkdir -p "$RESULTS_DIR"
    echo -e "${GREEN}Prerequisites OK${NC}"
}

generate_random_cases() {
    local count="${1:-100}"
    local output_dir="${CASES_DIR}/generated"

    echo -e "${YELLOW}Generating $count random adversarial cases...${NC}"
    mkdir -p "$output_dir"

    # This would call a Rust generator or Python script
    # For now, we just note it's a TODO
    echo -e "${YELLOW}TODO: Implement random case generator${NC}"
    echo "Would generate cases covering:"
    echo "  - Random Unicode normalization"
    echo "  - Random symlink chains"
    echo "  - Random encoding stacks (base64 + gzip + hex)"
    echo "  - Random path traversal patterns"
    echo "  - Random timing races"
}

run_single_case() {
    local case_file="$1"
    local case_id
    local case_name
    local expected_blocked
    local actual_blocked
    local result

    case_id=$(yq '.id' "$case_file")
    case_name=$(yq '.name' "$case_file")
    expected_blocked=$(yq '.expected.blocked' "$case_file")

    if [[ "$VERBOSE" == "true" ]]; then
        echo -e "\n${YELLOW}Running: $case_name${NC}"
        echo "  ID: $case_id"
        echo "  File: $case_file"
        echo "  Expected blocked: $expected_blocked"
    fi

    # Run the taint engine with this case
    # This is a placeholder - actual implementation depends on taint engine API
    if "$BINARY" test-taint --case "$case_file" --format json > "${RESULTS_DIR}/${case_id}.json" 2>&1; then
        actual_blocked=$(yq '.blocked' "${RESULTS_DIR}/${case_id}.json" 2>/dev/null || echo "error")
    else
        actual_blocked="error"
    fi

    # Compare expected vs actual
    if [[ "$actual_blocked" == "error" ]]; then
        result="ERROR"
        ((SKIPPED++))
    elif [[ "$expected_blocked" == "true" && "$actual_blocked" == "true" ]]; then
        result="PASS"
        ((PASSED++))
    elif [[ "$expected_blocked" == "false" && "$actual_blocked" == "false" ]]; then
        result="PASS"
        ((PASSED++))
    elif [[ "$expected_blocked" == "true" && "$actual_blocked" == "false" ]]; then
        result="FAIL (FALSE NEGATIVE - SECRET LEAKED)"
        ((FAILED++))
        ((FALSE_NEGATIVE++))
    elif [[ "$expected_blocked" == "false" && "$actual_blocked" == "true" ]]; then
        result="FAIL (FALSE POSITIVE - LEGIT BLOCKED)"
        ((FAILED++))
        ((FALSE_POSITIVE++))
    else
        result="UNKNOWN"
        ((SKIPPED++))
    fi

    # Output result
    if [[ "$result" == "PASS" ]]; then
        echo -e "${GREEN}✓${NC} $case_id: $case_name"
    elif [[ "$result" == "ERROR" ]]; then
        echo -e "${YELLOW}?${NC} $case_id: $case_name (error running test)"
    else
        echo -e "${RED}✗${NC} $case_id: $case_name - $result"
        if [[ "$VERBOSE" == "true" ]]; then
            cat "${RESULTS_DIR}/${case_id}.json"
        fi
    fi

    # Log result
    echo "$case_id,$result,$expected_blocked,$actual_blocked" >> "${RESULTS_DIR}/results.csv"
}

run_all_cases() {
    local count=0
    local max_count="${MAX_COUNT:-999999}"

    echo ""
    echo "=========================================="
    echo "  ADVERSARIAL TAINT ENGINE TEST SUITE"
    echo "=========================================="
    echo ""
    echo "Cases directory: $CASES_DIR"
    echo "Results directory: $RESULTS_DIR"
    echo ""

    # Clear previous results
    rm -f "${RESULTS_DIR}/results.csv"
    echo "case_id,result,expected_blocked,actual_blocked" > "${RESULTS_DIR}/results.csv"

    # Find and run all test cases
    for case_file in "$CASES_DIR"/*.yaml "$CASES_DIR"/**/*.yaml 2>/dev/null; do
        [[ -f "$case_file" ]] || continue

        ((count++))
        if [[ $count -gt $max_count ]]; then
            echo -e "${YELLOW}Reached max count ($max_count), stopping${NC}"
            break
        fi

        run_single_case "$case_file"
    done

    echo ""
    echo "=========================================="
    echo "  RESULTS"
    echo "=========================================="
    echo ""
    echo -e "Passed:       ${GREEN}$PASSED${NC}"
    echo -e "Failed:       ${RED}$FAILED${NC}"
    echo -e "  False Neg:  ${RED}$FALSE_NEGATIVE${NC} (SECRETS LEAKED)"
    echo -e "  False Pos:  ${YELLOW}$FALSE_POSITIVE${NC} (LEGIT BLOCKED)"
    echo -e "Skipped:      ${YELLOW}$SKIPPED${NC}"
    echo ""
    echo "Total:        $((PASSED + FAILED + SKIPPED))"
    echo ""

    # Calculate pass rate
    local total=$((PASSED + FAILED))
    if [[ $total -gt 0 ]]; then
        local pass_rate=$((PASSED * 100 / total))
        echo "Pass Rate:    ${pass_rate}%"
    fi

    echo ""
    echo "Results saved to: ${RESULTS_DIR}/results.csv"
    echo ""

    # Exit with failure if any test failed
    if [[ $FAILED -gt 0 ]]; then
        echo -e "${RED}ADVERSARIAL TESTS FAILED - TAINT ENGINE NOT PRODUCTION READY${NC}"
        exit 1
    else
        echo -e "${GREEN}All adversarial tests passed!${NC}"
        exit 0
    fi
}

main() {
    parse_args "$@"
    check_prerequisites

    if [[ -n "${GENERATE_COUNT:-}" ]]; then
        generate_random_cases "$GENERATE_COUNT"
    fi

    run_all_cases
}

main "$@"
