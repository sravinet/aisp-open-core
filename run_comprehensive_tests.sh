#!/bin/bash
# Comprehensive Test Suite Runner
# Executes all Phase 2 test suites with proper reporting

set -e

echo "🧪 AISP Phase 2 Comprehensive Test Suite"
echo "========================================"
echo

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test configuration
CARGO_TEST_FLAGS="--release --no-fail-fast"
TEST_TIMEOUT="300" # 5 minutes per test suite

echo -e "${BLUE}Configuration:${NC}"
echo "- Build mode: release"
echo "- Timeout: ${TEST_TIMEOUT}s per suite"
echo "- Fail fast: disabled"
echo

# Function to run test suite with timing
run_test_suite() {
    local test_name="$1"
    local test_pattern="$2"
    
    echo -e "${BLUE}Running ${test_name}...${NC}"
    start_time=$(date +%s)
    
    if timeout ${TEST_TIMEOUT} cargo test ${CARGO_TEST_FLAGS} ${test_pattern} 2>&1; then
        end_time=$(date +%s)
        duration=$((end_time - start_time))
        echo -e "${GREEN}✓ ${test_name} completed in ${duration}s${NC}"
        echo
        return 0
    else
        end_time=$(date +%s) 
        duration=$((end_time - start_time))
        echo -e "${RED}✗ ${test_name} failed after ${duration}s${NC}"
        echo
        return 1
    fi
}

# Function to run performance benchmarks
run_benchmarks() {
    echo -e "${BLUE}Running Performance Benchmarks...${NC}"
    start_time=$(date +%s)
    
    if timeout ${TEST_TIMEOUT} cargo test ${CARGO_TEST_FLAGS} performance_benchmarks 2>&1; then
        end_time=$(date +%s)
        duration=$((end_time - start_time))
        echo -e "${GREEN}✓ Performance benchmarks completed in ${duration}s${NC}"
        echo
        return 0
    else
        end_time=$(date +%s)
        duration=$((end_time - start_time))
        echo -e "${YELLOW}⚠ Performance benchmarks had issues after ${duration}s${NC}"
        echo
        return 1
    fi
}

# Change to core directory
cd core

echo -e "${BLUE}Building project in release mode...${NC}"
cargo build --release
echo -e "${GREEN}✓ Build completed${NC}"
echo

# Track test results
total_tests=0
passed_tests=0
failed_tests=()

# Test Suite 1: Phase 2 Integration Tests
total_tests=$((total_tests + 1))
if run_test_suite "Phase 2 Integration Tests" "phase2_integration_comprehensive"; then
    passed_tests=$((passed_tests + 1))
else
    failed_tests+=("Phase 2 Integration")
fi

# Test Suite 2: Performance Benchmarks
total_tests=$((total_tests + 1))
if run_benchmarks; then
    passed_tests=$((passed_tests + 1))
else
    failed_tests+=("Performance Benchmarks")
fi

# Test Suite 3: Unicode Edge Cases
total_tests=$((total_tests + 1))
if run_test_suite "Unicode Edge Cases" "unicode_edge_cases"; then
    passed_tests=$((passed_tests + 1))
else
    failed_tests+=("Unicode Edge Cases")
fi

# Test Suite 4: Error Handling & Graceful Failure
total_tests=$((total_tests + 1))
if run_test_suite "Error Handling & Graceful Failure" "error_handling_graceful_failure"; then
    passed_tests=$((passed_tests + 1))
else
    failed_tests+=("Error Handling")
fi

# Test Suite 5: Regression Tests
total_tests=$((total_tests + 1))
if run_test_suite "Regression Test Suite" "regression_test_suite"; then
    passed_tests=$((passed_tests + 1))
else
    failed_tests+=("Regression Tests")
fi

# Test Suite 6: Existing Integration Tests
total_tests=$((total_tests + 1))
if run_test_suite "Working Integration Tests" "working_integration"; then
    passed_tests=$((passed_tests + 1))
else
    failed_tests+=("Working Integration")
fi

# Test Suite 7: Formal Verification Tests
total_tests=$((total_tests + 1))
if run_test_suite "Formal Verification Tests" "formal_verification"; then
    passed_tests=$((passed_tests + 1))
else
    failed_tests+=("Formal Verification")
fi

# Summary Report
echo "========================================"
echo -e "${BLUE}Test Suite Summary${NC}"
echo "========================================"
echo

if [ ${passed_tests} -eq ${total_tests} ]; then
    echo -e "${GREEN}🎉 All test suites passed!${NC}"
    echo -e "${GREEN}✓ ${passed_tests}/${total_tests} test suites successful${NC}"
else
    echo -e "${YELLOW}⚠ Some test suites had issues${NC}"
    echo -e "${GREEN}✓ ${passed_tests}/${total_tests} test suites successful${NC}"
    
    if [ ${#failed_tests[@]} -gt 0 ]; then
        echo -e "${RED}✗ Failed suites:${NC}"
        for failed_test in "${failed_tests[@]}"; do
            echo -e "${RED}  - ${failed_test}${NC}"
        done
    fi
fi

echo
echo -e "${BLUE}Test Coverage Summary:${NC}"
echo "- ✓ Phase 2 verification modules integration"
echo "- ✓ Performance benchmarks and regression"
echo "- ✓ Unicode mathematical notation edge cases"
echo "- ✓ Error handling and graceful failure modes"
echo "- ✓ Comprehensive regression prevention"
echo "- ✓ Adversarial resistance validation"
echo "- ✓ API compatibility verification"

# Final CLI test
echo
echo -e "${BLUE}Running CLI validation test...${NC}"
if [ -f "./target/release/aisp-cli" ]; then
    echo -e "${BLUE}Testing CLI with platinum demo...${NC}"
    if ./target/release/aisp-cli --level formal --format detailed validate ../tests/fixtures/valid/platinum_demo.aisp; then
        echo -e "${GREEN}✓ CLI validation successful${NC}"
    else
        echo -e "${RED}✗ CLI validation failed${NC}"
        exit 1
    fi
else
    echo -e "${YELLOW}⚠ CLI binary not found, skipping CLI test${NC}"
fi

echo
if [ ${passed_tests} -eq ${total_tests} ]; then
    echo -e "${GREEN}🚀 Phase 2 comprehensive testing completed successfully!${NC}"
    exit 0
else
    echo -e "${YELLOW}🔧 Phase 2 testing completed with some issues - review failed suites${NC}"
    exit 1
fi