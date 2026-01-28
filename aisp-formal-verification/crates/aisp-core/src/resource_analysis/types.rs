//! Resource Analysis Core Types
//!
//! Focused types module for resource utilization analysis under 300 LOC.

use std::collections::HashMap;
use std::time::Duration;

/// Types of system resources
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResourceType {
    /// CPU processing power
    CPU,
    /// System memory (RAM)
    Memory,
    /// Network bandwidth
    Network,
    /// Storage/disk space
    Storage,
    /// GPU processing power
    GPU,
    /// Database connections
    DatabaseConnections,
    /// Thread pool
    ThreadPool,
    /// File descriptors
    FileDescriptors,
    /// Custom resource type
    Custom(String),
}

/// Resource measurement units
#[derive(Debug, Clone, PartialEq)]
pub enum ResourceUnit {
    /// Percentage (0-100)
    Percentage,
    /// Bytes
    Bytes,
    /// Bytes per second
    BytesPerSecond,
    /// Operations per second
    OperationsPerSecond,
    /// Count/number of items
    Count,
    /// Milliseconds
    Milliseconds,
    /// Custom unit
    Custom(String),
}

/// Resource instance information
#[derive(Debug, Clone)]
pub struct ResourceInstance {
    /// Resource identifier
    pub id: String,
    /// Resource type
    pub resource_type: ResourceType,
    /// Resource name/description
    pub name: String,
    /// Total capacity
    pub total_capacity: f64,
    /// Currently allocated
    pub allocated: f64,
    /// Currently in use
    pub used: f64,
    /// Available for allocation
    pub available: f64,
    /// Measurement units
    pub units: ResourceUnit,
    /// Resource location/node
    pub location: Option<String>,
    /// Resource metadata
    pub metadata: HashMap<String, String>,
}

/// Resource utilization metrics
#[derive(Debug, Clone)]
pub struct ResourceMetrics {
    /// Current utilization percentage
    pub current_utilization: f64,
    /// Average utilization over analysis period
    pub average_utilization: f64,
    /// Peak utilization observed
    pub peak_utilization: f64,
    /// Minimum utilization observed
    pub minimum_utilization: f64,
    /// Utilization variance
    pub utilization_variance: f64,
    /// Number of measurements
    pub measurement_count: usize,
    /// Time series data points
    pub time_series: Vec<UtilizationDataPoint>,
}

/// Single utilization measurement
#[derive(Debug, Clone)]
pub struct UtilizationDataPoint {
    /// Timestamp of measurement
    pub timestamp: Duration,
    /// Utilization value
    pub utilization: f64,
    /// Associated context
    pub context: Option<String>,
}

/// Resource utilization thresholds
#[derive(Debug, Clone)]
pub struct UtilizationThresholds {
    /// Warning threshold (typically 70-80%)
    pub warning_threshold: f64,
    /// Critical threshold (typically 85-95%)
    pub critical_threshold: f64,
    /// Optimal utilization target
    pub optimal_target: f64,
    /// Minimum acceptable utilization
    pub minimum_acceptable: f64,
}

/// Resource allocation pattern
#[derive(Debug, Clone)]
pub struct AllocationPattern {
    /// Pattern identifier
    pub id: String,
    /// Pattern type
    pub pattern_type: AllocationPatternType,
    /// Pattern description
    pub description: String,
    /// Frequency of occurrence
    pub frequency: PatternFrequency,
    /// Confidence in pattern detection
    pub confidence: f64,
    /// Associated resources
    pub associated_resources: Vec<String>,
    /// Pattern impact assessment
    pub impact: PatternImpact,
}

/// Types of allocation patterns
#[derive(Debug, Clone, PartialEq)]
pub enum AllocationPatternType {
    /// Burst allocation pattern
    BurstAllocation,
    /// Steady-state allocation
    SteadyState,
    /// Periodic allocation cycles
    PeriodicCycles,
    /// Memory leak pattern
    MemoryLeak,
    /// Resource hoarding
    ResourceHoarding,
    /// Inefficient allocation
    InefficientAllocation,
    /// Fragmented allocation
    FragmentedAllocation,
    /// Unknown pattern
    Unknown,
}

/// Pattern occurrence frequency
#[derive(Debug, Clone, PartialEq)]
pub enum PatternFrequency {
    /// Very rare occurrence
    VeryRare,
    /// Rare occurrence
    Rare,
    /// Occasional occurrence
    Occasional,
    /// Regular occurrence
    Regular,
    /// Frequent occurrence
    Frequent,
    /// Constant occurrence
    Constant,
}

/// Pattern impact assessment
#[derive(Debug, Clone)]
pub struct PatternImpact {
    /// Performance impact score (-1.0 to 1.0)
    pub performance_impact: f64,
    /// Efficiency impact score (-1.0 to 1.0)
    pub efficiency_impact: f64,
    /// Reliability impact score (-1.0 to 1.0)
    pub reliability_impact: f64,
    /// Overall impact score (-1.0 to 1.0)
    pub overall_impact: f64,
    /// Impact description
    pub description: String,
}

/// Resource bottleneck information
#[derive(Debug, Clone)]
pub struct ResourceBottleneck {
    /// Bottleneck identifier
    pub id: String,
    /// Bottlenecked resource
    pub resource_id: String,
    /// Bottleneck severity
    pub severity: BottleneckSeverity,
    /// Impact on throughput
    pub throughput_impact: f64,
    /// Impact on latency
    pub latency_impact: f64,
    /// Bottleneck duration
    pub duration: Duration,
    /// Root cause analysis
    pub root_cause: String,
    /// Recommended mitigation
    pub mitigation_recommendations: Vec<String>,
}

