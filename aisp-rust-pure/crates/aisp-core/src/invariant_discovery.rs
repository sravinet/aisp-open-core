//! Automated Invariant Discovery
//!
//! This module provides automated discovery of mathematical invariants in AISP documents.
//! It analyzes type systems, logical relationships, and numerical constraints to identify
//! likely mathematical properties that hold throughout the document.

use crate::{
    ast::{AispDocument, AispBlock, TypeExpression},
    error::AispResult,
    property_types::{PropertyFormula, FormulaStructure, AtomicFormula, Term, 
                     Quantifier as PropQuantifier, SourceLocation},
};
use std::collections::{HashMap, HashSet};
use std::time::Duration;

/// Discovered invariant with confidence scoring
#[derive(Debug, Clone)]
pub struct DiscoveredInvariant {
    /// Unique identifier for the invariant
    pub id: String,
    /// Human-readable name of the invariant
    pub name: String,
    /// Mathematical formula representation
    pub formula: PropertyFormula,
    /// Invariant classification
    pub invariant_type: InvariantType,
    /// Confidence score [0.0, 1.0]
    pub confidence: f64,
    /// Evidence supporting this invariant
    pub evidence: Vec<InvariantEvidence>,
    /// Source locations where this invariant was discovered
    pub sources: Vec<SourceLocation>,
    /// Whether this invariant has been verified
    pub verified: bool,
}

/// Classification of invariant types
#[derive(Debug, Clone, PartialEq)]
pub enum InvariantType {
    /// Type system invariants (e.g., ∀x:ℕ → x ≥ 0)
    TypeStructural,
    /// Type membership constraints
    TypeMembership,
    /// Functional invariants (e.g., f(x) = f(y) → x = y)
    FunctionalProperty,
    /// Function monotonicity properties
    FunctionalMonotonicity,
    /// Relational invariants (e.g., R(x,y) ∧ R(y,z) → R(x,z))
    RelationalInvariant,
    /// Numerical constraints (e.g., x + y = z → x ≤ z)
    NumericalInvariant,
    /// Logical consistency (e.g., P ∧ ¬P → ⊥)
    LogicalInvariant,
    /// Temporal properties (e.g., □(P → ◇Q))
    TemporalInvariant,
    /// Structural invariants (e.g., |S| ≥ 0)
    StructuralInvariant,
}

/// Evidence supporting an invariant
#[derive(Debug, Clone)]
pub struct InvariantEvidence {
    /// Type of evidence
    pub evidence_type: EvidenceType,
    /// Strength of evidence [0.0, 1.0]
    pub strength: f64,
    /// Human-readable description
    pub description: String,
    /// Source location of evidence
    pub location: SourceLocation,
}

/// Types of evidence that can support invariants
#[derive(Debug, Clone, PartialEq)]
pub enum EvidenceType {
    /// Evidence from type system enforcement
    TypeSystemEnforcement,
    /// Evidence from pattern matching
    PatternMatching,
    /// Evidence from mathematical analysis
    MathematicalAnalysis,
    /// Evidence from logical consistency
    LogicalConsistency,
    /// Evidence from empirical observation
    EmpiricalObservation,
    /// Evidence from formal verification
    FormalVerification,
}

/// Configuration for invariant discovery
#[derive(Debug, Clone)]
pub struct InvariantDiscoveryConfig {
    /// Maximum number of invariants to discover
    pub max_invariants: usize,
    /// Minimum confidence threshold for reporting invariants
    pub confidence_threshold: f64,
    /// Enable pattern-based discovery
    pub enable_patterns: bool,
    /// Enable numerical analysis
    pub enable_numerical_analysis: bool,
    /// Enable logical analysis
    pub enable_logical_analysis: bool,
    /// Enable structural analysis
    pub enable_structural_analysis: bool,
    /// Enable Z3 verification
    pub enable_z3_verification: bool,
    /// Verification timeout in milliseconds
    pub verification_timeout: u64,
}

impl Default for InvariantDiscoveryConfig {
    fn default() -> Self {
        Self {
            max_invariants: 50,
            confidence_threshold: 0.5,
            enable_patterns: true,
            enable_numerical_analysis: true,
            enable_logical_analysis: true,
            enable_structural_analysis: true,
            enable_z3_verification: false,
            verification_timeout: 5000,
        }
    }
}

