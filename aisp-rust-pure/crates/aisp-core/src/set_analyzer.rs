//! Set theory analysis for AISP documents
//!
//! This module provides set theory validation, operations analysis,
//! and membership checking for relational logic constructs.

use crate::ast::*;
use crate::error::*;
use std::collections::{HashMap, HashSet};

/// Set theory analyzer
pub struct SetAnalyzer {
    /// Known set definitions
    sets: HashMap<String, SetDefinition>,
    /// Set operations found
    operations: Vec<SetOperation>,
    /// Membership relationships
    memberships: HashMap<String, HashSet<String>>,
}

/// Definition of a set in the system
#[derive(Debug, Clone)]
pub struct SetDefinition {
    /// Set name
    pub name: String,
    /// Set type
    pub set_type: SetType,
    /// Known elements (if finite)
    pub elements: HashSet<String>,
    /// Set properties
    pub properties: SetProperties,
    /// Source location
    pub span: Span,
}

/// Types of sets
#[derive(Debug, Clone, PartialEq)]
pub enum SetType {
    /// Finite enumeration set {a, b, c}
    Finite,
    /// Infinite mathematical set (ℕ, ℤ, ℝ, etc.)
    Infinite,
    /// Derived set from operations
    Derived,
    /// Parameterized set
    Parameterized,
}

/// Set properties
#[derive(Debug, Clone)]
pub struct SetProperties {
    /// Is the set empty?
    pub is_empty: bool,
    /// Is the set finite?
    pub is_finite: bool,
    /// Estimated cardinality (if finite)
    pub cardinality: Option<usize>,
    /// Is the set well-ordered?
    pub is_well_ordered: bool,
}

/// Set operation detected in the document
#[derive(Debug, Clone)]
pub struct SetOperation {
    /// Operation type
    pub operation_type: SetOperationType,
    /// Input sets
    pub inputs: Vec<String>,
    /// Output set (if any)
    pub output: Option<String>,
    /// Source location
    pub span: Span,
}

/// Types of set operations
#[derive(Debug, Clone, PartialEq)]
pub enum SetOperationType {
    /// Union (A ∪ B)
    Union,
    /// Intersection (A ∩ B)
    Intersection,
    /// Difference (A \ B)
    Difference,
    /// Complement (A̅)
    Complement,
    /// Cartesian product (A × B)
    CartesianProduct,
    /// Power set (℘(A))
    PowerSet,
    /// Subset relation (A ⊆ B)
    Subset,
    /// Proper subset (A ⊂ B)
    ProperSubset,
    /// Set membership (a ∈ A)
    Membership,
}

/// Membership relationship check
#[derive(Debug, Clone)]
pub struct MembershipCheck {
    /// Element being checked
    pub element: String,
    /// Set being checked against
    pub set: String,
    /// Is the membership valid?
    pub is_valid: bool,
    /// Confidence level (0.0-1.0)
    pub confidence: f64,
    /// Reasoning for the check
    pub reasoning: String,
}

/// Result of set theory analysis
#[derive(Debug, Clone)]
pub struct SetAnalysisResult {
    /// All set definitions found
    pub sets: HashMap<String, SetDefinition>,
    /// Set operations detected
    pub operations: Vec<SetOperation>,
    /// Membership checks performed
    pub membership_checks: Vec<MembershipCheck>,
    /// Set hierarchy validation
    pub hierarchy_valid: bool,
    /// Empty set references
    pub empty_set_refs: Vec<String>,
    /// Analysis consistency score
    pub consistency_score: f64,
}

impl SetAnalyzer {
    /// Create a new set analyzer
    pub fn new() -> Self {
        Self {
            sets: HashMap::new(),
            operations: Vec::new(),
            memberships: HashMap::new(),
        }
    }

    /// Analyze set theory in the document
    pub fn analyze_document(&mut self, document: &AispDocument) -> SetAnalysisResult {
        self.sets.clear();
        self.operations.clear();
        self.memberships.clear();

        // Extract set definitions from types
        self.extract_set_definitions(document);

        // Analyze set operations in rules and functions
        self.analyze_set_operations(document);

        // Perform membership validation
        let membership_checks = self.validate_memberships();

        // Check set hierarchy consistency
        let hierarchy_valid = self.validate_set_hierarchy();

        // Find empty set references
        let empty_set_refs = self.find_empty_set_references();

        // Calculate consistency score
        let consistency_score = self.calculate_consistency_score(&membership_checks);

        SetAnalysisResult {
            sets: self.sets.clone(),
            operations: self.operations.clone(),
            membership_checks,
            hierarchy_valid,
            empty_set_refs,
            consistency_score,
        }
    }

