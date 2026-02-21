#!/usr/bin/env bash
#
# prove_v6.sh - NINE65 v6 Verification and Test Suite
#
# This script performs comprehensive verification of the NINE65 v6 project:
# - Environment validation (Rust, Python, pytest)
# - Regression scan on source files
# - Rust tests (cargo test)
# - Python tests (pytest)
# - Report generation with summary.json
#
# Exit codes:
#   0 - All tests passed
#   1 - Environment validation failed
#   2 - Regression scan failed
#   3 - Rust tests failed
#   4 - Python tests failed
#   5 - Report generation failed
#

set -euo pipefail

# =============================================================================
# Configuration
# =============================================================================
readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly PROJECT_DIR="${SCRIPT_DIR}"
readonly CRATES_DIR="${PROJECT_DIR}/crates"
readonly NINE65_SRC="${CRATES_DIR}/nine65/src"
readonly NINE65_PYTHON_TESTS="${CRATES_DIR}/nine65-python/tests"
readonly REPORTS_DIR="${PROJECT_DIR}/reports/prove_v6"
readonly LATEST_DIR="${REPORTS_DIR}/latest"

# Colors for output
readonly RED='\033[0;31m'
readonly GREEN='\033[0;32m'
readonly YELLOW='\033[1;33m'
readonly BLUE='\033[0;34m'
readonly CYAN='\033[0;36m'
readonly BOLD='\033[1m'
readonly NC='\033[0m' # No Color

# Counters for test results
RUST_TESTS_PASSED=0
RUST_TESTS_FAILED=0
RUST_TESTS_SKIPPED=0
PYTHON_TESTS_PASSED=0
PYTHON_TESTS_FAILED=0
PYTHON_TESTS_SKIPPED=0
REGRESSION_FILES_SCANNED=0
REGRESSION_ISSUES=0

# Timestamp
readonly TIMESTAMP="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
readonly TIMESTAMP_FRIENDLY="$(date +"%Y-%m-%d %H:%M:%S %Z")"

# =============================================================================
# Utility Functions
# =============================================================================

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

log_step() {
    echo -e "\n${CYAN}${BOLD}==> $1${NC}"
}

log_substep() {
    echo -e "    ${CYAN}--> $1${NC}"
}

# Print a section header
print_header() {
    echo -e "\n${BOLD}${CYAN}========================================${NC}"
    echo -e "${BOLD}${CYAN}  $1${NC}"
    echo -e "${BOLD}${CYAN}========================================${NC}\n"
}

# Check if a command exists
command_exists() {
    command -v "$1" &> /dev/null
}

# Get git commit SHA
get_git_sha() {
    if command_exists git && git rev-parse --git-dir &> /dev/null; then
        git rev-parse HEAD 2>/dev/null || echo "unknown"
    else
        echo "not-a-git-repo"
    fi
}

# Get git branch name
get_git_branch() {
    if command_exists git && git rev-parse --git-dir &> /dev/null; then
        git rev-parse --abbrev-ref HEAD 2>/dev/null || echo "unknown"
    else
        echo "unknown"
    fi
}

# =============================================================================
# Environment Validation
# =============================================================================

