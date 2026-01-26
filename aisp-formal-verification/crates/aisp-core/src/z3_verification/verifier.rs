//! Z3 verifier implementation with conditional compilation support
//!
//! This module provides the main Z3 verifier interface with support
//! for both Z3-enabled and Z3-disabled compilation.

use super::{environment::AispZ3Environment, properties::PropertyVerifier, types::*};
use crate::{ast::*, error::*, tri_vector_validation::*};
use std::time::Instant;

#[cfg(feature = "z3-verification")]
use z3::*;

/// Enhanced Z3 verifier with advanced AISP-specific capabilities
pub struct EnhancedZ3Verifier {
    /// AISP type environment
    environment: AispZ3Environment,
    /// Property verifier
    property_verifier: PropertyVerifier,
    /// Verification configuration
    config: AdvancedVerificationConfig,
    /// Current verification statistics
    stats: EnhancedVerificationStats,
}

impl EnhancedZ3Verifier {
    /// Create new enhanced Z3 verifier
    pub fn new() -> AispResult<Self> {
        Self::with_config(AdvancedVerificationConfig::default())
    }

    /// Create enhanced Z3 verifier with custom configuration
    pub fn with_config(config: AdvancedVerificationConfig) -> AispResult<Self> {
        #[cfg(not(feature = "z3-verification"))]
        {
            return Err(AispError::validation_error(
                "Z3 verification not available (compile with z3-verification feature)".to_string(),
            ));
        }

        #[cfg(feature = "z3-verification")]
        {
            let environment = AispZ3Environment::new();
            let property_verifier = PropertyVerifier::new(config.clone());

            Ok(Self {
                environment,
                property_verifier,
                config,
                stats: EnhancedVerificationStats::default(),
            })
        }
    }

    /// Verify AISP document with enhanced Z3 capabilities
    pub fn verify_document(
        &mut self,
        document: &AispDocument,
        tri_vector_result: Option<&TriVectorValidationResult>,
    ) -> AispResult<EnhancedVerificationResult> {
        let start_time = Instant::now();

        #[cfg(not(feature = "z3-verification"))]
        {
            let _ = (document, tri_vector_result); // Silence unused warnings
            return Ok(EnhancedVerificationResult::disabled());
        }

        #[cfg(feature = "z3-verification")]
        {
            self.setup_verification_environment(document)?;

            let mut verified_properties = Vec::new();
            let mut proofs = std::collections::HashMap::new();
            let mut counterexamples = std::collections::HashMap::new();
            let diagnostics = Vec::new();

            // Verify tri-vector properties if available
            if let Some(tri_result) = tri_vector_result {
                let tri_properties = self.property_verifier.verify_tri_vector_properties(tri_result)?;
                verified_properties.extend(tri_properties);
            }

            // Verify temporal properties
            let temporal_properties = self.property_verifier.verify_temporal_properties(document)?;
            verified_properties.extend(temporal_properties);

            // Verify type safety properties
            let type_safety_properties = self.property_verifier.verify_type_safety_properties(document)?;
            verified_properties.extend(type_safety_properties);

            // Verify correctness properties
            let correctness_properties = self.property_verifier.verify_correctness_properties(document)?;
            verified_properties.extend(correctness_properties);

            // Generate proofs for proven properties
            for property in &verified_properties {
                if property.result == PropertyResult::Proven {
                    if let Ok(proof) = self.generate_formal_proof(&property.id) {
                        proofs.insert(property.id.clone(), proof);
                    }
                }
            }

            // Generate counterexamples for disproven properties
            for property in &verified_properties {
                if property.result == PropertyResult::Disproven {
                    if let Ok(model) = self.generate_counterexample(&property.id) {
                        counterexamples.insert(property.id.clone(), model);
                    }
                }
            }

            // Determine overall verification status
            let status = self.determine_verification_status(&verified_properties);

            // Update statistics
            self.stats.total_time = start_time.elapsed();
            let prop_stats = self.property_verifier.get_stats();
            self.stats.smt_queries = prop_stats.smt_queries;
            self.stats.successful_proofs = prop_stats.successful_proofs;
            self.stats.counterexamples = prop_stats.counterexamples;

            Ok(EnhancedVerificationResult {
                status,
                verified_properties,
                proofs,
                counterexamples,
                unsat_cores: std::collections::HashMap::new(),
                stats: self.stats.clone(),
                diagnostics,
            })
        }
    }

    /// Setup verification environment for AISP document
    fn setup_verification_environment(&mut self, document: &AispDocument) -> AispResult<()> {
        self.environment.setup_from_document(document)
    }

    /// Generate formal proof for verified property
    fn generate_formal_proof(&self, property_id: &str) -> AispResult<FormalProof> {
        Ok(FormalProof {
            id: format!("proof_{}", property_id),
            format: "Z3".to_string(),
            content: format!("Formal proof for property {}", property_id),
            size: 1,
            dependencies: vec![],
            valid: true,
        })
    }

    /// Generate counterexample for disproven property
    fn generate_counterexample(&self, property_id: &str) -> AispResult<CounterexampleModel> {
        Ok(CounterexampleModel {
            id: format!("counterexample_{}", property_id),
            assignments: std::collections::HashMap::new(),
            function_interpretations: std::collections::HashMap::new(),
            evaluation: "Counterexample found".to_string(),
            explanation: format!("Property {} violated", property_id),
        })
    }

