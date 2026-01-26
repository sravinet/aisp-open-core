//! Level 5 Temporal Logic Analyzer for AISP
//! 
//! Provides advanced temporal reasoning capabilities including:
//! - Temporal operator analysis (□, ◊, X, U, R)  
//! - Linear Temporal Logic (LTL) model checking
//! - Computation Tree Logic (CTL) support
//! - Temporal constraint satisfaction and verification
//! - Path-based analysis and state space exploration
//! - Temporal specification validation and consistency

use crate::ast::*;
use crate::error::*;
use crate::conflict_types::ConflictSeverity;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;

/// Temporal analysis result
#[derive(Debug, Clone)]
pub struct TemporalAnalysis {
    /// Temporal analysis succeeded
    pub valid: bool,
    /// Temporal consistency score (0.0-1.0)
    pub consistency_score: f64,
    /// LTL formula analysis
    pub ltl_analysis: LtlAnalysis,
    /// CTL formula analysis  
    pub ctl_analysis: CtlAnalysis,
    /// Temporal constraint satisfaction
    pub temporal_constraints: TemporalConstraintAnalysis,
    /// State space analysis
    pub state_space: StateSpaceAnalysis,
    /// Model checking results
    pub model_checking: ModelCheckingResults,
    /// Detected temporal conflicts
    pub conflicts: Vec<TemporalConflict>,
    /// Temporal warnings
    pub warnings: Vec<AispWarning>,
}

/// Linear Temporal Logic analysis results
#[derive(Debug, Clone)]
pub struct LtlAnalysis {
    /// LTL formulas found in document
    pub formulas: Vec<LtlFormula>,
    /// Formula satisfiability  
    pub satisfiable: HashMap<String, bool>,
    /// Formula complexity metrics
    pub complexity: HashMap<String, FormulaComplexity>,
    /// Detected patterns
    pub patterns: Vec<TemporalPattern>,
}

/// Computation Tree Logic analysis results
#[derive(Debug, Clone)]  
pub struct CtlAnalysis {
    /// CTL formulas found in document
    pub formulas: Vec<CtlFormula>,
    /// Path quantifier analysis
    pub path_quantifiers: Vec<PathQuantifier>,
    /// Branching degree analysis
    pub branching_analysis: BranchingAnalysis,
}

/// Temporal constraint analysis
#[derive(Debug, Clone)]
pub struct TemporalConstraintAnalysis {
    /// All temporal constraints
    pub constraints: Vec<TemporalConstraint>,
    /// Constraint satisfaction status
    pub satisfaction: HashMap<String, ConstraintStatus>,
    /// Temporal dependencies
    pub dependencies: HashMap<String, Vec<String>>,
    /// Constraint ordering
    pub ordering: Vec<String>,
}

/// State space analysis results
#[derive(Debug, Clone)]
pub struct StateSpaceAnalysis {
    /// Identified states
    pub states: Vec<TemporalState>,
    /// State transitions
    pub transitions: Vec<StateTransition>,
    /// Reachability analysis
    pub reachability: ReachabilityAnalysis,
    /// Liveness properties
    pub liveness: LivenessAnalysis,
    /// Safety properties
    pub safety: SafetyAnalysis,
}

/// Model checking results
#[derive(Debug, Clone)]
pub struct ModelCheckingResults {
    /// Properties checked
    pub properties: Vec<TemporalProperty>,
    /// Verification results
    pub results: HashMap<String, VerificationResult>,
    /// Counterexamples found
    pub counterexamples: Vec<Counterexample>,
    /// Witness traces
    pub witnesses: Vec<ExecutionTrace>,
}

/// Linear Temporal Logic formula
#[derive(Debug, Clone)]
pub struct LtlFormula {
    pub id: String,
    pub formula: String,
    pub operators: Vec<TemporalOperator>,
    pub variables: Vec<String>,
    pub nesting_depth: usize,
    pub location: Span,
}

/// Computation Tree Logic formula
#[derive(Debug, Clone)]
pub struct CtlFormula {
    pub id: String,
    pub formula: String,
    pub path_quantifier: PathQuantifierType,
    pub temporal_operator: TemporalOperator,
    pub subformulas: Vec<String>,
    pub location: Span,
}

/// Temporal operators in AISP
#[derive(Debug, Clone, PartialEq)]
pub enum TemporalOperator {
    /// Always/Globally (□, G)
    Always,
    /// Eventually/Finally (◊, F)  
    Eventually,
    /// Next time (X)
    Next,
    /// Until (U)
    Until,
    /// Release (R)
    Release,
    /// Weak until (W)
    WeakUntil,
    /// Strong release (M)
    StrongRelease,
}

