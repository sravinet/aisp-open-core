//! Conflict detection for relational logic analysis
//!
//! This module detects and categorizes various types of conflicts
//! in AISP documents including type conflicts, constraint conflicts,
//! and logical inconsistencies.

use crate::ast::*;
use crate::error::*;
use crate::constraint_solver::{ConstraintAnalysisResult, ConstraintConflict};
use crate::conflict_types::ConflictSeverity;
use crate::type_graph::{TypeGraphResult, RelationType};
use crate::set_analyzer::SetAnalysisResult;
use crate::dependency_analyzer::{DependencyAnalysisResult, CircularDependency};
use std::collections::{HashMap, HashSet};

/// Conflict detector for relational analysis
pub struct ConflictDetector {
    /// Detected conflicts
    conflicts: Vec<RelationalConflict>,
    /// Conflict resolution suggestions
    resolutions: HashMap<String, Vec<ResolutionStrategy>>,
}

/// A relational conflict detected during analysis
#[derive(Debug, Clone)]
pub struct RelationalConflict {
    /// Unique conflict identifier
    pub id: String,
    /// Type of conflict
    pub conflict_type: ConflictType,
    /// Severity level
    pub severity: ConflictSeverity,
    /// Human-readable description
    pub description: String,
    /// Components involved in the conflict
    pub components: Vec<String>,
    /// Source location (if applicable)
    pub location: Option<Span>,
    /// Evidence supporting this conflict
    pub evidence: ConflictEvidence,
    /// Suggested resolution strategies
    pub resolution_strategies: Vec<ResolutionStrategy>,
}

/// Types of conflicts that can be detected
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConflictType {
    /// Type inheritance cycle
    TypeCycle,
    /// Incompatible type assignment
    TypeMismatch,
    /// Contradictory logical constraints
    ConstraintConflict,
    /// Unreachable constraint
    UnreachableConstraint,
    /// Set theory violation
    SetViolation,
    /// Circular dependency between components
    DependencyCycle,
    /// Undefined symbol reference
    UndefinedSymbol,
    /// Ambiguous type reference
    AmbiguousReference,
    /// Inconsistent cardinality
    CardinalityInconsistency,
}

/// Evidence supporting a conflict
#[derive(Debug, Clone)]
pub enum ConflictEvidence {
    /// Cycle detected in graph analysis
    CycleDetected { cycle: Vec<String> },
    /// Type compatibility analysis
    TypeIncompatibility { type_a: String, type_b: String, reason: String },
    /// Constraint satisfaction failure
    ConstraintViolation { constraint_id: String, reason: String },
    /// Set theory inconsistency
    SetInconsistency { set_name: String, violation: String },
    /// Dependency analysis result
    DependencyIssue { component: String, issue: String },
}

/// Resolution strategy for conflicts
#[derive(Debug, Clone)]
pub struct ResolutionStrategy {
    /// Strategy type
    pub strategy_type: StrategyType,
    /// Human-readable description
    pub description: String,
    /// Confidence in this strategy (0.0-1.0)
    pub confidence: f64,
    /// Estimated effort to implement
    pub effort: EffortLevel,
    /// Specific actions to take
    pub actions: Vec<String>,
}

/// Types of resolution strategies
#[derive(Debug, Clone, PartialEq)]
pub enum StrategyType {
    /// Remove or modify offending component
    Remove,
    /// Add missing information or constraints
    Add,
    /// Refactor to eliminate conflict
    Refactor,
    /// Change type definitions
    TypeChange,
    /// Reorder components
    Reorder,
    /// Use composition instead of inheritance
    UseComposition,
}

/// Effort level required for resolution
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum EffortLevel {
    /// Low effort (simple change)
    Low,
    /// Medium effort (moderate changes)
    Medium,
    /// High effort (significant refactoring)
    High,
    /// Critical effort (major redesign)
    Critical,
}

