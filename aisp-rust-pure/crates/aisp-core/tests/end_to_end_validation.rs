//! End-to-end validation pipeline integration tests
//!
//! This module tests the complete validation pipeline from document parsing
//! through all analysis levels including formal verification.

use aisp_core::{
    AispValidator, ValidationConfig, ValidationResult, QualityTier
};
use std::collections::HashMap;

/// Builder for creating test AISP documents
pub struct TestDocumentBuilder {
    header: String,
    metadata: Vec<String>,
    blocks: HashMap<String, String>,
}

impl TestDocumentBuilder {
    pub fn new() -> Self {
        Self {
            header: "𝔸5.1.TestDoc@2026-01-25".to_string(),
            metadata: vec![],
            blocks: HashMap::new(),
        }
    }

    pub fn with_header(mut self, header: &str) -> Self {
        self.header = header.to_string();
        self
    }

    pub fn with_metadata(mut self, metadata: &str) -> Self {
        self.metadata.push(metadata.to_string());
        self
    }

    pub fn with_meta_block(mut self, content: &str) -> Self {
        self.blocks.insert("meta".to_string(), content.to_string());
        self
    }

    pub fn with_types_block(mut self, content: &str) -> Self {
        self.blocks.insert("types".to_string(), content.to_string());
        self
    }

    pub fn with_rules_block(mut self, content: &str) -> Self {
        self.blocks.insert("rules".to_string(), content.to_string());
        self
    }

    pub fn with_functions_block(mut self, content: &str) -> Self {
        self.blocks.insert("functions".to_string(), content.to_string());
        self
    }

    pub fn with_evidence_block(mut self, content: &str) -> Self {
        self.blocks.insert("evidence".to_string(), content.to_string());
        self
    }

    pub fn build(self) -> String {
        let mut document = self.header;
        
        if !self.metadata.is_empty() {
            document.push('\n');
            for metadata in &self.metadata {
                document.push('\n');
                document.push_str(metadata);
            }
        }

        for (block_type, content) in &self.blocks {
            document.push('\n');
            document.push('\n');
            match block_type.as_str() {
                "meta" => document.push_str("⟦Ω:Meta⟧{"),
                "types" => document.push_str("⟦Σ:Types⟧{"),
                "rules" => document.push_str("⟦Γ:Rules⟧{"),
                "functions" => document.push_str("⟦Λ:Funcs⟧{"),
                "evidence" => document.push_str("⟦Ε⟧"),
                _ => continue,
            }
            
            if block_type != "evidence" {
                document.push('\n');
                document.push_str("  ");
                document.push_str(content);
                document.push('\n');
                document.push('}');
            } else {
                document.push_str(content);
            }
        }

        document
    }
}

/// Helper for asserting validation results
pub struct ValidationAssertion {
    result: ValidationResult,
}

impl ValidationAssertion {
    pub fn new(result: ValidationResult) -> Self {
        Self { result }
    }

    pub fn is_valid(self) -> Self {
        assert!(self.result.valid, "Document should be valid but got error: {:?}", self.result.error);
        self
    }

    pub fn is_invalid(self) -> Self {
        assert!(!self.result.valid, "Document should be invalid but was valid");
        self
    }

    pub fn has_quality_tier(self, expected: QualityTier) -> Self {
        assert_eq!(self.result.tier, expected, "Expected quality tier {:?} but got {:?}", expected, self.result.tier);
        self
    }

    pub fn has_error_count(self, expected: usize) -> Self {
        let actual_errors = if self.result.error.is_some() { 1 } else { 0 };
        assert_eq!(actual_errors, expected, "Expected {} errors but got {}: {:?}", expected, actual_errors, self.result.error);
        self
    }

    pub fn has_warning_count(self, expected: usize) -> Self {
        assert_eq!(self.result.warnings.len(), expected, "Expected {} warnings but got {}: {:?}", expected, self.result.warnings.len(), self.result.warnings);
        self
    }

    pub fn has_delta_above(self, threshold: f64) -> Self {
        assert!(self.result.delta >= threshold, "Expected delta >= {} but got {}", threshold, self.result.delta);
        self
    }

    pub fn has_delta_below(self, threshold: f64) -> Self {
        assert!(self.result.delta <= threshold, "Expected delta <= {} but got {}", threshold, self.result.delta);
        self
    }
}

#[test]
fn test_minimal_valid_document() {
    let document = TestDocumentBuilder::new()
        .with_meta_block("domain≜test\nversion≜\"1.0.0\"")
        .with_evidence_block("⟨δ≜0.8⟩")
        .build();

    let validator = AispValidator::new();
    let result = validator.validate(&document);

    ValidationAssertion::new(result)
        .is_valid()
        .has_quality_tier(QualityTier::Silver)
        .has_error_count(0)
        .has_delta_above(0.7);
}

#[test]
fn test_complete_platinum_document() {
    let document = TestDocumentBuilder::new()
        .with_header("𝔸5.1.GameLogic@2026-01-25")
        .with_metadata("γ≔⟨game,turn-based⟩")
        .with_metadata("ρ≔⟨protocol,state-transition⟩")
        .with_meta_block("domain≜game_logic\nversion≜\"1.0.0\"\ndescription≜\"Turn-based game state management\"\n∀D∈AISP:Ambig(D)<0.02")
        .with_types_block("GameState≜{Start,Playing,GameOver}\nPlayer≜{PlayerA,PlayerB}\nMove≜ℕ\nScore≜ℕ")
        .with_rules_block("∀s:GameState→Valid(s)\n∀p:Player→HasTurn(p)⇒CanMove(p)\n∀m:Move→ValidMove(m)⇒UpdateState(m)\n□(Playing→◊GameOver)")
        .with_functions_block("nextState≜λ(s,m).TransitionTo(s,m)\nisValidMove≜λm.ValidMove(m)\ncalculateScore≜λ(p,moves).Σ(moves)")
        .with_evidence_block("⟨δ≜0.85;φ≜100;τ≜◊⁺⟩")
        .build();

    let validator = AispValidator::new();
    let result = validator.validate_document(&document, ValidationLevel::Temporal).unwrap();

    ValidationAssertion::new(result)
        .is_valid()
        .has_quality_tier(QualityTier::Platinum)
        .has_error_count(0)
        .has_delta_above(0.8);
}