/// Path quantifier types for CTL
#[derive(Debug, Clone, PartialEq)]
pub enum PathQuantifierType {
    /// For all paths (A)
    AllPaths,
    /// Exists a path (E)  
    ExistsPath,
}

/// Path quantifier with temporal operator
#[derive(Debug, Clone)]
pub struct PathQuantifier {
    pub quantifier_type: PathQuantifierType,
    pub temporal_op: TemporalOperator,
    pub formula: String,
    pub location: Span,
}

/// Formula complexity metrics
#[derive(Debug, Clone)]
pub struct FormulaComplexity {
    /// Number of temporal operators
    pub operator_count: usize,
    /// Maximum nesting depth
    pub nesting_depth: usize,
    /// Number of variables
    pub variable_count: usize,
    /// Cyclic complexity
    pub cyclic_complexity: f64,
}

/// Temporal pattern recognition
#[derive(Debug, Clone)]
pub struct TemporalPattern {
    pub pattern_type: PatternType,
    pub description: String,
    pub instances: Vec<PatternInstance>,
    pub confidence: f64,
}

/// Types of temporal patterns
#[derive(Debug, Clone, PartialEq)]
pub enum PatternType {
    /// Safety pattern (□P)
    Safety,
    /// Liveness pattern (◊P)  
    Liveness,
    /// Response pattern (□(P → ◊Q))
    Response,
    /// Persistence pattern (◊□P)
    Persistence,
    /// Recurrence pattern (□◊P)
    Recurrence,
    /// Fairness pattern
    Fairness,
}

/// Pattern instance
#[derive(Debug, Clone)]
pub struct PatternInstance {
    pub formula: String,
    pub variables: Vec<String>,
    pub location: Span,
    pub strength: f64,
}

/// Branching analysis for CTL
#[derive(Debug, Clone)]
pub struct BranchingAnalysis {
    /// Average branching factor
    pub avg_branching_factor: f64,
    /// Maximum branching factor
    pub max_branching_factor: usize,
    /// Branching distribution
    pub branching_distribution: HashMap<usize, usize>,
    /// Non-deterministic points
    pub nondeterministic_points: Vec<String>,
}

/// Temporal constraint
#[derive(Debug, Clone)]
pub struct TemporalConstraint {
    pub id: String,
    pub constraint_type: TemporalConstraintType,
    pub formula: String,
    pub priority: ConstraintPriority,
    pub timeout: Option<f64>,
    pub location: Span,
}

/// Types of temporal constraints  
#[derive(Debug, Clone, PartialEq)]
pub enum TemporalConstraintType {
    /// Invariant (□P)
    Invariant,
    /// Eventual (◊P)
    Eventual,  
    /// Response (P → ◊Q)
    Response,
    /// Precedence (¬Q U P)
    Precedence,
    /// Absence (□¬P)
    Absence,
    /// Existence (◊P)
    Existence,
    /// Bounded existence (◊≤nP)
    BoundedExistence(usize),
}

/// Constraint priority levels
#[derive(Debug, Clone, PartialEq, Ord, PartialOrd, Eq)]
pub enum ConstraintPriority {
    /// Critical system property
    Critical,
    /// Important functional property
    High,
    /// Performance or quality property  
    Medium,
    /// Optional property
    Low,
}

/// Constraint satisfaction status
#[derive(Debug, Clone, PartialEq)]
pub enum ConstraintStatus {
    /// Constraint is satisfied
    Satisfied,
    /// Constraint is violated
    Violated,
    /// Constraint satisfaction unknown
    Unknown,
    /// Constraint is being checked
    Checking,
}

/// State in temporal model
#[derive(Debug, Clone)]
pub struct TemporalState {
    pub id: String,
    pub properties: HashMap<String, bool>,
    pub label: Option<String>,
    pub is_initial: bool,
    pub is_final: bool,
}

/// State transition
#[derive(Debug, Clone)]
pub struct StateTransition {
    pub from: String,
    pub to: String,
    pub guard: Option<String>,
    pub action: Option<String>,
    pub probability: Option<f64>,
}

