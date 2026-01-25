//! Quality analysis for AISP documents
//!
//! This module calculates quality metrics, delta values, ambiguity scores,
//! and determines quality tiers for AISP documents.

use crate::ast::*;
use crate::error::*;
use std::collections::HashMap;

/// Quality analyzer for AISP documents
pub struct QualityAnalyzer {
    /// Document being analyzed
    document: Option<AispDocument>,
    /// Computed metrics
    metrics: QualityMetrics,
}

/// Quality metrics calculated from the document
#[derive(Debug, Clone)]
pub struct QualityMetrics {
    /// Delta value (0.0 - 1.0)
    pub delta: f64,
    /// Ambiguity score (0.0 - 1.0)
    pub ambiguity: f64,
    /// Quality tier symbol
    pub tier_symbol: String,
    /// Quality tier name
    pub tier_name: String,
    /// Tier numeric value (0-4)
    pub tier_value: u8,
    /// Complexity score
    pub complexity: f64,
    /// Completeness score
    pub completeness: f64,
}

/// Quality tier definitions
#[derive(Debug, Clone, Copy)]
pub enum QualityTier {
    /// ⊘ - Reject (δ < 0.20)
    Reject = 0,
    /// ◊⁻ - Bronze (0.20 ≤ δ < 0.40)
    Bronze = 1,
    /// ◊ - Silver (0.40 ≤ δ < 0.60)
    Silver = 2,
    /// ◊⁺ - Gold (0.60 ≤ δ < 0.75)
    Gold = 3,
    /// ◊⁺⁺ - Platinum (δ ≥ 0.75)
    Platinum = 4,
}

impl QualityTier {
    /// Get tier symbol
    pub fn symbol(&self) -> &'static str {
        match self {
            QualityTier::Reject => "⊘",
            QualityTier::Bronze => "◊⁻",
            QualityTier::Silver => "◊",
            QualityTier::Gold => "◊⁺",
            QualityTier::Platinum => "◊⁺⁺",
        }
    }

    /// Get tier name
    pub fn name(&self) -> &'static str {
        match self {
            QualityTier::Reject => "Reject",
            QualityTier::Bronze => "Bronze",
            QualityTier::Silver => "Silver",
            QualityTier::Gold => "Gold",
            QualityTier::Platinum => "Platinum",
        }
    }

    /// Get tier from delta value
    pub fn from_delta(delta: f64) -> Self {
        if delta >= 0.75 {
            QualityTier::Platinum
        } else if delta >= 0.60 {
            QualityTier::Gold
        } else if delta >= 0.40 {
            QualityTier::Silver
        } else if delta >= 0.20 {
            QualityTier::Bronze
        } else {
            QualityTier::Reject
        }
    }
}

impl QualityAnalyzer {
    /// Create a new quality analyzer
    pub fn new() -> Self {
        Self {
            document: None,
            metrics: QualityMetrics::default(),
        }
    }

    /// Analyze quality of the given document
    pub fn analyze_document(&mut self, document: &AispDocument) -> QualityMetrics {
        self.document = Some(document.clone());
        
        // Calculate individual metrics
        self.calculate_delta(document);
        self.calculate_ambiguity(document);
        self.calculate_complexity(document);
        self.calculate_completeness(document);
        
        // Determine quality tier
        let tier = QualityTier::from_delta(self.metrics.delta);
        self.metrics.tier_symbol = tier.symbol().to_string();
        self.metrics.tier_name = tier.name().to_string();
        self.metrics.tier_value = tier as u8;
        
        self.metrics.clone()
    }

    /// Calculate delta value based on document quality indicators
    fn calculate_delta(&mut self, document: &AispDocument) {
        let mut score = 0.0;
        let mut max_score = 0.0;

        // Check for presence of required blocks (20 points)
        let required_blocks = ["Meta", "Types", "Rules", "Functions", "Evidence"];
        let mut present_blocks = 0;
        
        for block in &document.blocks {
            match block {
                AispBlock::Meta(_) => present_blocks += 1,
                AispBlock::Types(_) => present_blocks += 1,
                AispBlock::Rules(_) => present_blocks += 1,
                AispBlock::Functions(_) => present_blocks += 1,
                AispBlock::Evidence(_) => present_blocks += 1,
            }
        }
        
        score += (present_blocks as f64 / required_blocks.len() as f64) * 20.0;
        max_score += 20.0;

        // Evaluate type system quality (25 points)
        if let Some(types_score) = self.evaluate_type_system(document) {
            score += types_score;
        }
        max_score += 25.0;

        // Evaluate logical rules quality (25 points)
        if let Some(rules_score) = self.evaluate_rules_system(document) {
            score += rules_score;
        }
        max_score += 25.0;

        // Evaluate function definitions quality (20 points)
        if let Some(functions_score) = self.evaluate_functions_system(document) {
            score += functions_score;
        }
        max_score += 20.0;

        // Meta-information quality (10 points)
        if let Some(meta_score) = self.evaluate_meta_quality(document) {
            score += meta_score;
        }
        max_score += 10.0;

        // Normalize to 0.0-1.0 range
        self.metrics.delta = if max_score > 0.0 {
            (score / max_score).min(1.0).max(0.0)
        } else {
            0.0
        };
    }

