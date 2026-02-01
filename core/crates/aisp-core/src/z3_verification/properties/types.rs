//! Z3 Property Verification Types
//!
//! Core type definitions for Z3-based property verification.

use crate::{
    error::AispResult,
    proof_types::*,
};
use std::collections::HashMap;
use std::time::Duration;

/// Verified property with Z3-backed proof
#[derive(Debug, Clone)]
pub struct VerifiedProperty {
    /// Unique property identifier
    pub id: String,
    /// Property category
    pub category: PropertyCategory,
    /// Human-readable description
    pub description: String,
    /// SMT formula used for verification
    pub smt_formula: String,
    /// Verification result
    pub result: PropertyResult,
    /// Time taken for verification
    pub verification_time: Duration,
    /// Proof certificate for traceability
    pub proof_certificate: String,
}

/// Categories of properties that can be verified
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PropertyCategory {
    /// Tri-vector orthogonality constraints
    TriVectorOrthogonality,
    /// Temporal logic properties
    TemporalLogic,
    /// Type safety properties
    TypeSafety,
    /// Mathematical consistency
    MathematicalConsistency,
    /// Protocol correctness
    ProtocolCorrectness,
    /// Security properties
    Security,
    /// Performance properties
    Performance,
    /// Custom property category
    Custom(String),
}

/// Result of property verification
#[derive(Debug, Clone, PartialEq)]
pub enum PropertyResult {
    /// Property is proven true
    Proven,
    /// Property is disproven (counterexample found)
    Disproven,
    /// Result is unknown (timeout, resource limits)
    Unknown,
    /// Error during verification
    Error(String),
    /// Property type not supported
    Unsupported,
}

/// Enhanced verification statistics
#[derive(Debug, Default, Clone)]
pub struct EnhancedVerificationStats {
    /// Number of successful proofs
    pub successful_proofs: usize,
    /// Number of counterexamples found
    pub counterexamples: usize,
    /// Number of SMT queries executed
    pub smt_queries: usize,
    /// Total verification time
    pub total_time: Duration,
    /// Memory usage statistics
    pub memory_usage: MemoryStats,
    /// Solver performance metrics
    pub solver_metrics: SolverMetrics,
    /// Property complexity distribution
    pub complexity_distribution: HashMap<ComplexityLevel, usize>,
}

/// Memory usage statistics
#[derive(Debug, Default, Clone)]
pub struct MemoryStats {
    /// Peak memory usage in bytes
    pub peak_memory: usize,
    /// Average memory usage in bytes
    pub average_memory: usize,
    /// Memory usage per property category
    pub category_memory: HashMap<PropertyCategory, usize>,
}

/// Solver performance metrics
#[derive(Debug, Default, Clone)]
pub struct SolverMetrics {
    /// Number of solver restarts
    pub restarts: usize,
    /// Number of lemmas learned
    pub learned_lemmas: usize,
    /// Number of conflicts encountered
    pub conflicts: usize,
    /// Average query complexity
    pub avg_query_complexity: f64,
    /// Solver efficiency score
    pub efficiency_score: f64,
}

/// Property complexity levels
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ComplexityLevel {
    Low,
    Medium,
    High,
    VeryHigh,
    Extreme,
}

/// Advanced verification configuration
#[derive(Debug, Clone)]
pub struct AdvancedVerificationConfig {
    /// Timeout for individual SMT queries
    pub smt_timeout: Duration,
    /// Maximum memory limit for verification
    pub memory_limit: usize,
    /// Enable proof generation
    pub generate_proofs: bool,
    /// Enable counterexample generation
    pub generate_counterexamples: bool,
    /// Solver strategy configuration
    pub solver_strategy: SolverStrategy,
    /// Parallel verification settings
    pub parallel_config: ParallelConfig,
    /// Optimization settings
    pub optimization: OptimizationConfig,
    /// Debugging configuration
    pub debug_config: DebugConfig,
}

