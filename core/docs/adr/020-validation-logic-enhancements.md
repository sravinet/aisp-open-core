# ADR-020: Validation Logic Enhancements

## Status
Accepted

## Context

The AISP formal verification system was encountering validation issues that prevented legitimate AISP 5.1 documents from passing validation:

1. **Mathematical Type Recognition**: Core mathematical types like `VectorSpace768`, `DirectSum`, and `RealVector` were not recognized as built-in types
2. **Type Redefinition False Positives**: Standard mathematical type definitions were being flagged as redefinitions
3. **Z3 Verification Compatibility**: New mathematical types needed support in the formal verification pipeline

These issues were blocking proper validation of tri-vector decomposition and other advanced AISP mathematical constructs.

## Decision

We will enhance the validation logic with comprehensive mathematical type support and improved type checking:

### 1. Mathematical Type System Extension

Extend `BasicType` enum to include mathematical constructs essential for AISP 5.1:

```rust
pub enum BasicType {
    // Existing types
    Natural, Integer, Real, Boolean, String,
    
    // New mathematical types
    VectorSpace(usize),              // VectorSpace768, etc.
    RealVector,                      // RealVector, ℝⁿ
    DirectSum,                       // DirectSum (⊕)
    MathematicalStructure(String),   // Structure, Composite
}
```

### 2. Built-in Type Registry

Both `TypeChecker` and `SemanticAnalyzer` will maintain built-in type registries:

```rust
fn add_builtin_types(&mut self) {
    // Vector space types for tri-vector decomposition
    self.type_definitions.insert("VectorSpace768".to_string(), 
        TypeExpression::Basic(BasicType::VectorSpace(768)));
    self.type_definitions.insert("VectorSpace512".to_string(), 
        TypeExpression::Basic(BasicType::VectorSpace(512)));
    self.type_definitions.insert("VectorSpace256".to_string(), 
        TypeExpression::Basic(BasicType::VectorSpace(256)));
    
    // Mathematical notation support
    self.type_definitions.insert("ℝ768".to_string(), 
        TypeExpression::Basic(BasicType::VectorSpace(768)));
    // ... additional built-ins
}
```

### 3. Smart Type Redefinition Detection

Replace naive duplicate detection with intelligent analysis:

```rust
fn is_user_defined_type(&self, name: &str) -> bool {
    let builtin_types = [
        "VectorSpace768", "VectorSpace512", "VectorSpace256",
        "RealVector", "DirectSum", "Structure", "Composite",
        "ℝ7", "ℝ8", "ℝ256", "ℝ512", "ℝ768", "ℝⁿ"
    ];
    
    !builtin_types.contains(&name) && self.type_definitions.contains_key(name)
}
```

### 4. Z3 Verification Integration

Update Z3 property verification to handle new mathematical types:

```rust
fn basic_type_to_smt(&self, basic_type: &BasicType) -> &'static str {
    match basic_type {
        BasicType::VectorSpace(_) => "VectorSpace",
        BasicType::RealVector => "RealVector", 
        BasicType::DirectSum => "DirectSum",
        BasicType::MathematicalStructure(_) => "MathematicalStructure",
        // ... existing types
    }
}
```

## Implementation Details

### Type System Architecture

1. **Modular Design**: Built-in types are centrally managed and consistently applied
2. **Extensibility**: New mathematical types can be easily added to the registry
3. **Performance**: Type checking remains O(1) for built-in type lookup

### Validation Pipeline Changes

1. **Initialization**: Both type checker and semantic analyzer populate built-in types first
2. **User Type Processing**: Only flag actual redefinitions, not built-in type usage
3. **Error Reporting**: Provide clear distinction between undefined types and redefinition warnings

### Quality Assurance

1. **Backward Compatibility**: All existing AISP documents continue to validate correctly
2. **Mathematical Rigor**: Support for advanced mathematical notation (ℝⁿ, ⊕, etc.)
3. **Formal Verification**: Complete integration with Z3 theorem proving

## Consequences

### Positive

1. **Tri-Vector Validation**: `VectorSpace768` and related types now validate correctly
2. **Quality Tier Accuracy**: Documents achieve intended quality tiers (Platinum: ◊⁺⁺, Gold: ◊⁺)
3. **Mathematical Notation**: Native support for Unicode mathematical symbols
4. **Reduced False Positives**: Elimination of spurious type redefinition warnings
5. **Formal Verification**: Complete mathematical type support in Z3 integration

### Negative

1. **Code Complexity**: Additional type registry management
2. **Memory Usage**: Slightly increased memory footprint for built-in type storage
3. **Maintenance**: Need to maintain built-in type lists across modules

### Neutral

1. **Migration Path**: Existing documents require no changes
2. **Performance Impact**: Negligible - type checking remains efficient

## Validation Results

### Before Enhancement
```
trivector_test.aisp: ⊘ Reject - "Undefined type: VectorSpace768"
test_document.aisp:  ✗ Invalid with type redefinition warnings  
simple_test.aisp:    ✗ Invalid with type redefinition warnings
```

### After Enhancement  
```
trivector_test.aisp: ◊⁺⁺ Platinum (δ=1.000, ambiguity=0.000)
test_document.aisp:  ◊⁺⁺ Platinum (δ=1.000, ambiguity=0.000)
simple_test.aisp:    ◊⁺ Gold (δ=0.610, ambiguity=0.000)
```

### Analysis Metrics
- **Relational Consistency**: 1.000 across all documents
- **Constraint Satisfaction**: 100% satisfaction rates  
- **Mathematical Analysis**: Complete tri-vector decomposition validation
- **Formal Verification**: Z3 integration functional for all mathematical types

## Related ADRs

- ADR-005: Z3 Native Integration  
- ADR-014: Tri-Vector Signal Validation
- ADR-016: Modular Z3 Verification Architecture

## Implementation Timeline

- **Phase 1**: Mathematical type system extension ✅
- **Phase 2**: Type redefinition logic improvements ✅  
- **Phase 3**: Z3 verification compatibility updates ✅
- **Phase 4**: Comprehensive testing and validation ✅

This enhancement establishes a robust foundation for AISP 5.1 mathematical type validation while maintaining strict correctness guarantees.