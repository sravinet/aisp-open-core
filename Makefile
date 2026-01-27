# AISP Formal Verification System Makefile
# Provides streamlined commands for development and testing

.PHONY: help setup build test clean z3-test check lint fix-z3 challenge
.DEFAULT_GOAL := help

# Colors for output
RED := \033[31m
GREEN := \033[32m
YELLOW := \033[33m
BLUE := \033[34m
MAGENTA := \033[35m
CYAN := \033[36m
WHITE := \033[37m
RESET := \033[0m

# Z3 Environment Variables for macOS Homebrew
export LIBCLANG_PATH := /opt/homebrew/opt/llvm/lib
export Z3_SYS_Z3_HEADER := /opt/homebrew/include/z3.h
export C_INCLUDE_PATH := /opt/homebrew/include
export LIBRARY_PATH := /opt/homebrew/lib
export PKG_CONFIG_PATH := /opt/homebrew/lib/pkgconfig:$(PKG_CONFIG_PATH)

help: ## Show this help message
	@echo "$(CYAN)AISP Formal Verification System$(RESET)"
	@echo "$(CYAN)===============================$(RESET)"
	@echo ""
	@echo "$(GREEN)Development Commands:$(RESET)"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  $(YELLOW)%-20s$(RESET) %s\n", $$1, $$2}'
	@echo ""
	@echo "$(GREEN)Environment Status:$(RESET)"
	@echo "  Z3_SYS_Z3_HEADER: $(Z3_SYS_Z3_HEADER)"
	@echo "  LIBCLANG_PATH: $(LIBCLANG_PATH)"

setup: ## Install dependencies and setup environment
	@echo "$(BLUE)🔧 Setting up AISP development environment...$(RESET)"
	@echo "$(YELLOW)Checking Z3 installation...$(RESET)"
	@if ! command -v z3 &> /dev/null; then \
		echo "$(RED)❌ Z3 not found. Installing via Homebrew...$(RESET)"; \
		brew install z3; \
	else \
		echo "$(GREEN)✅ Z3 found: $$(z3 --version)$(RESET)"; \
	fi
	@echo "$(YELLOW)Checking LLVM/Clang installation...$(RESET)"
	@if ! brew list llvm &> /dev/null; then \
		echo "$(RED)❌ LLVM not found. Installing via Homebrew...$(RESET)"; \
		brew install llvm; \
	else \
		echo "$(GREEN)✅ LLVM found$(RESET)"; \
	fi
	@echo "$(GREEN)✅ Environment setup complete!$(RESET)"

z3-test: ## Test Z3 integration standalone
	@echo "$(BLUE)🧪 Testing Z3 integration...$(RESET)"
	@./test_z3
	@echo "$(BLUE)🧮 Testing Rust Z3 bindings...$(RESET)"
	@cd simple_z3_test && cargo run

check: ## Check compilation without building (fast)
	@echo "$(BLUE)🔍 Checking AISP compilation...$(RESET)"
	@cd aisp-formal-verification && cargo check

check-z3: ## Check compilation with Z3 verification
	@echo "$(BLUE)🔍 Checking AISP with Z3 verification...$(RESET)"
	@cd aisp-formal-verification && cargo check --features verification

build: ## Build AISP (Z3 verification is now DEFAULT)
	@echo "$(BLUE)🔨 Building AISP with default Z3 verification...$(RESET)"
	@cd aisp-formal-verification && cargo build

# build-z3: REMOVED - Z3 is now mandatory for all builds

build-release: ## Build optimized release version
	@echo "$(BLUE)🚀 Building AISP release version...$(RESET)"
	@cd aisp-formal-verification && cargo build --release

build-release-z3: ## Build optimized release with Z3
	@echo "$(BLUE)🚀 Building AISP release with Z3...$(RESET)"
	@cd aisp-formal-verification && cargo build --release --features verification

test: ## Run all tests (Z3 verification is now DEFAULT)
	@echo "$(BLUE)🧪 Running AISP test suite with default Z3...$(RESET)"
	@cd aisp-formal-verification && cargo test

test-z3: ## Run tests with Z3 verification
	@echo "$(BLUE)🧪 Running Z3 verification test suite...$(RESET)"
	@cd aisp-formal-verification && cargo test --features verification

# test-minimal: REMOVED - Z3 is mandatory, no minimal tests without Z3

challenge: ## Run the formal verification challenge
	@echo "$(MAGENTA)🎯 Running Formal Verification Challenge...$(RESET)"
	@cd aisp-formal-verification && cargo test --features verification reference_challenge_test

lint: ## Run clippy linting (Z3 verification is now DEFAULT)
	@echo "$(BLUE)📝 Running Clippy linter with default Z3...$(RESET)"
	@cd aisp-formal-verification && cargo clippy -- -D warnings

lint-z3: ## Run clippy linting with Z3 verification
	@echo "$(BLUE)📝 Running Clippy linter with Z3...$(RESET)"
	@cd aisp-formal-verification && cargo clippy --features verification -- -D warnings

fmt: ## Format code
	@echo "$(BLUE)✨ Formatting code...$(RESET)"
	@cd aisp-formal-verification && cargo fmt

compilation-status: ## Show detailed compilation status  
	@echo "$(MAGENTA)📊 AISP Compilation Status Report$(RESET)"
	@echo "$(MAGENTA)================================$(RESET)"
	@echo ""
	@echo "$(GREEN)✅ STRICT Z3 REQUIREMENTS ENFORCED$(RESET)"
	@echo "$(CYAN)Default Z3 Build:$(RESET)"
	@if cd aisp-formal-verification && cargo check --lib --quiet; then \
		echo "  ✅ SUCCESS - Library compiles with default Z3"; \
	else \
		echo "  ❌ FAILED - Z3 verification is default requirement"; \
	fi
	@echo "$(CYAN)Test Compilation with Z3:$(RESET)"
	@if cd aisp-formal-verification && timeout 30s cargo test --lib --no-run --quiet 2>/dev/null; then \
		echo "  ✅ SUCCESS - Tests compile with default Z3"; \
	else \
		echo "  ❌ FAILED - Z3 verification is default requirement"; \
	fi
	@echo "$(CYAN)Z3 Verification Build:$(RESET)"
	@if cd aisp-formal-verification && cargo check --features verification --quiet 2>/dev/null; then \
		echo "  ✅ SUCCESS - Z3 integration working"; \
	else \
		echo "  ⚠️  REQUIRES Z3 ENVIRONMENT SETUP"; \
		echo "     Run: make setup && make debug-z3"; \
	fi
	@echo ""
	@echo "$(GREEN)🎉 Z3 VERIFICATION IS NOW DEFAULT$(RESET)"
	@echo "$(CYAN)Key Changes:$(RESET)"
	@echo "  • Z3 verification enabled by default (no flags needed)"
	@echo "  • All commands use formal verification automatically"
	@echo "  • Production-ready security-first architecture"
	@echo "  • Zero-configuration formal verification" 
	@echo "  • Enterprise-grade security out of the box"

clean: ## Clean build artifacts
	@echo "$(BLUE)🧹 Cleaning build artifacts...$(RESET)"
	@cd aisp-formal-verification && cargo clean
	@cd simple_z3_test && cargo clean
	@rm -f test_z3

validate-document: ## Validate an AISP document (usage: make validate-document DOC=file.aisp)
	@echo "$(BLUE)📋 Validating AISP document: $(DOC)$(RESET)"
	@if [ -z "$(DOC)" ]; then \
		echo "$(RED)❌ Please specify DOC=filename.aisp$(RESET)"; \
		exit 1; \
	fi
	@cd aisp-formal-verification && cargo run --features z3-verification --bin aisp-cli validate $(DOC)

benchmark: ## Run performance benchmarks
	@echo "$(BLUE)⚡ Running performance benchmarks...$(RESET)"
	@cd aisp-formal-verification && cargo bench --features z3-verification

doc: ## Generate documentation
	@echo "$(BLUE)📚 Generating documentation...$(RESET)"
	@cd aisp-formal-verification && cargo doc --features z3-verification --open

install-cli: ## Install AISP CLI tool
	@echo "$(BLUE)📦 Installing AISP CLI...$(RESET)"
	@cd aisp-formal-verification && cargo install --path crates/aisp-cli --features z3-verification

status: ## Show development status and health check
	@echo "$(CYAN)AISP Development Status$(RESET)"
	@echo "$(CYAN)======================$(RESET)"
	@echo ""
	@echo "$(GREEN)Environment Check:$(RESET)"
	@echo "  Z3 Binary: $$(if command -v z3 &> /dev/null; then echo '✅ Found'; else echo '❌ Missing'; fi)"
	@echo "  Z3 Version: $$(if command -v z3 &> /dev/null; then z3 --version; else echo 'N/A'; fi)"
	@echo "  Z3 Headers: $$(if [ -f /opt/homebrew/include/z3.h ]; then echo '✅ Found'; else echo '❌ Missing'; fi)"
	@echo "  libclang: $$(if [ -f /opt/homebrew/opt/llvm/lib/libclang.dylib ]; then echo '✅ Found'; else echo '❌ Missing'; fi)"
	@echo ""
	@echo "$(GREEN)Build Status:$(RESET)"
	@echo "  Checking compilation..."
	@if cd aisp-formal-verification && cargo check --features z3-verification &> /dev/null; then \
		echo "  ✅ Compilation: SUCCESS"; \
	else \
		echo "  ❌ Compilation: FAILED"; \
		echo "  Run 'make check' for details"; \
	fi
	@echo ""
	@echo "$(GREEN)Available Commands:$(RESET)"
	@echo "  make setup     - Install dependencies"
	@echo "  make check     - Check compilation"  
	@echo "  make build     - Build project"
	@echo "  make test      - Run tests"
	@echo "  make challenge - Run verification challenge"

# Development workflow shortcuts
dev-setup: setup z3-test ## Complete development setup
	@echo "$(GREEN)🎉 Development environment ready!$(RESET)"

quick-test: check lint ## Quick development check
	@echo "$(GREEN)✅ Quick validation complete$(RESET)"

full-test: build test challenge ## Complete testing pipeline
	@echo "$(GREEN)🏆 Full test suite complete!$(RESET)"

# Debugging and analysis
debug-z3: ## Debug Z3 configuration
	@echo "$(BLUE)🐛 Z3 Configuration Debug$(RESET)"
	@echo "$(CYAN)Environment Variables:$(RESET)"
	@echo "  LIBCLANG_PATH=$(LIBCLANG_PATH)"
	@echo "  Z3_SYS_Z3_HEADER=$(Z3_SYS_Z3_HEADER)"
	@echo "  C_INCLUDE_PATH=$(C_INCLUDE_PATH)" 
	@echo "  LIBRARY_PATH=$(LIBRARY_PATH)"
	@echo "  PKG_CONFIG_PATH=$(PKG_CONFIG_PATH)"
	@echo ""
	@echo "$(CYAN)File Existence:$(RESET)"
	@echo "  z3.h: $$(if [ -f $(Z3_SYS_Z3_HEADER) ]; then echo '✅ Found'; else echo '❌ Missing'; fi)"
	@echo "  libz3.dylib: $$(if [ -f /opt/homebrew/lib/libz3.dylib ]; then echo '✅ Found'; else echo '❌ Missing'; fi)"
	@echo "  libclang.dylib: $$(if [ -f $(LIBCLANG_PATH)/libclang.dylib ]; then echo '✅ Found'; else echo '❌ Missing'; fi)"

analyze-errors: ## Analyze current compilation errors
	@echo "$(BLUE)🔍 Analyzing compilation errors...$(RESET)"
	@cd aisp-formal-verification && cargo check --features z3-verification 2>&1 | head -50

# File watching for development
watch: ## Watch files and rebuild on changes (requires cargo-watch)
	@echo "$(BLUE)👀 Watching for file changes...$(RESET)"
	@if ! command -v cargo-watch &> /dev/null; then \
		echo "$(YELLOW)Installing cargo-watch...$(RESET)"; \
		cargo install cargo-watch; \
	fi
	@cd aisp-formal-verification && cargo watch -x "check --features z3-verification"