//! Level 4 Relational Logic Analyzer for AISP (Refactored)
//! 
//! This is the new modular implementation using focused SRP components:
//! - ConstraintSolver: Constraint satisfaction and conflict detection  
//! - SetAnalyzer: Set theory validation and membership checking
//! - DependencyAnalyzer: Component dependency analysis and cycle detection
//! - TypeGraphAnalyzer: Type relationship graph construction and analysis
//! - ConflictDetector: Relational conflict detection and resolution

use crate::ast::*;
use crate::error::*;
use crate::constraint_solver::{ConstraintSolver, ConstraintAnalysisResult};
use crate::set_analyzer::{SetAnalyzer, SetAnalysisResult};
use crate::dependency_analyzer::{DependencyAnalyzer, DependencyAnalysisResult};
use crate::type_graph::{TypeGraphAnalyzer, TypeGraphResult};
use crate::conflict_detector::{ConflictDetector, ConflictDetectionResult, RelationalConflict};
use std::collections::HashMap;

/// Unified relational analysis result combining all focused analyzers
#[derive(Debug, Clone)]
pub struct RelationalAnalysis {
    /// Analysis succeeded without critical errors
    pub valid: bool,
    /// Overall relational consistency score (0.0-1.0)
    pub consistency_score: f64,
    /// Set theory validation results
    pub set_analysis: SetAnalysisResult,
    /// Type relationship graph analysis
    pub type_graph: TypeGraphResult,
    /// Constraint satisfaction analysis
    pub constraint_analysis: ConstraintAnalysisResult,
    /// Dependency analysis results
    pub dependency_analysis: DependencyAnalysisResult,
    /// Detected conflicts and resolutions
    pub conflict_analysis: ConflictDetectionResult,
    /// Analysis warnings
    pub warnings: Vec<AispWarning>,
}

/// Level 4 Relational Logic Analyzer (Refactored with SRP modules)
pub struct RelationalAnalyzer {
    /// Constraint satisfaction solver
    constraint_solver: ConstraintSolver,
    /// Set theory analyzer
    set_analyzer: SetAnalyzer,
    /// Component dependency analyzer
    dependency_analyzer: DependencyAnalyzer,
    /// Type relationship graph analyzer
    type_graph_analyzer: TypeGraphAnalyzer,
    /// Conflict detector
    conflict_detector: ConflictDetector,
    /// Analysis warnings collector
    warnings: Vec<AispWarning>,
}

impl RelationalAnalyzer {
    /// Create new relational analyzer with focused components
    pub fn new() -> Self {
        Self {
            constraint_solver: ConstraintSolver::new(),
            set_analyzer: SetAnalyzer::new(),
            dependency_analyzer: DependencyAnalyzer::new(),
            type_graph_analyzer: TypeGraphAnalyzer::new(),
            conflict_detector: ConflictDetector::new(),
            warnings: Vec::new(),
        }
    }

    /// Perform complete Level 4 relational analysis using focused modules
    pub fn analyze(
        &mut self, 
        doc: &AispDocument, 
        type_env: &HashMap<String, TypeExpression>
    ) -> AispResult<RelationalAnalysis> {
        // Reset analysis state
        self.warnings.clear();

        // 1. Build type relationship graph
        let type_graph = self.type_graph_analyzer.build_graph(doc)
            .map_err(|e| {
                self.warnings.push(AispWarning::warning(format!("Type graph error: {}", e)));
                e
            })?;

        // 2. Analyze set theory constructs
        let set_analysis = self.set_analyzer.analyze_document(doc);

        // 3. Extract and validate constraints
        let constraint_analysis = self.constraint_solver.extract_constraints(doc);

        // 4. Perform dependency analysis
        let dependency_analysis = self.dependency_analyzer.analyze_document(doc);

        // 5. Detect and analyze conflicts across all analyses
        let conflict_analysis = self.conflict_detector.detect_conflicts(
            &type_graph,
            &constraint_analysis,
            &set_analysis,
            &dependency_analysis,
        );

        // 6. Calculate overall consistency score
        let consistency_score = self.calculate_consistency_score(
            &type_graph,
            &set_analysis,
            &constraint_analysis,
            &dependency_analysis,
            &conflict_analysis,
        );

        // 7. Determine if analysis is valid (no critical conflicts)
        let valid = self.determine_validity(
            &type_graph,
            &constraint_analysis,
            &conflict_analysis,
            consistency_score,
        );

        Ok(RelationalAnalysis {
            valid,
            consistency_score,
            set_analysis,
            type_graph,
            constraint_analysis,
            dependency_analysis,
            conflict_analysis,
            warnings: self.warnings.clone(),
        })
    }