/// Result of conflict detection
#[derive(Debug, Clone)]
pub struct ConflictDetectionResult {
    /// All detected conflicts
    pub conflicts: Vec<RelationalConflict>,
    /// Conflicts by type
    pub conflicts_by_type: HashMap<ConflictType, Vec<String>>,
    /// Conflicts by severity
    pub conflicts_by_severity: HashMap<ConflictSeverity, Vec<String>>,
    /// Total conflict score
    pub total_conflict_score: f64,
    /// Most critical conflicts
    pub critical_conflicts: Vec<String>,
    /// Suggested resolution order
    pub resolution_order: Vec<String>,
}

impl ConflictDetector {
    /// Create a new conflict detector
    pub fn new() -> Self {
        Self {
            conflicts: Vec::new(),
            resolutions: HashMap::new(),
        }
    }

    /// Detect conflicts from analysis results
    pub fn detect_conflicts(
        &mut self,
        type_graph: &TypeGraphResult,
        constraint_analysis: &ConstraintAnalysisResult,
        set_analysis: &SetAnalysisResult,
        dependency_analysis: &DependencyAnalysisResult,
    ) -> ConflictDetectionResult {
        self.conflicts.clear();
        self.resolutions.clear();

        // Detect type-related conflicts
        self.detect_type_conflicts(type_graph);

        // Detect constraint conflicts
        self.detect_constraint_conflicts(constraint_analysis);

        // Detect set theory conflicts
        self.detect_set_conflicts(set_analysis);

        // Detect dependency conflicts
        self.detect_dependency_conflicts(dependency_analysis);

        // Cross-analysis conflict detection
        self.detect_cross_analysis_conflicts(type_graph, constraint_analysis);

        // Generate resolution strategies
        self.generate_resolution_strategies();

        // Analyze and categorize results
        self.analyze_conflict_results()
    }

    /// Detect conflicts in type graph analysis
    fn detect_type_conflicts(&mut self, type_graph: &TypeGraphResult) {
        // Type hierarchy cycles
        for (cycle_idx, cycle) in type_graph.cycles.iter().enumerate() {
            let conflict = RelationalConflict {
                id: format!("type_cycle_{}", cycle_idx),
                conflict_type: ConflictType::TypeCycle,
                severity: ConflictSeverity::Error,
                description: format!(
                    "Circular type inheritance detected: {}",
                    cycle.join(" -> ")
                ),
                components: cycle.clone(),
                location: None,
                evidence: ConflictEvidence::CycleDetected { cycle: cycle.clone() },
                resolution_strategies: vec![
                    ResolutionStrategy {
                        strategy_type: StrategyType::UseComposition,
                        description: "Replace inheritance with composition".to_string(),
                        confidence: 0.8,
                        effort: EffortLevel::Medium,
                        actions: vec!["Remove circular inheritance".to_string(), "Use composition pattern".to_string()],
                    },
                    ResolutionStrategy {
                        strategy_type: StrategyType::Refactor,
                        description: "Introduce intermediate types to break cycle".to_string(),
                        confidence: 0.6,
                        effort: EffortLevel::High,
                        actions: vec!["Add intermediate type".to_string(), "Split inheritance chain".to_string()],
                    },
                ],
            };
            self.conflicts.push(conflict);
        }

        // Type compatibility conflicts
        for ((type_a, type_b), compatibility) in &type_graph.compatibility {
            if *compatibility == crate::type_graph::CompatibilityLevel::Incompatible {
                // Check if these types are being used together inappropriately
                if self.types_used_together(type_a, type_b) {
                    let conflict = RelationalConflict {
                        id: format!("type_mismatch_{}_{}", type_a, type_b),
                        conflict_type: ConflictType::TypeMismatch,
                        severity: ConflictSeverity::Warning,
                        description: format!(
                            "Incompatible types {} and {} used together",
                            type_a, type_b
                        ),
                        components: vec![type_a.clone(), type_b.clone()],
                        location: None,
                        evidence: ConflictEvidence::TypeIncompatibility {
                            type_a: type_a.clone(),
                            type_b: type_b.clone(),
                            reason: "Types are fundamentally incompatible".to_string(),
                        },
                        resolution_strategies: vec![
                            ResolutionStrategy {
                                strategy_type: StrategyType::TypeChange,
                                description: "Use compatible types or add conversion".to_string(),
                                confidence: 0.7,
                                effort: EffortLevel::Low,
                                actions: vec!["Change type declaration".to_string()],
                            },
                        ],
                    };
                    self.conflicts.push(conflict);
                }
            }
        }
    }