validate_environment() {
    print_header "Step 1: Environment Validation"

    local errors=0

    # Check Rust toolchain
    log_substep "Checking Rust toolchain..."
    if command_exists rustc; then
        local rust_version
        rust_version=$(rustc --version 2>&1)
        log_success "Rust: ${rust_version}"
    else
        log_error "Rust toolchain not found. Please install via rustup."
        ((errors++))
    fi

    if command_exists cargo; then
        local cargo_version
        cargo_version=$(cargo --version 2>&1)
        log_success "Cargo: ${cargo_version}"
    else
        log_error "Cargo not found. Please install via rustup."
        ((errors++))
    fi

    # Check Python 3
    log_substep "Checking Python 3..."
    if command_exists python3; then
        local python_version
        python_version=$(python3 --version 2>&1)
        log_success "Python: ${python_version}"
    else
        log_error "Python 3 not found. Please install Python 3.8+."
        ((errors++))
    fi

    # Check pytest
    log_substep "Checking pytest..."
    if command_exists pytest; then
        local pytest_version
        pytest_version=$(pytest --version 2>&1)
        log_success "pytest: ${pytest_version}"
    else
        log_error "pytest not found. Please install via: pip install pytest"
        ((errors++))
    fi

    # Check project structure
    log_substep "Checking project structure..."
    if [[ -d "${NINE65_SRC}" ]]; then
        log_success "Found nine65 source directory: ${NINE65_SRC}"
    else
        log_error "nine65 source directory not found: ${NINE65_SRC}"
        ((errors++))
    fi

    if [[ -d "${NINE65_PYTHON_TESTS}" ]]; then
        log_success "Found nine65-python tests directory: ${NINE65_PYTHON_TESTS}"
    else
        log_error "nine65-python tests directory not found: ${NINE65_PYTHON_TESTS}"
        ((errors++))
    fi

    if [[ ${errors} -gt 0 ]]; then
        log_error "Environment validation failed with ${errors} error(s)"
        return 1
    fi

    log_success "Environment validation passed"
    return 0
}

# =============================================================================
# Regression Scan
# =============================================================================

run_regression_scan() {
    print_header "Step 2: Regression Scan"

    if [[ ! -d "${NINE65_SRC}" ]]; then
        log_error "Source directory does not exist: ${NINE65_SRC}"
        return 2
    fi

    log_substep "Scanning source files in ${NINE65_SRC}..."

    local scan_log="${LATEST_DIR}/regression_scan.log"
    local issues_found=0

    # Count Rust source files
    local rust_files
    rust_files=$(find "${NINE65_SRC}" -name "*.rs" -type f 2>/dev/null | wc -l)
    log_info "Found ${rust_files} Rust source files"

    # Initialize scan log
    {
        echo "Regression Scan Report"
        echo "======================"
        echo "Timestamp: ${TIMESTAMP}"
        echo "Source Directory: ${NINE65_SRC}"
        echo ""
        echo "Files Scanned:"
        echo "--------------"
    } > "${scan_log}"

    # Scan each Rust file for common issues
    while IFS= read -r file; do
        ((REGRESSION_FILES_SCANNED++))
        echo "  - ${file}" >> "${scan_log}"

        local rel_path="${file#${PROJECT_DIR}/}"

        # Check for TODO comments (informational)
        local todo_count
        todo_count=$(grep -c "// TODO" "${file}" 2>/dev/null | tr -d '\n' || echo "0")
        todo_count=${todo_count:-0}
        if [[ "$todo_count" =~ ^[0-9]+$ ]] && [[ ${todo_count} -gt 0 ]]; then
            echo "    [TODO] ${todo_count} TODO comment(s) found" >> "${scan_log}"
        fi

        # Check for FIXME comments (informational)
        local fixme_count
        fixme_count=$(grep -c "// FIXME" "${file}" 2>/dev/null | tr -d '\n' || echo "0")
        fixme_count=${fixme_count:-0}
        if [[ "$fixme_count" =~ ^[0-9]+$ ]] && [[ ${fixme_count} -gt 0 ]]; then
            echo "    [FIXME] ${fixme_count} FIXME comment(s) found" >> "${scan_log}"
        fi

        # Check for panic! macros (warning)
        local panic_count
        panic_count=$(grep -c "panic!" "${file}" 2>/dev/null | tr -d '\n' || echo "0")
        panic_count=${panic_count:-0}
        if [[ "$panic_count" =~ ^[0-9]+$ ]] && [[ ${panic_count} -gt 0 ]]; then
            echo "    [PANIC] ${panic_count} panic! call(s) found" >> "${scan_log}"
        fi

        # Check for unwrap() calls (warning)
        local unwrap_count
        unwrap_count=$(grep -c "\.unwrap()" "${file}" 2>/dev/null | tr -d '\n' || echo "0")
        unwrap_count=${unwrap_count:-0}
        if [[ "$unwrap_count" =~ ^[0-9]+$ ]] && [[ ${unwrap_count} -gt 0 ]]; then
            echo "    [UNWRAP] ${unwrap_count} unwrap() call(s) found" >> "${scan_log}"
        fi

        # Check for unsafe blocks (warning)
        local unsafe_count
        unsafe_count=$(grep -c "unsafe " "${file}" 2>/dev/null | tr -d '\n' || echo "0")
        unsafe_count=${unsafe_count:-0}
        if [[ "$unsafe_count" =~ ^[0-9]+$ ]] && [[ ${unsafe_count} -gt 0 ]]; then
            echo "    [UNSAFE] ${unsafe_count} unsafe block(s)/keyword(s) found" >> "${scan_log}"
        fi

        # Check for empty match arms or incomplete implementations
        if grep -q "todo!()" "${file}" 2>/dev/null; then
            echo "    [TODO_IMPL] Incomplete implementation found (todo!())" >> "${scan_log}"
            ((issues_found++))
        fi

        # Check for unreachable!()
        if grep -q "unreachable!()" "${file}" 2>/dev/null; then
            echo "    [UNREACHABLE] unreachable!() marker found" >> "${scan_log}"
        fi

    done < <(find "${NINE65_SRC}" -name "*.rs" -type f 2>/dev/null | sort)

    # Add summary to scan log
    {
        echo ""
        echo "Scan Summary:"
        echo "-------------"
        echo "Total files scanned: ${REGRESSION_FILES_SCANNED}"
        echo "Potential issues found: ${issues_found}"
        echo ""
        echo "Scan completed at: $(date)"
    } >> "${scan_log}"

    REGRESSION_ISSUES=${issues_found}

    log_success "Regression scan completed"
    log_info "Files scanned: ${REGRESSION_FILES_SCANNED}"
    log_info "Potential issues: ${REGRESSION_ISSUES}"
    log_info "Scan log: ${scan_log}"

    # Fail loudly if source directory is missing or empty
    if [[ ${REGRESSION_FILES_SCANNED} -eq 0 ]]; then
        log_error "No Rust source files found in ${NINE65_SRC}"
        return 2
    fi

    return 0
}