/// Statistics about the discovery process
#[derive(Debug, Clone)]
pub struct DiscoveryStats {
    /// Total analysis time
    pub total_time: Duration,
    /// Number of type invariants discovered
    pub type_invariants: usize,
    /// Number of functional invariants discovered
    pub functional_invariants: usize,
    /// Number of verified invariants
    pub verified_correct: usize,
    /// Number of disproven invariants
    pub disproven: usize,
    /// Time spent on verification
    pub verification_time: Duration,
}

impl Default for DiscoveryStats {
    fn default() -> Self {
        Self {
            total_time: Duration::new(0, 0),
            type_invariants: 0,
            functional_invariants: 0,
            verified_correct: 0,
            disproven: 0,
            verification_time: Duration::new(0, 0),
        }
    }
}

/// Main invariant discovery engine
pub struct InvariantDiscovery {
    config: InvariantDiscoveryConfig,
    discovered_invariants: Vec<DiscoveredInvariant>,
    discovery_stats: DiscoveryStats,
}

impl InvariantDiscovery {
    /// Create a new invariant discovery engine
    pub fn new(config: InvariantDiscoveryConfig) -> Self {
        Self {
            config,
            discovered_invariants: Vec::new(),
            discovery_stats: DiscoveryStats::default(),
        }
    }

    /// Discover invariants in an AISP document
    pub fn discover_invariants(&mut self, document: &AispDocument) -> AispResult<Vec<DiscoveredInvariant>> {
        let start_time = std::time::Instant::now();
        
        // Clear previous results
        self.discovered_invariants.clear();
        self.discovery_stats = DiscoveryStats::default();
        
        // Analyze different aspects of the document
        self.discover_type_invariants(document)?;
        self.discover_function_invariants(document)?;
        
        // Filter by confidence threshold
        let mut result: Vec<DiscoveredInvariant> = self.discovered_invariants.iter()
            .filter(|inv| inv.confidence >= self.config.confidence_threshold)
            .cloned()
            .collect();
        
        // Limit results
        if result.len() > self.config.max_invariants {
            result.truncate(self.config.max_invariants);
        }
        
        // Update statistics
        self.discovery_stats.total_time = start_time.elapsed();
        self.discovery_stats.type_invariants = result.iter()
            .filter(|inv| matches!(inv.invariant_type, InvariantType::TypeStructural | InvariantType::TypeMembership))
            .count();
        self.discovery_stats.functional_invariants = result.iter()
            .filter(|inv| matches!(inv.invariant_type, InvariantType::FunctionalProperty | InvariantType::FunctionalMonotonicity))
            .count();
        
        Ok(result)
    }
    
    /// Discover type-related invariants
    fn discover_type_invariants(&mut self, document: &AispDocument) -> AispResult<()> {
        for block in &document.blocks {
            if let AispBlock::Types(types_block) = block {
                for (type_name, type_def) in &types_block.definitions {
                    self.analyze_type_definition(type_name, type_def)?;
                }
            }
        }
        Ok(())
    }
    
    /// Discover function-related invariants
    fn discover_function_invariants(&mut self, _document: &AispDocument) -> AispResult<()> {
        // Simplified function analysis
        // In practice, this would analyze function definitions for patterns
        Ok(())
    }
    
    /// Analyze a type definition for invariants
    fn analyze_type_definition(&mut self, type_name: &str, type_def: &TypeExpression) -> AispResult<()> {
        match type_def {
            TypeExpression::Natural => {
                self.add_type_non_negativity_invariant(type_name)?;
            }
            TypeExpression::Enumeration(variants) => {
                self.add_enumeration_membership_invariant(type_name, variants)?;
            }
            _ => {
                // Generic type invariants
                self.add_generic_type_invariant(type_name)?;
            }
        }
        Ok(())
    }
    