    /// Check if two types are used together (simplified heuristic)
    fn types_used_together(&self, _type_a: &str, _type_b: &str) -> bool {
        // TODO: Implement actual usage analysis
        // This would require analyzing function signatures, variable declarations, etc.
        false
    }

    /// Detect conflicts in constraint analysis
    fn detect_constraint_conflicts(&mut self, constraint_analysis: &ConstraintAnalysisResult) {
        // Convert constraint conflicts to relational conflicts
        for conflict in &constraint_analysis.conflicts {
            let relational_conflict = RelationalConflict {
                id: format!("constraint_conflict_{}", conflict.constraint_ids.join("_")),
                conflict_type: ConflictType::ConstraintConflict,
                severity: conflict.severity.clone(),
                description: conflict.description.clone(),
                components: conflict.constraint_ids.clone(),
                location: None,
                evidence: ConflictEvidence::ConstraintViolation {
                    constraint_id: conflict.constraint_ids.join(", "),
                    reason: conflict.description.clone(),
                },
                resolution_strategies: vec![
                    ResolutionStrategy {
                        strategy_type: StrategyType::Remove,
                        description: conflict.resolution.clone()
                            .unwrap_or_else(|| "Remove one of the conflicting constraints".to_string()),
                        confidence: 0.6,
                        effort: EffortLevel::Low,
                        actions: vec!["Remove conflicting constraint".to_string()],
                    },
                ],
            };
            self.conflicts.push(relational_conflict);
        }

        // Unreachable constraints
        for constraint_id in &constraint_analysis.unsatisfied {
            if self.is_constraint_unreachable(constraint_id, constraint_analysis) {
                let conflict = RelationalConflict {
                    id: format!("unreachable_constraint_{}", constraint_id),
                    conflict_type: ConflictType::UnreachableConstraint,
                    severity: ConflictSeverity::Warning,
                    description: format!("Constraint {} is unreachable", constraint_id),
                    components: vec![constraint_id.clone()],
                    location: None,
                    evidence: ConflictEvidence::ConstraintViolation {
                        constraint_id: constraint_id.clone(),
                        reason: "Constraint cannot be satisfied given current definitions".to_string(),
                    },
                    resolution_strategies: vec![
                        ResolutionStrategy {
                            strategy_type: StrategyType::Remove,
                            description: "Remove unreachable constraint".to_string(),
                            confidence: 0.8,
                            effort: EffortLevel::Low,
                            actions: vec!["Remove constraint".to_string()],
                        },
                        ResolutionStrategy {
                            strategy_type: StrategyType::Add,
                            description: "Add missing definitions to make constraint reachable".to_string(),
                            confidence: 0.5,
                            effort: EffortLevel::Medium,
                            actions: vec!["Add missing type/function definitions".to_string()],
                        },
                    ],
                };
                self.conflicts.push(conflict);
            }
        }
    }

    /// Check if a constraint is unreachable
    fn is_constraint_unreachable(&self, _constraint_id: &str, _analysis: &ConstraintAnalysisResult) -> bool {
        // TODO: Implement reachability analysis
        // This would check if the constraint can ever be satisfied
        false
    }