    /// Evaluate quality of the type system
    fn evaluate_type_system(&self, document: &AispDocument) -> Option<f64> {
        for block in &document.blocks {
            if let AispBlock::Types(types_block) = block {
                let mut score = 0.0;
                
                // Points for number of types (up to 10 points)
                let type_count = types_block.definitions.len();
                score += (type_count.min(5) as f64 / 5.0) * 10.0;
                
                // Points for type complexity (up to 15 points)
                let mut complexity_score: f64 = 0.0;
                for (_, definition) in &types_block.definitions {
                    match &definition.type_expr {
                        TypeExpression::Basic(_) => complexity_score += 1.0,
                        TypeExpression::Enumeration(values) => {
                            complexity_score += 2.0 + (values.len().min(3) as f64);
                        }
                        TypeExpression::Array { .. } => complexity_score += 3.0,
                        TypeExpression::Function { .. } => complexity_score += 4.0,
                        TypeExpression::Reference(_) => complexity_score += 1.0,
                        TypeExpression::Tuple(_) => complexity_score += 2.5,
                        TypeExpression::Generic { .. } => complexity_score += 3.5,
                    }
                }
                score += complexity_score.min(15.0);
                
                return Some(score);
            }
        }
        None
    }

    /// Evaluate quality of the rules system
    fn evaluate_rules_system(&self, document: &AispDocument) -> Option<f64> {
        for block in &document.blocks {
            if let AispBlock::Rules(rules_block) = block {
                let mut score = 0.0;
                
                // Points for number of rules (up to 10 points)
                let rule_count = rules_block.rules.len();
                score += (rule_count.min(5) as f64 / 5.0) * 10.0;
                
                // Points for rule complexity (up to 15 points)
                let mut complexity_score: f64 = 0.0;
                for rule in &rules_block.rules {
                    // Basic rule = 2 points
                    complexity_score += 2.0;
                    
                    // Quantifier adds 2 points
                    if rule.quantifier.is_some() {
                        complexity_score += 2.0;
                    }
                    
                    // Complex expressions add points
                    if let LogicalExpression::Variable(expr) = &rule.expression {
                        if expr.contains('→') || expr.contains('⇒') {
                            complexity_score += 1.0;
                        }
                        if expr.contains('□') || expr.contains('◊') {
                            complexity_score += 2.0; // Temporal logic
                        }
                    }
                }
                score += complexity_score.min(15.0);
                
                return Some(score);
            }
        }
        None
    }

    /// Evaluate quality of the functions system
    fn evaluate_functions_system(&self, document: &AispDocument) -> Option<f64> {
        for block in &document.blocks {
            if let AispBlock::Functions(functions_block) = block {
                let mut score = 0.0;
                
                // Points for number of functions (up to 8 points)
                let function_count = functions_block.functions.len();
                score += (function_count.min(4) as f64 / 4.0) * 8.0;
                
                // Points for function complexity (up to 12 points)
                let mut complexity_score = 0.0;
                for (_, function) in &functions_block.functions {
                    // Basic function = 2 points
                    complexity_score += 2.0;
                    
                    // Multiple parameters add points
                    complexity_score += (function.lambda.parameters.len().min(3) as f64);
                }
                score += complexity_score.min(12.0);
                
                return Some(score);
            }
        }
        None
    }

    /// Evaluate quality of meta information
    fn evaluate_meta_quality(&self, document: &AispDocument) -> Option<f64> {
        for block in &document.blocks {
            if let AispBlock::Meta(meta_block) = block {
                let mut score = 0.0;
                
                // Points for essential meta fields
                let essential_fields = ["domain", "version"];
                for field in &essential_fields {
                    if meta_block.entries.contains_key(*field) {
                        score += 3.0;
                    }
                }
                
                // Points for additional meta entries (up to 4 points)
                let additional_entries = meta_block.entries.len().saturating_sub(essential_fields.len());
                score += (additional_entries.min(2) as f64 / 2.0) * 4.0;
                
                return Some(score);
            }
        }
        None
    }

    /// Calculate ambiguity score
    fn calculate_ambiguity(&mut self, document: &AispDocument) {
        let mut ambiguity_factors = Vec::new();
        
        // Check for ambiguous type references
        ambiguity_factors.push(self.check_type_ambiguity(document));
        
        // Check for ambiguous symbol usage
        ambiguity_factors.push(self.check_symbol_ambiguity(document));
        
        // Check for unclear logical expressions
        ambiguity_factors.push(self.check_logical_ambiguity(document));
        
        // Average ambiguity score
        let sum: f64 = ambiguity_factors.iter().sum();
        self.metrics.ambiguity = if ambiguity_factors.is_empty() {
            1.0 // Maximum ambiguity if no analysis possible
        } else {
            sum / ambiguity_factors.len() as f64
        };
    }

