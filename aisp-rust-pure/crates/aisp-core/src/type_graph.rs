//! Type relationship graph for AISP documents
//!
//! This module constructs and analyzes type relationship graphs,
//! including inheritance analysis, compatibility checking, and
//! type hierarchy validation.

use crate::ast::*;
use crate::error::*;
use std::collections::{HashMap, HashSet};

/// Type relationship graph analyzer
pub struct TypeGraphAnalyzer {
    /// Type nodes in the graph
    nodes: HashMap<String, TypeNode>,
    /// Type relationship edges
    edges: Vec<TypeRelation>,
    /// Type compatibility cache
    compatibility_cache: HashMap<(String, String), CompatibilityLevel>,
}

/// Node in the type relationship graph
#[derive(Debug, Clone)]
pub struct TypeNode {
    /// Type name
    pub name: String,
    /// Type definition
    pub definition: TypeExpression,
    /// Computed type properties
    pub properties: TypeProperties,
    /// Direct relationships to other types
    pub relationships: Vec<String>,
    /// Source location
    pub span: Span,
}

/// Edge representing a relationship between types
#[derive(Debug, Clone)]
pub struct TypeRelation {
    /// Source type
    pub from: String,
    /// Target type
    pub to: String,
    /// Type of relationship
    pub relation_type: RelationType,
    /// Confidence in this relationship (0.0-1.0)
    pub confidence: f64,
    /// Evidence for this relationship
    pub evidence: RelationEvidence,
}

/// Types of relationships between types
#[derive(Debug, Clone, PartialEq)]
pub enum RelationType {
    /// Subtype relationship (A <: B)
    Subtype,
    /// Supertype relationship (A :> B)
    Supertype,
    /// Equivalent types (A ≡ B)
    Equivalent,
    /// Related types (some connection)
    Related,
    /// Disjoint types (A ∩ B = ∅)
    Disjoint,
    /// Overlapping types (A ∩ B ≠ ∅)
    Overlapping,
}

/// Evidence for type relationships
#[derive(Debug, Clone)]
pub enum RelationEvidence {
    /// Direct inheritance in definition
    Inheritance,
    /// Mathematical subset relationship
    SetInclusion,
    /// Structural compatibility
    StructuralSimilarity,
    /// Implicit conversion rules
    ImplicitConversion,
    /// User-defined relationship
    UserDefined,
}

/// Type compatibility levels
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CompatibilityLevel {
    /// Types are identical
    Identical,
    /// Types are directly compatible (no conversion needed)
    Compatible,
    /// Types are related but need explicit conversion
    Related,
    /// Types are incompatible
    Incompatible,
}

/// Properties derived from type analysis
#[derive(Debug, Clone)]
pub struct TypeProperties {
    /// Is the type finite or infinite?
    pub finite: Option<bool>,
    /// Estimated cardinality (if finite)
    pub cardinality: Option<usize>,
    /// Does the type support ordering?
    pub ordered: bool,
    /// Does the type support arithmetic operations?
    pub numeric: bool,
    /// Can the type be enumerated?
    pub enumerable: bool,
    /// Type complexity score
    pub complexity: f64,
}

/// Result of type graph analysis
#[derive(Debug, Clone)]
pub struct TypeGraphResult {
    /// All type nodes
    pub nodes: HashMap<String, TypeNode>,
    /// All type relationships
    pub edges: Vec<TypeRelation>,
    /// Detected cycles in type hierarchy
    pub cycles: Vec<Vec<String>>,
    /// Type compatibility matrix
    pub compatibility: HashMap<(String, String), CompatibilityLevel>,
    /// Type hierarchy depth map
    pub hierarchy_depths: HashMap<String, usize>,
    /// Root types (no supertypes)
    pub root_types: Vec<String>,
}

