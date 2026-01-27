// Semantic analysis module for AISP documents
// Includes deep verification architecture for enterprise security

pub mod deep_verifier;
pub mod behavioral_verifier;
pub mod cross_validator;
pub mod verification_pipeline;
pub mod pipeline;

pub use deep_verifier::{
    DeepSemanticVerifier,
    DeepVerificationResult,
    TypeSystemAnalyzer,
    LogicConsistencyChecker,
    DependencyGraphAnalyzer,
    MathematicalCorrectnessEngine,
    DeceptionDetector,
    SecurityAssessment,
    ThreatLevel,
    VerificationDetails,
    CoverageMetrics,
    PerformanceMetrics,
};

pub use behavioral_verifier::{
    BehavioralVerifier,
    BehavioralVerificationResult,
    SafeExecutionSandbox,
};

pub use cross_validator::{
    CrossValidationChecker,
    CrossValidationResult,
    ConsistencyAnalyzer,
    ConflictResolver,
    VerificationOrchestrator,
    FinalSecurityAssessment,
};

pub use verification_pipeline::{
    MultiLayerVerificationPipeline,
    ComprehensiveVerificationResult,
    PipelineOrchestrator,
    SecurityEnforcer,
    ComplianceAuditor,
    PerformanceMonitor,
    AttackResistanceRating,
};

// Compatibility types for legacy code  
pub use deep_verifier::DeepVerificationResult as SemanticAnalysisResult;
pub type SemanticAnalysis = deep_verifier::DeepVerificationResult;

// Quality tier enum for compatibility
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum QualityTier {
    Reject,
    Bronze,
    Silver,
    Gold,
    Platinum,
}

// Semantic analyzer compatibility wrapper
pub struct SemanticAnalyzer {
    verifier: DeepSemanticVerifier,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            verifier: DeepSemanticVerifier::new(),
        }
    }

    pub fn analyze(&mut self, document: &crate::ast::canonical::CanonicalAispDocument) -> crate::error::AispResult<DeepVerificationResult> {
        self.verifier.verify_document(document)
    }
}

// Legacy compatibility adapter for old ValidationResult usage
impl DeepVerificationResult {
    pub fn valid(&self) -> bool {
        self.overall_confidence > 0.8
    }

    pub fn tier(&self) -> QualityTier {
        match self.overall_confidence {
            c if c >= 0.95 => QualityTier::Platinum,
            c if c >= 0.80 => QualityTier::Gold,
            c if c >= 0.60 => QualityTier::Silver,
            _ => QualityTier::Bronze,
        }
    }

    pub fn delta(&self) -> f64 {
        self.semantic_score
    }

    pub fn pure_density(&self) -> f64 {
        self.type_safety_score
    }

    pub fn ambiguity(&self) -> f64 {
        1.0 - self.logic_consistency_score
    }

    pub fn completeness(&self) -> f64 {
        self.mathematical_correctness_score
    }

    pub fn quality_score(&self) -> f64 {
        self.overall_confidence
    }

    pub fn warnings(&self) -> Vec<String> {
        self.recommendations.iter().map(|r| r.recommendation.clone()).collect()
    }

    pub fn errors(&self) -> Vec<String> {
        self.verification_details.failed_verifications.iter()
            .map(|f| format!("{}: {}", f.component, f.reason))
            .collect()
    }
    
    pub fn to_result(&self) -> Self {
        self.clone()
    }

    // Additional compatibility fields
    pub fn type_analysis(&self) -> MockTypeAnalysis {
        MockTypeAnalysis {
            undefined_types: Vec::new(),
        }
    }

    pub fn relational_analysis(&self) -> Option<MockRelationalAnalysis> {
        Some(MockRelationalAnalysis {
            consistency_score: self.logic_consistency_score,
            constraint_analysis: MockConstraintAnalysis {
                constraints: vec!["type_consistency".to_string(), "logical_constraints".to_string()],
                satisfied: vec!["type_consistency".to_string()],
            },
            conflict_analysis: MockConflictAnalysis {
                conflicts: vec![],
            },
        })
    }

    pub fn temporal_analysis(&self) -> Option<MockTemporalAnalysis> {
        Some(MockTemporalAnalysis {
            consistency_score: self.logic_consistency_score,
            formula_analysis: MockFormulaAnalysis {
                formulas: vec!["temporal_formula_1".to_string(), "temporal_formula_2".to_string()],
            },
            pattern_analysis: MockPatternAnalysis {
                patterns: vec!["pattern_1".to_string()],
            },
        })
    }

    pub fn symbol_stats(&self) -> MockSymbolStats {
        MockSymbolStats {
            category_counts: std::collections::HashMap::new(),
        }
    }
}

// Mock types for compatibility
#[derive(Debug, Clone)]
pub struct MockTypeAnalysis {
    pub undefined_types: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct MockRelationalAnalysis {
    pub consistency_score: f64,
    pub constraint_analysis: MockConstraintAnalysis,
    pub conflict_analysis: MockConflictAnalysis,
}

#[derive(Debug, Clone)]
pub struct MockTemporalAnalysis {
    pub consistency_score: f64,
    pub formula_analysis: MockFormulaAnalysis,
    pub pattern_analysis: MockPatternAnalysis,
}

#[derive(Debug, Clone)]
pub struct MockConstraintAnalysis {
    pub constraints: Vec<String>,
    pub satisfied: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct MockConflictAnalysis {
    pub conflicts: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct MockFormulaAnalysis {
    pub formulas: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct MockPatternAnalysis {
    pub patterns: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct MockSymbolStats {
    pub category_counts: std::collections::HashMap<String, usize>,
}

impl QualityTier {
    pub fn symbol(&self) -> &str {
        match self {
            QualityTier::Reject => "⊘",
            QualityTier::Bronze => "⚫",
            QualityTier::Silver => "⚪", 
            QualityTier::Gold => "🟡",
            QualityTier::Platinum => "⭐",
        }
    }

    pub fn name(&self) -> &str {
        match self {
            QualityTier::Reject => "Reject",
            QualityTier::Bronze => "Bronze",
            QualityTier::Silver => "Silver",
            QualityTier::Gold => "Gold", 
            QualityTier::Platinum => "Platinum",
        }
    }

    pub fn value(&self) -> u8 {
        match self {
            QualityTier::Reject => 0,
            QualityTier::Bronze => 1,
            QualityTier::Silver => 2,
            QualityTier::Gold => 3,
            QualityTier::Platinum => 4,
        }
    }
}