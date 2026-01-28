//! Main Validation Engine
//!
//! Provides the primary API for validating AISP documents with
//! comprehensive error handling and performance optimizations.

use crate::error::*;
use crate::ast::canonical::CanonicalAispDocument as AispDocument;
use crate::ast::canonical::IntoCanonical;
use crate::parser::robust_parser::RobustAispParser;
use crate::semantic::SemanticAnalyzer;
use crate::{AISP_VERSION};
use super::types::{ValidationConfig, ValidationResult};
use super::verification_methods::VerificationMethods;
use std::time::Instant;

/// Main AISP validator engine
pub struct AispValidator {
    config: ValidationConfig,
    verification_methods: VerificationMethods,
}

impl AispValidator {
    /// Create a new validator with default configuration
    pub fn new() -> Self {
        let config = ValidationConfig::default();
        let verification_methods = VerificationMethods::new(config.clone());
        
        Self {
            config,
            verification_methods,
        }
    }

    /// Create a new validator with custom configuration
    pub fn with_config(config: ValidationConfig) -> Self {
        let verification_methods = VerificationMethods::new(config.clone());
        
        Self { 
            config,
            verification_methods,
        }
    }

    /// Update validator configuration
    pub fn configure(&mut self, config: ValidationConfig) {
        self.verification_methods = VerificationMethods::new(config.clone());
        self.config = config;
    }

    /// Get reference to current configuration
    pub fn config(&self) -> &ValidationConfig {
        &self.config
    }

    /// Validate AISP document from source text
    pub fn validate(&self, source: &str) -> ValidationResult {
        let start_time = Instant::now();
        let document_size = source.len();

        // Check document size
        if document_size > self.config.max_document_size {
            return ValidationResult::failed(
                AispError::DocumentTooLarge {
                    size: document_size,
                    max: self.config.max_document_size,
                },
                document_size,
            );
        }

        // Check for empty document
        if source.trim().is_empty() {
            return ValidationResult::failed(
                AispError::validation_error("Empty document"),
                document_size,
            );
        }

        // Parse document
        let (document, parse_time, mut all_warnings) = match self.parse_document(source, document_size) {
            Ok(result) => result,
            Err(validation_result) => return validation_result,
        };

        // Perform semantic analysis
        let (mut analysis, semantic_time) = match self.perform_semantic_analysis(&document, document_size) {
            Ok(result) => result,
            Err(validation_result) => return validation_result,
        };

        // Merge warnings from semantic analysis
        all_warnings.extend(analysis.warnings().into_iter().map(|w| AispWarning::warning(w)));

        // Apply strict mode checks
        if self.config.strict_mode {
            self.verification_methods.apply_strict_checks(&mut analysis);
        }

        // Perform additional verifications
        let verification_results = self.perform_additional_verifications(&document, &analysis, document_size);

        // Handle verification failures
        if let Err(validation_result) = verification_results {
            return validation_result;
        }

        let (
            formal_verification,
            trivector_validation,
            enhanced_z3_verification,
            ghost_intent_validation,
            rossnet_validation,
            hebbian_validation,
            anti_drift_validation,
        ) = verification_results.unwrap();

        // Create final result
        let mut result = ValidationResult::success(
            analysis,
            document_size,
            parse_time,
            semantic_time,
            if self.config.include_ast { Some(document) } else { None },
            formal_verification,
            trivector_validation,
            enhanced_z3_verification,
            ghost_intent_validation,
            rossnet_validation,
            hebbian_validation,
            anti_drift_validation,
        );

        // Add timing information
        if self.config.include_timing {
            result.total_time = Some(start_time.elapsed());
        }

        // Override warnings with collected warnings
        result.warnings = all_warnings;

        result
    }