/// Solver strategy configuration
#[derive(Debug, Clone)]
pub struct SolverStrategy {
    /// Primary solving strategy
    pub primary_strategy: StrategyType,
    /// Fallback strategies
    pub fallback_strategies: Vec<StrategyType>,
    /// Strategy selection criteria
    pub selection_criteria: StrategySelection,
    /// Custom solver parameters
    pub custom_params: HashMap<String, String>,
}

/// Types of solving strategies
#[derive(Debug, Clone, PartialEq)]
pub enum StrategyType {
    /// Quantifier-free linear arithmetic
    QFLIA,
    /// Quantifier-free nonlinear arithmetic
    QFNIA,
    /// Quantifier-free linear real arithmetic
    QFLRA,
    /// Arrays with extensionality
    ArraysEx,
    /// Bit-vectors
    QFBv,
    /// Uninterpreted functions
    QFUf,
    /// Custom strategy
    Custom(String),
}

/// Strategy selection criteria
#[derive(Debug, Clone)]
pub struct StrategySelection {
    /// Property complexity threshold
    pub complexity_threshold: f64,
    /// Timeout-based selection
    pub timeout_based: bool,
    /// Memory-based selection
    pub memory_based: bool,
    /// Historical performance weighting
    pub performance_weighting: f64,
}

/// Parallel verification configuration
#[derive(Debug, Clone)]
pub struct ParallelConfig {
    /// Enable parallel verification
    pub enabled: bool,
    /// Number of worker threads
    pub worker_threads: usize,
    /// Load balancing strategy
    pub load_balancing: LoadBalancingStrategy,
    /// Inter-thread communication settings
    pub communication: CommunicationConfig,
}

/// Load balancing strategies
#[derive(Debug, Clone, PartialEq)]
pub enum LoadBalancingStrategy {
    RoundRobin,
    WorkStealing,
    ComplexityBased,
    Dynamic,
}

/// Inter-thread communication configuration
#[derive(Debug, Clone)]
pub struct CommunicationConfig {
    /// Shared state synchronization
    pub sync_frequency: Duration,
    /// Result sharing enabled
    pub share_results: bool,
    /// Lemma sharing enabled
    pub share_lemmas: bool,
    /// Communication overhead threshold
    pub overhead_threshold: f64,
}

/// Optimization configuration
#[derive(Debug, Clone)]
pub struct OptimizationConfig {
    /// Enable formula simplification
    pub simplify_formulas: bool,
    /// Enable formula caching
    pub cache_formulas: bool,
    /// Enable incremental solving
    pub incremental_solving: bool,
    /// Cache configuration
    pub cache_config: CacheConfig,
    /// Preprocessing options
    pub preprocessing: PreprocessingOptions,
}

/// Formula caching configuration
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Maximum cache size
    pub max_size: usize,
    /// Cache eviction policy
    pub eviction_policy: EvictionPolicy,
    /// Cache hit ratio target
    pub target_hit_ratio: f64,
}

/// Cache eviction policies
#[derive(Debug, Clone, PartialEq)]
pub enum EvictionPolicy {
    LRU,
    LFU,
    FIFO,
    Random,
    Adaptive,
}

/// Formula preprocessing options
#[derive(Debug, Clone)]
pub struct PreprocessingOptions {
    /// Enable algebraic simplification
    pub algebraic_simplification: bool,
    /// Enable term rewriting
    pub term_rewriting: bool,
    /// Enable quantifier elimination
    pub quantifier_elimination: bool,
    /// Enable clause learning
    pub clause_learning: bool,
}

/// Debug configuration
#[derive(Debug, Clone)]
pub struct DebugConfig {
    /// Enable verbose logging
    pub verbose: bool,
    /// Log SMT formulas
    pub log_formulas: bool,
    /// Log solver statistics
    pub log_statistics: bool,
    /// Enable proof tracing
    pub trace_proofs: bool,
    /// Debug output directory
    pub debug_dir: Option<String>,
}