    /// Calculate overall consistency score from all analysis components
    fn calculate_consistency_score(
        &self,
        type_graph: &TypeGraphResult,
        set_analysis: &SetAnalysisResult,
        constraint_analysis: &ConstraintAnalysisResult,
        dependency_analysis: &DependencyAnalysisResult,
        conflict_analysis: &ConflictDetectionResult,
    ) -> f64 {
        let mut scores = Vec::new();
        let mut weights = Vec::new();

        // Type graph consistency (weight: 25%)
        let type_score = if type_graph.cycles.is_empty() { 1.0 } else { 0.3 };
        scores.push(type_score);
        weights.push(0.25);

        // Set analysis consistency (weight: 20%)
        let set_score = set_analysis.consistency_score;
        scores.push(set_score);
        weights.push(0.20);

        // Constraint satisfaction (weight: 30%)
        let constraint_score = constraint_analysis.satisfaction_score;
        scores.push(constraint_score);
        weights.push(0.30);

        // Dependency health (weight: 15%)
        let dependency_score = if dependency_analysis.circular_deps.is_empty() { 1.0 } else { 0.5 };
        scores.push(dependency_score);
        weights.push(0.15);

        // Conflict penalty (weight: 10%)
        let conflict_penalty = (conflict_analysis.total_conflict_score / 100.0).min(1.0);
        let conflict_score = (1.0 - conflict_penalty).max(0.0);
        scores.push(conflict_score);
        weights.push(0.10);

        // Weighted average
        let weighted_sum: f64 = scores.iter().zip(weights.iter()).map(|(s, w)| s * w).sum();
        let total_weight: f64 = weights.iter().sum();
        
        if total_weight > 0.0 {
            weighted_sum / total_weight
        } else {
            0.0
        }
    }

    /// Determine if the relational analysis is valid
    fn determine_validity(
        &self,
        type_graph: &TypeGraphResult,
        constraint_analysis: &ConstraintAnalysisResult,
        conflict_analysis: &ConflictDetectionResult,
        consistency_score: f64,
    ) -> bool {
        // Check for blocking issues
        let no_type_cycles = type_graph.cycles.is_empty();
        let no_critical_conflicts = !conflict_analysis.critical_conflicts.is_empty();
        let sufficient_consistency = consistency_score >= 0.7;
        let reasonable_constraint_satisfaction = constraint_analysis.satisfaction_score >= 0.6;

        no_type_cycles && 
        !no_critical_conflicts && 
        sufficient_consistency && 
        reasonable_constraint_satisfaction
    }

    /// Add warning to the analysis
    fn add_warning(&mut self, message: String) {
        self.warnings.push(AispWarning::warning(message));
    }