    /// Check for type-related ambiguity
    fn check_type_ambiguity(&self, _document: &AispDocument) -> f64 {
        // TODO: Implement type ambiguity detection
        // For now, return low ambiguity
        0.1
    }

    /// Check for symbol-related ambiguity
    fn check_symbol_ambiguity(&self, _document: &AispDocument) -> f64 {
        // TODO: Implement symbol ambiguity detection
        // For now, return low ambiguity
        0.05
    }

    /// Check for logical expression ambiguity
    fn check_logical_ambiguity(&self, _document: &AispDocument) -> f64 {
        // TODO: Implement logical ambiguity detection
        // For now, return very low ambiguity
        0.02
    }

    /// Calculate complexity score
    fn calculate_complexity(&mut self, document: &AispDocument) {
        let mut complexity = 0.0;
        
        // Type complexity
        for block in &document.blocks {
            match block {
                AispBlock::Types(types_block) => {
                    complexity += types_block.definitions.len() as f64 * 0.1;
                }
                AispBlock::Rules(rules_block) => {
                    complexity += rules_block.rules.len() as f64 * 0.15;
                }
                AispBlock::Functions(functions_block) => {
                    complexity += functions_block.functions.len() as f64 * 0.2;
                }
                _ => {}
            }
        }
        
        self.metrics.complexity = complexity.min(1.0);
    }

    /// Calculate completeness score
    fn calculate_completeness(&mut self, document: &AispDocument) {
        let required_blocks = 5.0; // Meta, Types, Rules, Functions, Evidence
        let present_blocks = document.blocks.len() as f64;
        
        self.metrics.completeness = (present_blocks / required_blocks).min(1.0);
    }
}

impl Default for QualityMetrics {
    fn default() -> Self {
        Self {
            delta: 0.0,
            ambiguity: 1.0,
            tier_symbol: "⊘".to_string(),
            tier_name: "Reject".to_string(),
            tier_value: 0,
            complexity: 0.0,
            completeness: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_document() -> AispDocument {
        AispDocument {
            header: DocumentHeader {
                version: "5.1".to_string(),
                name: "test".to_string(),
                date: "2026-01-25".to_string(),
                metadata: None,
            },
            metadata: DocumentMetadata {
                domain: Some("test".to_string()),
                protocol: None,
            },
            blocks: vec![
                AispBlock::Meta(MetaBlock {
                    entries: {
                        let mut entries = HashMap::new();
                        entries.insert("domain".to_string(), MetaEntry {
                            key: "domain".to_string(),
                            value: MetaValue::String("test".to_string()),
                            span: Span::new(1, 1, 1, 10),
                        });
                        entries
                    },
                    span: Span::new(1, 1, 2, 1),
                }),
            ],
            span: Span::new(1, 1, 10, 1),
        }
    }

    #[test]
    fn test_quality_tier_from_delta() {
        assert_eq!(QualityTier::from_delta(0.8) as u8, QualityTier::Platinum as u8);
        assert_eq!(QualityTier::from_delta(0.65) as u8, QualityTier::Gold as u8);
        assert_eq!(QualityTier::from_delta(0.45) as u8, QualityTier::Silver as u8);
        assert_eq!(QualityTier::from_delta(0.25) as u8, QualityTier::Bronze as u8);
        assert_eq!(QualityTier::from_delta(0.15) as u8, QualityTier::Reject as u8);
    }

    #[test]
    fn test_quality_tier_symbols() {
        assert_eq!(QualityTier::Platinum.symbol(), "◊⁺⁺");
        assert_eq!(QualityTier::Gold.symbol(), "◊⁺");
        assert_eq!(QualityTier::Silver.symbol(), "◊");
        assert_eq!(QualityTier::Bronze.symbol(), "◊⁻");
        assert_eq!(QualityTier::Reject.symbol(), "⊘");
    }

    #[test]
    fn test_basic_quality_analysis() {
        let mut analyzer = QualityAnalyzer::new();
        let document = create_test_document();
        
        let metrics = analyzer.analyze_document(&document);
        
        // Should have some delta value
        assert!(metrics.delta >= 0.0);
        assert!(metrics.delta <= 1.0);
        
        // Should have some ambiguity
        assert!(metrics.ambiguity >= 0.0);
        assert!(metrics.ambiguity <= 1.0);
        
        // Should have valid tier
        assert!(!metrics.tier_symbol.is_empty());
        assert!(!metrics.tier_name.is_empty());
    }

    #[test]
    fn test_meta_quality_evaluation() {
        let analyzer = QualityAnalyzer::new();
        let document = create_test_document();
        
        let meta_score = analyzer.evaluate_meta_quality(&document);
        
        assert!(meta_score.is_some());
        assert!(meta_score.unwrap() > 0.0);
    }

    #[test]
    fn test_delta_calculation() {
        let mut analyzer = QualityAnalyzer::new();
        let document = create_test_document();
        
        analyzer.calculate_delta(&document);
        
        assert!(analyzer.metrics.delta >= 0.0);
        assert!(analyzer.metrics.delta <= 1.0);
    }
}