/// Reachability analysis results
#[derive(Debug, Clone)]
pub struct ReachabilityAnalysis {
    /// States reachable from initial states
    pub reachable_states: HashSet<String>,
    /// Unreachable states
    pub unreachable_states: HashSet<String>,
    /// Strongly connected components
    pub sccs: Vec<Vec<String>>,
    /// Dead states (no outgoing transitions)
    pub dead_states: HashSet<String>,
}

/// Liveness analysis
#[derive(Debug, Clone)]
pub struct LivenessAnalysis {
    /// Liveness properties
    pub properties: Vec<LivenessProperty>,
    /// Fair paths analysis
    pub fairness: FairnessAnalysis,
    /// Progress measures
    pub progress: Vec<ProgressMeasure>,
}

/// Safety analysis
#[derive(Debug, Clone)]
pub struct SafetyAnalysis {
    /// Safety properties
    pub properties: Vec<SafetyProperty>,
    /// Invariant checking results
    pub invariants: Vec<InvariantResult>,
    /// Bad state analysis  
    pub bad_states: Vec<String>,
}

/// Liveness property
#[derive(Debug, Clone)]
pub struct LivenessProperty {
    pub name: String,
    pub formula: String,
    pub satisfied: bool,
    pub witness: Option<ExecutionTrace>,
}

/// Safety property  
#[derive(Debug, Clone)]
pub struct SafetyProperty {
    pub name: String,
    pub formula: String,
    pub satisfied: bool,
    pub counterexample: Option<Counterexample>,
}

/// Fairness analysis
#[derive(Debug, Clone)]
pub struct FairnessAnalysis {
    /// Fair scheduling constraints
    pub fairness_constraints: Vec<String>,
    /// Fair execution paths
    pub fair_paths: Vec<ExecutionTrace>,
    /// Fairness violations
    pub violations: Vec<FairnessViolation>,
}

/// Progress measure
#[derive(Debug, Clone)]
pub struct ProgressMeasure {
    pub name: String,
    pub metric: ProgressMetric,
    pub threshold: Option<f64>,
    pub current_value: f64,
}

/// Progress metric types
#[derive(Debug, Clone, PartialEq)]
pub enum ProgressMetric {
    /// Average progress per step
    AverageProgress,
    /// Maximum progress achieved
    MaxProgress,
    /// Progress variance
    ProgressVariance,
    /// Starvation freedom
    StarvationFreedom,
}

/// Invariant checking result
#[derive(Debug, Clone)]
pub struct InvariantResult {
    pub invariant: String,
    pub holds: bool,
    pub violated_at: Option<String>,
    pub witness: Option<ExecutionTrace>,
}

/// Fairness violation
#[derive(Debug, Clone)]
pub struct FairnessViolation {
    pub constraint: String,
    pub trace: ExecutionTrace,
    pub severity: ViolationSeverity,
}

/// Violation severity levels
#[derive(Debug, Clone, PartialEq)]
pub enum ViolationSeverity {
    /// Critical system failure
    Critical,
    /// Functional violation
    Major,
    /// Performance degradation
    Minor,
    /// Informational
    Info,
}

/// Temporal property for model checking
#[derive(Debug, Clone)]
pub struct TemporalProperty {
    pub name: String,
    pub property_type: PropertyType,
    pub formula: String,
    pub description: String,
}

/// Property types for verification
#[derive(Debug, Clone, PartialEq)]
pub enum PropertyType {
    /// Safety property
    Safety,
    /// Liveness property
    Liveness,
    /// Fairness property
    Fairness,
    /// Performance property
    Performance,
    /// Security property
    Security,
}

/// Verification result
#[derive(Debug, Clone)]
pub struct VerificationResult {
    pub property_name: String,
    pub result: VerificationStatus,
    pub execution_time: f64,
    pub memory_usage: usize,
    pub certificate: Option<String>,
}

/// Verification status
#[derive(Debug, Clone, PartialEq)]
pub enum VerificationStatus {
    /// Property verified successfully  
    Verified,
    /// Property violated
    Violated,
    /// Verification inconclusive
    Unknown,
    /// Verification timeout
    Timeout,
    /// Verification error
    Error(String),
}

/// Counterexample for violated properties
#[derive(Debug, Clone)]
pub struct Counterexample {
    pub property: String,
    pub trace: ExecutionTrace,
    pub explanation: String,
    pub minimized: bool,
}

/// Execution trace  
#[derive(Debug, Clone)]
pub struct ExecutionTrace {
    pub states: Vec<String>,
    pub transitions: Vec<String>,
    pub length: usize,
    pub is_infinite: bool,
    pub loop_point: Option<usize>,
}