    /// Extract set definitions from the document
    fn extract_set_definitions(&mut self, document: &AispDocument) {
        for block in &document.blocks {
            match block {
                AispBlock::Types(types_block) => {
                    self.extract_type_sets(types_block);
                }
                AispBlock::Meta(meta_block) => {
                    self.extract_meta_sets(meta_block);
                }
                _ => {}
            }
        }

        // Add built-in mathematical sets
        self.add_builtin_sets();
    }

    /// Extract sets from type definitions
    fn extract_type_sets(&mut self, types_block: &TypesBlock) {
        for (name, definition) in &types_block.definitions {
            match &definition.type_expr {
                TypeExpression::Enumeration(values) => {
                    let set_def = SetDefinition {
                        name: name.clone(),
                        set_type: SetType::Finite,
                        elements: values.iter().cloned().collect(),
                        properties: SetProperties {
                            is_empty: values.is_empty(),
                            is_finite: true,
                            cardinality: Some(values.len()),
                            is_well_ordered: true, // Assume enumerations are well-ordered
                        },
                        span: definition.span.clone(),
                    };
                    self.sets.insert(name.clone(), set_def);

                    // Add membership relationships
                    let mut members = HashSet::new();
                    for value in values {
                        members.insert(value.clone());
                    }
                    self.memberships.insert(name.clone(), members);
                }
                TypeExpression::Basic(basic_type) => {
                    let (set_type, is_finite, cardinality) = match basic_type {
                        BasicType::Natural => (SetType::Infinite, false, None),
                        BasicType::Integer => (SetType::Infinite, false, None),
                        BasicType::Real => (SetType::Infinite, false, None),
                        BasicType::Boolean => (SetType::Finite, true, Some(2)),
                        BasicType::String => (SetType::Infinite, false, None),
                    };

                    let set_def = SetDefinition {
                        name: name.clone(),
                        set_type,
                        elements: HashSet::new(),
                        properties: SetProperties {
                            is_empty: false,
                            is_finite,
                            cardinality,
                            is_well_ordered: matches!(basic_type, BasicType::Natural | BasicType::Integer),
                        },
                        span: definition.span.clone(),
                    };
                    self.sets.insert(name.clone(), set_def);
                }
                _ => {}
            }
        }
    }

    /// Extract sets from meta information
    fn extract_meta_sets(&mut self, _meta_block: &MetaBlock) {
        // TODO: Extract set definitions from meta constraints
    }

    /// Add built-in mathematical sets
    fn add_builtin_sets(&mut self) {
        let builtins = vec![
            ("ℕ", SetType::Infinite, false, None),
            ("ℤ", SetType::Infinite, false, None),
            ("ℝ", SetType::Infinite, false, None),
            ("ℚ", SetType::Infinite, false, None),
            ("𝔹", SetType::Finite, true, Some(2)),
            ("∅", SetType::Finite, true, Some(0)),
        ];

        for (name, set_type, is_finite, cardinality) in builtins {
            let set_def = SetDefinition {
                name: name.to_string(),
                set_type,
                elements: if name == "𝔹" {
                    ["true", "false"].iter().map(|s| s.to_string()).collect()
                } else {
                    HashSet::new()
                },
                properties: SetProperties {
                    is_empty: name == "∅",
                    is_finite,
                    cardinality,
                    is_well_ordered: matches!(name, "ℕ" | "ℤ" | "ℚ" | "𝔹"),
                },
                span: Span::new(0, 0, 0, 0), // Built-in, no source location
            };
            self.sets.insert(name.to_string(), set_def);
        }
    }

    /// Analyze set operations in the document
    fn analyze_set_operations(&mut self, document: &AispDocument) {
        for block in &document.blocks {
            match block {
                AispBlock::Rules(rules_block) => {
                    self.analyze_rule_operations(rules_block);
                }
                AispBlock::Functions(functions_block) => {
                    self.analyze_function_operations(functions_block);
                }
                _ => {}
            }
        }
    }

    /// Analyze set operations in rules
    fn analyze_rule_operations(&mut self, rules_block: &RulesBlock) {
        for rule in &rules_block.rules {
            if let LogicalExpression::Variable(expr) = &rule.expression {
                self.extract_operations_from_expression(expr, &rule.span);
            }
        }
    }