    /// Detect conflicts in set analysis
    fn detect_set_conflicts(&mut self, set_analysis: &SetAnalysisResult) {
        // Set hierarchy violations
        if !set_analysis.hierarchy_valid {
            let conflict = RelationalConflict {
                id: "set_hierarchy_invalid".to_string(),
                conflict_type: ConflictType::SetViolation,
                severity: ConflictSeverity::Error,
                description: "Set hierarchy is invalid".to_string(),
                components: vec!["set_hierarchy".to_string()],
                location: None,
                evidence: ConflictEvidence::SetInconsistency {
                    set_name: "hierarchy".to_string(),
                    violation: "Set hierarchy contains inconsistencies".to_string(),
                },
                resolution_strategies: vec![
                    ResolutionStrategy {
                        strategy_type: StrategyType::Refactor,
                        description: "Restructure set definitions to fix hierarchy".to_string(),
                        confidence: 0.7,
                        effort: EffortLevel::Medium,
                        actions: vec!["Review set definitions".to_string(), "Fix hierarchical issues".to_string()],
                    },
                ],
            };
            self.conflicts.push(conflict);
        }

        // Membership check failures
        for membership_check in &set_analysis.membership_checks {
            if !membership_check.is_valid {
                let conflict = RelationalConflict {
                    id: format!("membership_violation_{}_{}", membership_check.element, membership_check.set),
                    conflict_type: ConflictType::SetViolation,
                    severity: ConflictSeverity::Warning,
                    description: format!(
                        "Invalid set membership: {} ∉ {}",
                        membership_check.element, membership_check.set
                    ),
                    components: vec![membership_check.element.clone(), membership_check.set.clone()],
                    location: None,
                    evidence: ConflictEvidence::SetInconsistency {
                        set_name: membership_check.set.clone(),
                        violation: format!("Element {} is not a valid member", membership_check.element),
                    },
                    resolution_strategies: vec![
                        ResolutionStrategy {
                            strategy_type: StrategyType::TypeChange,
                            description: "Ensure element type matches set type".to_string(),
                            confidence: 0.8,
                            effort: EffortLevel::Low,
                            actions: vec!["Fix type declaration".to_string()],
                        },
                    ],
                };
                self.conflicts.push(conflict);
            }
        }
    }

    /// Detect conflicts in dependency analysis
    fn detect_dependency_conflicts(&mut self, dependency_analysis: &DependencyAnalysisResult) {
        // Circular dependencies
        for (cycle_idx, circular_dep) in dependency_analysis.circular_deps.iter().enumerate() {
            let severity = match circular_dep.severity {
                crate::dependency_analyzer::CycleSeverity::Minor => ConflictSeverity::Warning,
                crate::dependency_analyzer::CycleSeverity::Major => ConflictSeverity::Error,
                crate::dependency_analyzer::CycleSeverity::Critical => ConflictSeverity::Critical,
            };

            let conflict = RelationalConflict {
                id: format!("dependency_cycle_{}", cycle_idx),
                conflict_type: ConflictType::DependencyCycle,
                severity,
                description: format!(
                    "Circular dependency detected: {}",
                    circular_dep.cycle.join(" -> ")
                ),
                components: circular_dep.cycle.clone(),
                location: None,
                evidence: ConflictEvidence::DependencyIssue {
                    component: circular_dep.cycle.join(", "),
                    issue: "Circular dependency prevents proper resolution order".to_string(),
                },
                resolution_strategies: vec![
                    ResolutionStrategy {
                        strategy_type: StrategyType::Reorder,
                        description: circular_dep.resolution.clone()
                            .unwrap_or_else(|| "Break circular dependency".to_string()),
                        confidence: 0.7,
                        effort: EffortLevel::Medium,
                        actions: vec!["Remove circular reference".to_string(), "Use forward declarations".to_string()],
                    },
                ],
            };
            self.conflicts.push(conflict);
        }

        // Unreachable components
        for unreachable in &dependency_analysis.unreachable {
            let conflict = RelationalConflict {
                id: format!("unreachable_component_{}", unreachable),
                conflict_type: ConflictType::UndefinedSymbol,
                severity: ConflictSeverity::Warning,
                description: format!("Component {} is unreachable", unreachable),
                components: vec![unreachable.clone()],
                location: None,
                evidence: ConflictEvidence::DependencyIssue {
                    component: unreachable.clone(),
                    issue: "Component has no incoming dependencies".to_string(),
                },
                resolution_strategies: vec![
                    ResolutionStrategy {
                        strategy_type: StrategyType::Remove,
                        description: "Remove unused component".to_string(),
                        confidence: 0.9,
                        effort: EffortLevel::Low,
                        actions: vec!["Delete unreachable component".to_string()],
                    },
                ],
            };
            self.conflicts.push(conflict);
        }
    }

