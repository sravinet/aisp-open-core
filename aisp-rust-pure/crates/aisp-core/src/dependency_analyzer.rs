//! Dependency analysis for AISP documents
//!
//! This module analyzes dependencies between components including types,
//! functions, and rules to detect circular dependencies and build
//! topological ordering for validation.

use crate::ast::*;
use crate::error::*;
use std::collections::{HashMap, HashSet, VecDeque};

/// Dependency analyzer for AISP components
pub struct DependencyAnalyzer {
    /// Component dependencies map
    dependencies: HashMap<String, Vec<String>>,
    /// Reverse dependency map for quick lookups
    reverse_deps: HashMap<String, Vec<String>>,
    /// All component names
    components: HashSet<String>,
}

/// Component dependency information
#[derive(Debug, Clone)]
pub struct ComponentDependency {
    /// Component name
    pub name: String,
    /// Direct dependencies
    pub dependencies: Vec<String>,
    /// Dependency type
    pub dep_type: DependencyType,
    /// Source location
    pub span: Span,
}

/// Types of dependencies
#[derive(Debug, Clone, PartialEq)]
pub enum DependencyType {
    /// Type reference dependency
    TypeReference,
    /// Function call dependency
    FunctionCall,
    /// Parameter type dependency
    ParameterType,
    /// Return type dependency
    ReturnType,
    /// Meta constraint dependency
    MetaConstraint,
}

/// Circular dependency chain
#[derive(Debug, Clone)]
pub struct CircularDependency {
    /// Components in the cycle
    pub cycle: Vec<String>,
    /// Dependency types involved
    pub dep_types: Vec<DependencyType>,
    /// Severity of the cycle
    pub severity: CycleSeverity,
    /// Resolution suggestion
    pub resolution: Option<String>,
}

/// Severity of circular dependencies
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CycleSeverity {
    /// Can be resolved automatically
    Minor,
    /// Requires manual intervention
    Major,
    /// Prevents compilation
    Critical,
}

/// Result of dependency analysis
#[derive(Debug, Clone)]
pub struct DependencyAnalysisResult {
    /// All component dependencies
    pub dependencies: HashMap<String, Vec<String>>,
    /// Detected circular dependencies
    pub circular_deps: Vec<CircularDependency>,
    /// Topological ordering of components
    pub topological_order: Vec<String>,
    /// Unreachable (dead) components
    pub unreachable: Vec<String>,
    /// Dependency depth map
    pub depth_map: HashMap<String, usize>,
    /// Components with no dependencies
    pub leaf_components: Vec<String>,
}

impl DependencyAnalyzer {
    /// Create a new dependency analyzer
    pub fn new() -> Self {
        Self {
            dependencies: HashMap::new(),
            reverse_deps: HashMap::new(),
            components: HashSet::new(),
        }
    }

    /// Analyze dependencies in the document
    pub fn analyze_document(&mut self, document: &AispDocument) -> DependencyAnalysisResult {
        self.dependencies.clear();
        self.reverse_deps.clear();
        self.components.clear();

        // Extract all components and their dependencies
        self.extract_dependencies(document);

        // Build reverse dependency map
        self.build_reverse_dependencies();

        // Detect circular dependencies
        let circular_deps = self.detect_circular_dependencies();

        // Calculate topological ordering
        let topological_order = self.topological_sort();

        // Find unreachable components
        let unreachable = self.find_unreachable_components();

        // Calculate dependency depths
        let depth_map = self.calculate_dependency_depths();

        // Find leaf components (no dependencies)
        let leaf_components = self.find_leaf_components();

        DependencyAnalysisResult {
            dependencies: self.dependencies.clone(),
            circular_deps,
            topological_order,
            unreachable,
            depth_map,
            leaf_components,
        }
    }

    /// Extract dependencies from all document blocks
    fn extract_dependencies(&mut self, document: &AispDocument) {
        for block in &document.blocks {
            match block {
                AispBlock::Types(types_block) => {
                    self.extract_type_dependencies(types_block);
                }
                AispBlock::Functions(functions_block) => {
                    self.extract_function_dependencies(functions_block);
                }
                AispBlock::Rules(rules_block) => {
                    self.extract_rule_dependencies(rules_block);
                }
                AispBlock::Meta(meta_block) => {
                    self.extract_meta_dependencies(meta_block);
                }
                _ => {}
            }
        }
    }

