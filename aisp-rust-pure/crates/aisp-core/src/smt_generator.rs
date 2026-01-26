//! SMT Generator Module - Re-exports from Modular Components
//!
//! This module provides a unified interface to the SMT-LIB generation
//! functionality by re-exporting components from focused modules.

// Re-export core types and structures  
pub use crate::smt_types::*;

// Re-export formula conversion functionality
pub use crate::smt_formula_converter::SMTFormulaConverter;

// Re-export main generator
pub use crate::smt_generator_main::SMTGenerator;

// For backward compatibility, provide aliases
pub type SMTProgram = crate::smt_types::SMTProgram;
pub type SMTExpectedResult = crate::smt_types::SMTExpectedResult;
pub type SMTGenerationResult = crate::smt_types::SMTGenerationResult;
pub type SMTGenerationStats = crate::smt_types::SMTGenerationStats;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::property_types::*;
    use crate::ast::*;

    #[test]
    fn test_module_exports() {
        // Test that we can create instances from re-exported types
        let _generator = SMTGenerator::new();
        let _converter = SMTFormulaConverter::new();
        
        // Test type aliases work
        let _result: SMTExpectedResult = SMTExpectedResult::Unsat;
        let _stats = SMTGenerationStats::new();
    }

    #[test]
    fn test_backward_compatibility() {
        // Ensure the old interface still works through aliases
        let expected_result: SMTExpectedResult = SMTExpectedResult::Sat;
        assert_eq!(expected_result, SMTExpectedResult::Sat);
        
        let _generator: SMTGenerator = SMTGenerator::new();
    }

    #[test]
    fn test_smt_program_creation() {
        let program = SMTProgram::new();
        assert!(program.script.is_empty());
        assert!(program.property_map.is_empty());
    }

    #[test]
    fn test_smt_generation_stats() {
        let stats = SMTGenerationStats::new();
        assert_eq!(stats.properties_encoded, 0);
        assert_eq!(stats.generation_time_ms, 0);
    }
}