    /// Analyze set operations in functions
    fn analyze_function_operations(&mut self, functions_block: &FunctionsBlock) {
        for (_, function) in &functions_block.functions {
            if let LogicalExpression::Variable(expr) = &function.lambda.body {
                self.extract_operations_from_expression(expr, &function.span);
            }
        }
    }

    /// Extract set operations from logical expressions
    fn extract_operations_from_expression(&mut self, expr: &str, span: &Span) {
        // Look for set operation symbols
        if expr.contains('∈') {
            // Membership operation
            self.operations.push(SetOperation {
                operation_type: SetOperationType::Membership,
                inputs: vec![], // TODO: Parse actual operands
                output: None,
                span: span.clone(),
            });
        }
        
        if expr.contains('⊆') {
            // Subset operation
            self.operations.push(SetOperation {
                operation_type: SetOperationType::Subset,
                inputs: vec![], // TODO: Parse actual operands
                output: None,
                span: span.clone(),
            });
        }
        
        if expr.contains('∪') {
            // Union operation
            self.operations.push(SetOperation {
                operation_type: SetOperationType::Union,
                inputs: vec![], // TODO: Parse actual operands
                output: None,
                span: span.clone(),
            });
        }
        
        if expr.contains('∩') {
            // Intersection operation
            self.operations.push(SetOperation {
                operation_type: SetOperationType::Intersection,
                inputs: vec![], // TODO: Parse actual operands
                output: None,
                span: span.clone(),
            });
        }
    }

    /// Validate membership relationships
    fn validate_memberships(&self) -> Vec<MembershipCheck> {
        let mut checks = Vec::new();

        for (set_name, members) in &self.memberships {
            if let Some(set_def) = self.sets.get(set_name) {
                for member in members {
                    let is_valid = match set_def.set_type {
                        SetType::Finite => set_def.elements.contains(member),
                        SetType::Infinite => true, // Assume valid for infinite sets
                        _ => true,
                    };

                    checks.push(MembershipCheck {
                        element: member.clone(),
                        set: set_name.clone(),
                        is_valid,
                        confidence: if is_valid { 1.0 } else { 0.0 },
                        reasoning: if is_valid {
                            "Element is defined in the set".to_string()
                        } else {
                            "Element not found in finite set definition".to_string()
                        },
                    });
                }
            }
        }

        checks
    }

    /// Validate set hierarchy consistency
    fn validate_set_hierarchy(&self) -> bool {
        // Check for circular dependencies in set definitions
        let mut visited = HashSet::new();
        let mut recursion_stack = HashSet::new();

        for set_name in self.sets.keys() {
            if !visited.contains(set_name) {
                if !self.is_hierarchy_valid_dfs(set_name, &mut visited, &mut recursion_stack) {
                    return false;
                }
            }
        }

        true
    }

    /// DFS to check for cycles in set hierarchy
    fn is_hierarchy_valid_dfs(
        &self,
        set_name: &str,
        visited: &mut HashSet<String>,
        recursion_stack: &mut HashSet<String>,
    ) -> bool {
        visited.insert(set_name.to_string());
        recursion_stack.insert(set_name.to_string());

        // Check dependencies of this set
        // TODO: Implement actual dependency checking based on set definitions

        recursion_stack.remove(set_name);
        true
    }

    /// Find empty set references
    fn find_empty_set_references(&self) -> Vec<String> {
        let mut empty_refs = Vec::new();

        for (name, set_def) in &self.sets {
            if set_def.properties.is_empty {
                empty_refs.push(name.clone());
            }
        }

        empty_refs
    }