/// Temporal conflict
#[derive(Debug, Clone)]
pub struct TemporalConflict {
    pub conflict_type: TemporalConflictType,
    pub description: String,
    pub formulas: Vec<String>,
    pub severity: ConflictSeverity,
    pub location: Option<Span>,
    pub resolution: Option<String>,
}

/// Types of temporal conflicts
#[derive(Debug, Clone, PartialEq)]
pub enum TemporalConflictType {
    /// Contradictory temporal requirements
    ContradictoryRequirements,
    /// Unsatisfiable formula combination
    UnsatisfiableFormula,
    /// Temporal deadlock
    TemporalDeadlock,
    /// Liveness vs safety conflict
    LivenessSafetyConflict,
    /// Fairness violation
    FairnessConflict,
}


impl fmt::Display for TemporalOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Always => write!(f, "□"),
            Self::Eventually => write!(f, "◊"),
            Self::Next => write!(f, "X"),
            Self::Until => write!(f, "U"),
            Self::Release => write!(f, "R"),
            Self::WeakUntil => write!(f, "W"),
            Self::StrongRelease => write!(f, "M"),
        }
    }
}

/// Level 5 Temporal Logic Analyzer
pub struct TemporalAnalyzer {
    /// Known temporal operators
    operators: HashMap<char, TemporalOperator>,
    /// State space model
    state_space: HashMap<String, TemporalState>,
    /// Collected constraints
    constraints: Vec<TemporalConstraint>,
    /// Analysis warnings
    warnings: Vec<AispWarning>,
}

impl TemporalAnalyzer {
    /// Create new temporal analyzer
    pub fn new() -> Self {
        let mut operators = HashMap::new();
        operators.insert('□', TemporalOperator::Always);
        operators.insert('◊', TemporalOperator::Eventually);
        operators.insert('X', TemporalOperator::Next);
        operators.insert('U', TemporalOperator::Until);
        operators.insert('R', TemporalOperator::Release);
        operators.insert('W', TemporalOperator::WeakUntil);
        operators.insert('M', TemporalOperator::StrongRelease);
        
        Self {
            operators,
            state_space: HashMap::new(),
            constraints: Vec::new(),
            warnings: Vec::new(),
        }
    }

    /// Perform complete Level 5 temporal analysis
    pub fn analyze(&mut self, doc: &AispDocument) -> AispResult<TemporalAnalysis> {
        // Reset state
        self.state_space.clear();
        self.constraints.clear();
        self.warnings.clear();

        // Analyze LTL formulas
        let ltl_analysis = self.analyze_ltl(doc)?;

        // Analyze CTL formulas  
        let ctl_analysis = self.analyze_ctl(doc)?;

        // Extract temporal constraints
        let temporal_constraints = self.analyze_temporal_constraints(doc)?;

        // Build state space model
        let state_space = self.analyze_state_space(doc)?;

        // Perform model checking
        let model_checking = self.perform_model_checking(&ltl_analysis, &ctl_analysis, &state_space)?;

        // Detect temporal conflicts
        let conflicts = self.detect_temporal_conflicts(&ltl_analysis, &ctl_analysis, &temporal_constraints)?;

        // Calculate consistency score
        let consistency_score = self.calculate_temporal_consistency(
            &ltl_analysis,
            &ctl_analysis, 
            &temporal_constraints,
            &conflicts
        );

        // Check overall validity
        let valid = conflicts.iter().all(|c| c.severity != ConflictSeverity::Critical) &&
                   consistency_score >= 0.8 &&
                   model_checking.results.values().all(|r| r.result != VerificationStatus::Error("".to_string()));

        Ok(TemporalAnalysis {
            valid,
            consistency_score,
            ltl_analysis,
            ctl_analysis,
            temporal_constraints,
            state_space,
            model_checking,
            conflicts,
            warnings: self.warnings.clone(),
        })
    }

    /// Analyze Linear Temporal Logic formulas
    fn analyze_ltl(&mut self, doc: &AispDocument) -> AispResult<LtlAnalysis> {
        let mut formulas = Vec::new();
        let mut satisfiable = HashMap::new();
        let mut complexity = HashMap::new();
        let mut patterns = Vec::new();

        // Extract LTL formulas from rules and functions
        for block in &doc.blocks {
            match block {
                AispBlock::Rules(rules) => {
                    for rule in &rules.rules {
                        if let Some(formula) = self.extract_ltl_from_rule(rule)? {
                            let id = formula.id.clone();
                            
                            // Analyze satisfiability (simplified)
                            let sat = self.check_ltl_satisfiability(&formula)?;
                            satisfiable.insert(id.clone(), sat);
                            
                            // Calculate complexity
                            let comp = self.calculate_formula_complexity(&formula);
                            complexity.insert(id, comp);
                            
                            formulas.push(formula);
                        }
                    }
                }
                _ => {}
            }
        }

        // Detect temporal patterns
        patterns = self.detect_temporal_patterns(&formulas)?;

        Ok(LtlAnalysis {
            formulas,
            satisfiable,
            complexity,
            patterns,
        })
    }