    /// Get detailed analysis report
    pub fn get_analysis_report(&self, analysis: &RelationalAnalysis) -> String {
        let mut report = String::new();
        
        report.push_str(&format!("=== AISP Level 4 Relational Analysis Report ===\n"));
        report.push_str(&format!("Overall Status: {}\n", if analysis.valid { "VALID" } else { "INVALID" }));
        report.push_str(&format!("Consistency Score: {:.2}%\n\n", analysis.consistency_score * 100.0));

        // Type graph summary
        report.push_str(&format!("Type Graph Analysis:\n"));
        report.push_str(&format!("  - Types: {}\n", analysis.type_graph.nodes.len()));
        report.push_str(&format!("  - Relationships: {}\n", analysis.type_graph.edges.len()));
        report.push_str(&format!("  - Cycles: {}\n", analysis.type_graph.cycles.len()));
        report.push_str(&format!("  - Root Types: {}\n\n", analysis.type_graph.root_types.len()));

        // Set analysis summary
        report.push_str(&format!("Set Theory Analysis:\n"));
        report.push_str(&format!("  - Sets: {}\n", analysis.set_analysis.sets.len()));
        report.push_str(&format!("  - Operations: {}\n", analysis.set_analysis.operations.len()));
        report.push_str(&format!("  - Membership Checks: {}\n", analysis.set_analysis.membership_checks.len()));
        report.push_str(&format!("  - Hierarchy Valid: {}\n\n", analysis.set_analysis.hierarchy_valid));

        // Constraint analysis summary
        report.push_str(&format!("Constraint Analysis:\n"));
        report.push_str(&format!("  - Total Constraints: {}\n", analysis.constraint_analysis.constraints.len()));
        report.push_str(&format!("  - Satisfied: {}\n", analysis.constraint_analysis.satisfied.len()));
        report.push_str(&format!("  - Unsatisfied: {}\n", analysis.constraint_analysis.unsatisfied.len()));
        report.push_str(&format!("  - Conflicts: {}\n\n", analysis.constraint_analysis.conflicts.len()));

        // Dependency analysis summary
        report.push_str(&format!("Dependency Analysis:\n"));
        report.push_str(&format!("  - Components: {}\n", analysis.dependency_analysis.dependencies.len()));
        report.push_str(&format!("  - Circular Dependencies: {}\n", analysis.dependency_analysis.circular_deps.len()));
        report.push_str(&format!("  - Unreachable: {}\n\n", analysis.dependency_analysis.unreachable.len()));

        // Conflict analysis summary
        report.push_str(&format!("Conflict Analysis:\n"));
        report.push_str(&format!("  - Total Conflicts: {}\n", analysis.conflict_analysis.conflicts.len()));
        report.push_str(&format!("  - Critical: {}\n", analysis.conflict_analysis.critical_conflicts.len()));
        report.push_str(&format!("  - Conflict Score: {:.1}\n\n", analysis.conflict_analysis.total_conflict_score));

        if !analysis.warnings.is_empty() {
            report.push_str(&format!("Warnings ({}):\n", analysis.warnings.len()));
            for warning in &analysis.warnings {
                report.push_str(&format!("  - {}\n", warning.message));
            }
        }

        report
    }
}

impl Default for RelationalAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_document() -> AispDocument {
        let mut type_definitions = HashMap::new();
        type_definitions.insert("State".to_string(), TypeDefinition {
            name: "State".to_string(),
            type_expr: TypeExpression::Enumeration(vec!["A".to_string(), "B".to_string()]),
            span: Span::new(1, 1, 1, 20),
        });

