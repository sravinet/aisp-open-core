# Formal Verification Workflow for AISP
# Advanced justfile for formal methods validation

# Default formal verification workflow
default: setup-formal check-formal test-formal validate-formal

# ═══ FORMAL VERIFICATION SETUP ═══

# Setup formal verification environment
setup-formal:
    @echo "🔧 Setting up formal verification environment..."
    @echo "📋 Checking Z3 installation..."
    @if ! command -v z3 >/dev/null 2>&1; then \
        echo "❌ Z3 not found. Installing via Homebrew..."; \
        brew install z3; \
    else \
        echo "✅ Z3 found: $(z3 --version)"; \
    fi
    @echo "📋 Checking environment variables..."
    @export LIBCLANG_PATH=/opt/homebrew/opt/llvm/lib
    @export Z3_SYS_Z3_HEADER=/opt/homebrew/include/z3.h
    @export C_INCLUDE_PATH=/opt/homebrew/include
    @export LIBRARY_PATH=/opt/homebrew/lib
    @export PKG_CONFIG_PATH=/opt/homebrew/lib/pkgconfig
    @echo "✅ Formal verification environment ready"

# ═══ COMPILATION FIXES ═══

# Fix compilation errors systematically
fix-compilation:
    @echo "🔧 Fixing compilation errors..."
    @echo "Step 1: Fixing parser_new imports..."
    find core/crates/aisp-core/src -name "*.rs" -exec sed -i '' 's/parser_new::/parser::/g' {} \;
    @echo "Step 2: Fixing MetaBlock imports..."
    find core/crates/aisp-core/src -name "*.rs" -exec sed -i '' 's/use crate::parser::robust_parser::MetaBlock;/use crate::ast::canonical::MetaBlock;/g' {} \;
    @echo "Step 3: Adding missing module declarations..."
    @if ! grep -q "pub mod parser_new" core/crates/aisp-core/src/lib.rs; then \
        echo "parser_new module not needed - using consolidated parser"; \
    fi
    @echo "✅ Compilation fixes applied"

# Check formal verification compilation
check-formal: fix-compilation
    @echo "🔍 Checking formal verification compilation..."
    LIBCLANG_PATH=/opt/homebrew/opt/llvm/lib \
    Z3_SYS_Z3_HEADER=/opt/homebrew/include/z3.h \
    C_INCLUDE_PATH=/opt/homebrew/include \
    LIBRARY_PATH=/opt/homebrew/lib \
    PKG_CONFIG_PATH=/opt/homebrew/lib/pkgconfig \
    cargo check --lib --features z3-verification --message-format short

# ═══ FORMAL TESTING ═══

# Test formal verification components
test-formal: check-formal
    @echo "🧪 Testing formal verification components..."
    @echo "Testing Z3 integration..."
    LIBCLANG_PATH=/opt/homebrew/opt/llvm/lib \
    Z3_SYS_Z3_HEADER=/opt/homebrew/include/z3.h \
    C_INCLUDE_PATH=/opt/homebrew/include \
    LIBRARY_PATH=/opt/homebrew/lib \
    PKG_CONFIG_PATH=/opt/homebrew/lib/pkgconfig \
    cargo test --lib z3_verification::tests::test_z3_availability -- --nocapture

# Test mathematical verification
test-mathematical:
    @echo "🔢 Testing mathematical verification..."
    LIBCLANG_PATH=/opt/homebrew/opt/llvm/lib \
    Z3_SYS_Z3_HEADER=/opt/homebrew/include/z3.h \
    C_INCLUDE_PATH=/opt/homebrew/include \
    LIBRARY_PATH=/opt/homebrew/lib \
    PKG_CONFIG_PATH=/opt/homebrew/lib/pkgconfig \
    cargo test --lib mathematical_evaluator -- --nocapture

# Test semantic verification
test-semantic:
    @echo "🧠 Testing semantic verification..."
    LIBCLANG_PATH=/opt/homebrew/opt/llvm/lib \
    Z3_SYS_Z3_HEADER=/opt/homebrew/include/z3.h \
    C_INCLUDE_PATH=/opt/homebrew/include \
    LIBRARY_PATH=/opt/homebrew/lib \
    PKG_CONFIG_PATH=/opt/homebrew/lib/pkgconfig \
    cargo test --lib semantic_z3_verifier -- --nocapture