    /// Detect cross-analysis conflicts
    fn detect_cross_analysis_conflicts(
        &mut self,
        type_graph: &TypeGraphResult,
        constraint_analysis: &ConstraintAnalysisResult,
    ) {
        // Check for type-constraint mismatches
        for constraint in &constraint_analysis.constraints {
            for var in &constraint.variables {
                if !type_graph.nodes.contains_key(var) {
                    let conflict = RelationalConflict {
                        id: format!("undefined_type_{}", var),
                        conflict_type: ConflictType::UndefinedSymbol,
                        severity: ConflictSeverity::Error,
                        description: format!("Undefined type {} referenced in constraint", var),
                        components: vec![var.clone()],
                        location: Some(constraint.span.clone()),
                        evidence: ConflictEvidence::TypeIncompatibility {
                            type_a: var.clone(),
                            type_b: "undefined".to_string(),
                            reason: "Type not defined in type system".to_string(),
                        },
                        resolution_strategies: vec![
                            ResolutionStrategy {
                                strategy_type: StrategyType::Add,
                                description: "Define the missing type".to_string(),
                                confidence: 0.8,
                                effort: EffortLevel::Low,
                                actions: vec!["Add type definition".to_string()],
                            },
                        ],
                    };
                    self.conflicts.push(conflict);
                }
            }
        }
    }

    /// Generate resolution strategies for all conflicts
    fn generate_resolution_strategies(&mut self) {
        for conflict in &self.conflicts {
            let strategies = self.suggest_additional_strategies(&conflict.conflict_type);
            self.resolutions.insert(conflict.id.clone(), strategies);
        }
    }

    /// Suggest additional resolution strategies based on conflict type
    fn suggest_additional_strategies(&self, conflict_type: &ConflictType) -> Vec<ResolutionStrategy> {
        match conflict_type {
            ConflictType::TypeCycle => vec![
                ResolutionStrategy {
                    strategy_type: StrategyType::UseComposition,
                    description: "Use composition instead of inheritance".to_string(),
                    confidence: 0.8,
                    effort: EffortLevel::Medium,
                    actions: vec!["Replace inheritance with composition".to_string()],
                },
            ],
            ConflictType::TypeMismatch => vec![
                ResolutionStrategy {
                    strategy_type: StrategyType::Add,
                    description: "Add type conversion functions".to_string(),
                    confidence: 0.6,
                    effort: EffortLevel::Low,
                    actions: vec!["Define conversion functions".to_string()],
                },
            ],
            _ => vec![],
        }
    }