/// Property verification context
#[derive(Debug)]
pub struct PropertyVerificationContext {
    /// Current verification session
    pub session_id: String,
    /// Active property verifications
    pub active_verifications: HashMap<String, PropertyVerificationTask>,
    /// Shared verification state
    pub shared_state: SharedVerificationState,
    /// Context statistics
    pub context_stats: ContextStatistics,
}

/// Individual property verification task
#[derive(Debug, Clone)]
pub struct PropertyVerificationTask {
    /// Task identifier
    pub task_id: String,
    /// Property being verified
    pub property: VerifiedProperty,
    /// Task status
    pub status: TaskStatus,
    /// Start time
    pub start_time: std::time::Instant,
    /// Resource usage
    pub resource_usage: TaskResourceUsage,
}

/// Status of verification task
#[derive(Debug, Clone, PartialEq)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed(PropertyResult),
    Failed(String),
    Cancelled,
    Timeout,
}

/// Resource usage for individual task
#[derive(Debug, Clone)]
pub struct TaskResourceUsage {
    /// CPU time consumed
    pub cpu_time: Duration,
    /// Memory usage
    pub memory_usage: usize,
    /// Number of solver calls
    pub solver_calls: usize,
    /// Formula complexity
    pub formula_complexity: f64,
}

/// Shared verification state
#[derive(Debug)]
pub struct SharedVerificationState {
    /// Shared lemma database
    pub lemma_database: LemmaDatabase,
    /// Shared counterexample database
    pub counterexample_database: CounterexampleDatabase,
    /// Performance metrics
    pub performance_metrics: PerformanceMetrics,
}

/// Database of learned lemmas
#[derive(Debug)]
pub struct LemmaDatabase {
    /// Stored lemmas
    pub lemmas: HashMap<String, Lemma>,
    /// Lemma usage statistics
    pub usage_stats: HashMap<String, LemmaUsageStats>,
    /// Lemma effectiveness scores
    pub effectiveness_scores: HashMap<String, f64>,
}

/// Individual lemma
#[derive(Debug, Clone)]
pub struct Lemma {
    /// Lemma identifier
    pub id: String,
    /// Lemma formula
    pub formula: String,
    /// Property categories where applicable
    pub applicable_categories: Vec<PropertyCategory>,
    /// Learning context
    pub context: String,
    /// Quality score
    pub quality_score: f64,
}

/// Lemma usage statistics
#[derive(Debug, Clone)]
pub struct LemmaUsageStats {
    /// Times used
    pub usage_count: usize,
    /// Success rate
    pub success_rate: f64,
    /// Average time savings
    pub avg_time_savings: Duration,
    /// Last used timestamp
    pub last_used: std::time::Instant,
}

/// Database of counterexamples
#[derive(Debug)]
pub struct CounterexampleDatabase {
    /// Stored counterexamples
    pub counterexamples: HashMap<String, Counterexample>,
    /// Counterexample patterns
    pub patterns: Vec<CounterexamplePattern>,
}

/// Individual counterexample
#[derive(Debug, Clone)]
pub struct Counterexample {
    /// Counterexample identifier
    pub id: String,
    /// Property that was disproven
    pub property_id: String,
    /// Variable assignments
    pub assignments: HashMap<String, String>,
    /// Counterexample explanation
    pub explanation: String,
    /// Discovery timestamp
    pub discovered: std::time::Instant,
}

/// Counterexample pattern
#[derive(Debug, Clone)]
pub struct CounterexamplePattern {
    /// Pattern identifier
    pub id: String,
    /// Pattern description
    pub description: String,
    /// Property categories affected
    pub affected_categories: Vec<PropertyCategory>,
    /// Pattern frequency
    pub frequency: usize,
}

/// Performance metrics tracking
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    /// Throughput metrics
    pub throughput: ThroughputMetrics,
    /// Latency metrics
    pub latency: LatencyMetrics,
    /// Resource efficiency metrics
    pub efficiency: EfficiencyMetrics,
}

