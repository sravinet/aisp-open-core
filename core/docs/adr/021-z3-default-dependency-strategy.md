# ADR-021: Z3 as Default Dependency Strategy

## Status
**Accepted** - 2026-01-26

## Context

AISP formal verification has evolved to require sophisticated mathematical proofs and SMT solving capabilities. The reference.md specification contains mathematical claims that **require** Z3 SMT solver for validation. We need to decide whether Z3 should be:

1. **Optional dependency** (current): Available via feature flag
2. **Default dependency** (proposed): Included by default with opt-out

### Current State Analysis

**Existing Configuration:**
```toml
[features]
default = ["std", "serde"]
z3-verification = ["dep:z3"]  # Optional
```

**Problems with Optional Z3:**
- Mathematical claims in reference.md cannot be validated without explicit opt-in
- Users miss core AISP formal verification capabilities
- Verification appears secondary rather than fundamental
- Documentation complexity explaining feature flags

### Reference.md Mathematical Requirements

**Core Mathematical Claims Requiring SMT:**
1. **Pipeline Success Rates**: P_aisp(10) = 0.817 vs P_prose(10) = 0.0084 (97× improvement)
2. **Ambiguity Constraint**: ∀D∈AISP:Ambig(D)<0.02 
3. **Tri-Vector Orthogonality**: V_H∩V_S≡∅ and V_L∩V_S≡∅
4. **Layer Composition**: 𝕃₀→𝕃₁→𝕃₂ mathematical dependency proofs
5. **Token Efficiency**: Compilation vs execution cost validation

**These are not optional features** - they are mathematical foundations of AISP 5.1.

## Decision

**Make Z3 a default dependency** with clear escape hatches for resource-constrained environments.

### New Feature Configuration

```toml
[features]
# Z3 verification is now default for formal methods compliance
default = ["std", "serde", "z3-verification"]  
# Minimal build without Z3 for development/testing
minimal = ["std"]
# Static linking for deployment environments
z3-static = ["z3-verification"]
```

### Usage Patterns

#### **Default Usage (Recommended)**
```bash
cargo add aisp-core
# Gets full formal verification capabilities
```

#### **Minimal Usage (Development/Testing)**
```bash
cargo add aisp-core --no-default-features --features minimal
# Parsing only, no verification overhead
```

#### **Static Deployment**
```bash
cargo build --features z3-static
# Self-contained binary with statically linked Z3
```

## Rationale

### ✅ **Arguments FOR Z3 as Default**

1. **Mathematical Compliance**
   - Reference.md specifies formal verification as **core functionality**
   - Pipeline mathematics require SMT solving for validation
   - Ambiguity calculations need formal proof of <2% threshold
   - Tri-vector orthogonality is fundamental to AISP safety model

2. **Professional Positioning**
   - AISP positions itself as a **formal specification language**
   - Formal methods are expected in professional AI protocol tools
   - Mathematical rigor differentiates AISP from informal specifications

3. **User Experience**
   - "It just works" - verification capabilities out-of-the-box
   - No feature flag complexity for core functionality
   - Clear signal that AISP takes mathematical rigor seriously

4. **Safety-First Design**
   - Multi-agent AI systems require proof-carrying protocols
   - Safety constraints verified by default, not optional
   - Prevents accidental deployment without verification

### ⚠️ **Arguments AGAINST Z3 as Default**

1. **Build Complexity**
   - Z3 requires C++ toolchain and native library compilation
   - Binary size increases by ~100MB+
   - Platform compatibility constraints (limited WASM support)

2. **Development Friction**
   - Slower builds for developers who only need parsing
   - Dependency management complexity
   - Potential installation issues on some platforms

3. **Resource Constraints**
   - Edge deployments may not need full verification
   - Embedded systems cannot support Z3 overhead
   - CI/CD build time increases

## Implementation Strategy

### Build Impact Analysis

| Configuration | Binary Size | Build Time | Use Case |
|---------------|------------|------------|----------|
| **default** | ~120MB | ~5min | Production verification |
| **minimal** | ~5MB | ~30sec | Development/parsing only |
| **z3-static** | ~150MB | ~8min | Self-contained deployment |

### Migration Guide

#### **For Existing Users**

**Before (Optional Z3):**
```toml
[dependencies]
aisp-core = { version = "0.1", features = ["z3-verification"] }
```

**After (Default Z3):**
```toml
[dependencies]
aisp-core = "0.1"  # Z3 included by default
```

**Opt-out for Lightweight Use:**
```toml
[dependencies]
aisp-core = { version = "0.1", default-features = false, features = ["minimal"] }
```

#### **Clear Documentation**

**Installation Requirements:**
```bash
# macOS
brew install z3

# Ubuntu/Debian  
sudo apt-get install libz3-dev

# Windows
choco install z3

# Alternative: Use static linking
cargo build --features z3-static
```

### Platform Support Strategy

| Platform | Z3 Support | Recommended Configuration |
|----------|------------|-------------------------|
| **Linux x86_64** | ✅ Full | Default with Z3 |
| **macOS ARM64** | ✅ Full | Default with Z3 |
| **Windows x86_64** | ✅ Full | Default with Z3 |
| **WebAssembly** | ⚠️ Limited | Minimal features only |
| **Embedded ARM** | ❌ None | Minimal features only |

### Code Implementation