    /// Analyze conflict detection results
    fn analyze_conflict_results(&self) -> ConflictDetectionResult {
        let mut conflicts_by_type = HashMap::new();
        let mut conflicts_by_severity = HashMap::new();
        let mut total_score = 0.0;
        let mut critical_conflicts = Vec::new();

        for conflict in &self.conflicts {
            // Group by type
            conflicts_by_type
                .entry(conflict.conflict_type.clone())
                .or_insert_with(Vec::new)
                .push(conflict.id.clone());

            // Group by severity
            conflicts_by_severity
                .entry(conflict.severity.clone())
                .or_insert_with(Vec::new)
                .push(conflict.id.clone());

            // Calculate score contribution
            let score_contribution = match conflict.severity {
                ConflictSeverity::Critical => 10.0,
                ConflictSeverity::Error => 5.0,
                ConflictSeverity::Major => 3.0,
                ConflictSeverity::Warning => 1.0,
                ConflictSeverity::Minor => 0.2,
                ConflictSeverity::Info => 0.1,
            };
            total_score += score_contribution;

            // Track critical conflicts
            if matches!(conflict.severity, ConflictSeverity::Critical | ConflictSeverity::Error) {
                critical_conflicts.push(conflict.id.clone());
            }
        }

        // Determine resolution order (critical first, then by effort)
        let mut resolution_order = self.conflicts.iter().map(|c| c.id.clone()).collect::<Vec<_>>();
        resolution_order.sort_by(|a, b| {
            let conflict_a = self.conflicts.iter().find(|c| c.id == *a).unwrap();
            let conflict_b = self.conflicts.iter().find(|c| c.id == *b).unwrap();
            
            // First by severity (most severe first)
            match conflict_b.severity.cmp(&conflict_a.severity) {
                std::cmp::Ordering::Equal => {
                    // Then by minimum effort of resolution strategies
                    let min_effort_a = conflict_a.resolution_strategies.iter()
                        .map(|s| &s.effort)
                        .min()
                        .unwrap_or(&EffortLevel::Critical);
                    let min_effort_b = conflict_b.resolution_strategies.iter()
                        .map(|s| &s.effort)
                        .min()
                        .unwrap_or(&EffortLevel::Critical);
                    min_effort_a.cmp(min_effort_b)
                }
                other => other,
            }
        });

        ConflictDetectionResult {
            conflicts: self.conflicts.clone(),
            conflicts_by_type,
            conflicts_by_severity,
            total_conflict_score: total_score,
            critical_conflicts,
            resolution_order,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constraint_solver::*;
    use crate::type_graph::*;
    use crate::set_analyzer::*;
    use crate::dependency_analyzer::*;

    #[test]
    fn test_type_cycle_detection() {
        let mut detector = ConflictDetector::new();
        
        let type_graph = TypeGraphResult {
            nodes: HashMap::new(),
            edges: vec![],
            cycles: vec![vec!["A".to_string(), "B".to_string(), "A".to_string()]],
            compatibility: HashMap::new(),
            hierarchy_depths: HashMap::new(),
            root_types: vec![],
        };
        
        detector.detect_type_conflicts(&type_graph);
        assert!(!detector.conflicts.is_empty());
        assert_eq!(detector.conflicts[0].conflict_type, ConflictType::TypeCycle);
    }

    #[test]
    fn test_constraint_conflict_conversion() {
        let mut detector = ConflictDetector::new();
        
        let constraint_analysis = ConstraintAnalysisResult {
            constraints: vec![],
            satisfied: vec![],
            unsatisfied: vec![],
            conflicts: vec![ConstraintConflict {
                constraint_ids: vec!["c1".to_string(), "c2".to_string()],
                severity: ConflictSeverity::Error,
                description: "Test conflict".to_string(),
                resolution: Some("Remove one".to_string()),
            }],
            satisfaction_score: 0.5,
        };
        
        detector.detect_constraint_conflicts(&constraint_analysis);
        assert!(!detector.conflicts.is_empty());
        assert_eq!(detector.conflicts[0].conflict_type, ConflictType::ConstraintConflict);
    }

    #[test]
    fn test_conflict_severity_ordering() {
        assert!(ConflictSeverity::Critical > ConflictSeverity::Error);
        assert!(ConflictSeverity::Error > ConflictSeverity::Warning);
        assert!(ConflictSeverity::Warning > ConflictSeverity::Minor);
    }

    #[test]
    fn test_effort_level_ordering() {
        assert!(EffortLevel::Critical > EffortLevel::High);
        assert!(EffortLevel::High > EffortLevel::Medium);
        assert!(EffortLevel::Medium > EffortLevel::Low);
    }
}