//! Property Extractor Module - Re-exports from Modular Components
//!
//! This module provides a unified interface to the property extraction
//! functionality by re-exporting components from focused modules.

// Re-export core types and structures
pub use crate::property_types::*;

// Re-export conversion functionality
pub use crate::formula_converter::FormulaConverter;

// Re-export property creation
pub use crate::property_factory::PropertyFactory;

// Re-export main extractor
pub use crate::property_extractor_main::PropertyExtractor;

// For backward compatibility, provide aliases
pub type ExtractedProperty = crate::property_types::ExtractedProperty;
pub type PropertyType = crate::property_types::PropertyType;
pub type PropertyFormula = crate::property_types::PropertyFormula;
pub type FormulaStructure = crate::property_types::FormulaStructure;
pub type PropertyComplexity = crate::property_types::PropertyComplexity;
pub type PropertyContext = crate::property_types::PropertyContext;
pub type SourceLocation = crate::property_types::SourceLocation;
pub type PropertyExtractionStats = crate::property_types::PropertyExtractionStats;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;

    #[test]
    fn test_module_exports() {
        // Test that we can create instances from re-exported types
        let _extractor = PropertyExtractor::new();
        let _factory = PropertyFactory::new();
        
        // Test type aliases work
        let _property_type: PropertyType = PropertyType::TypeSafety;
        let _complexity = PropertyComplexity {
            quantifier_depth: 1,
            logical_connectives: 1,
            function_applications: 1,
            variable_count: 1,
            difficulty_score: 5,
        };
    }

    #[test]
    fn test_backward_compatibility() {
        // Ensure the old interface still works through aliases
        let property_type: PropertyType = PropertyType::TemporalSafety;
        assert_eq!(property_type, PropertyType::TemporalSafety);
        
        let _extractor: PropertyExtractor = PropertyExtractor::new();
    }
}