    /// Extract LTL formula from logical rule
    fn extract_ltl_from_rule(&self, rule: &LogicalRule) -> AispResult<Option<LtlFormula>> {
        // Check if rule contains temporal operators
        let rule_text = format!("{:?}", rule.expression);
        let mut operators = Vec::new();
        let mut variables = Vec::new();
        
        // Scan for temporal operators
        for ch in rule_text.chars() {
            if let Some(op) = self.operators.get(&ch) {
                operators.push(op.clone());
            }
        }
        
        if operators.is_empty() {
            return Ok(None);
        }

        // Extract variables (simplified)
        if let Some(quantifier) = &rule.quantifier {
            variables.push(quantifier.variable.clone());
        }

        let formula = LtlFormula {
            id: format!("ltl_{:?}", rule.span),
            formula: rule_text,
            operators,
            variables,
            nesting_depth: 1, // TODO: Calculate actual nesting
            location: rule.span.clone(),
        };

        Ok(Some(formula))
    }

    /// Check LTL formula satisfiability (simplified)
    fn check_ltl_satisfiability(&self, _formula: &LtlFormula) -> AispResult<bool> {
        // TODO: Implement proper LTL satisfiability checking
        // This would typically use tableau methods or automata construction
        Ok(true) // Simplified assumption
    }

    /// Calculate formula complexity metrics
    fn calculate_formula_complexity(&self, formula: &LtlFormula) -> FormulaComplexity {
        FormulaComplexity {
            operator_count: formula.operators.len(),
            nesting_depth: formula.nesting_depth,
            variable_count: formula.variables.len(),
            cyclic_complexity: 1.0, // TODO: Calculate actual cyclic complexity
        }
    }

    /// Detect common temporal patterns
    fn detect_temporal_patterns(&self, formulas: &[LtlFormula]) -> AispResult<Vec<TemporalPattern>> {
        let mut patterns = Vec::new();

        for formula in formulas {
            // Detect safety pattern (□P)
            if formula.operators.len() == 1 && formula.operators[0] == TemporalOperator::Always {
                patterns.push(TemporalPattern {
                    pattern_type: PatternType::Safety,
                    description: "Global safety property".to_string(),
                    instances: vec![PatternInstance {
                        formula: formula.formula.clone(),
                        variables: formula.variables.clone(),
                        location: formula.location.clone(),
                        strength: 0.9,
                    }],
                    confidence: 0.9,
                });
            }

            // Detect liveness pattern (◊P)
            if formula.operators.len() == 1 && formula.operators[0] == TemporalOperator::Eventually {
                patterns.push(TemporalPattern {
                    pattern_type: PatternType::Liveness,
                    description: "Eventually property".to_string(),
                    instances: vec![PatternInstance {
                        formula: formula.formula.clone(),
                        variables: formula.variables.clone(),
                        location: formula.location.clone(),
                        strength: 0.8,
                    }],
                    confidence: 0.8,
                });
            }

            // TODO: Detect more complex patterns (response, persistence, etc.)
        }

        Ok(patterns)
    }

    /// Analyze Computation Tree Logic formulas
    fn analyze_ctl(&mut self, _doc: &AispDocument) -> AispResult<CtlAnalysis> {
        // TODO: Implement CTL analysis
        // This would involve path quantifier analysis and branching time logic
        Ok(CtlAnalysis {
            formulas: Vec::new(),
            path_quantifiers: Vec::new(),
            branching_analysis: BranchingAnalysis {
                avg_branching_factor: 1.0,
                max_branching_factor: 1,
                branching_distribution: HashMap::new(),
                nondeterministic_points: Vec::new(),
            },
        })
    }

    /// Analyze temporal constraints
    fn analyze_temporal_constraints(&mut self, _doc: &AispDocument) -> AispResult<TemporalConstraintAnalysis> {
        // TODO: Implement temporal constraint analysis
        Ok(TemporalConstraintAnalysis {
            constraints: Vec::new(),
            satisfaction: HashMap::new(),
            dependencies: HashMap::new(),
            ordering: Vec::new(),
        })
    }