    /// Extract dependencies from type definitions
    fn extract_type_dependencies(&mut self, types_block: &TypesBlock) {
        for (name, definition) in &types_block.definitions {
            self.components.insert(name.clone());
            let deps = self.extract_type_expression_dependencies(&definition.type_expr);
            self.dependencies.insert(name.clone(), deps);
        }
    }

    /// Extract dependencies from a type expression
    fn extract_type_expression_dependencies(&self, type_expr: &TypeExpression) -> Vec<String> {
        let mut deps = Vec::new();

        match type_expr {
            TypeExpression::Reference(type_name) => {
                deps.push(type_name.clone());
            }
            TypeExpression::Array { element_type, .. } => {
                deps.extend(self.extract_type_expression_dependencies(element_type));
            }
            TypeExpression::Function { input, output } => {
                deps.extend(self.extract_type_expression_dependencies(input));
                deps.extend(self.extract_type_expression_dependencies(output));
            }
            TypeExpression::Tuple(elements) => {
                for element in elements {
                    deps.extend(self.extract_type_expression_dependencies(element));
                }
            }
            TypeExpression::Generic { .. } => {
                // TODO: Handle generic type dependencies
            }
            _ => {} // Basic types and enumerations have no dependencies
        }

        deps
    }

    /// Extract dependencies from function definitions
    fn extract_function_dependencies(&mut self, functions_block: &FunctionsBlock) {
        for (name, function) in &functions_block.functions {
            self.components.insert(name.clone());
            let mut deps = Vec::new();

            // TODO: Parse function body for dependencies
            // For now, simplified dependency extraction

            self.dependencies.insert(name.clone(), deps);
        }
    }

    /// Extract dependencies from logical rules
    fn extract_rule_dependencies(&mut self, _rules_block: &RulesBlock) {
        // TODO: Extract dependencies from rule expressions
        // Rules might reference types, functions, or other rules
    }

    /// Extract dependencies from meta information
    fn extract_meta_dependencies(&mut self, _meta_block: &MetaBlock) {
        // TODO: Extract dependencies from meta constraints
        // Meta blocks might reference types or define constraints
    }

    /// Build reverse dependency map
    fn build_reverse_dependencies(&mut self) {
        self.reverse_deps.clear();

        for (component, deps) in &self.dependencies {
            for dep in deps {
                self.reverse_deps
                    .entry(dep.clone())
                    .or_insert_with(Vec::new)
                    .push(component.clone());
            }
        }
    }

    /// Detect circular dependencies using DFS
    fn detect_circular_dependencies(&self) -> Vec<CircularDependency> {
        let mut cycles = Vec::new();
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        let mut path = Vec::new();

        for component in &self.components {
            if !visited.contains(component) {
                self.dfs_cycle_detection(
                    component,
                    &mut visited,
                    &mut rec_stack,
                    &mut path,
                    &mut cycles,
                );
            }
        }

        cycles
    }

    /// DFS helper for cycle detection
    fn dfs_cycle_detection(
        &self,
        node: &str,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
        path: &mut Vec<String>,
        cycles: &mut Vec<CircularDependency>,
    ) {
        visited.insert(node.to_string());
        rec_stack.insert(node.to_string());
        path.push(node.to_string());

        if let Some(dependencies) = self.dependencies.get(node) {
            for dep in dependencies {
                if !visited.contains(dep) {
                    self.dfs_cycle_detection(dep, visited, rec_stack, path, cycles);
                } else if rec_stack.contains(dep) {
                    // Found a cycle
                    if let Some(cycle_start) = path.iter().position(|x| x == dep) {
                        let cycle = path[cycle_start..].to_vec();
                        let severity = self.assess_cycle_severity(&cycle);
                        
                        cycles.push(CircularDependency {
                            cycle,
                            dep_types: vec![DependencyType::TypeReference], // Simplified
                            severity,
                            resolution: Some(self.suggest_cycle_resolution()),
                        });
                    }
                }
            }
        }

        path.pop();
        rec_stack.remove(node);
    }

