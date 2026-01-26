//! Theorem Prover Module - Re-exports from Modular Components
//!
//! This module provides a unified interface to the automated theorem prover
//! functionality by re-exporting components from focused modules.

// Re-export core types and structures
pub use crate::proof_types::*;

// Re-export axiom system functionality
pub use crate::axiom_system::*;

// Re-export proof search functionality
pub use crate::proof_search::*;

// Re-export main theorem prover
pub use crate::theorem_prover_main::{TheoremProver, ProofSummary};

// For backward compatibility, provide aliases
pub type ProofResult = crate::proof_types::ProofResult;
pub type ProofOutcome = crate::proof_types::ProofOutcome;
pub type FormalProof = crate::proof_types::FormalProof;
pub type Counterexample = crate::proof_types::Counterexample;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::property_types::*;
    use std::collections::HashSet;

    #[test]
    fn test_module_exports() {
        // Test that we can create instances from re-exported types
        let _prover = TheoremProver::new();
        
        // Test type aliases work
        let _result: ProofResult = ProofResult::new(ProofOutcome::Proven);
        let _outcome: ProofOutcome = ProofOutcome::Unknown;
    }

    #[test]
    fn test_backward_compatibility() {
        // Ensure the old interface still works through aliases
        let result: ProofResult = ProofResult::new(ProofOutcome::Proven);
        assert!(result.is_proven());
        
        let _prover: TheoremProver = TheoremProver::new();
    }

    #[test]
    fn test_axiom_creation() {
        let axiom = Axiom::new(
            "test".to_string(),
            FormulaStructure::Atomic(AtomicFormula {
                predicate: "P".to_string(),
                terms: vec![],
                type_signature: None,
            }),
            AxiomType::Logical,
            5,
        );
        assert_eq!(axiom.name, "test");
    }

    #[test]
    fn test_proof_search_engine() {
        let engine = ProofSearchEngine::new(vec![], vec![]);
        let stats = engine.get_stats();
        assert_eq!(stats.steps_explored, 0);
    }
}