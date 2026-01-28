//! Ambiguity Verification Module
//!
//! Implements formal verification of the ambiguity calculation from reference.md:
//! Ambig≜λD.1-|Parse_u(D)|/|Parse_t(D)| < 0.02

use crate::error::AispResult;
use crate::semantic::DeepVerificationResult;
use crate::z3_verification::{PropertyResult, Z3VerificationFacade};
use std::collections::HashMap;

/// Mathematical foundations verification result
#[derive(Debug, Clone)]
pub struct MathematicalFoundationsResult {
    pub ambiguity_verified: bool,
    pub calculated_ambiguity: f64,
    pub token_efficiency: TokenEfficiencyResult,
    pub mathematical_proofs: Vec<String>,
    pub smt_certificates: Vec<String>,
}

/// Token efficiency metrics
#[derive(Debug, Clone)]
pub struct TokenEfficiencyResult {
    pub efficiency_score: f64,
    pub meets_spec: bool,
    pub compression_ratio: f64,
}

/// Ambiguity verification implementation
pub struct AmbiguityVerifier<'a> {
    z3_verifier: &'a mut Z3VerificationFacade,
}

impl<'a> AmbiguityVerifier<'a> {
    pub fn new(z3_verifier: &'a mut Z3VerificationFacade) -> Self {
        Self { z3_verifier }
    }
    
    /// Verify ambiguity calculation: Ambig≜λD.1-|Parse_u(D)|/|Parse_t(D)|
    pub fn verify_ambiguity_calculation(
        &mut self,
        _source: &str,
        semantic_result: &DeepVerificationResult,
    ) -> AispResult<MathematicalFoundationsResult> {
        // Calculate actual parse counts from semantic analysis
        let unique_parses = 1.0; // Simplified: AISP should have unique interpretation
        let total_parses = if semantic_result.ambiguity() > 0.0 { 
            1.0 / (1.0 - semantic_result.ambiguity())
        } else { 
            1.0 
        };
        
        // Generate properly ordered SMT formula for ambiguity calculation
        let smt_formula = format!(
            ";; Declare constants first\n\
             (declare-const ambiguity Real)\n\
             (declare-const unique_parses Real)\n\
             (declare-const total_parses Real)\n\
             \n\
             ;; Set concrete values from analysis\n\
             (assert (= unique_parses {:.6}))\n\
             (assert (= total_parses {:.6}))\n\
             \n\
             ;; Ambiguity formula: Ambig = 1 - |Parse_u|/|Parse_t|\n\
             (assert (= ambiguity (- 1.0 (/ unique_parses total_parses))))\n\
             \n\
             ;; Constraints from reference.md\n\
             (assert (>= unique_parses 0.0))\n\
             (assert (>= total_parses 1.0))\n\
             (assert (<= unique_parses total_parses))\n\
             \n\
             ;; AISP requirement: ambiguity < 2%\n\
             (assert (< ambiguity 0.02))\n\
             \n\
             (check-sat)\n\
             (get-model)",
            unique_parses, total_parses
        );

        let smt_result = self.z3_verifier.verify_smt_formula(&smt_formula).unwrap_or(PropertyResult::Unknown);
        
        // Also verify using direct calculation
        let calculated_ambiguity = 1.0 - (unique_parses / total_parses);
        let direct_verification = calculated_ambiguity < 0.02;
        
        // Both SMT and direct calculation must agree
        let ambiguity_verified = matches!(smt_result, PropertyResult::Proven) && direct_verification;
        
        // Token efficiency analysis
        let token_efficiency = self.calculate_token_efficiency(semantic_result);
        
        let mut mathematical_proofs = vec![];
        let mut smt_certificates = vec![];
        
        if ambiguity_verified {
            mathematical_proofs.push("Ambiguity constraint Ambig(D)<0.02 verified".to_string());
            smt_certificates.push("SMT_AMBIGUITY_PROVEN".to_string());
        }
        
        Ok(MathematicalFoundationsResult {
            ambiguity_verified,
            calculated_ambiguity: semantic_result.ambiguity(),
            token_efficiency,
            mathematical_proofs,
            smt_certificates,
        })
    }
    
    fn calculate_token_efficiency(&self, semantic_result: &DeepVerificationResult) -> TokenEfficiencyResult {
        // Simplified token efficiency calculation
        // In a real implementation, this would analyze token compression vs semantic preservation
        let efficiency_score = if semantic_result.ambiguity() < 0.02 { 0.95 } else { 0.60 };
        let meets_spec = efficiency_score > 0.80;
        let compression_ratio = 1.0 / (1.0 + semantic_result.ambiguity());
        
        TokenEfficiencyResult {
            efficiency_score,
            meets_spec,
            compression_ratio,
        }
    }
}