    /// Add non-negativity invariant for natural number types
    fn add_type_non_negativity_invariant(&mut self, type_name: &str) -> AispResult<()> {
        let invariant = DiscoveredInvariant {
            id: format!("type_nonneg_{}", type_name),
            name: format!("Non-negativity of {}", type_name),
            formula: self.create_non_negativity_formula(type_name)?,
            invariant_type: InvariantType::TypeStructural,
            confidence: 0.95,
            evidence: vec![InvariantEvidence {
                evidence_type: EvidenceType::TypeSystemEnforcement,
                strength: 0.95,
                description: "Natural number types are non-negative by definition".to_string(),
                location: SourceLocation {
                    block_type: "Types".to_string(),
                    line: None,
                    column: None,
                    source_text: Some(format!("{}≜ℕ", type_name)),
                },
            }],
            sources: vec![SourceLocation {
                block_type: "Types".to_string(),
                line: None,
                column: None,
                source_text: Some(format!("{}≜ℕ", type_name)),
            }],
            verified: false,
        };
        
        self.discovered_invariants.push(invariant);
        Ok(())
    }
    
    /// Add enumeration membership invariant
    fn add_enumeration_membership_invariant(&mut self, type_name: &str, variants: &[String]) -> AispResult<()> {
        let invariant = DiscoveredInvariant {
            id: format!("enum_membership_{}", type_name),
            name: format!("Membership constraint for {}", type_name),
            formula: self.create_membership_formula(type_name, variants)?,
            invariant_type: InvariantType::TypeMembership,
            confidence: 0.90,
            evidence: vec![InvariantEvidence {
                evidence_type: EvidenceType::TypeSystemEnforcement,
                strength: 0.90,
                description: format!("Enumeration {} must be one of {:?}", type_name, variants),
                location: SourceLocation {
                    block_type: "Types".to_string(),
                    line: None,
                    column: None,
                    source_text: Some(format!("{}≜{{{}}}", type_name, variants.join(","))),
                },
            }],
            sources: vec![SourceLocation {
                block_type: "Types".to_string(),
                line: None,
                column: None,
                source_text: Some(format!("{}≜{{{}}}", type_name, variants.join(","))),
            }],
            verified: false,
        };
        
        self.discovered_invariants.push(invariant);
        Ok(())
    }
    
    /// Add generic type invariant
    fn add_generic_type_invariant(&mut self, type_name: &str) -> AispResult<()> {
        let invariant = DiscoveredInvariant {
            id: format!("generic_type_{}", type_name),
            name: format!("Type consistency for {}", type_name),
            formula: self.create_generic_type_formula(type_name)?,
            invariant_type: InvariantType::TypeStructural,
            confidence: 0.75,
            evidence: vec![InvariantEvidence {
                evidence_type: EvidenceType::TypeSystemEnforcement,
                strength: 0.75,
                description: format!("Type {} must be well-formed", type_name),
                location: SourceLocation {
                    block_type: "Types".to_string(),
                    line: None,
                    column: None,
                    source_text: Some(format!("{}≜...", type_name)),
                },
            }],
            sources: vec![SourceLocation {
                block_type: "Types".to_string(),
                line: None,
                column: None,
                source_text: Some(format!("{}≜...", type_name)),
            }],
            verified: false,
        };
        
        self.discovered_invariants.push(invariant);
        Ok(())
    }
    
    /// Create non-negativity formula
    fn create_non_negativity_formula(&self, type_name: &str) -> AispResult<PropertyFormula> {
        Ok(PropertyFormula {
            structure: FormulaStructure::Universal(
                PropQuantifier {
                    variable: "x".to_string(),
                    variable_type: Some(type_name.to_string()),
                    domain: None,
                },
                Box::new(FormulaStructure::Atomic(AtomicFormula {
                    predicate: "≥".to_string(),
                    terms: vec![
                        Term::Variable("x".to_string(), Some(type_name.to_string())),
                        Term::Constant("0".to_string(), "ℕ".to_string()),
                    ],
                    type_signature: None,
                }))
            ),
            quantifiers: vec![PropQuantifier {
                variable: "x".to_string(),
                variable_type: Some(type_name.to_string()),
                domain: None,
            }],
            free_variables: HashSet::new(),
            predicates: {
                let mut set = HashSet::new();
                set.insert("≥".to_string());
                set
            },
            functions: HashSet::new(),
            constants: {
                let mut set = HashSet::new();
                set.insert("0".to_string());
                set
            },
        })
    }
    
