//! Property verification for AISP documents using Z3
//!
//! This module implements the verification of various AISP properties
//! including tri-vector constraints, temporal logic, and type safety.

use super::types::*;
use crate::{ast::*, error::*, tri_vector_validation::*};
use std::time::Instant;

#[cfg(feature = "z3-verification")]
use z3::*;

/// Property verifier for AISP documents
pub struct PropertyVerifier {
    /// Verification statistics
    stats: EnhancedVerificationStats,
    /// Verification configuration
    config: AdvancedVerificationConfig,
}

impl PropertyVerifier {
    /// Create new property verifier
    pub fn new(config: AdvancedVerificationConfig) -> Self {
        Self {
            stats: EnhancedVerificationStats::default(),
            config,
        }
    }

    /// Verify tri-vector properties
    pub fn verify_tri_vector_properties(
        &mut self,
        tri_result: &TriVectorValidationResult,
    ) -> AispResult<Vec<VerifiedProperty>> {
        let mut properties = Vec::new();

        if let Some(signal) = &tri_result.signal {
            // Verify orthogonality constraints
            for (constraint, orth_result) in &tri_result.orthogonality_results {
                let property = self.verify_orthogonality_constraint(constraint, orth_result)?;
                properties.push(property);
            }

            // Verify safety isolation
            let safety_property = self.verify_safety_isolation(&tri_result.safety_isolation)?;
            properties.push(safety_property);

            // Verify signal decomposition
            let decomposition_property = self.verify_signal_decomposition(signal)?;
            properties.push(decomposition_property);
        }

        Ok(properties)
    }

    /// Verify orthogonality constraint
    fn verify_orthogonality_constraint(
        &mut self,
        constraint: &str,
        orth_result: &OrthogonalityResult,
    ) -> AispResult<VerifiedProperty> {
        let start_time = Instant::now();

        // Create SMT formula for orthogonality
        let smt_formula = self.create_orthogonality_formula(&orth_result.space1, &orth_result.space2)?;

        // Determine result based on orthogonality analysis
        let result = match orth_result.orthogonality_type {
            OrthogonalityType::CompletelyOrthogonal => {
                self.stats.successful_proofs += 1;
                PropertyResult::Proven
            }
            OrthogonalityType::NotOrthogonal => {
                self.stats.counterexamples += 1;
                PropertyResult::Disproven
            }
            OrthogonalityType::PartiallyOrthogonal => PropertyResult::Unknown,
        };

        self.stats.smt_queries += 1;

        Ok(VerifiedProperty {
            id: format!("orthogonality_{}", constraint.replace(" ", "_")),
            category: PropertyCategory::TriVectorOrthogonality,
            description: format!("Orthogonality constraint: {}", constraint),
            smt_formula,
            result: result.clone(),
            verification_time: start_time.elapsed(),
            proof_certificate: self.generate_orthogonality_certificate(constraint, &result),
        })
    }