/// Bottleneck severity levels
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum BottleneckSeverity {
    /// Low severity bottleneck
    Low,
    /// Medium severity bottleneck
    Medium,
    /// High severity bottleneck
    High,
    /// Critical severity bottleneck
    Critical,
}

/// Optimization recommendation
#[derive(Debug, Clone)]
pub struct OptimizationRecommendation {
    /// Recommendation identifier
    pub id: String,
    /// Target resource
    pub target_resource: String,
    /// Optimization type
    pub optimization_type: OptimizationType,
    /// Expected benefit
    pub expected_benefit: OptimizationBenefit,
    /// Implementation complexity
    pub complexity: ImplementationComplexity,
    /// Priority level
    pub priority: RecommendationPriority,
    /// Detailed recommendation
    pub recommendation: String,
    /// Prerequisites
    pub prerequisites: Vec<String>,
}

/// Types of optimizations
#[derive(Debug, Clone, PartialEq)]
pub enum OptimizationType {
    /// Scale up resources
    ScaleUp,
    /// Scale down resources
    ScaleDown,
    /// Better resource allocation
    ReallocationStrategy,
    /// Caching improvements
    CachingOptimization,
    /// Algorithm optimization
    AlgorithmOptimization,
    /// Configuration tuning
    ConfigurationTuning,
    /// Hardware upgrade
    HardwareUpgrade,
    /// Custom optimization
    Custom(String),
}

/// Expected optimization benefit
#[derive(Debug, Clone)]
pub struct OptimizationBenefit {
    /// Performance improvement percentage
    pub performance_improvement: f64,
    /// Cost reduction percentage
    pub cost_reduction: f64,
    /// Efficiency improvement percentage
    pub efficiency_improvement: f64,
    /// Reliability improvement percentage
    pub reliability_improvement: f64,
}

/// Implementation complexity levels
#[derive(Debug, Clone, PartialEq)]
pub enum ImplementationComplexity {
    /// Very easy to implement
    VeryEasy,
    /// Easy to implement
    Easy,
    /// Moderate complexity
    Moderate,
    /// Complex implementation
    Complex,
    /// Very complex implementation
    VeryComplex,
}

/// Recommendation priority levels
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum RecommendationPriority {
    /// Low priority recommendation
    Low,
    /// Medium priority recommendation
    Medium,
    /// High priority recommendation
    High,
    /// Critical priority recommendation
    Critical,
    /// Emergency priority recommendation
    Emergency,
}

impl ResourceInstance {
    /// Calculate utilization percentage
    pub fn utilization_percentage(&self) -> f64 {
        if self.total_capacity > 0.0 {
            (self.used / self.total_capacity) * 100.0
        } else {
            0.0
        }
    }

    /// Calculate allocation percentage
    pub fn allocation_percentage(&self) -> f64 {
        if self.total_capacity > 0.0 {
            (self.allocated / self.total_capacity) * 100.0
        } else {
            0.0
        }
    }

    /// Check if resource is over-utilized
    pub fn is_over_utilized(&self, threshold: f64) -> bool {
        self.utilization_percentage() > threshold
    }

    /// Check if resource is under-utilized
    pub fn is_under_utilized(&self, threshold: f64) -> bool {
        self.utilization_percentage() < threshold
    }
}

impl Default for UtilizationThresholds {
    fn default() -> Self {
        Self {
            warning_threshold: 75.0,
            critical_threshold: 90.0,
            optimal_target: 60.0,
            minimum_acceptable: 10.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_type_equality() {
        assert_eq!(ResourceType::CPU, ResourceType::CPU);
        assert_ne!(ResourceType::CPU, ResourceType::Memory);
        assert_eq!(ResourceType::Custom("cache".to_string()), 
                  ResourceType::Custom("cache".to_string()));
    }

    #[test]
    fn test_resource_instance_utilization() {
        let resource = ResourceInstance {
            id: "cpu-01".to_string(),
            resource_type: ResourceType::CPU,
            name: "Main CPU".to_string(),
            total_capacity: 100.0,
            allocated: 80.0,
            used: 60.0,
            available: 20.0,
            units: ResourceUnit::Percentage,
            location: None,
            metadata: HashMap::new(),
        };

        assert_eq!(resource.utilization_percentage(), 60.0);
        assert_eq!(resource.allocation_percentage(), 80.0);
        assert!(!resource.is_over_utilized(75.0));
        assert!(resource.is_over_utilized(50.0));
    }

    #[test]
    fn test_bottleneck_severity_ordering() {
        use BottleneckSeverity::*;
        assert!(Low < Medium);
        assert!(Medium < High);
        assert!(High < Critical);
    }

    #[test]
    fn test_recommendation_priority_ordering() {
        use RecommendationPriority::*;
        assert!(Low < Medium);
        assert!(Medium < High);
        assert!(High < Critical);
        assert!(Critical < Emergency);
    }

    #[test]
    fn test_default_thresholds() {
        let thresholds = UtilizationThresholds::default();
        assert_eq!(thresholds.warning_threshold, 75.0);
        assert_eq!(thresholds.critical_threshold, 90.0);
        assert_eq!(thresholds.optimal_target, 60.0);
        assert_eq!(thresholds.minimum_acceptable, 10.0);
    }
}