    /// Create membership formula for enumeration
    fn create_membership_formula(&self, type_name: &str, variants: &[String]) -> AispResult<PropertyFormula> {
        Ok(PropertyFormula {
            structure: FormulaStructure::Universal(
                PropQuantifier {
                    variable: "x".to_string(),
                    variable_type: Some(type_name.to_string()),
                    domain: None,
                },
                Box::new(FormulaStructure::Atomic(AtomicFormula {
                    predicate: "∈".to_string(),
                    terms: vec![
                        Term::Variable("x".to_string(), Some(type_name.to_string())),
                        Term::Constant(format!("{{{}}}", variants.join(",")), type_name.to_string()),
                    ],
                    type_signature: None,
                }))
            ),
            quantifiers: vec![PropQuantifier {
                variable: "x".to_string(),
                variable_type: Some(type_name.to_string()),
                domain: None,
            }],
            free_variables: HashSet::new(),
            predicates: {
                let mut set = HashSet::new();
                set.insert("∈".to_string());
                set
            },
            functions: HashSet::new(),
            constants: {
                let mut set = HashSet::new();
                set.insert(format!("{{{}}}", variants.join(",")));
                set
            },
        })
    }
    
    /// Create generic type formula
    fn create_generic_type_formula(&self, type_name: &str) -> AispResult<PropertyFormula> {
        Ok(PropertyFormula {
            structure: FormulaStructure::Universal(
                PropQuantifier {
                    variable: "x".to_string(),
                    variable_type: Some(type_name.to_string()),
                    domain: None,
                },
                Box::new(FormulaStructure::Atomic(AtomicFormula {
                    predicate: "WellFormed".to_string(),
                    terms: vec![
                        Term::Variable("x".to_string(), Some(type_name.to_string())),
                    ],
                    type_signature: None,
                }))
            ),
            quantifiers: vec![PropQuantifier {
                variable: "x".to_string(),
                variable_type: Some(type_name.to_string()),
                domain: None,
            }],
            free_variables: HashSet::new(),
            predicates: {
                let mut set = HashSet::new();
                set.insert("WellFormed".to_string());
                set
            },
            functions: HashSet::new(),
            constants: HashSet::new(),
        })
    }
    
    /// Export invariants to JSON format
    pub fn export_json(&self, invariants: &[DiscoveredInvariant]) -> String {
        // Simplified JSON export
        let mut json = String::from("{\n");
        json.push_str("  \"invariants\": [\n");
        
        for (i, inv) in invariants.iter().enumerate() {
            json.push_str(&format!("    {{\n"));
            json.push_str(&format!("      \"id\": \"{}\",\n", inv.id));
            json.push_str(&format!("      \"name\": \"{}\",\n", inv.name));
            json.push_str(&format!("      \"confidence\": {:.2},\n", inv.confidence));
            json.push_str(&format!("      \"verified\": {}\n", inv.verified));
            json.push_str(&format!("    }}"));
            if i < invariants.len() - 1 {
                json.push_str(",");
            }
            json.push_str("\n");
        }
        
        json.push_str("  ]\n");
        json.push_str("}\n");
        json
    }
    
    /// Export invariants to SMT-LIB format
    pub fn export_smt_lib(&self, invariants: &[DiscoveredInvariant]) -> String {
        let mut smt = String::new();
        smt.push_str("; Generated invariants\n");
        
        for inv in invariants {
            smt.push_str(&format!("; {}: {}\n", inv.id, inv.name));
            smt.push_str(&format!("; Confidence: {:.2}\n", inv.confidence));
            smt.push_str("(assert true)\n"); // Simplified
            smt.push_str("\n");
        }
        
        smt
    }
    
    /// Export invariants to human-readable format
    pub fn export_human_readable(&self, invariants: &[DiscoveredInvariant]) -> String {
        let mut output = String::new();
        output.push_str("Discovered Invariants\n");
        output.push_str("====================\n\n");
        
        for (i, invariant) in invariants.iter().enumerate() {
            output.push_str(&format!("{}. {}\n", i + 1, invariant.name));
            output.push_str(&format!("   ID: {}\n", invariant.id));
            output.push_str(&format!("   Type: {:?}\n", invariant.invariant_type));
            output.push_str(&format!(
                "   Confidence: {:.2}%\n",
                invariant.confidence * 100.0
            ));
            output.push_str(&format!("   Verified: {}\n", invariant.verified));
            output.push_str(&format!(
                "   Evidence: {} sources\n",
                invariant.evidence.len()
            ));
            output.push_str("\n");
        }
        
        output
    }
}