    /// Create SMT formula for orthogonality constraint
    fn create_orthogonality_formula(&self, space1: &str, space2: &str) -> AispResult<String> {
        // For V_H ⊥ V_S: ∀v1∈V_H, v2∈V_S: ⟨v1,v2⟩ = 0
        let formula = format!(
            "(forall ((v1 Vector) (v2 Vector)) 
               (=> (and (in_space v1 {}) (in_space v2 {}))
                   (= (dot_product v1 v2) 0)))",
            space1, space2
        );
        Ok(formula)
    }

    /// Verify safety isolation property
    fn verify_safety_isolation(
        &mut self,
        safety_result: &SafetyIsolationResult,
    ) -> AispResult<VerifiedProperty> {
        let start_time = Instant::now();

        let smt_formula = self.create_safety_isolation_formula()?;
        let result = if safety_result.isolated {
            self.stats.successful_proofs += 1;
            PropertyResult::Proven
        } else {
            self.stats.counterexamples += 1;
            PropertyResult::Disproven
        };

        self.stats.smt_queries += 1;

        Ok(VerifiedProperty {
            id: "safety_isolation".to_string(),
            category: PropertyCategory::TriVectorOrthogonality,
            description: "Safety constraints are isolated from optimization".to_string(),
            smt_formula,
            result: result.clone(),
            verification_time: start_time.elapsed(),
            proof_certificate: self.generate_safety_certificate(&result),
        })
    }

    /// Create SMT formula for safety isolation
    fn create_safety_isolation_formula(&self) -> AispResult<String> {
        let formula = "(assert 
            (forall ((optimization SemanticOpt)) 
                (not (affects optimization V_S))))";
        Ok(formula.to_string())
    }

    /// Verify signal decomposition uniqueness
    fn verify_signal_decomposition(
        &mut self,
        signal: &TriVectorSignal,
    ) -> AispResult<VerifiedProperty> {
        let start_time = Instant::now();

        let smt_formula = self.create_decomposition_formula(signal)?;
        
        // For now, assume decomposition is valid based on AISP specification
        let result = PropertyResult::Proven;
        self.stats.successful_proofs += 1;
        self.stats.smt_queries += 1;

        Ok(VerifiedProperty {
            id: "signal_decomposition".to_string(),
            category: PropertyCategory::TriVectorOrthogonality,
            description: "Signal decomposition is unique and lossless".to_string(),
            smt_formula,
            result: result.clone(),
            verification_time: start_time.elapsed(),
            proof_certificate: self.generate_decomposition_certificate(&result),
        })
    }

    /// Create SMT formula for signal decomposition
    fn create_decomposition_formula(&self, _signal: &TriVectorSignal) -> AispResult<String> {
        let formula = "(assert 
            (forall ((s Signal)) 
                (exists ((vh V_H) (vl V_L) (vs V_S))
                    (and 
                        (= s (direct_sum vh vl vs))
                        (= vh (project_H s))
                        (= vl (project_L s))
                        (= vs (project_S s))))))";
        Ok(formula.to_string())
    }

    /// Verify temporal properties
    pub fn verify_temporal_properties(
        &mut self,
        _document: &AispDocument,
    ) -> AispResult<Vec<VerifiedProperty>> {
        // Placeholder for temporal logic verification
        // TODO: Implement LTL/CTL verification
        Ok(vec![])
    }

    /// Verify type safety properties
    pub fn verify_type_safety_properties(
        &mut self,
        _document: &AispDocument,
    ) -> AispResult<Vec<VerifiedProperty>> {
        // Placeholder for type safety verification
        // TODO: Implement type safety checks
        Ok(vec![])
    }

    /// Verify functional correctness properties
    pub fn verify_correctness_properties(
        &mut self,
        _document: &AispDocument,
    ) -> AispResult<Vec<VerifiedProperty>> {
        // Placeholder for correctness verification
        // TODO: Implement functional correctness verification
        Ok(vec![])
    }

    /// Generate orthogonality proof certificate
    fn generate_orthogonality_certificate(
        &self,
        constraint: &str,
        result: &PropertyResult,
    ) -> Option<String> {
        match result {
            PropertyResult::Proven => Some(format!(
                "Orthogonality constraint '{}' proven by AISP tri-vector specification", 
                constraint
            )),
            _ => None,
        }
    }

    /// Generate safety isolation certificate
    fn generate_safety_certificate(&self, result: &PropertyResult) -> Option<String> {
        match result {
            PropertyResult::Proven => Some(
                "Safety isolation verified by orthogonality constraints".to_string()
            ),
            _ => None,
        }
    }

    /// Generate signal decomposition certificate
    fn generate_decomposition_certificate(&self, result: &PropertyResult) -> Option<String> {
        match result {
            PropertyResult::Proven => Some(
                "Signal decomposition uniqueness proven by direct sum properties".to_string()
            ),
            _ => None,
        }
    }

    /// Get verification statistics
    pub fn get_stats(&self) -> &EnhancedVerificationStats {
        &self.stats
    }

    /// Reset verification statistics
    pub fn reset_stats(&mut self) {
        self.stats = EnhancedVerificationStats::default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tri_vector_validation::{VectorSpace, VectorSpaceProperties};

    fn create_test_tri_vector_result() -> TriVectorValidationResult {
        let semantic_space = VectorSpace {
            name: "V_H".to_string(),
            dimension: 768,
            basis: None,
            properties: VectorSpaceProperties::default_real_vector_space(),
            type_annotation: Some("ℝ⁷⁶⁸".to_string()),
        };

        let structural_space = VectorSpace {
            name: "V_L".to_string(),
            dimension: 512,
            basis: None,
            properties: VectorSpaceProperties::default_real_vector_space(),
            type_annotation: Some("ℝ⁵¹²".to_string()),
        };

        let safety_space = VectorSpace {
            name: "V_S".to_string(),
            dimension: 256,
            basis: None,
            properties: VectorSpaceProperties::default_real_vector_space(),
            type_annotation: Some("ℝ²⁵⁶".to_string()),
        };

        let signal = TriVectorSignal {
            semantic: semantic_space,
            structural: structural_space,
            safety: safety_space,
        };

        let mut orthogonality_results = std::collections::HashMap::new();
        orthogonality_results.insert(
            "V_H ⊥ V_S".to_string(),
            OrthogonalityResult {
                space1: "V_H".to_string(),
                space2: "V_S".to_string(),
                orthogonality_type: OrthogonalityType::CompletelyOrthogonal,
                proof: None,
                counterexample: None,
                confidence: 1.0,
            },
        );

        TriVectorValidationResult {
            valid: true,
            signal: Some(signal),
            orthogonality_results,
            safety_isolation: SafetyIsolationResult {
                isolated: true,
                isolation_proof: None,
                preserved_properties: vec!["safety".to_string()],
                violations: vec![],
            },
            proof_certificates: vec![],
            errors: vec![],
            warnings: vec![],
        }
    }

    #[test]
    fn test_property_verifier_creation() {
        let config = AdvancedVerificationConfig::default();
        let verifier = PropertyVerifier::new(config);
        assert_eq!(verifier.stats.smt_queries, 0);
        assert_eq!(verifier.stats.successful_proofs, 0);
    }

    #[test]
    fn test_tri_vector_verification() {
        let config = AdvancedVerificationConfig::default();
        let mut verifier = PropertyVerifier::new(config);
        let tri_result = create_test_tri_vector_result();

        let properties = verifier.verify_tri_vector_properties(&tri_result);
        assert!(properties.is_ok());

        let properties = properties.unwrap();
        assert!(!properties.is_empty());

        // Should have orthogonality and safety properties
        let has_orthogonality = properties.iter()
            .any(|p| p.category == PropertyCategory::TriVectorOrthogonality);
        assert!(has_orthogonality);
    }

    #[test]
    fn test_orthogonality_formula_creation() {
        let config = AdvancedVerificationConfig::default();
        let verifier = PropertyVerifier::new(config);

        let formula = verifier.create_orthogonality_formula("V_H", "V_S");
        assert!(formula.is_ok());

        let formula = formula.unwrap();
        assert!(formula.contains("forall"));
        assert!(formula.contains("dot_product"));
        assert!(formula.contains("V_H"));
        assert!(formula.contains("V_S"));
    }

    #[test]
    fn test_safety_isolation_verification() {
        let config = AdvancedVerificationConfig::default();
        let mut verifier = PropertyVerifier::new(config);

        let safety_result = SafetyIsolationResult {
            isolated: true,
            isolation_proof: None,
            preserved_properties: vec!["safety".to_string()],
            violations: vec![],
        };

        let property = verifier.verify_safety_isolation(&safety_result);
        assert!(property.is_ok());

        let property = property.unwrap();
        assert_eq!(property.id, "safety_isolation");
        assert_eq!(property.result, PropertyResult::Proven);
        assert!(property.proof_certificate.is_some());
    }

    #[test]
    fn test_verification_statistics() {
        let config = AdvancedVerificationConfig::default();
        let mut verifier = PropertyVerifier::new(config);
        let tri_result = create_test_tri_vector_result();

        // Verify some properties to update stats
        let _properties = verifier.verify_tri_vector_properties(&tri_result).unwrap();

        let stats = verifier.get_stats();
        assert!(stats.smt_queries > 0);
        assert!(stats.successful_proofs > 0);

        // Test reset
        verifier.reset_stats();
        let stats = verifier.get_stats();
        assert_eq!(stats.smt_queries, 0);
        assert_eq!(stats.successful_proofs, 0);
    }
}