#[test]
fn test_document_with_syntax_errors() {
    let document = TestDocumentBuilder::new()
        .with_meta_block("domain≜test\ninvalid_syntax_here!!!")
        .with_evidence_block("⟨δ≜invalid⟩")
        .build();

    let validator = AispValidator::new();
    let result = validator.validate_document(&document, ValidationLevel::Syntax).unwrap();

    ValidationAssertion::new(result)
        .is_invalid()
        .has_error_count(1); // At least one syntax error
}

#[test]
fn test_document_with_semantic_errors() {
    let document = TestDocumentBuilder::new()
        .with_meta_block("domain≜test\nversion≜\"1.0.0\"")
        .with_types_block("State≜{A,B,C}\nTransition≜UndefinedType")
        .with_evidence_block("⟨δ≜0.8⟩")
        .build();

    let validator = AispValidator::new();
    let result = validator.validate_document(&document, ValidationLevel::Semantic).unwrap();

    ValidationAssertion::new(result)
        .is_invalid()
        .has_error_count(1); // Undefined type error
}

#[test]
fn test_relational_analysis_level() {
    let document = TestDocumentBuilder::new()
        .with_meta_block("domain≜test\nversion≜\"1.0.0\"")
        .with_types_block("State≜{A,B,C}\nTransition≜State→State")
        .with_rules_block("∀s:State→Valid(s)")
        .with_evidence_block("⟨δ≜0.8⟩")
        .build();

    let validator = AispValidator::new();
    let result = validator.validate_document(&document, ValidationLevel::Relational).unwrap();

    ValidationAssertion::new(result)
        .is_valid()
        .has_quality_tier(QualityTier::Gold)
        .has_error_count(0);
}

#[test]
fn test_temporal_analysis_level() {
    let document = TestDocumentBuilder::new()
        .with_meta_block("domain≜test\nversion≜\"1.0.0\"")
        .with_types_block("State≜{A,B,C}")
        .with_rules_block("∀s:State→Valid(s)\n□(A→◊B)")
        .with_evidence_block("⟨δ≜0.85;τ≜◊⁺⟩")
        .build();

    let validator = AispValidator::new();
    let result = validator.validate_document(&document, ValidationLevel::Temporal).unwrap();

    ValidationAssertion::new(result)
        .is_valid()
        .has_quality_tier(QualityTier::Platinum)
        .has_error_count(0);
}

#[test]
fn test_formal_verification_level() {
    let document = TestDocumentBuilder::new()
        .with_meta_block("domain≜test\nversion≜\"1.0.0\"\n∀D∈AISP:Ambig(D)<0.02")
        .with_types_block("State≜{A,B,C}")
        .with_rules_block("∀s:State→Valid(s)\n□(A→◊B)")
        .with_functions_block("next≜λs.Next(s)")
        .with_evidence_block("⟨δ≜0.9;φ≜100;τ≜◊⁺⟩")
        .build();

    let validator = AispValidator::new();
    let result = validator.validate_document(&document, ValidationLevel::Formal).unwrap();

    ValidationAssertion::new(result)
        .is_valid()
        .has_quality_tier(QualityTier::Platinum)
        .has_error_count(0)
        .has_delta_above(0.8);
}

#[test]
fn test_validation_level_progression() {
    let document = TestDocumentBuilder::new()
        .with_meta_block("domain≜test\nversion≜\"1.0.0\"")
        .with_types_block("State≜{A,B,C}")
        .with_evidence_block("⟨δ≜0.8⟩")
        .build();

    let validator = AispValidator::new();

    // Test each validation level in progression
    let syntax_result = validator.validate_document(&document, ValidationLevel::Syntax).unwrap();
    ValidationAssertion::new(syntax_result).is_valid().has_error_count(0);

    let semantic_result = validator.validate_document(&document, ValidationLevel::Semantic).unwrap();
    ValidationAssertion::new(semantic_result).is_valid().has_error_count(0);

    let relational_result = validator.validate_document(&document, ValidationLevel::Relational).unwrap();
    ValidationAssertion::new(relational_result).is_valid().has_error_count(0);

    let temporal_result = validator.validate_document(&document, ValidationLevel::Temporal).unwrap();
    ValidationAssertion::new(temporal_result).is_valid().has_error_count(0);

    let formal_result = validator.validate_document(&document, ValidationLevel::Formal).unwrap();
    ValidationAssertion::new(formal_result).is_valid().has_error_count(0);
}

#[test]
fn test_validation_config_options() {
    let document = TestDocumentBuilder::new()
        .with_meta_block("domain≜test\nversion≜\"1.0.0\"")
        .with_evidence_block("⟨δ≜0.8⟩")
        .build();

    let mut config = ValidationConfig::default();
    config.strict_mode = true;
    config.timing_enabled = true;
    config.max_document_size = Some(1000);

    let validator = AispValidator::with_config(config);
    let result = validator.validate_document(&document, ValidationLevel::Semantic).unwrap();

    ValidationAssertion::new(result)
        .is_valid()
        .has_error_count(0);
}