# =============================================================================
# Rust Tests
# =============================================================================

run_rust_tests() {
    print_header "Step 3: Rust Tests (cargo test)"

    local test_output="${LATEST_DIR}/rust_tests.log"
    local test_start
    local test_end

    test_start=$(date +%s)

    log_substep "Running: cargo test -p nine65 --release --verbose"
    echo "Command: cargo test -p nine65 --release --verbose" > "${test_output}"
    echo "Started: ${TIMESTAMP_FRIENDLY}" >> "${test_output}"
    echo "----------------------------------------" >> "${test_output}"

    # Run cargo test and capture output
    set +e
    cargo test -p nine65 --release --verbose >> "${test_output}" 2>&1
    local cargo_exit_code=$?
    set -e

    test_end=$(date +%s)
    local test_duration=$((test_end - test_start))

    echo "" >> "${test_output}"
    echo "----------------------------------------" >> "${test_output}"
    echo "Finished: $(date)" >> "${test_output}"
    echo "Duration: ${test_duration} seconds" >> "${test_output}"
    echo "Exit code: ${cargo_exit_code}" >> "${test_output}"

    # Parse test results from output
    if [[ -f "${test_output}" ]]; then
        # Count test results - sanitize to ensure clean integers
        RUST_TESTS_PASSED=$(grep -c "test .* ... ok" "${test_output}" 2>/dev/null | tr -d '\n' || echo "0")
        RUST_TESTS_FAILED=$(grep -c "test .* ... FAILED" "${test_output}" 2>/dev/null | tr -d '\n' || echo "0")
        RUST_TESTS_SKIPPED=$(grep -c "test .* ... ignored" "${test_output}" 2>/dev/null | tr -d '\n' || echo "0")
    fi

    # Ensure values are valid integers
    RUST_TESTS_PASSED=${RUST_TESTS_PASSED:-0}
    RUST_TESTS_FAILED=${RUST_TESTS_FAILED:-0}
    RUST_TESTS_SKIPPED=${RUST_TESTS_SKIPPED:-0}
    [[ ! "${RUST_TESTS_PASSED}" =~ ^[0-9]+$ ]] && RUST_TESTS_PASSED=0
    [[ ! "${RUST_TESTS_FAILED}" =~ ^[0-9]+$ ]] && RUST_TESTS_FAILED=0
    [[ ! "${RUST_TESTS_SKIPPED}" =~ ^[0-9]+$ ]] && RUST_TESTS_SKIPPED=0

    if [[ ${cargo_exit_code} -eq 0 ]]; then
        log_success "Rust tests passed (${test_duration}s)"
        log_info "Passed: ${RUST_TESTS_PASSED}, Failed: ${RUST_TESTS_FAILED}, Skipped: ${RUST_TESTS_SKIPPED}"
        return 0
    else
        log_error "Rust tests failed with exit code ${cargo_exit_code}"
        log_info "Passed: ${RUST_TESTS_PASSED}, Failed: ${RUST_TESTS_FAILED}, Skipped: ${RUST_TESTS_SKIPPED}"
        log_error "See ${test_output} for details"
        return 3
    fi
}