        AispDocument {
            header: DocumentHeader {
                version: "5.1".to_string(),
                name: "test".to_string(),
                date: "2026-01-25".to_string(),
                metadata: None,
            },
            metadata: DocumentMetadata {
                domain: None,
                protocol: None,
            },
            blocks: vec![AispBlock::Types(TypesBlock {
                definitions: type_definitions,
                span: Span::new(1, 1, 3, 1),
            })],
            span: Span::new(1, 1, 10, 1),
        }
    }

    #[test]
    fn test_modular_relational_analysis() {
        let mut analyzer = RelationalAnalyzer::new();
        let document = create_test_document();
        let type_env = HashMap::new();

        let result = analyzer.analyze(&document, &type_env);
        assert!(result.is_ok());

        let analysis = result.unwrap();
        assert!(analysis.consistency_score >= 0.0);
        assert!(analysis.consistency_score <= 1.0);
    }

    #[test]
    fn test_consistency_score_calculation() {
        let analyzer = RelationalAnalyzer::new();
        
        // Create minimal analysis results for testing
        let type_graph = TypeGraphResult {
            nodes: HashMap::new(),
            edges: vec![],
            cycles: vec![], // No cycles = good
            compatibility: HashMap::new(),
            hierarchy_depths: HashMap::new(),
            root_types: vec![],
        };

        let set_analysis = SetAnalysisResult {
            sets: HashMap::new(),
            operations: vec![],
            membership_checks: vec![],
            hierarchy_valid: true,
            empty_set_refs: vec![],
            consistency_score: 0.8,
        };

        let constraint_analysis = ConstraintAnalysisResult {
            constraints: vec![],
            satisfied: vec![],
            unsatisfied: vec![],
            conflicts: vec![],
            satisfaction_score: 0.9,
        };

        let dependency_analysis = DependencyAnalysisResult {
            dependencies: HashMap::new(),
            circular_deps: vec![], // No circular deps = good
            topological_order: vec![],
            unreachable: vec![],
            depth_map: HashMap::new(),
            leaf_components: vec![],
        };

        let conflict_analysis = ConflictDetectionResult {
            conflicts: vec![],
            conflicts_by_type: HashMap::new(),
            conflicts_by_severity: HashMap::new(),
            total_conflict_score: 0.0, // No conflicts = good
            critical_conflicts: vec![],
            resolution_order: vec![],
        };

        let score = analyzer.calculate_consistency_score(
            &type_graph,
            &set_analysis,
            &constraint_analysis,
            &dependency_analysis,
            &conflict_analysis,
        );

        // Should be high since all components are healthy
        assert!(score > 0.8);
    }

    #[test]
    fn test_validity_determination() {
        let analyzer = RelationalAnalyzer::new();
        
        let type_graph = TypeGraphResult {
            nodes: HashMap::new(),
            edges: vec![],
            cycles: vec![], // No cycles
            compatibility: HashMap::new(),
            hierarchy_depths: HashMap::new(),
            root_types: vec![],
        };

        let constraint_analysis = ConstraintAnalysisResult {
            constraints: vec![],
            satisfied: vec![],
            unsatisfied: vec![],
            conflicts: vec![],
            satisfaction_score: 0.8,
        };

        let conflict_analysis = ConflictDetectionResult {
            conflicts: vec![],
            conflicts_by_type: HashMap::new(),
            conflicts_by_severity: HashMap::new(),
            total_conflict_score: 0.0,
            critical_conflicts: vec![], // No critical conflicts
            resolution_order: vec![],
        };

        let valid = analyzer.determine_validity(
            &type_graph,
            &constraint_analysis,
            &conflict_analysis,
            0.8, // Good consistency score
        );

        assert!(valid);
    }

    #[test]
    fn test_analysis_report_generation() {
        let analyzer = RelationalAnalyzer::new();
        
        // Create a minimal analysis for report testing
        let analysis = RelationalAnalysis {
            valid: true,
            consistency_score: 0.85,
            set_analysis: SetAnalysisResult {
                sets: HashMap::new(),
                operations: vec![],
                membership_checks: vec![],
                hierarchy_valid: true,
                empty_set_refs: vec![],
                consistency_score: 0.8,
            },
            type_graph: TypeGraphResult {
                nodes: HashMap::new(),
                edges: vec![],
                cycles: vec![],
                compatibility: HashMap::new(),
                hierarchy_depths: HashMap::new(),
                root_types: vec![],
            },
            constraint_analysis: ConstraintAnalysisResult {
                constraints: vec![],
                satisfied: vec![],
                unsatisfied: vec![],
                conflicts: vec![],
                satisfaction_score: 0.9,
            },
            dependency_analysis: DependencyAnalysisResult {
                dependencies: HashMap::new(),
                circular_deps: vec![],
                topological_order: vec![],
                unreachable: vec![],
                depth_map: HashMap::new(),
                leaf_components: vec![],
            },
            conflict_analysis: ConflictDetectionResult {
                conflicts: vec![],
                conflicts_by_type: HashMap::new(),
                conflicts_by_severity: HashMap::new(),
                total_conflict_score: 0.0,
                critical_conflicts: vec![],
                resolution_order: vec![],
            },
            warnings: vec![],
        };

        let report = analyzer.get_analysis_report(&analysis);
        assert!(report.contains("VALID"));
        assert!(report.contains("85.00%"));
        assert!(report.contains("Type Graph Analysis"));
        assert!(report.contains("Set Theory Analysis"));
        assert!(report.contains("Constraint Analysis"));
        assert!(report.contains("Dependency Analysis"));
        assert!(report.contains("Conflict Analysis"));
    }
}