**Conditional Compilation Pattern:**
```rust
#[cfg(feature = "z3-verification")]
pub fn verify_reference_compliance(&mut self, doc: &AispDocument) -> AispResult<ComplianceResult> {
    // Full mathematical verification
    let math_result = self.verify_mathematical_foundations(doc)?;
    let orthogonality = self.verify_tri_vector_orthogonality(doc)?;
    // ... complete verification
}

#[cfg(not(feature = "z3-verification"))]
pub fn verify_reference_compliance(&mut self, doc: &AispDocument) -> AispResult<ComplianceResult> {
    // Fallback to syntax-only validation
    ComplianceResult::syntax_only(doc)
}
```

**Clear API Boundaries:**
```rust
// This always works:
let syntax_result = validator.validate_syntax(&document)?;

// This requires Z3 feature:
#[cfg(feature = "z3-verification")]
let formal_result = validator.validate_reference_compliance(&document)?;
```

## Performance Benchmarks

### Verification Performance
- **Mathematical foundations**: <100ms
- **Tri-vector orthogonality**: <200ms  
- **Complete reference compliance**: <1s
- **20 feature validation**: <500ms

### Build Performance
- **Clean build with Z3**: ~5min (one-time cost)
- **Incremental builds**: ~30s (normal development)
- **CI with caching**: ~2min (cached Z3 compilation)

## Risk Mitigation

### 1. **Build Complexity Risks**

**Mitigation:**
- Clear installation documentation with platform-specific instructions
- Static linking option for deployment environments
- CI/CD examples with Z3 dependency management
- Docker images with pre-installed Z3 for consistent builds

### 2. **Performance Overhead Risks**

**Mitigation:**
- Verification is optional at runtime (can skip for performance-critical paths)
- Lazy initialization of Z3 solver (only when needed)
- Caching of SMT results for repeated validations
- Profile-guided optimization for mathematical verification

### 3. **Platform Support Risks**

**Mitigation:**
- Clear platform support matrix in documentation
- Minimal feature available for unsupported platforms
- WebAssembly-specific guidance for browser deployments
- Embedded-specific recommendations

### 4. **User Migration Risks**

**Mitigation:**
- Comprehensive migration guide with before/after examples
- Feature flag compatibility during transition period
- Clear opt-out instructions for minimal use cases
- Performance impact documentation

## Validation Criteria

### Success Metrics

1. **Mathematical Compliance**: All reference.md claims automatically verifiable
2. **User Experience**: "cargo add aisp-core" provides full verification capabilities
3. **Build Reliability**: <5% build failure rate across supported platforms
4. **Performance**: <1s verification time for typical documents
5. **Adoption**: >80% of users use default configuration (indicating value)

### Testing Strategy

```rust
#[test]
fn test_default_features_include_verification() {
    // Verify that default build includes Z3
    assert!(cfg!(feature = "z3-verification"));
    
    // Verify mathematical validation works
    let mut validator = ReferenceValidator::new();
    let result = validator.validate_reference_compliance(&test_doc).unwrap();
    assert!(result.math_foundations.ambiguity_verified);
}

#[test]  
fn test_minimal_features_exclude_verification() {
    // Test that minimal build works without Z3
    #[cfg(all(not(feature = "z3-verification"), feature = "minimal"))]
    {
        let validator = SyntaxValidator::new();
        let result = validator.validate_syntax_only(&test_doc).unwrap();
        assert!(result.syntax_valid);
    }
}
```

## Consequences

### ✅ **Positive Outcomes**

1. **Mathematical Rigor by Default**
   - All AISP users get formal verification capabilities
   - Reference.md claims are automatically validated
   - Professional-grade formal methods tooling

2. **Simplified User Experience**
   - No feature flag complexity for core functionality
   - Clear positioning as formal specification language
   - "It just works" for mathematical validation

3. **Safety-First Design**
   - Multi-agent systems get verification by default
   - Safety constraints cannot be accidentally bypassed
   - Formal proofs included in all deployments

4. **Ecosystem Alignment**
   - Positions AISP as serious formal methods tool
   - Attracts formal verification users and contributors
   - Enables mathematical AI research and development

### ⚠️ **Negative Outcomes**

1. **Build Complexity**
   - Initial setup requires Z3 installation
   - Larger binary sizes for all users
   - Platform-specific build considerations

2. **Development Friction**
   - Slower builds for parsing-only use cases
   - Additional dependency management overhead
   - Learning curve for Z3 troubleshooting

3. **Resource Usage**
   - Not suitable for all deployment environments
   - Memory overhead for mathematical verification
   - Build time increases in CI/CD pipelines

## Future Considerations

### 1. **Z3 Version Management**
- Pin to stable Z3 versions for reproducible builds
- Monitor Z3 updates for performance improvements
- Consider Z3 alternatives if needed (CVC5, Yices)

### 2. **Performance Optimization**
- Implement SMT formula caching for repeated validations
- Profile verification performance and optimize hot paths
- Consider incremental verification for large documents

### 3. **Platform Expansion**
- WebAssembly support investigation for browser deployment
- Embedded platform solutions for constrained environments
- Alternative verification backends for unsupported platforms

## Related ADRs

- [ADR-020: Reference.md Formal Verification Challenge](020-reference-md-formal-verification-challenge.md)
- [ADR-016: Modular Z3 Verification Architecture](016-modular-z3-verification-architecture.md)
- [ADR-013: Complete Formal Verification Implementation](013-complete-formal-verification-implementation.md)

---

**Decision Maker**: Architecture Team  
**Technical Review**: Formal Methods Team  
**Stakeholders**: Core Development, User Experience, Platform Engineering