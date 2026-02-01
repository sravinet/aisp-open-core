//! Multi-Layer Verification Pipeline Implementation
//!
//! This module implements a comprehensive verification pipeline following Single Responsibility 
//! Principle with focused modules for different verification aspects.

pub mod core_types;
pub mod pipeline_orchestrator; 
pub mod security_enforcer;
pub mod compliance_auditor;
pub mod performance_monitor;
pub mod adversarial_testing;
pub mod main_pipeline;

// Re-export all public items
pub use core_types::*;
pub use pipeline_orchestrator::*;
pub use security_enforcer::*;
pub use compliance_auditor::*;
pub use performance_monitor::*;
pub use adversarial_testing::*;
pub use main_pipeline::*;