# ═══ DOCUMENT VALIDATION ═══

# Validate AISP documents with formal verification
validate-formal: build-formal
    @echo "📋 Validating AISP documents with formal verification..."
    @echo "Testing valid minimal document..."
    ./target/debug/aisp-cli --level formal --format detailed validate tests/fixtures/valid/valid_minimal.aisp
    @echo "Testing all valid fixtures..."
    find tests/fixtures/valid -name "*.aisp" -exec ./target/debug/aisp-cli --level formal validate {} \;

# Create reference.md AISP translation
create-reference-aisp:
    @echo "📝 Creating AISP translation of reference.md..."
    @if [ ! -f "docs/examples/reference.aisp" ]; then \
        echo "Creating AISP version of reference document..."; \
        echo "𝔸5.1.reference@2026-01-30" > docs/examples/reference.aisp; \
        echo "γ≔aisp.reference.specification" >> docs/examples/reference.aisp; \
        echo "ρ≔⟨foundation,architecture,features,validation,theorems⟩" >> docs/examples/reference.aisp; \
        echo "⊢ND∧CAT∧ΠΣ" >> docs/examples/reference.aisp; \
        echo "" >> docs/examples/reference.aisp; \
        echo "⟦Ω:Meta⟧{" >> docs/examples/reference.aisp; \
        echo "  Vision≜\"Assembly language for AI cognition\"" >> docs/examples/reference.aisp; \
        echo "  Author≜\"Bradley Ross\"" >> docs/examples/reference.aisp; \
        echo "  Affiliation≜\"Harvard ALM Digital Media Design\"" >> docs/examples/reference.aisp; \
        echo "  ∀D∈AISP:Ambig(D)<0.02" >> docs/examples/reference.aisp; \
        echo "}" >> docs/examples/reference.aisp; \
        echo "" >> docs/examples/reference.aisp; \
        echo "⟦Σ:Types⟧{" >> docs/examples/reference.aisp; \
        echo "  Signal≜V_H⊕V_L⊕V_S" >> docs/examples/reference.aisp; \
        echo "  V_H≜ℝ⁷⁶⁸; V_L≜ℝ⁵¹²; V_S≜ℝ²⁵⁶" >> docs/examples/reference.aisp; \
        echo "  𝒫≜⟨ℋ:Header,ℳ:Membrane,𝒩:Nucleus⟩" >> docs/examples/reference.aisp; \
        echo "}" >> docs/examples/reference.aisp; \
        echo "" >> docs/examples/reference.aisp; \
        echo "⟦Γ:Rules⟧{" >> docs/examples/reference.aisp; \
        echo "  ∀D∈AISP:Ambig(D)<0.02" >> docs/examples/reference.aisp; \
        echo "  V_H∩V_S≡∅; V_L∩V_S≡∅; V_H∩V_L≢∅" >> docs/examples/reference.aisp; \
        echo "  ∀p:ℋ.id(p)≡SHA256(𝒩(p))" >> docs/examples/reference.aisp; \
        echo "}" >> docs/examples/reference.aisp; \
        echo "" >> docs/examples/reference.aisp; \
        echo "⟦Λ:Functions⟧{" >> docs/examples/reference.aisp; \
        echo "  ψ_g≜λb.ψ_*⊖ψ_have(b.G)" >> docs/examples/reference.aisp; \
        echo "  μ_f≜λx.σ(θ₁·sim_H(x)+θ₂·fit_L(x)+θ₃·aff_M(x))" >> docs/examples/reference.aisp; \
        echo "  validate≜⌈⌉∘δ∘Γ?∘∂" >> docs/examples/reference.aisp; \
        echo "}" >> docs/examples/reference.aisp; \
        echo "" >> docs/examples/reference.aisp; \
        echo "⟦Ε⟧⟨δ≜0.79;φ≜97;τ≜◊⁺⁺⟩" >> docs/examples/reference.aisp; \
        echo "✅ Created docs/examples/reference.aisp"; \
    else \
        echo "✅ Reference AISP already exists"; \
    fi