    /// Analyze state space
    fn analyze_state_space(&mut self, _doc: &AispDocument) -> AispResult<StateSpaceAnalysis> {
        // TODO: Implement state space construction and analysis
        Ok(StateSpaceAnalysis {
            states: Vec::new(),
            transitions: Vec::new(),
            reachability: ReachabilityAnalysis {
                reachable_states: HashSet::new(),
                unreachable_states: HashSet::new(),
                sccs: Vec::new(),
                dead_states: HashSet::new(),
            },
            liveness: LivenessAnalysis {
                properties: Vec::new(),
                fairness: FairnessAnalysis {
                    fairness_constraints: Vec::new(),
                    fair_paths: Vec::new(),
                    violations: Vec::new(),
                },
                progress: Vec::new(),
            },
            safety: SafetyAnalysis {
                properties: Vec::new(),
                invariants: Vec::new(),
                bad_states: Vec::new(),
            },
        })
    }

    /// Perform model checking
    fn perform_model_checking(
        &mut self,
        _ltl: &LtlAnalysis,
        _ctl: &CtlAnalysis,
        _state_space: &StateSpaceAnalysis,
    ) -> AispResult<ModelCheckingResults> {
        // TODO: Implement model checking algorithms
        // This would include automata-based LTL checking and CTL model checking
        Ok(ModelCheckingResults {
            properties: Vec::new(),
            results: HashMap::new(),
            counterexamples: Vec::new(),
            witnesses: Vec::new(),
        })
    }

    /// Detect temporal conflicts
    fn detect_temporal_conflicts(
        &self,
        _ltl: &LtlAnalysis,
        _ctl: &CtlAnalysis,
        _constraints: &TemporalConstraintAnalysis,
    ) -> AispResult<Vec<TemporalConflict>> {
        // TODO: Implement conflict detection
        Ok(Vec::new())
    }

    /// Calculate temporal consistency score
    fn calculate_temporal_consistency(
        &self,
        ltl: &LtlAnalysis,
        _ctl: &CtlAnalysis,
        _constraints: &TemporalConstraintAnalysis,
        conflicts: &[TemporalConflict],
    ) -> f64 {
        let formula_score = if ltl.formulas.is_empty() {
            1.0
        } else {
            let satisfied = ltl.satisfiable.values().filter(|&&x| x).count();
            satisfied as f64 / ltl.formulas.len() as f64
        };

        let conflict_penalty = conflicts.iter()
            .map(|c| match c.severity {
                ConflictSeverity::Critical => 0.4,
                ConflictSeverity::Error => 0.3,
                ConflictSeverity::Major => 0.2,
                ConflictSeverity::Warning => 0.15,
                ConflictSeverity::Minor => 0.1,
                ConflictSeverity::Info => 0.05,
            })
            .sum::<f64>();

        (formula_score - conflict_penalty).max(0.0)
    }
}

impl Default for TemporalAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::AispParser;

    #[test]
    fn test_temporal_operator_display() {
        assert_eq!(format!("{}", TemporalOperator::Always), "□");
        assert_eq!(format!("{}", TemporalOperator::Eventually), "◊");
        assert_eq!(format!("{}", TemporalOperator::Next), "X");
        assert_eq!(format!("{}", TemporalOperator::Until), "U");
    }

    #[test]
    fn test_temporal_analyzer_creation() {
        let analyzer = TemporalAnalyzer::new();
        assert!(!analyzer.operators.is_empty());
        assert!(analyzer.operators.contains_key(&'□'));
        assert!(analyzer.operators.contains_key(&'◊'));
    }

    #[test]
    fn test_formula_complexity_calculation() {
        let analyzer = TemporalAnalyzer::new();
        let formula = LtlFormula {
            id: "test".to_string(),
            formula: "□(p → ◊q)".to_string(),
            operators: vec![TemporalOperator::Always, TemporalOperator::Eventually],
            variables: vec!["p".to_string(), "q".to_string()],
            nesting_depth: 2,
            location: Span::new(1, 1, 1, 10),
        };

        let complexity = analyzer.calculate_formula_complexity(&formula);
        assert_eq!(complexity.operator_count, 2);
        assert_eq!(complexity.variable_count, 2);
        assert_eq!(complexity.nesting_depth, 2);
    }
}