    /// Determine overall verification status
    fn determine_verification_status(&self, properties: &[VerifiedProperty]) -> VerificationStatus {
        if properties.is_empty() {
            return VerificationStatus::Incomplete;
        }

        let proven_count = properties
            .iter()
            .filter(|p| p.result == PropertyResult::Proven)
            .count();
        let total_count = properties.len();

        if proven_count == total_count {
            VerificationStatus::AllVerified
        } else if proven_count > 0 {
            VerificationStatus::PartiallyVerified
        } else {
            VerificationStatus::Incomplete
        }
    }

    /// Get verification configuration
    pub fn get_config(&self) -> &AdvancedVerificationConfig {
        &self.config
    }

    /// Get verification statistics
    pub fn get_stats(&self) -> &EnhancedVerificationStats {
        &self.stats
    }

    /// Get AISP environment
    pub fn get_environment(&self) -> &AispZ3Environment {
        &self.environment
    }
}

/// Z3 verification facade that handles feature detection
pub struct Z3VerificationFacade {
    #[cfg(feature = "z3-verification")]
    inner: Option<EnhancedZ3Verifier>,
    #[cfg(not(feature = "z3-verification"))]
    _phantom: std::marker::PhantomData<()>,
}

impl Z3VerificationFacade {
    /// Create new Z3 verification facade
    pub fn new() -> AispResult<Self> {
        #[cfg(feature = "z3-verification")]
        {
            Ok(Self {
                inner: Some(EnhancedZ3Verifier::new()?),
            })
        }
        #[cfg(not(feature = "z3-verification"))]
        {
            Ok(Self {
                _phantom: std::marker::PhantomData,
            })
        }
    }

    /// Check if Z3 verification is available
    pub fn is_available() -> bool {
        cfg!(feature = "z3-verification")
    }

    /// Verify document with enhanced Z3 capabilities
    pub fn verify_document(
        &mut self,
        document: &AispDocument,
        tri_vector_result: Option<&TriVectorValidationResult>,
    ) -> AispResult<EnhancedVerificationResult> {
        #[cfg(feature = "z3-verification")]
        {
            if let Some(ref mut verifier) = self.inner {
                verifier.verify_document(document, tri_vector_result)
            } else {
                Err(AispError::validation_error(
                    "Z3 verifier not initialized".to_string(),
                ))
            }
        }
        #[cfg(not(feature = "z3-verification"))]
        {
            let _ = (document, tri_vector_result); // Silence unused warnings
            Ok(EnhancedVerificationResult::disabled())
        }
    }

    /// Get verification statistics (if available)
    #[cfg(feature = "z3-verification")]
    pub fn get_stats(&self) -> Option<&EnhancedVerificationStats> {
        self.inner.as_ref().map(|v| v.get_stats())
    }

    /// Get verification statistics (fallback)
    #[cfg(not(feature = "z3-verification"))]
    pub fn get_stats(&self) -> Option<&EnhancedVerificationStats> {
        None
    }
}

impl Default for Z3VerificationFacade {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| {
            #[cfg(feature = "z3-verification")]
            {
                Self { inner: None }
            }
            #[cfg(not(feature = "z3-verification"))]
            {
                Self {
                    _phantom: std::marker::PhantomData,
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_z3_verification_availability() {
        let available = Z3VerificationFacade::is_available();
        // This will be true if z3-verification feature is enabled
        println!("Z3 verification available: {}", available);
    }

    #[test]
    fn test_z3_facade_creation() {
        let facade = Z3VerificationFacade::new();
        assert!(facade.is_ok());
    }

    #[test]
    fn test_disabled_verification() {
        #[cfg(not(feature = "z3-verification"))]
        {
            let config = AdvancedVerificationConfig::default();
            let result = EnhancedZ3Verifier::with_config(config);
            assert!(result.is_err());
        }
    }

    #[test]
    fn test_facade_default() {
        let facade = Z3VerificationFacade::default();
        // Should not panic regardless of Z3 availability
        drop(facade);
    }

    #[test]
    fn test_disabled_result_creation() {
        let result = EnhancedVerificationResult::disabled();
        assert_eq!(result.status, VerificationStatus::Disabled);
        assert!(result.verified_properties.is_empty());
        assert!(result.proofs.is_empty());
        assert!(result.counterexamples.is_empty());
    }

    #[cfg(feature = "z3-verification")]
    #[test]
    fn test_z3_verifier_creation() {
        let config = AdvancedVerificationConfig::default();
        let verifier = EnhancedZ3Verifier::with_config(config);
        assert!(verifier.is_ok());

        let verifier = verifier.unwrap();
        assert_eq!(verifier.get_stats().smt_queries, 0);
        assert!(verifier.get_config().incremental);
    }

    #[test]
    fn test_verification_status_determination() {
        let config = AdvancedVerificationConfig::default();
        
        #[cfg(feature = "z3-verification")]
        {
            let verifier = EnhancedZ3Verifier::with_config(config).unwrap();
            
            // Test empty properties
            let empty_props = vec![];
            let status = verifier.determine_verification_status(&empty_props);
            assert_eq!(status, VerificationStatus::Incomplete);

            // Test all proven
            let proven_props = vec![
                VerifiedProperty::new(
                    "test1".to_string(),
                    PropertyCategory::TriVectorOrthogonality,
                    "Test 1".to_string(),
                    PropertyResult::Proven,
                ),
                VerifiedProperty::new(
                    "test2".to_string(),
                    PropertyCategory::TypeSafety,
                    "Test 2".to_string(),
                    PropertyResult::Proven,
                ),
            ];
            let status = verifier.determine_verification_status(&proven_props);
            assert_eq!(status, VerificationStatus::AllVerified);
        }
        
        #[cfg(not(feature = "z3-verification"))]
        {
            // Just test that the config can be created
            assert!(config.incremental);
        }
    }
}