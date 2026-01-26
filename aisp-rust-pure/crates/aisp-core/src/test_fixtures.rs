//! Canonical Test Fixtures and Builders
//!
//! This module provides standardized test fixtures and builders to ensure
//! consistent and maintainable tests across all AISP components.

#[cfg(test)]
use crate::property_types::*;
#[cfg(test)]
use std::collections::{HashMap, HashSet};

#[cfg(test)]
/// Builder for creating test formal properties
pub struct TestPropertyBuilder {
    id: String,
    name: String,
    property_type: PropertyType,
    formula: Option<PropertyFormula>,
    context: Option<PropertyContext>,
}

#[cfg(test)]
impl TestPropertyBuilder {
    /// Create new test property builder
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            name: format!("Test Property {}", id),
            property_type: PropertyType::TypeSafety,
            formula: None,
            context: None,
        }
    }

    /// Set property type
    pub fn with_type(mut self, property_type: PropertyType) -> Self {
        self.property_type = property_type;
        self
    }

    /// Set property name
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    /// Add simple atomic formula
    pub fn with_atomic_formula(mut self, predicate: &str) -> Self {
        self.formula = Some(PropertyFormula {
            structure: FormulaStructure::Atomic(AtomicFormula {
                predicate: predicate.to_string(),
                terms: vec![],
                type_signature: None,
            }),
            quantifiers: vec![],
            free_variables: HashSet::new(),
            predicates: {
                let mut set = HashSet::new();
                set.insert(predicate.to_string());
                set
            },
            functions: HashSet::new(),
            constants: HashSet::new(),
        });
        self
    }

    /// Add universal quantification formula
    pub fn with_universal_formula(mut self, var: &str, predicate: &str) -> Self {
        let quantifier = Quantifier {
            variable: var.to_string(),
            variable_type: Some("Int".to_string()),
            domain: None,
        };

        let inner = FormulaStructure::Atomic(AtomicFormula {
            predicate: predicate.to_string(),
            terms: vec![Term::Variable(var.to_string(), Some("Int".to_string()))],
            type_signature: None,
        });

        self.formula = Some(PropertyFormula {
            structure: FormulaStructure::Universal(quantifier.clone(), Box::new(inner)),
            quantifiers: vec![quantifier],
            free_variables: HashSet::new(),
            predicates: {
                let mut set = HashSet::new();
                set.insert(predicate.to_string());
                set
            },
            functions: HashSet::new(),
            constants: HashSet::new(),
        });
        self
    }

    /// Build the extracted property
    pub fn build(self) -> ExtractedProperty {
        let formula = self.formula.unwrap_or_else(|| PropertyFormula {
            structure: FormulaStructure::Atomic(AtomicFormula {
                predicate: "DefaultPredicate".to_string(),
                terms: vec![],
                type_signature: None,
            }),
            quantifiers: vec![],
            free_variables: HashSet::new(),
            predicates: HashSet::new(),
            functions: HashSet::new(),
            constants: HashSet::new(),
        });

        ExtractedProperty {
            id: self.id,
            name: self.name,
            property_type: self.property_type,
            formula,
            context: self.context.unwrap_or_else(|| PropertyContext {
                type_definitions: HashMap::new(),
                function_definitions: HashMap::new(),
                constants: HashMap::new(),
                dependencies: vec![],
            }),
            source_location: SourceLocation {
                block_type: "Test".to_string(),
                line: Some(1),
                column: Some(1),
                source_text: Some("test property".to_string()),
            },
            complexity: PropertyComplexity {
                quantifier_depth: 1,
                logical_connectives: 1,
                function_applications: 0,
                variable_count: 1,
                difficulty_score: 2,
            },
        }
    }
}

#[cfg(test)]
/// Assertion helpers for testing
pub mod assertions {
    use super::*;

    /// Assert that a result is ok and return the value
    pub fn assert_ok<T, E: std::fmt::Debug>(result: Result<T, E>) -> T {
        match result {
            Ok(value) => value,
            Err(err) => panic!("Expected Ok but got Err: {:?}", err),
        }
    }

    /// Assert that a result is an error
    pub fn assert_err<T: std::fmt::Debug, E>(result: Result<T, E>) {
        match result {
            Ok(value) => panic!("Expected Err but got Ok: {:?}", value),
            Err(_) => {},
        }
    }

    /// Assert that two properties are equivalent
    pub fn assert_properties_equivalent(prop1: &ExtractedProperty, prop2: &ExtractedProperty) {
        assert_eq!(prop1.id, prop2.id);
        assert_eq!(prop1.property_type, prop2.property_type);
        assert_eq!(prop1.formula.structure, prop2.formula.structure);
    }

    /// Assert that a formula has expected structure
    pub fn assert_formula_structure(formula: &FormulaStructure, expected_type: &str) {
        match (formula, expected_type) {
            (FormulaStructure::Atomic(_), "atomic") => {},
            (FormulaStructure::Universal(_, _), "universal") => {},
            (FormulaStructure::Existential(_, _), "existential") => {},
            (FormulaStructure::Conjunction(_), "conjunction") => {},
            (FormulaStructure::Disjunction(_), "disjunction") => {},
            (FormulaStructure::Implication(_, _), "implication") => {},
            (FormulaStructure::Negation(_), "negation") => {},
            _ => panic!("Formula structure mismatch: expected {}, got {:?}", expected_type, formula),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_fixtures::assertions::*;

    #[test]
    fn test_property_builder() {
        let property = TestPropertyBuilder::new("test_1")
            .with_type(PropertyType::TypeSafety)
            .with_name("Test Type Safety Property")
            .with_atomic_formula("P")
            .build();

        assert_eq!(property.id, "test_1");
        assert_eq!(property.property_type, PropertyType::TypeSafety);
        assert_formula_structure(&property.formula.structure, "atomic");
    }

    #[test]
    fn test_universal_formula_builder() {
        let property = TestPropertyBuilder::new("universal_test")
            .with_universal_formula("x", "TypeSafe")
            .build();

        assert_formula_structure(&property.formula.structure, "universal");
        assert_eq!(property.formula.quantifiers.len(), 1);
        assert_eq!(property.formula.quantifiers[0].variable, "x");
    }

    #[test]
    fn test_assertion_helpers() {
        let result: Result<i32, String> = Ok(42);
        let value = assert_ok(result);
        assert_eq!(value, 42);

        let error_result: Result<i32, String> = Err("test error".to_string());
        assert_err(error_result);
    }
}