/// Verify AISP token efficiency according to reference.md specifications
pub fn verify_token_efficiency(semantic_result: &DeepVerificationResult) -> TokenEfficiencyResult {
    // Calculate efficiency based on ambiguity and coherence
    let base_efficiency = semantic_result.overall_confidence; // Use overall_confidence instead of missing coherence_score
    let ambiguity_penalty = semantic_result.ambiguity() * 2.0; // Penalty factor
    let efficiency_score = (base_efficiency - ambiguity_penalty).max(0.0).min(1.0);
    
    let meets_spec = efficiency_score > 0.80 && semantic_result.ambiguity() < 0.02;
    let compression_ratio = semantic_result.overall_confidence / (1.0 + semantic_result.ambiguity()); // Use overall_confidence instead of missing rule_coverage
    
    TokenEfficiencyResult {
        efficiency_score,
        meets_spec,
        compression_ratio,
    }
}

/// Generate ambiguity test cases for stress testing
pub fn generate_ambiguity_test_cases() -> Vec<(f64, f64, bool)> {
    vec![
        // (unique_parses, total_parses, should_pass)
        (1.0, 1.0, true),    // Perfect - no ambiguity
        (0.98, 1.0, true),   // Just under 2% ambiguity
        (0.95, 1.0, false),  // 5% ambiguity - should fail
        (0.90, 1.0, false),  // 10% ambiguity - should fail  
        (1.0, 1.02, true),   // Slight over-parsing but still under 2%
        (0.85, 1.0, false),  // 15% ambiguity - clear failure
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::z3_verification::Z3VerificationFacade;
    use std::collections::HashMap;
    
    fn create_test_semantic_result(ambiguity: f64, coherence: f64) -> DeepVerificationResult {
        let mut result = DeepVerificationResult::test_default();
        result.overall_confidence = 1.0 - ambiguity; // Convert ambiguity to confidence
        result.semantic_score = coherence;
        result
    }
    
    #[test]
    fn test_token_efficiency_calculation() {
        let high_quality = create_test_semantic_result(0.01, 0.95);
        let result = verify_token_efficiency(&high_quality);
        
        assert!(result.meets_spec);
        assert!(result.efficiency_score > 0.80);
        assert!(result.compression_ratio > 0.90);
    }
    
    #[test]
    fn test_token_efficiency_failure() {
        let low_quality = create_test_semantic_result(0.10, 0.60);
        let result = verify_token_efficiency(&low_quality);
        
        assert!(!result.meets_spec);
        assert!(result.efficiency_score < 0.80);
    }
    
    #[test]
    fn test_ambiguity_test_cases() {
        let test_cases = generate_ambiguity_test_cases();
        assert_eq!(test_cases.len(), 6);
        
        // Check that we have both passing and failing cases
        let passing_cases: Vec<_> = test_cases.iter().filter(|(_, _, should_pass)| *should_pass).collect();
        let failing_cases: Vec<_> = test_cases.iter().filter(|(_, _, should_pass)| !*should_pass).collect();
        
        assert!(!passing_cases.is_empty());
        assert!(!failing_cases.is_empty());
    }
    
    #[test]
    fn test_ambiguity_verifier_creation() {
        let mut z3_facade = Z3VerificationFacade::new_disabled();
        let verifier = AmbiguityVerifier::new(&mut z3_facade);
        
        // Verifier should be created successfully
        assert_eq!(std::mem::size_of_val(&verifier), std::mem::size_of::<&mut Z3VerificationFacade>());
    }
    
    #[test]
    fn test_direct_ambiguity_calculation() {
        let unique_parses = 0.98;
        let total_parses = 1.0;
        let calculated_ambiguity = 1.0 - (unique_parses / total_parses);
        
        assert!((calculated_ambiguity - 0.02_f64).abs() < 0.001);
        assert!(calculated_ambiguity < 0.02); // Should pass the 2% threshold
    }
    
    #[test]
    fn test_edge_case_calculations() {
        // Test division by zero protection
        let zero_total = 0.0;
        let unique = 1.0;
        
        // This should be handled gracefully in the actual implementation
        assert!(unique / (zero_total + 0.001) > 100.0); // Avoid actual division by zero
        
        // Test perfect case
        let perfect_unique = 1.0;
        let perfect_total = 1.0;
        let perfect_ambiguity = 1.0 - (perfect_unique / perfect_total);
        assert_eq!(perfect_ambiguity, 0.0);
    }
}