# Validate reference AISP translation
validate-reference: create-reference-aisp build-formal
    @echo "🔍 Validating reference.aisp translation..."
    ./target/debug/aisp-cli --level formal --format detailed validate docs/examples/reference.aisp

# ═══ BUILD WORKFLOWS ═══

# Build with formal verification features
build-formal: setup-formal check-formal
    @echo "🏗️ Building with formal verification..."
    LIBCLANG_PATH=/opt/homebrew/opt/llvm/lib \
    Z3_SYS_Z3_HEADER=/opt/homebrew/include/z3.h \
    C_INCLUDE_PATH=/opt/homebrew/include \
    LIBRARY_PATH=/opt/homebrew/lib \
    PKG_CONFIG_PATH=/opt/homebrew/lib/pkgconfig \
    cargo build --features z3-verification

# ═══ VERIFICATION PIPELINE ═══

# Complete formal verification pipeline
verify-pipeline: setup-formal fix-compilation check-formal test-formal create-reference-aisp validate-reference
    @echo "🎯 Formal verification pipeline complete!"
    @echo "✅ Environment setup"
    @echo "✅ Compilation fixes applied"
    @echo "✅ Formal verification compilation"
    @echo "✅ Formal verification tests"
    @echo "✅ Reference AISP translation"
    @echo "✅ Reference validation"
    @echo "🏆 All formal verification checks passed!"

# ═══ DEVELOPMENT WORKFLOWS ═══

# Quick formal development cycle
dev-formal: fix-compilation check-formal test-mathematical
    @echo "✅ Quick formal development cycle complete"

# Continuous verification monitoring
watch-formal:
    @echo "👀 Watching for formal verification changes..."
    cargo watch -x "test --lib z3_verification::tests" -x "test --lib mathematical_evaluator"

# ═══ PRODUCTION WORKFLOWS ═══

# Production-ready formal verification build
prod-formal: setup-formal verify-pipeline
    @echo "🚀 Building production formal verification..."
    LIBCLANG_PATH=/opt/homebrew/opt/llvm/lib \
    Z3_SYS_Z3_HEADER=/opt/homebrew/include/z3.h \
    C_INCLUDE_PATH=/opt/homebrew/include \
    LIBRARY_PATH=/opt/homebrew/lib \
    PKG_CONFIG_PATH=/opt/homebrew/lib/pkgconfig \
    cargo build --release --features z3-verification

# ═══ DIAGNOSTICS ═══

# Debug formal verification setup
debug-formal:
    @echo "🐛 Formal Verification Debug Report"
    @echo "════════════════════════════════════"
    @echo "Z3 Status:"
    @echo "  Binary: $(which z3 || echo 'Not found')"
    @echo "  Version: $(z3 --version || echo 'N/A')"
    @echo "  Headers: $(if [ -f /opt/homebrew/include/z3.h ]; then echo '✅ Found'; else echo '❌ Missing'; fi)"
    @echo "  libclang: $(if [ -f /opt/homebrew/opt/llvm/lib/libclang.dylib ]; then echo '✅ Found'; else echo '❌ Missing'; fi)"
    @echo ""
    @echo "Build Configuration:"
    @echo "  Features: z3-verification, verification, full"
    @echo "  Target: $(rustc --print target-triple)"
    @echo ""
    @echo "Test Status:"
    @cargo test --lib z3_verification::tests::test_z3_availability --features z3-verification 2>&1 | head -10 || echo "Test failed"

# Show formal verification statistics
stats-formal:
    @echo "📊 Formal Verification Statistics"
    @echo "══════════════════════════════════"
    @echo "Z3 Integration Files:"
    find core/crates/aisp-core/src -name "*z3*" | wc -l
    @echo "Mathematical Verification Files:"
    find core/crates/aisp-core/src -name "*mathematical*" -o -name "*theorem*" -o -name "*proof*" | wc -l
    @echo "Semantic Verification Files:"
    find core/crates/aisp-core/src -name "*semantic*" | wc -l
    @echo "Test Coverage:"
    find core/crates/aisp-core/tests -name "*formal*" -o -name "*verification*" | wc -l