    /// Assess the severity of a circular dependency
    fn assess_cycle_severity(&self, cycle: &[String]) -> CycleSeverity {
        // Simple heuristic: longer cycles are more critical
        match cycle.len() {
            2 => CycleSeverity::Minor,
            3..=5 => CycleSeverity::Major,
            _ => CycleSeverity::Critical,
        }
    }

    /// Suggest resolution for circular dependency
    fn suggest_cycle_resolution(&self) -> String {
        "Consider using forward declarations or breaking the cycle through composition".to_string()
    }

    /// Perform topological sort to get component ordering
    fn topological_sort(&self) -> Vec<String> {
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        let mut result = Vec::new();
        let mut queue = VecDeque::new();

        // Initialize in-degree count
        for component in &self.components {
            in_degree.insert(component.clone(), 0);
        }

        // Calculate in-degrees (component depends on deps, so deps have outgoing edges)
        for (component, deps) in &self.dependencies {
            for dep in deps {
                // component depends on dep, so component should come after dep
                if let Some(count) = in_degree.get_mut(component) {
                    *count += 1;
                }
            }
        }

        // Find nodes with no incoming edges
        for (component, &degree) in &in_degree {
            if degree == 0 {
                queue.push_back(component.clone());
            }
        }

        // Process queue
        while let Some(component) = queue.pop_front() {
            result.push(component.clone());

            // When we process a component, reduce in-degree of components that depend on it
            for (other_component, deps) in &self.dependencies {
                if deps.contains(&component) {
                    if let Some(count) = in_degree.get_mut(other_component) {
                        *count -= 1;
                        if *count == 0 {
                            queue.push_back(other_component.clone());
                        }
                    }
                }
            }
        }

        result
    }

    /// Find unreachable components (no reverse dependencies)
    fn find_unreachable_components(&self) -> Vec<String> {
        let mut unreachable = Vec::new();

        for component in &self.components {
            // If component has no reverse dependencies and is not a root component
            if !self.reverse_deps.contains_key(component) && 
               self.dependencies.get(component).map_or(true, |deps| !deps.is_empty()) {
                unreachable.push(component.clone());
            }
        }

        unreachable
    }

    /// Calculate dependency depth for each component
    fn calculate_dependency_depths(&self) -> HashMap<String, usize> {
        let mut depths = HashMap::new();
        let mut visited = HashSet::new();

        for component in &self.components {
            if !depths.contains_key(component) {
                self.calculate_depth_dfs(component, &mut depths, &mut visited);
            }
        }

        depths
    }

    /// DFS helper for depth calculation
    fn calculate_depth_dfs(
        &self,
        component: &str,
        depths: &mut HashMap<String, usize>,
        visited: &mut HashSet<String>,
    ) -> usize {
        if let Some(&depth) = depths.get(component) {
            return depth;
        }

        if visited.contains(component) {
            // Cycle detected, set depth to 0 to prevent infinite recursion
            depths.insert(component.to_string(), 0);
            return 0;
        }

        visited.insert(component.to_string());

        let depth = if let Some(deps) = self.dependencies.get(component) {
            if deps.is_empty() {
                // Leaf component (no dependencies) has depth 1
                1
            } else {
                // Component with dependencies has depth = max(dependency depths) + 1
                let mut max_dep_depth = 0;
                for dep in deps {
                    let dep_depth = self.calculate_depth_dfs(dep, depths, visited);
                    max_dep_depth = max_dep_depth.max(dep_depth);
                }
                max_dep_depth + 1
            }
        } else {
            // No dependencies recorded means depth 1 (leaf)
            1
        };

        visited.remove(component);
        depths.insert(component.to_string(), depth);
        depth
    }