    /// Calculate consistency score
    fn calculate_consistency_score(&self, membership_checks: &[MembershipCheck]) -> f64 {
        if membership_checks.is_empty() {
            return 1.0;
        }

        let valid_count = membership_checks.iter().filter(|check| check.is_valid).count();
        valid_count as f64 / membership_checks.len() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_document() -> AispDocument {
        let mut type_definitions = HashMap::new();
        type_definitions.insert("Colors".to_string(), TypeDefinition {
            name: "Colors".to_string(),
            type_expr: TypeExpression::Enumeration(vec!["Red".to_string(), "Blue".to_string(), "Green".to_string()]),
            span: Span::new(1, 1, 1, 20),
        });

        AispDocument {
            header: DocumentHeader {
                version: "5.1".to_string(),
                name: "test".to_string(),
                date: "2026-01-25".to_string(),
                metadata: None,
            },
            metadata: DocumentMetadata {
                domain: None,
                protocol: None,
            },
            blocks: vec![AispBlock::Types(TypesBlock {
                definitions: type_definitions,
                span: Span::new(1, 1, 3, 1),
            })],
            span: Span::new(1, 1, 10, 1),
        }
    }

    #[test]
    fn test_set_extraction() {
        let mut analyzer = SetAnalyzer::new();
        let document = create_test_document();

        let result = analyzer.analyze_document(&document);

        assert!(result.sets.contains_key("Colors"));
        assert!(result.sets.contains_key("ℕ")); // Built-in set
        assert!(result.sets.contains_key("∅")); // Empty set

        let colors_set = &result.sets["Colors"];
        assert_eq!(colors_set.set_type, SetType::Finite);
        assert!(colors_set.elements.contains("Red"));
        assert!(colors_set.elements.contains("Blue"));
        assert!(colors_set.elements.contains("Green"));
    }

    #[test]
    fn test_builtin_sets() {
        let mut analyzer = SetAnalyzer::new();
        analyzer.add_builtin_sets();

        assert!(analyzer.sets.contains_key("ℕ"));
        assert!(analyzer.sets.contains_key("ℤ"));
        assert!(analyzer.sets.contains_key("ℝ"));
        assert!(analyzer.sets.contains_key("𝔹"));
        assert!(analyzer.sets.contains_key("∅"));

        let empty_set = &analyzer.sets["∅"];
        assert!(empty_set.properties.is_empty);
        assert_eq!(empty_set.properties.cardinality, Some(0));

        let bool_set = &analyzer.sets["𝔹"];
        assert!(!bool_set.properties.is_empty);
        assert_eq!(bool_set.properties.cardinality, Some(2));
    }

    #[test]
    fn test_membership_validation() {
        let mut analyzer = SetAnalyzer::new();
        
        // Create a finite set
        let set_def = SetDefinition {
            name: "TestSet".to_string(),
            set_type: SetType::Finite,
            elements: ["a", "b", "c"].iter().map(|s| s.to_string()).collect(),
            properties: SetProperties {
                is_empty: false,
                is_finite: true,
                cardinality: Some(3),
                is_well_ordered: true,
            },
            span: Span::new(1, 1, 1, 10),
        };
        
        analyzer.sets.insert("TestSet".to_string(), set_def);
        analyzer.memberships.insert("TestSet".to_string(), 
            ["a", "b", "c", "d"].iter().map(|s| s.to_string()).collect());

        let checks = analyzer.validate_memberships();
        assert_eq!(checks.len(), 4);

        // Check that a, b, c are valid members but d is not
        let valid_members: HashSet<_> = checks.iter()
            .filter(|check| check.is_valid)
            .map(|check| check.element.clone())
            .collect();
        
        assert!(valid_members.contains("a"));
        assert!(valid_members.contains("b"));
        assert!(valid_members.contains("c"));
        assert!(!valid_members.contains("d"));
    }

    #[test]
    fn test_set_operation_detection() {
        let mut analyzer = SetAnalyzer::new();
        
        analyzer.extract_operations_from_expression("x ∈ S", &Span::new(1, 1, 1, 10));
        analyzer.extract_operations_from_expression("A ⊆ B", &Span::new(2, 1, 2, 10));
        analyzer.extract_operations_from_expression("C ∪ D", &Span::new(3, 1, 3, 10));

        assert_eq!(analyzer.operations.len(), 3);
        
        let op_types: Vec<_> = analyzer.operations.iter()
            .map(|op| &op.operation_type)
            .collect();
        
        assert!(op_types.contains(&&SetOperationType::Membership));
        assert!(op_types.contains(&&SetOperationType::Subset));
        assert!(op_types.contains(&&SetOperationType::Union));
    }

    #[test]
    fn test_consistency_score() {
        let analyzer = SetAnalyzer::new();
        
        let checks = vec![
            MembershipCheck {
                element: "a".to_string(),
                set: "S".to_string(),
                is_valid: true,
                confidence: 1.0,
                reasoning: "Valid".to_string(),
            },
            MembershipCheck {
                element: "b".to_string(),
                set: "S".to_string(),
                is_valid: true,
                confidence: 1.0,
                reasoning: "Valid".to_string(),
            },
            MembershipCheck {
                element: "c".to_string(),
                set: "S".to_string(),
                is_valid: false,
                confidence: 0.0,
                reasoning: "Invalid".to_string(),
            },
        ];

        let score = analyzer.calculate_consistency_score(&checks);
        assert!((score - 0.6666666666666666).abs() < 1e-10);
    }
}