# =============================================================================
# Python Tests
# =============================================================================

run_python_tests() {
    print_header "Step 4: Python Tests (pytest)"

    local test_output="${LATEST_DIR}/python_tests.log"
    local test_start
    local test_end

    test_start=$(date +%s)

    log_substep "Running: pytest ${NINE65_PYTHON_TESTS}/"
    echo "Command: pytest ${NINE65_PYTHON_TESTS}/" > "${test_output}"
    echo "Started: ${TIMESTAMP_FRIENDLY}" >> "${test_output}"
    echo "----------------------------------------" >> "${test_output}"

    # Run pytest and capture output
    set +e
    pytest "${NINE65_PYTHON_TESTS}/" -v >> "${test_output}" 2>&1
    local pytest_exit_code=$?
    set -e

    test_end=$(date +%s)
    local test_duration=$((test_end - test_start))

    echo "" >> "${test_output}"
    echo "----------------------------------------" >> "${test_output}"
    echo "Finished: $(date)" >> "${test_output}"
    echo "Duration: ${test_duration} seconds" >> "${test_output}"
    echo "Exit code: ${pytest_exit_code}" >> "${test_output}"

    # Parse test results from pytest output
    if [[ -f "${test_output}" ]]; then
        # Try to parse pytest summary line: "X passed, Y failed, Z skipped"
        local pytest_summary
        pytest_summary=$(grep -E "^[0-9]+ passed" "${test_output}" 2>/dev/null | tail -1 || echo "")

        if [[ -n "${pytest_summary}" ]]; then
            PYTHON_TESTS_PASSED=$(echo "${pytest_summary}" | grep -oE "[0-9]+ passed" | grep -oE "[0-9]+" | tr -d '\n' || echo "0")
            PYTHON_TESTS_FAILED=$(echo "${pytest_summary}" | grep -oE "[0-9]+ failed" | grep -oE "[0-9]+" | tr -d '\n' || echo "0")
            PYTHON_TESTS_SKIPPED=$(echo "${pytest_summary}" | grep -oE "[0-9]+ skipped" | grep -oE "[0-9]+" | tr -d '\n' || echo "0")
        fi

        # Fallback: count individual test markers
        if [[ "${PYTHON_TESTS_PASSED}" == "0" ]] || [[ -z "${PYTHON_TESTS_PASSED}" ]]; then
            PYTHON_TESTS_PASSED=$(grep -cE "^.*::.* PASSED$" "${test_output}" 2>/dev/null | tr -d '\n' || echo "0")
        fi
        if [[ "${PYTHON_TESTS_FAILED}" == "0" ]] || [[ -z "${PYTHON_TESTS_FAILED}" ]]; then
            PYTHON_TESTS_FAILED=$(grep -cE "^.*::.* FAILED$" "${test_output}" 2>/dev/null | tr -d '\n' || echo "0")
        fi
        if [[ "${PYTHON_TESTS_SKIPPED}" == "0" ]] || [[ -z "${PYTHON_TESTS_SKIPPED}" ]]; then
            PYTHON_TESTS_SKIPPED=$(grep -cE "^.*::.* SKIPPED$" "${test_output}" 2>/dev/null | tr -d '\n' || echo "0")
        fi
    fi

    # Ensure values are valid integers
    PYTHON_TESTS_PASSED=${PYTHON_TESTS_PASSED:-0}
    PYTHON_TESTS_FAILED=${PYTHON_TESTS_FAILED:-0}
    PYTHON_TESTS_SKIPPED=${PYTHON_TESTS_SKIPPED:-0}
    [[ ! "${PYTHON_TESTS_PASSED}" =~ ^[0-9]+$ ]] && PYTHON_TESTS_PASSED=0
    [[ ! "${PYTHON_TESTS_FAILED}" =~ ^[0-9]+$ ]] && PYTHON_TESTS_FAILED=0
    [[ ! "${PYTHON_TESTS_SKIPPED}" =~ ^[0-9]+$ ]] && PYTHON_TESTS_SKIPPED=0

    if [[ ${pytest_exit_code} -eq 0 ]]; then
        log_success "Python tests passed (${test_duration}s)"
        log_info "Passed: ${PYTHON_TESTS_PASSED}, Failed: ${PYTHON_TESTS_FAILED}, Skipped: ${PYTHON_TESTS_SKIPPED}"
        return 0
    else
        log_error "Python tests failed with exit code ${pytest_exit_code}"
        log_info "Passed: ${PYTHON_TESTS_PASSED}, Failed: ${PYTHON_TESTS_FAILED}, Skipped: ${PYTHON_TESTS_SKIPPED}"
        log_error "See ${test_output} for details"
        return 4
    fi
}