    /// Find components with no dependencies (leaf nodes)
    fn find_leaf_components(&self) -> Vec<String> {
        self.components
            .iter()
            .filter(|component| {
                self.dependencies
                    .get(*component)
                    .map_or(true, |deps| deps.is_empty())
            })
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_dependency_extraction() {
        let mut analyzer = DependencyAnalyzer::new();
        
        // Create types with dependencies: B -> A, C -> B
        let mut definitions = HashMap::new();
        definitions.insert("A".to_string(), TypeDefinition {
            name: "A".to_string(),
            type_expr: TypeExpression::Basic(BasicType::Natural),
            span: Span::new(1, 1, 1, 10),
        });
        definitions.insert("B".to_string(), TypeDefinition {
            name: "B".to_string(),
            type_expr: TypeExpression::Reference("A".to_string()),
            span: Span::new(2, 1, 2, 10),
        });
        definitions.insert("C".to_string(), TypeDefinition {
            name: "C".to_string(),
            type_expr: TypeExpression::Reference("B".to_string()),
            span: Span::new(3, 1, 3, 10),
        });

        let types_block = TypesBlock {
            definitions,
            span: Span::new(1, 1, 4, 1),
        };

        analyzer.extract_type_dependencies(&types_block);

        assert_eq!(analyzer.dependencies.get("A").unwrap().len(), 0);
        assert_eq!(analyzer.dependencies.get("B").unwrap(), &vec!["A"]);
        assert_eq!(analyzer.dependencies.get("C").unwrap(), &vec!["B"]);
    }

    #[test]
    fn test_circular_dependency_detection() {
        let mut analyzer = DependencyAnalyzer::new();
        
        // Create circular dependency: A -> B -> A
        analyzer.components.insert("A".to_string());
        analyzer.components.insert("B".to_string());
        analyzer.dependencies.insert("A".to_string(), vec!["B".to_string()]);
        analyzer.dependencies.insert("B".to_string(), vec!["A".to_string()]);

        let cycles = analyzer.detect_circular_dependencies();
        assert!(!cycles.is_empty());
        assert_eq!(cycles[0].cycle.len(), 2);
    }

    #[test]
    fn test_topological_sort() {
        let mut analyzer = DependencyAnalyzer::new();
        
        // Create dependency chain: C -> B -> A
        analyzer.components.insert("A".to_string());
        analyzer.components.insert("B".to_string());
        analyzer.components.insert("C".to_string());
        analyzer.dependencies.insert("A".to_string(), vec![]);
        analyzer.dependencies.insert("B".to_string(), vec!["A".to_string()]);
        analyzer.dependencies.insert("C".to_string(), vec!["B".to_string()]);

        let order = analyzer.topological_sort();
        
        // A should come before B, B should come before C
        let a_pos = order.iter().position(|x| x == "A").unwrap();
        let b_pos = order.iter().position(|x| x == "B").unwrap();
        let c_pos = order.iter().position(|x| x == "C").unwrap();
        
        assert!(a_pos < b_pos);
        assert!(b_pos < c_pos);
    }

    #[test]
    fn test_depth_calculation() {
        let mut analyzer = DependencyAnalyzer::new();
        
        analyzer.components.insert("A".to_string());
        analyzer.components.insert("B".to_string());
        analyzer.components.insert("C".to_string());
        analyzer.dependencies.insert("A".to_string(), vec![]);
        analyzer.dependencies.insert("B".to_string(), vec!["A".to_string()]);
        analyzer.dependencies.insert("C".to_string(), vec!["B".to_string()]);

        let depths = analyzer.calculate_dependency_depths();
        
        assert_eq!(depths.get("A").unwrap(), &1);
        assert_eq!(depths.get("B").unwrap(), &2);
        assert_eq!(depths.get("C").unwrap(), &3);
    }

    #[test]
    fn test_leaf_components() {
        let mut analyzer = DependencyAnalyzer::new();
        
        analyzer.components.insert("A".to_string());
        analyzer.components.insert("B".to_string());
        analyzer.dependencies.insert("A".to_string(), vec![]);
        analyzer.dependencies.insert("B".to_string(), vec!["A".to_string()]);

        let leaves = analyzer.find_leaf_components();
        assert_eq!(leaves, vec!["A"]);
    }
}