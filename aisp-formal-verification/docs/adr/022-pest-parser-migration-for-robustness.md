# ADR-022: Pest Parser Migration for Robust AISP Document Processing

**Date**: 2026-01-27  
**Status**: Proposed  
**Priority**: P0 - Critical Security Fix  
**Deciders**: AISP Core Team, Security Review Board  

## Context

### Current Parser Limitations Discovered

During formal methods security assessment, critical vulnerabilities were identified in the current hand-written AISP parser:

```rust
Error: ParseError { line: 13, column: 3, message: "Expected '⟦' to start block" }
```

**Security Issues Identified:**
1. **Complete Verification Bypass**: Parse errors prevent any verification from occurring
2. **Brittle Unicode Handling**: Manual parsing of complex Unicode mathematical symbols
3. **No Error Recovery**: Single syntax error invalidates entire document
4. **Attack Vector**: Adversaries can craft malformed documents to bypass security checks

### Formal Methods Challenge Results

Testing with sophisticated attack vectors revealed:
- **100% bypass rate** through parse errors
- **No partial verification** capability  
- **Fragile symbol handling** for mathematical notation
- **Poor diagnostic quality** for debugging failures

## Decision

**Migrate from hand-written parser to Pest-based parsing framework** for the following critical improvements:

### 1. **Robust Grammar Definition**
```pest
// AISP Grammar Definition (aisp.pest)
aisp_document = { SOI ~ header ~ document_metadata? ~ aisp_blocks ~ EOI }

header = { "𝔸" ~ version ~ "." ~ identifier ~ "@" ~ date }
version = { ASCII_DIGIT+ ~ "." ~ ASCII_DIGIT+ }
identifier = { (ASCII_ALPHANUMERIC | "-" | "_" | ".")+ }
date = { ASCII_DIGIT{4} ~ "-" ~ ASCII_DIGIT{2} ~ "-" ~ ASCII_DIGIT{2} }

// Unicode Mathematical Blocks
aisp_blocks = { aisp_block+ }
aisp_block = { omega_block | sigma_block | gamma_block | lambda_block | chi_block | epsilon_block }

omega_block = { "⟦" ~ "Ω" ~ ":" ~ "Meta" ~ "⟧" ~ "{" ~ meta_entries ~ "}" }
sigma_block = { "⟦" ~ "Σ" ~ ":" ~ "Types" ~ "⟧" ~ "{" ~ type_definitions ~ "}" }
gamma_block = { "⟦" ~ "Γ" ~ ":" ~ "Rules" ~ "⟧" ~ "{" ~ rule_definitions ~ "}" }
lambda_block = { "⟦" ~ "Λ" ~ ":" ~ ("Funcs" | "Functions") ~ "⟧" ~ "{" ~ function_definitions ~ "}" }
chi_block = { "⟦" ~ "Χ" ~ ":" ~ "Errors" ~ "⟧" ~ "{" ~ error_definitions ~ "}" }
epsilon_block = { "⟦" ~ "Ε" ~ (":" ~ "Evidence")? ~ "⟧" ~ "⟨" ~ evidence_entries ~ "⟩" }

// Mathematical Expressions with Error Recovery
mathematical_expr = { term ~ (binary_op ~ term)* }
term = { unicode_symbol | identifier | number | "(" ~ mathematical_expr ~ ")" }
unicode_symbol = { 
    "∀" | "∃" | "∈" | "∉" | "∩" | "∪" | "⊆" | "⊇" | "≡" | "≠" | "≤" | "≥" | 
    "→" | "←" | "↔" | "⊕" | "⊗" | "λ" | "δ" | "φ" | "ε" | "π" | "ρ" | "τ" |
    "ℕ" | "ℝ" | "ℂ" | "ℚ" | "ℤ" | "𝕊" | "𝔹" | "◊" | "⊤" | "⊥" | "⊘"
}

// Error Recovery Rules
WHITESPACE = _{ " " | "\t" | "\n" | "\r" }
COMMENT = _{ "//" ~ (!"\n" ~ ANY)* }

// Graceful degradation for malformed blocks
malformed_block = { "⟦" ~ (!("⟧" | EOI) ~ ANY)* ~ ("⟧" | &EOI) }
```

### 2. **Error Recovery and Partial Parsing**
```rust
// Enhanced Parser with Recovery (src/parser/pest_parser.rs)
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar/aisp.pest"]
pub struct AispParser;

pub struct RobustAispParser {
    strict_mode: bool,
    recovery_enabled: bool,
}

impl RobustAispParser {
    pub fn parse_with_recovery(&self, input: &str) -> ParseResult {
        match AispParser::parse(Rule::aisp_document, input) {
            Ok(pairs) => self.build_ast_from_pairs(pairs),
            Err(pest_error) => {
                if self.recovery_enabled {
                    self.parse_with_error_recovery(input, pest_error)
                } else {
                    Err(ParseError::from(pest_error))
                }
            }
        }
    }
    
    fn parse_with_error_recovery(&self, input: &str, original_error: pest::error::Error<Rule>) -> ParseResult {
        let mut document = AispDocument::default();
        let mut errors = vec![ParseError::from(original_error)];
        
        // Try parsing individual blocks
        for block_match in self.extract_block_boundaries(input) {
            match self.parse_single_block(&block_match.content) {
                Ok(block) => {
                    document.blocks.push(block);
                    log::info!("Recovered block: {}", block_match.block_type);
                },
                Err(e) => {
                    errors.push(e);
                    log::warn!("Failed to parse block {}: {}", block_match.block_type, e);
                }
            }
        }
        
        ParseResult {
            document,
            errors,
            recovery_applied: true,
            partial_success: !document.blocks.is_empty(),
        }
    }
}
```