impl TypeGraphAnalyzer {
    /// Create a new type graph analyzer
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
            compatibility_cache: HashMap::new(),
        }
    }

    /// Build type relationship graph from document
    pub fn build_graph(&mut self, document: &AispDocument) -> AispResult<TypeGraphResult> {
        self.nodes.clear();
        self.edges.clear();
        self.compatibility_cache.clear();

        // Extract type definitions and create nodes
        self.extract_type_nodes(document)?;

        // Add built-in types
        self.add_builtin_types();

        // Analyze relationships between types
        self.analyze_type_relationships();

        // Detect cycles in type hierarchy
        let cycles = self.detect_hierarchy_cycles();

        // Build compatibility matrix
        let compatibility = self.build_compatibility_matrix();

        // Calculate hierarchy depths
        let hierarchy_depths = self.calculate_hierarchy_depths();

        // Find root types
        let root_types = self.find_root_types();

        Ok(TypeGraphResult {
            nodes: self.nodes.clone(),
            edges: self.edges.clone(),
            cycles,
            compatibility,
            hierarchy_depths,
            root_types,
        })
    }

    /// Extract type nodes from document
    fn extract_type_nodes(&mut self, document: &AispDocument) -> AispResult<()> {
        for block in &document.blocks {
            if let AispBlock::Types(types_block) = block {
                for (name, definition) in &types_block.definitions {
                    let properties = self.analyze_type_properties(&definition.type_expr);
                    
                    let node = TypeNode {
                        name: name.clone(),
                        definition: definition.type_expr.clone(),
                        properties,
                        relationships: Vec::new(),
                        span: definition.span.clone(),
                    };
                    
                    self.nodes.insert(name.clone(), node);
                }
            }
        }
        Ok(())
    }

    /// Add built-in mathematical types
    fn add_builtin_types(&mut self) {
        let builtins = [
            ("ℕ", BasicType::Natural, true, false, None),
            ("ℤ", BasicType::Integer, true, false, None),
            ("ℝ", BasicType::Real, true, false, None),
            ("ℚ", BasicType::Real, true, false, None), // Rationals as subset of reals
            ("𝔹", BasicType::Boolean, false, true, Some(2)),
            ("String", BasicType::String, true, false, None),
        ];

        for (name, basic_type, ordered, finite, cardinality) in builtins {
            let properties = TypeProperties {
                finite: Some(finite),
                cardinality,
                ordered,
                numeric: matches!(basic_type, BasicType::Natural | BasicType::Integer | BasicType::Real),
                enumerable: !matches!(basic_type, BasicType::Real),
                complexity: 1.0, // Built-ins have low complexity
            };

            let node = TypeNode {
                name: name.to_string(),
                definition: TypeExpression::Basic(basic_type),
                properties,
                relationships: Vec::new(),
                span: Span::new(0, 0, 0, 0), // Built-in, no source location
            };

            self.nodes.insert(name.to_string(), node);
        }
    }

    /// Analyze properties of a type expression
    fn analyze_type_properties(&self, type_expr: &TypeExpression) -> TypeProperties {
        match type_expr {
            TypeExpression::Basic(basic_type) => {
                match basic_type {
                    BasicType::Natural => TypeProperties {
                        finite: Some(false),
                        cardinality: None,
                        ordered: true,
                        numeric: true,
                        enumerable: true,
                        complexity: 1.0,
                    },
                    BasicType::Integer => TypeProperties {
                        finite: Some(false),
                        cardinality: None,
                        ordered: true,
                        numeric: true,
                        enumerable: true,
                        complexity: 1.2,
                    },
                    BasicType::Real => TypeProperties {
                        finite: Some(false),
                        cardinality: None,
                        ordered: true,
                        numeric: true,
                        enumerable: false,
                        complexity: 1.5,
                    },
                    BasicType::Boolean => TypeProperties {
                        finite: Some(true),
                        cardinality: Some(2),
                        ordered: false,
                        numeric: false,
                        enumerable: true,
                        complexity: 0.5,
                    },
                    BasicType::String => TypeProperties {
                        finite: Some(false),
                        cardinality: None,
                        ordered: true, // Lexicographic
                        numeric: false,
                        enumerable: true,
                        complexity: 1.3,
                    },
                }
            }
            TypeExpression::Enumeration(values) => TypeProperties {
                finite: Some(true),
                cardinality: Some(values.len()),
                ordered: false, // Unless explicitly ordered
                numeric: false,
                enumerable: true,
                complexity: 0.8 + (values.len() as f64 * 0.1),
            },
            TypeExpression::Array { element_type, size } => {
                let element_props = self.analyze_type_properties(element_type);
                TypeProperties {
                    finite: Some(size.is_some()),
                    cardinality: size.and_then(|s| {
                        element_props.cardinality.map(|c| c.pow(s as u32))
                    }),
                    ordered: true, // Arrays have indices
                    numeric: false,
                    enumerable: element_props.enumerable && size.is_some(),
                    complexity: element_props.complexity + 1.0,
                }
            }
            TypeExpression::Function { input, output } => {
                let input_props = self.analyze_type_properties(input);
                let output_props = self.analyze_type_properties(output);
                TypeProperties {
                    finite: Some(false), // Function spaces are typically infinite
                    cardinality: None,
                    ordered: false,
                    numeric: false,
                    enumerable: false,
                    complexity: input_props.complexity + output_props.complexity + 2.0,
                }
            }
            TypeExpression::Tuple(elements) => {
                let mut complexity = 0.5;
                let mut all_finite = true;
                let mut total_cardinality = Some(1);

                for element in elements {
                    let props = self.analyze_type_properties(element);
                    complexity += props.complexity;
                    if props.finite != Some(true) {
                        all_finite = false;
                        total_cardinality = None;
                    }
                    if let (Some(total), Some(elem_card)) = (total_cardinality, props.cardinality) {
                        total_cardinality = Some(total * elem_card);
                    }
                }

                TypeProperties {
                    finite: Some(all_finite),
                    cardinality: total_cardinality,
                    ordered: true, // Tuples have component ordering
                    numeric: false,
                    enumerable: all_finite,
                    complexity,
                }
            }
            TypeExpression::Generic { .. } => TypeProperties {
                finite: None,
                cardinality: None,
                ordered: false,
                numeric: false,
                enumerable: false,
                complexity: 3.0, // Generic types are complex
            },
            _ => TypeProperties {
                finite: None,
                cardinality: None,
                ordered: false,
                numeric: false,
                enumerable: false,
                complexity: 2.0, // Default for complex types
            }
        }
    }

    /// Analyze relationships between all types
    fn analyze_type_relationships(&mut self) {
        let type_names: Vec<String> = self.nodes.keys().cloned().collect();
        
        for i in 0..type_names.len() {
            for j in 0..type_names.len() {
                if i != j {
                    let type_a = &type_names[i];
                    let type_b = &type_names[j];
                    
                    if let (Some(node_a), Some(node_b)) = (
                        self.nodes.get(type_a),
                        self.nodes.get(type_b)
                    ) {
                        let (relation, evidence, confidence) = 
                            self.infer_type_relationship(&node_a.definition, &node_b.definition);
                        
                        if relation != RelationType::Disjoint || confidence > 0.3 {
                            self.edges.push(TypeRelation {
                                from: type_a.clone(),
                                to: type_b.clone(),
                                relation_type: relation,
                                confidence,
                                evidence,
                            });
                        }
                    }
                }
            }
        }
    }

    /// Infer relationship between two type expressions
    fn infer_type_relationship(
        &self, 
        type_a: &TypeExpression, 
        type_b: &TypeExpression
    ) -> (RelationType, RelationEvidence, f64) {
        match (type_a, type_b) {
            // Identical types
            (a, b) if a == b => (RelationType::Equivalent, RelationEvidence::StructuralSimilarity, 1.0),

            // Mathematical type hierarchy
            (TypeExpression::Basic(BasicType::Natural), TypeExpression::Basic(BasicType::Integer)) => {
                (RelationType::Subtype, RelationEvidence::SetInclusion, 1.0)
            }
            (TypeExpression::Basic(BasicType::Integer), TypeExpression::Basic(BasicType::Real)) => {
                (RelationType::Subtype, RelationEvidence::SetInclusion, 1.0)
            }
            (TypeExpression::Basic(BasicType::Natural), TypeExpression::Basic(BasicType::Real)) => {
                (RelationType::Subtype, RelationEvidence::SetInclusion, 0.9)
            }

            // Array type relationships
            (
                TypeExpression::Array { element_type: elem_a, size: size_a },
                TypeExpression::Array { element_type: elem_b, size: size_b }
            ) => {
                let (elem_rel, _, elem_conf) = self.infer_type_relationship(elem_a, elem_b);
                match (size_a, size_b, elem_rel) {
                    (Some(_), None, RelationType::Equivalent) => {
                        (RelationType::Subtype, RelationEvidence::StructuralSimilarity, elem_conf * 0.9)
                    }
                    (_, _, RelationType::Equivalent) => {
                        (RelationType::Equivalent, RelationEvidence::StructuralSimilarity, elem_conf)
                    }
                    _ => (RelationType::Related, RelationEvidence::StructuralSimilarity, elem_conf * 0.5)
                }
            }

            // Enumeration relationships
            (TypeExpression::Enumeration(values_a), TypeExpression::Enumeration(values_b)) => {
                let set_a: HashSet<_> = values_a.iter().collect();
                let set_b: HashSet<_> = values_b.iter().collect();

                if set_a == set_b {
                    (RelationType::Equivalent, RelationEvidence::SetInclusion, 1.0)
                } else if set_a.is_subset(&set_b) {
                    (RelationType::Subtype, RelationEvidence::SetInclusion, 0.95)
                } else if set_b.is_subset(&set_a) {
                    (RelationType::Supertype, RelationEvidence::SetInclusion, 0.95)
                } else if !set_a.is_disjoint(&set_b) {
                    let overlap_ratio = set_a.intersection(&set_b).count() as f64 / 
                                      set_a.union(&set_b).count() as f64;
                    (RelationType::Overlapping, RelationEvidence::SetInclusion, overlap_ratio)
                } else {
                    (RelationType::Disjoint, RelationEvidence::SetInclusion, 1.0)
                }
            }

            // Function type relationships (contravariant in input, covariant in output)
            (
                TypeExpression::Function { input: in_a, output: out_a },
                TypeExpression::Function { input: in_b, output: out_b }
            ) => {
                let (input_rel, _, input_conf) = self.infer_type_relationship(in_b, in_a); // Contravariant
                let (output_rel, _, output_conf) = self.infer_type_relationship(out_a, out_b); // Covariant
                
                let confidence = (input_conf + output_conf) / 2.0;
                
                match (input_rel, output_rel) {
                    (RelationType::Subtype, RelationType::Subtype) => {
                        (RelationType::Subtype, RelationEvidence::StructuralSimilarity, confidence)
                    }
                    (RelationType::Equivalent, RelationType::Equivalent) => {
                        (RelationType::Equivalent, RelationEvidence::StructuralSimilarity, confidence)
                    }
                    _ => (RelationType::Related, RelationEvidence::StructuralSimilarity, confidence * 0.7)
                }
            }

            // Reference relationships
            (TypeExpression::Reference(name_a), TypeExpression::Reference(name_b)) => {
                if name_a == name_b {
                    (RelationType::Equivalent, RelationEvidence::StructuralSimilarity, 1.0)
                } else {
                    // Try to resolve references and compare
                    (RelationType::Related, RelationEvidence::StructuralSimilarity, 0.3)
                }
            }

            // Default: types are typically disjoint unless proven otherwise
            _ => (RelationType::Disjoint, RelationEvidence::StructuralSimilarity, 0.1)
        }
    }

    /// Detect cycles in type hierarchy
    fn detect_hierarchy_cycles(&self) -> Vec<Vec<String>> {
        let mut cycles = Vec::new();
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        let mut path = Vec::new();

        // Build adjacency list for subtype relationships
        let mut adj_list: HashMap<String, Vec<String>> = HashMap::new();
        for node_name in self.nodes.keys() {
            adj_list.insert(node_name.clone(), Vec::new());
        }

        for edge in &self.edges {
            if edge.relation_type == RelationType::Subtype {
                adj_list.entry(edge.from.clone())
                    .or_default()
                    .push(edge.to.clone());
            }
        }

        // DFS to find cycles
        for node in self.nodes.keys() {
            if !visited.contains(node) {
                self.dfs_cycle_detect(
                    node,
                    &adj_list,
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
    fn dfs_cycle_detect(
        &self,
        node: &str,
        adj_list: &HashMap<String, Vec<String>>,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
        path: &mut Vec<String>,
        cycles: &mut Vec<Vec<String>>,
    ) {
        visited.insert(node.to_string());
        rec_stack.insert(node.to_string());
        path.push(node.to_string());

        if let Some(neighbors) = adj_list.get(node) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    self.dfs_cycle_detect(neighbor, adj_list, visited, rec_stack, path, cycles);
                } else if rec_stack.contains(neighbor) {
                    // Found cycle
                    if let Some(start_idx) = path.iter().position(|x| x == neighbor) {
                        let cycle = path[start_idx..].to_vec();
                        cycles.push(cycle);
                    }
                }
            }
        }

        path.pop();
        rec_stack.remove(node);
    }

    /// Build type compatibility matrix
    fn build_compatibility_matrix(&mut self) -> HashMap<(String, String), CompatibilityLevel> {
        let mut matrix = HashMap::new();
        
        for edge in &self.edges {
            let compatibility = match edge.relation_type {
                RelationType::Equivalent => CompatibilityLevel::Identical,
                RelationType::Subtype | RelationType::Supertype => CompatibilityLevel::Compatible,
                RelationType::Related | RelationType::Overlapping => CompatibilityLevel::Related,
                RelationType::Disjoint => CompatibilityLevel::Incompatible,
            };
            
            matrix.insert((edge.from.clone(), edge.to.clone()), compatibility);
        }

        // Add self-compatibility for all types
        for type_name in self.nodes.keys() {
            matrix.insert((type_name.clone(), type_name.clone()), CompatibilityLevel::Identical);
        }

        self.compatibility_cache = matrix.clone();
        matrix
    }

    /// Calculate hierarchy depths for each type
    fn calculate_hierarchy_depths(&self) -> HashMap<String, usize> {
        let mut depths = HashMap::new();
        let mut visited = HashSet::new();

        for type_name in self.nodes.keys() {
            if !visited.contains(type_name) {
                self.calculate_depth_dfs(type_name, &mut depths, &mut visited, &mut HashSet::new());
            }
        }

        depths
    }

    /// DFS helper for depth calculation
    fn calculate_depth_dfs(
        &self,
        type_name: &str,
        depths: &mut HashMap<String, usize>,
        visited: &mut HashSet<String>,
        in_progress: &mut HashSet<String>,
    ) -> usize {
        if let Some(&depth) = depths.get(type_name) {
            return depth;
        }

        if in_progress.contains(type_name) {
            // Cycle detected, return arbitrary depth
            return 0;
        }

        visited.insert(type_name.to_string());
        in_progress.insert(type_name.to_string());

        let max_super_depth = self.edges.iter()
            .filter(|e| e.from == type_name && e.relation_type == RelationType::Subtype)
            .map(|e| self.calculate_depth_dfs(&e.to, depths, visited, in_progress))
            .max()
            .unwrap_or(0);

        let depth = max_super_depth + 1;
        depths.insert(type_name.to_string(), depth);
        in_progress.remove(type_name);
        
        depth
    }

    /// Find root types (no supertypes)
    fn find_root_types(&self) -> Vec<String> {
        let mut root_types = Vec::new();
        
        for type_name in self.nodes.keys() {
            let has_supertype = self.edges.iter().any(|e| {
                e.from == *type_name && e.relation_type == RelationType::Subtype
            });
            
            if !has_supertype {
                root_types.push(type_name.clone());
            }
        }
        
        root_types
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builtin_type_hierarchy() {
        let mut analyzer = TypeGraphAnalyzer::new();
        analyzer.add_builtin_types();
        analyzer.analyze_type_relationships();

        // Find Natural -> Integer relationship
        let nat_to_int = analyzer.edges.iter()
            .find(|e| e.from == "ℕ" && e.to == "ℤ" && e.relation_type == RelationType::Subtype);
        assert!(nat_to_int.is_some());

        // Find Integer -> Real relationship  
        let int_to_real = analyzer.edges.iter()
            .find(|e| e.from == "ℤ" && e.to == "ℝ" && e.relation_type == RelationType::Subtype);
        assert!(int_to_real.is_some());
    }

    #[test]
    fn test_enumeration_relationships() {
        let analyzer = TypeGraphAnalyzer::new();
        
        let enum_a = TypeExpression::Enumeration(vec!["A".to_string(), "B".to_string()]);
        let enum_b = TypeExpression::Enumeration(vec!["A".to_string(), "B".to_string(), "C".to_string()]);
        
        let (relation, _, confidence) = analyzer.infer_type_relationship(&enum_a, &enum_b);
        assert_eq!(relation, RelationType::Subtype);
        assert!(confidence > 0.9);
    }

    #[test]
    fn test_type_properties() {
        let analyzer = TypeGraphAnalyzer::new();
        
        let bool_type = TypeExpression::Basic(BasicType::Boolean);
        let props = analyzer.analyze_type_properties(&bool_type);
        
        assert_eq!(props.finite, Some(true));
        assert_eq!(props.cardinality, Some(2));
        assert!(props.enumerable);
        assert!(!props.numeric);
    }

    #[test]
    fn test_array_type_relationships() {
        let analyzer = TypeGraphAnalyzer::new();
        
        let fixed_array = TypeExpression::Array {
            element_type: Box::new(TypeExpression::Basic(BasicType::Natural)),
            size: Some(5),
        };
        let dynamic_array = TypeExpression::Array {
            element_type: Box::new(TypeExpression::Basic(BasicType::Natural)),
            size: None,
        };
        
        let (relation, _, _) = analyzer.infer_type_relationship(&fixed_array, &dynamic_array);
        assert_eq!(relation, RelationType::Subtype);
    }

    #[test]
    fn test_function_type_relationships() {
        let analyzer = TypeGraphAnalyzer::new();
        
        let func_a = TypeExpression::Function {
            input: Box::new(TypeExpression::Basic(BasicType::Integer)),
            output: Box::new(TypeExpression::Basic(BasicType::Natural)),
        };
        let func_b = TypeExpression::Function {
            input: Box::new(TypeExpression::Basic(BasicType::Natural)),
            output: Box::new(TypeExpression::Basic(BasicType::Integer)),
        };
        
        let (relation, _, _) = analyzer.infer_type_relationship(&func_a, &func_b);
        assert_eq!(relation, RelationType::Subtype); // Contravariant input, covariant output
    }
}