/// Throughput measurements
#[derive(Debug, Clone)]
pub struct ThroughputMetrics {
    /// Properties verified per second
    pub properties_per_second: f64,
    /// SMT queries per second
    pub queries_per_second: f64,
    /// Peak throughput
    pub peak_throughput: f64,
    /// Average throughput
    pub average_throughput: f64,
}

/// Latency measurements
#[derive(Debug, Clone)]
pub struct LatencyMetrics {
    /// Average verification latency
    pub average_latency: Duration,
    /// 95th percentile latency
    pub p95_latency: Duration,
    /// 99th percentile latency
    pub p99_latency: Duration,
    /// Maximum latency observed
    pub max_latency: Duration,
}

/// Efficiency measurements
#[derive(Debug, Clone)]
pub struct EfficiencyMetrics {
    /// CPU utilization
    pub cpu_utilization: f64,
    /// Memory efficiency
    pub memory_efficiency: f64,
    /// Cache hit ratio
    pub cache_hit_ratio: f64,
    /// Solver efficiency
    pub solver_efficiency: f64,
}

/// Context statistics
#[derive(Debug, Clone)]
pub struct ContextStatistics {
    /// Total properties verified
    pub total_properties: usize,
    /// Success rate
    pub success_rate: f64,
    /// Average verification time
    pub avg_verification_time: Duration,
    /// Resource usage distribution
    pub resource_distribution: HashMap<String, f64>,
}

impl Default for AdvancedVerificationConfig {
    fn default() -> Self {
        Self {
            smt_timeout: Duration::from_secs(30),
            memory_limit: 1024 * 1024 * 1024, // 1GB
            generate_proofs: true,
            generate_counterexamples: true,
            solver_strategy: SolverStrategy::default(),
            parallel_config: ParallelConfig::default(),
            optimization: OptimizationConfig::default(),
            debug_config: DebugConfig::default(),
        }
    }
}

impl Default for SolverStrategy {
    fn default() -> Self {
        Self {
            primary_strategy: StrategyType::QFLIA,
            fallback_strategies: vec![
                StrategyType::QFNIA,
                StrategyType::QFLRA,
            ],
            selection_criteria: StrategySelection {
                complexity_threshold: 0.8,
                timeout_based: true,
                memory_based: true,
                performance_weighting: 0.7,
            },
            custom_params: HashMap::new(),
        }
    }
}

impl Default for ParallelConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            worker_threads: num_cpus::get(),
            load_balancing: LoadBalancingStrategy::Dynamic,
            communication: CommunicationConfig {
                sync_frequency: Duration::from_millis(100),
                share_results: true,
                share_lemmas: true,
                overhead_threshold: 0.1,
            },
        }
    }
}

impl Default for OptimizationConfig {
    fn default() -> Self {
        Self {
            simplify_formulas: true,
            cache_formulas: true,
            incremental_solving: true,
            cache_config: CacheConfig {
                max_size: 10000,
                eviction_policy: EvictionPolicy::LRU,
                target_hit_ratio: 0.8,
            },
            preprocessing: PreprocessingOptions {
                algebraic_simplification: true,
                term_rewriting: true,
                quantifier_elimination: false,
                clause_learning: true,
            },
        }
    }
}