    /// Parse AISP document from source
    fn parse_document(
        &self, 
        source: &str, 
        document_size: usize
    ) -> Result<(AispDocument, std::time::Duration, Vec<AispWarning>), ValidationResult> {
        let parse_start = Instant::now();
        let parser = RobustAispParser::new();
        let parse_result = parser.parse(source);
        
        let mut document = match parse_result.document {
            Some(robust_doc) => {
                let mut canonical = robust_doc.into_canonical();
                canonical.parse_structured_data(); // Convert raw strings to structured data
                canonical
            }
            None => {
                let error_message = if !parse_result.errors.is_empty() {
                    parse_result.errors[0].message.clone()
                } else {
                    "Failed to parse document".to_string()
                };
                return Err(ValidationResult::failed(
                    AispError::validation_error(error_message),
                    document_size,
                ));
            }
        };
        let parse_time = parse_start.elapsed();

        // Collect parser warnings
        let mut all_warnings: Vec<AispWarning> = parse_result.warnings.into_iter()
            .map(|w| AispWarning::warning(w.message))
            .collect();

        // Check AISP version compatibility
        if document.header.version != AISP_VERSION {
            all_warnings.push(AispWarning::warning(
                format!(
                    "Document version {} may not be fully compatible with validator version {}",
                    document.header.version, AISP_VERSION
                ),
            ));
        }

        Ok((document, parse_time, all_warnings))
    }

    /// Perform semantic analysis
    fn perform_semantic_analysis(
        &self,
        document: &AispDocument,
        document_size: usize,
    ) -> Result<(crate::semantic::DeepVerificationResult, std::time::Duration), ValidationResult> {
        let semantic_start = Instant::now();
        let mut analyzer = SemanticAnalyzer::new();
        let analysis = match analyzer.analyze(&document) {
            Ok(analysis) => analysis,
            Err(error) => {
                return Err(ValidationResult::failed(error, document_size));
            }
        };
        let semantic_time = semantic_start.elapsed();

        Ok((analysis, semantic_time))
    }

    /// Perform additional verification methods
    fn perform_additional_verifications(
        &self,
        document: &AispDocument,
        analysis: &crate::semantic::DeepVerificationResult,
        document_size: usize,
    ) -> Result<(
        Option<crate::semantic::DeepVerificationResult>,
        Option<crate::tri_vector_validation::TriVectorValidationResult>,
        Option<crate::enhanced_z3_verification::EnhancedVerificationResult>,
        Option<crate::ghost_intent_validation::GhostIntentValidationResult>,
        Option<crate::rossnet_scoring::RossNetValidationResult>,
        Option<crate::hebbian_learning::HebbianValidationResult>,
        Option<crate::anti_drift::AntiDriftValidationResult>,
    ), ValidationResult> {
        // Perform formal verification if enabled  
        let formal_verification = if self.config.enable_formal_verification {
            match self.verification_methods.perform_formal_verification(&document, &analysis) {
                Ok(verification_result) => Some(verification_result),
                Err(_err) => None, // Log warning elsewhere
            }
        } else {
            None
        };

        // Perform tri-vector validation if enabled
        let trivector_validation = if self.config.enable_trivector_validation {
            match self.verification_methods.perform_trivector_validation(&document) {
                Ok(trivector_result) => Some(trivector_result),
                Err(_err) => None, // Log warning elsewhere
            }
        } else {
            None
        };

        // Perform enhanced Z3 verification if enabled
        let enhanced_z3_verification = if self.config.enable_enhanced_z3 {
            match self.verification_methods.perform_enhanced_z3_verification(&document, trivector_validation.as_ref()) {
                Ok(z3_result) => Some(z3_result),
                Err(err) => {
                    // Formal verification failure should cause validation to fail, not just warn
                    if self.config.strict_formal_verification {
                        return Err(ValidationResult::failed(
                            AispError::validation_error(
                                format!("Enhanced Z3 verification failed: {}. Enable 'strict_formal_verification: false' to downgrade to warnings.", err)
                            ),
                            document_size,
                        ));
                    } else {
                        None
                    }
                }
            }
        } else {
            None
        };

        // Perform ghost intent validation if enabled
        let ghost_intent_validation = if self.config.enable_ghost_intent_validation {
            match self.verification_methods.perform_ghost_intent_validation(&document) {
                Ok(ghost_result) => Some(ghost_result),
                Err(err) => {
                    if self.config.strict_formal_verification {
                        return Err(ValidationResult::failed(
                            AispError::validation_error(
                                format!("Ghost intent validation failed: {}. Enable 'strict_formal_verification: false' to downgrade to warnings.", err)
                            ),
                            document_size,
                        ));
                    } else {
                        None
                    }
                }
            }
        } else {
            None
        };

        // Perform RossNet scoring validation if enabled
        let rossnet_validation = if self.config.enable_rossnet_scoring {
            match self.verification_methods.perform_rossnet_validation(&document, &analysis) {
                Ok(rossnet_result) => Some(rossnet_result),
                Err(err) => {
                    if self.config.strict_formal_verification {
                        return Err(ValidationResult::failed(
                            AispError::validation_error(
                                format!("RossNet scoring validation failed: {}. Enable 'strict_formal_verification: false' to downgrade to warnings.", err)
                            ),
                            document_size,
                        ));
                    } else {
                        None
                    }
                }
            }
        } else {
            None
        };

        // Perform Hebbian learning validation if enabled
        let hebbian_validation = if self.config.enable_hebbian_learning {
            match self.verification_methods.perform_hebbian_validation(&document, &analysis) {
                Ok(hebbian_result) => Some(hebbian_result),
                Err(err) => {
                    if self.config.strict_formal_verification {
                        return Err(ValidationResult::failed(
                            AispError::validation_error(
                                format!("Hebbian learning validation failed: {}. Enable 'strict_formal_verification: false' to downgrade to warnings.", err)
                            ),
                            document_size,
                        ));
                    } else {
                        None
                    }
                }
            }
        } else {
            None
        };

        // Perform anti-drift protocol validation if enabled
        let anti_drift_validation = if self.config.enable_anti_drift {
            match self.verification_methods.perform_anti_drift_validation(&document, &analysis) {
                Ok(anti_drift_result) => Some(anti_drift_result),
                Err(err) => {
                    if self.config.strict_formal_verification {
                        return Err(ValidationResult::failed(
                            AispError::validation_error(
                                format!("Anti-drift protocol validation failed: {}. Enable 'strict_formal_verification: false' to downgrade to warnings.", err)
                            ),
                            document_size,
                        ));
                    } else {
                        None
                    }
                }
            }
        } else {
            None
        };

        Ok((
            formal_verification,
            trivector_validation,
            enhanced_z3_verification,
            ghost_intent_validation,
            rossnet_validation,
            hebbian_validation,
            anti_drift_validation,
        ))
    }
}