### 3. **Comprehensive Unicode Support**
```rust
// Unicode Mathematical Symbol Registry (src/symbols/unicode_math.rs)
pub struct UnicodeSymbolRegistry {
    symbol_map: HashMap<&'static str, MathematicalSymbol>,
}

impl UnicodeSymbolRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            symbol_map: HashMap::new(),
        };
        
        // Register all AISP mathematical symbols with variants
        registry.register_variants(&[
            ("∀", "forall", SymbolType::Quantifier),
            ("∃", "exists", SymbolType::Quantifier),
            ("∈", "in", SymbolType::SetRelation),
            ("⟦", "[[", SymbolType::BlockDelimiter),
            ("⟧", "]]", SymbolType::BlockDelimiter),
            ("⟨", "<", SymbolType::EvidenceDelimiter),
            ("⟩", ">", SymbolType::EvidenceDelimiter),
            ("≜", "def=", SymbolType::Definition),
            ("◊⁺⁺", "tier-max", SymbolType::QualityTier),
        ]);
        
        registry
    }
}
```

## Consequences

### **Positive Consequences**

#### 1. **Security Hardening**
- ✅ **Parse errors no longer bypass verification**
- ✅ **Partial verification possible on malformed documents**  
- ✅ **Robust handling of adversarial input**
- ✅ **Better error diagnostics for security analysis**

#### 2. **Development Velocity**
- ✅ **Grammar-driven development** - easier to extend AISP syntax
- ✅ **Automatic parser generation** - reduces manual parsing code
- ✅ **Better testing capabilities** with grammar-based property testing
- ✅ **IDE support** for grammar files

#### 3. **Maintainability**
- ✅ **Declarative grammar** easier to understand than procedural parser
- ✅ **Centralized syntax definition** in .pest files
- ✅ **Version-controlled grammar** for AISP language evolution

#### 4. **Performance (Expected)**
```rust
// Benchmark Results (Projected)
Parse Time Comparison:
- Hand-written parser: ~250μs (success) / immediate failure (error)
- Pest parser: ~180μs (success) / ~120μs (partial recovery)

Memory Usage:
- Hand-written: Higher due to manual state management
- Pest: Lower due to optimized parsing engine

Error Recovery:
- Hand-written: 0% (immediate failure)
- Pest: 75-90% partial success rate on malformed input
```

### **Negative Consequences**

#### 1. **Migration Effort**
- ⚠️ **Significant refactoring** required for existing parser code
- ⚠️ **Learning curve** for team members unfamiliar with Pest
- ⚠️ **Testing migration** - comprehensive test suite updates needed

#### 2. **Dependency Addition**
- ⚠️ **External dependency** on Pest crate
- ⚠️ **Compile-time grammar processing** 
- ⚠️ **Potential future maintenance** if Pest is abandoned

#### 3. **Grammar Complexity**
- ⚠️ **Mathematical notation complexity** may stress Pest's capabilities
- ⚠️ **Unicode handling** requires careful attention
- ⚠️ **Error message quality** may need customization

## Implementation Plan

### **Phase 1: Grammar Definition (Week 1)**
```toml
# Add to Cargo.toml
[dependencies]
pest = "2.7"
pest_derive = "2.7"

[build-dependencies]
pest_generator = "2.7"
```

### **Phase 2: Parser Migration (Week 2-3)**
1. Create comprehensive AISP grammar file
2. Implement RobustAispParser with error recovery
3. Add Unicode symbol registry
4. Create parser benchmarks

### **Phase 3: Integration & Testing (Week 4)**
1. Integrate new parser with verification engine
2. Update all existing tests
3. Add adversarial input test suite
4. Performance validation

### **Phase 4: Deployment (Week 5)**
1. Feature flag rollout
2. Monitoring and metrics
3. Documentation updates
4. Team training

## Success Metrics

### **Security Metrics**
- **Parse Success Rate**: >95% on well-formed documents
- **Partial Recovery Rate**: >85% on malformed documents  
- **Adversarial Bypass Rate**: <5% (down from 100%)

### **Performance Metrics**
- **Parse Time**: <200μs for typical documents
- **Memory Usage**: <500KB for complex documents
- **Error Recovery Time**: <100μs additional overhead

### **Quality Metrics**
- **Test Coverage**: >90% for grammar rules
- **Documentation Coverage**: 100% for public APIs
- **Error Message Quality**: Actionable diagnostics in >90% of cases

## Alternatives Considered

### **Alternative 1: Improve Hand-Written Parser**
- ❌ **High effort** for incremental improvement
- ❌ **Still vulnerable** to sophisticated attacks
- ❌ **Limited error recovery** capabilities

### **Alternative 2: Nom Parser Combinators**
- ⚠️ **More flexible** but higher complexity
- ⚠️ **Steeper learning curve** than Pest
- ⚠️ **No built-in grammar definition** format

### **Alternative 3: Custom Recursive Descent**
- ❌ **Reinventing the wheel** 
- ❌ **Higher maintenance burden**
- ❌ **No community ecosystem**

## Decision Rationale

**Pest provides the optimal balance of robustness, maintainability, and security** for AISP document parsing:

1. **Grammar-First Approach**: Declarative syntax definition matches AISP's formal mathematical nature
2. **Battle-Tested**: Proven in production Rust applications  
3. **Error Recovery**: Built-in capabilities for partial parsing
4. **Unicode Support**: Excellent handling of mathematical notation
5. **Performance**: Optimized parsing engine with minimal overhead
6. **Ecosystem**: Active maintenance and community support

This migration addresses the **critical security vulnerability** where parse errors completely bypass verification, while setting up the foundation for future AISP language evolution.

**Recommendation**: **Approve** for immediate implementation as P0 security fix.