impl Default for DebugConfig {
    fn default() -> Self {
        Self {
            verbose: false,
            log_formulas: false,
            log_statistics: true,
            trace_proofs: false,
            debug_dir: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_property_category_equality() {
        assert_eq!(PropertyCategory::TriVectorOrthogonality, PropertyCategory::TriVectorOrthogonality);
        assert_ne!(PropertyCategory::TriVectorOrthogonality, PropertyCategory::TemporalLogic);
        
        let custom1 = PropertyCategory::Custom("test".to_string());
        let custom2 = PropertyCategory::Custom("test".to_string());
        assert_eq!(custom1, custom2);
    }

    #[test]
    fn test_property_result_equality() {
        assert_eq!(PropertyResult::Proven, PropertyResult::Proven);
        assert_ne!(PropertyResult::Proven, PropertyResult::Disproven);
        
        let error1 = PropertyResult::Error("test".to_string());
        let error2 = PropertyResult::Error("test".to_string());
        assert_eq!(error1, error2);
    }

    #[test]
    fn test_complexity_level_ordering() {
        assert!(ComplexityLevel::Low < ComplexityLevel::Extreme);
        assert!(ComplexityLevel::Medium < ComplexityLevel::High);
        assert!(ComplexityLevel::High < ComplexityLevel::VeryHigh);
    }

    #[test]
    fn test_strategy_type_variants() {
        assert_eq!(StrategyType::QFLIA, StrategyType::QFLIA);
        assert_ne!(StrategyType::QFLIA, StrategyType::QFNIA);
        
        let custom = StrategyType::Custom("custom".to_string());
        assert!(matches!(custom, StrategyType::Custom(_)));
    }

    #[test]
    fn test_task_status_variants() {
        assert_eq!(TaskStatus::Pending, TaskStatus::Pending);
        assert_ne!(TaskStatus::Pending, TaskStatus::Running);
        
        let completed = TaskStatus::Completed(PropertyResult::Proven);
        assert!(matches!(completed, TaskStatus::Completed(_)));
        
        let failed = TaskStatus::Failed("error".to_string());
        assert!(matches!(failed, TaskStatus::Failed(_)));
    }

    #[test]
    fn test_load_balancing_strategies() {
        assert_eq!(LoadBalancingStrategy::RoundRobin, LoadBalancingStrategy::RoundRobin);
        assert_ne!(LoadBalancingStrategy::RoundRobin, LoadBalancingStrategy::WorkStealing);
    }

    #[test]
    fn test_eviction_policies() {
        assert_eq!(EvictionPolicy::LRU, EvictionPolicy::LRU);
        assert_ne!(EvictionPolicy::LRU, EvictionPolicy::LFU);
    }

    #[test]
    fn test_default_configuration() {
        let config = AdvancedVerificationConfig::default();
        assert_eq!(config.smt_timeout, Duration::from_secs(30));
        assert!(config.generate_proofs);
        assert!(config.generate_counterexamples);
        assert!(config.parallel_config.enabled);
        assert!(config.optimization.simplify_formulas);
        assert!(!config.debug_config.verbose);
    }

    #[test]
    fn test_solver_strategy_defaults() {
        let strategy = SolverStrategy::default();
        assert_eq!(strategy.primary_strategy, StrategyType::QFLIA);
        assert_eq!(strategy.fallback_strategies.len(), 2);
        assert!(strategy.selection_criteria.timeout_based);
        assert!(strategy.selection_criteria.memory_based);
    }

    #[test]
    fn test_performance_metrics_structure() {
        let metrics = PerformanceMetrics {
            throughput: ThroughputMetrics {
                properties_per_second: 10.0,
                queries_per_second: 100.0,
                peak_throughput: 20.0,
                average_throughput: 8.0,
            },
            latency: LatencyMetrics {
                average_latency: Duration::from_millis(100),
                p95_latency: Duration::from_millis(200),
                p99_latency: Duration::from_millis(500),
                max_latency: Duration::from_millis(1000),
            },
            efficiency: EfficiencyMetrics {
                cpu_utilization: 0.8,
                memory_efficiency: 0.7,
                cache_hit_ratio: 0.9,
                solver_efficiency: 0.85,
            },
        };
        
        assert_eq!(metrics.throughput.properties_per_second, 10.0);
        assert_eq!(metrics.latency.average_latency, Duration::from_millis(100));
        assert_eq!(metrics.efficiency.cpu_utilization, 0.8);
    }
}