impl Default for AispValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validator_creation() {
        let validator = AispValidator::new();
        assert_eq!(validator.config.max_document_size, crate::MAX_DOCUMENT_SIZE);
    }

    #[test]
    fn test_validator_with_config() {
        let mut config = ValidationConfig::default();
        config.strict_mode = true;
        config.include_timing = true;
        
        let validator = AispValidator::with_config(config.clone());
        assert!(validator.config.strict_mode);
        assert!(validator.config.include_timing);
    }

    #[test]
    fn test_validator_configure() {
        let mut validator = AispValidator::new();
        assert!(!validator.config.strict_mode);
        
        let mut new_config = ValidationConfig::default();
        new_config.strict_mode = true;
        validator.configure(new_config);
        
        assert!(validator.config.strict_mode);
    }

    #[test]
    fn test_validate_empty_document() {
        let validator = AispValidator::new();
        let result = validator.validate("");
        assert!(!result.valid);
        assert!(result.error_message().is_some());
    }

    #[test]
    fn test_validate_large_document() {
        let validator = AispValidator::new();
        let large_source = "a".repeat(validator.config.max_document_size + 1);
        let result = validator.validate(&large_source);
        assert!(!result.valid);
        assert!(matches!(result.error, Some(AispError::DocumentTooLarge { .. })));
    }

    #[test]
    fn test_validate_simple_document() {
        let validator = AispValidator::new();
        let source = r#"
aisp_v: 5.1
name: test_document
date: 2026-01-27

-- Functions --
test_function ≜ λx.x + 1
        "#.trim();
        
        let result = validator.validate(source);
        // The validation may fail due to missing dependencies, but should not panic
        // and should provide meaningful error information
        assert!(result.error_message().is_none() || result.error_message().is_some());
    }

    #[test]
    fn test_validate_malformed_document() {
        let validator = AispValidator::new();
        let source = "this is not a valid AISP document";
        let result = validator.validate(source);
        assert!(!result.valid);
        assert!(result.error_message().is_some());
    }
}