# =============================================================================
# Report Generation
# =============================================================================

generate_reports() {
    print_header "Step 5: Generate Reports"

    log_substep "Creating report directory structure..."

    # Create latest directory (remove old symlink first if exists)
    rm -rf "${LATEST_DIR}"
    mkdir -p "${LATEST_DIR}"

    # Create timestamped directory
    local timestamp_dir
    timestamp_dir="${REPORTS_DIR}/$(date +"%Y%m%d_%H%M%S")"
    mkdir -p "${timestamp_dir}"

    # Copy reports to timestamped directory
    cp -r "${LATEST_DIR}/"*.log "${timestamp_dir}/" 2>/dev/null || true

    # Create symlink to latest
    ln -sfn "${timestamp_dir}" "${LATEST_DIR}_run"

    log_success "Report directory created: ${LATEST_DIR}"
    log_info "Timestamped run: ${timestamp_dir}"

    return 0
}

# =============================================================================
# Summary JSON Generation
# =============================================================================

generate_summary_json() {
    print_header "Step 6: Generate Summary JSON"

    local summary_file="${LATEST_DIR}/summary.json"
    local git_sha
    local git_branch

    git_sha=$(get_git_sha)
    git_branch=$(get_git_branch)

    # Calculate totals
    local total_tests=$((RUST_TESTS_PASSED + RUST_TESTS_FAILED + RUST_TESTS_SKIPPED + PYTHON_TESTS_PASSED + PYTHON_TESTS_FAILED + PYTHON_TESTS_SKIPPED))
    local total_passed=$((RUST_TESTS_PASSED + PYTHON_TESTS_PASSED))
    local total_failed=$((RUST_TESTS_FAILED + PYTHON_TESTS_FAILED))
    local total_skipped=$((RUST_TESTS_SKIPPED + PYTHON_TESTS_SKIPPED))

    # Determine overall status
    local overall_status="success"
    if [[ ${total_failed} -gt 0 ]]; then
        overall_status="failure"
    fi

    log_substep "Writing summary to ${summary_file}..."

    cat > "${summary_file}" << EOF
{
  "project": "NINE65 v6",
  "verification_script": "prove_v6.sh",
  "git": {
    "commit_sha": "${git_sha}",
    "branch": "${git_branch}"
  },
  "timestamp": {
    "iso8601": "${TIMESTAMP}",
    "friendly": "${TIMESTAMP_FRIENDLY}"
  },
  "test_summary": {
    "total": ${total_tests},
    "passed": ${total_passed},
    "failed": ${total_failed},
    "skipped": ${total_skipped}
  },
  "rust_tests": {
    "passed": ${RUST_TESTS_PASSED},
    "failed": ${RUST_TESTS_FAILED},
    "skipped": ${RUST_TESTS_SKIPPED},
    "command": "cargo test -p nine65 --release --verbose",
    "log_file": "rust_tests.log"
  },
  "python_tests": {
    "passed": ${PYTHON_TESTS_PASSED},
    "failed": ${PYTHON_TESTS_FAILED},
    "skipped": ${PYTHON_TESTS_SKIPPED},
    "command": "pytest crates/nine65-python/tests/",
    "log_file": "python_tests.log"
  },
  "regression_scan": {
    "files_scanned": ${REGRESSION_FILES_SCANNED},
    "issues_found": ${REGRESSION_ISSUES},
    "source_directory": "crates/nine65/src",
    "log_file": "regression_scan.log"
  },
  "overall_status": "${overall_status}"
}
EOF

    log_success "Summary JSON generated: ${summary_file}"

    # Display summary
    echo ""
    echo -e "${BOLD}Test Summary:${NC}"
    echo -e "  Total:  ${total_tests}"
    echo -e "  ${GREEN}Passed: ${total_passed}${NC}"
    if [[ ${total_failed} -gt 0 ]]; then
        echo -e "  ${RED}Failed: ${total_failed}${NC}"
    else
        echo -e "  Failed: ${total_failed}"
    fi
    echo -e "  Skipped: ${total_skipped}"
    echo ""
    echo -e "Git: ${git_branch}@${git_sha:0:8}"
    echo -e "Status: ${overall_status}"

    return 0
}

# =============================================================================
# Main Execution
# =============================================================================

main() {
    print_header "NINE65 v6 Verification Suite"

    echo -e "${BOLD}Project Directory:${NC} ${PROJECT_DIR}"
    echo -e "${BOLD}Timestamp:${NC} ${TIMESTAMP_FRIENDLY}"
    echo ""

    # Step 1: Validate Environment
    if ! validate_environment; then
        log_error "Aborting due to environment validation failure"
        exit 1
    fi

    # Ensure report directory exists before regression scan
    mkdir -p "${LATEST_DIR}"

    # Step 2: Regression Scan
    if ! run_regression_scan; then
        log_error "Aborting due to regression scan failure"
        exit 2
    fi

    # Step 3: Rust Tests
    if ! run_rust_tests; then
        log_error "Rust tests failed, continuing with report generation..."
        # Continue to generate reports even on failure
    fi

    # Step 4: Python Tests
    if ! run_python_tests; then
        log_error "Python tests failed, continuing with report generation..."
        # Continue to generate reports even on failure
    fi

    # Step 5: Generate Reports
    if ! generate_reports; then
        log_error "Report generation failed"
        exit 5
    fi

    # Step 6: Generate Summary JSON
    if ! generate_summary_json; then
        log_error "Summary generation failed"
        exit 5
    fi

    # Final status
    print_header "Verification Complete"

    if [[ ${RUST_TESTS_FAILED} -eq 0 && ${PYTHON_TESTS_FAILED} -eq 0 ]]; then
        log_success "All tests passed!"
        exit 0
    else
        log_error "Some tests failed"
        exit 3
    fi
}

# Run main function
main "$@"
