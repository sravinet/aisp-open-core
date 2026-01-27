//! Canonical AISP AST Types - Single Source of Truth
//! 
//! This module provides the unified, production-ready AST representation
//! that replaces both ast::AispDocument and robust_parser::AispDocument
//! to eliminate type system fragmentation.

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// Source location span information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
}

/// Document header information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DocumentHeader {
    /// AISP version (e.g., "5.1")
    pub version: String,
    /// Document name
    pub name: String,
    /// Creation date
    pub date: String,
    /// Optional metadata
    pub metadata: Option<HeaderMetadata>,
}

/// Additional header metadata
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HeaderMetadata {
    pub author: Option<String>,
    pub description: Option<String>,
    pub tags: Vec<String>,
}

/// Document metadata 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DocumentMetadata {
    pub domain: Option<String>,
    pub protocol: Option<String>,
}

/// Canonical AISP Document representation - SINGLE SOURCE OF TRUTH
/// 
/// This replaces both `ast::AispDocument` and `robust_parser::AispDocument`
/// with a unified, production-ready type that all modules use consistently.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanonicalAispDocument {
    pub header: DocumentHeader,
    pub metadata: DocumentMetadata,
    pub blocks: Vec<CanonicalAispBlock>,
    pub span: Option<Span>,
}

/// Canonical Block representation with consistent method access patterns
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CanonicalAispBlock {
    Meta(MetaBlock),
    Types(TypesBlock),
    Rules(RulesBlock), 
    Functions(FunctionsBlock),
    Evidence(EvidenceBlock),
}

impl CanonicalAispBlock {
    /// Get block type name - CANONICAL method access pattern
    pub fn block_type(&self) -> &'static str {
        match self {
            CanonicalAispBlock::Meta(_) => "Meta",
            CanonicalAispBlock::Types(_) => "Types",
            CanonicalAispBlock::Rules(_) => "Rules", 
            CanonicalAispBlock::Functions(_) => "Functions",
            CanonicalAispBlock::Evidence(_) => "Evidence",
        }
    }
    
    /// Get block as meta block if applicable
    pub fn as_meta(&self) -> Option<&MetaBlock> {
        match self {
            CanonicalAispBlock::Meta(meta) => Some(meta),
            _ => None,
        }
    }
    
    /// Get block as types block if applicable
    pub fn as_types(&self) -> Option<&TypesBlock> {
        match self {
            CanonicalAispBlock::Types(types) => Some(types),
            _ => None,
        }
    }
    
    /// Get block as rules block if applicable
    pub fn as_rules(&self) -> Option<&RulesBlock> {
        match self {
            CanonicalAispBlock::Rules(rules) => Some(rules),
            _ => None,
        }
    }
    
    /// Get block as functions block if applicable
    pub fn as_functions(&self) -> Option<&FunctionsBlock> {
        match self {
            CanonicalAispBlock::Functions(functions) => Some(functions),
            _ => None,
        }
    }
    
    /// Get block as evidence block if applicable
    pub fn as_evidence(&self) -> Option<&EvidenceBlock> {
        match self {
            CanonicalAispBlock::Evidence(evidence) => Some(evidence),
            _ => None,
        }
    }
}

/// Meta block for document metadata and configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetaBlock {
    pub entries: HashMap<String, MetaEntry>,
    pub raw_entries: Vec<String>, // Keep raw strings for parsing
    pub span: Option<Span>,
}

/// Types block for type definitions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypesBlock {
    pub definitions: HashMap<String, TypeDefinition>,
    pub raw_definitions: Vec<String>, // Keep raw strings for parsing later
    pub span: Option<Span>,
}

/// Rules block for logical rules and constraints
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RulesBlock {
    pub rules: Vec<LogicalRule>,
    pub raw_rules: Vec<String>, // Keep raw strings for parsing
    pub span: Option<Span>,
}

/// Functions block for function definitions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionsBlock {
    pub functions: Vec<FunctionDefinition>,
    pub raw_functions: Vec<String>, // Keep raw strings for parsing
    pub span: Option<Span>,
}

/// Evidence block for validation evidence
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvidenceBlock {
    pub delta: Option<f64>,
    pub phi: Option<u64>, 
    pub tau: Option<String>,
    pub metrics: HashMap<String, f64>,
    pub raw_evidence: Vec<String>, // Keep raw strings for parsing
    pub span: Option<Span>,
}

/// Type definition with canonical structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypeDefinition {
    pub name: String,
    pub type_expr: TypeExpression,
    pub span: Option<Span>,
}

/// Type expression for type system representation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TypeExpression {
    Basic(BasicType),
    Set(Box<TypeExpression>),
    Union(Vec<TypeExpression>),
    Product(Vec<TypeExpression>),
    Function {
        params: Vec<TypeExpression>,
        return_type: Box<TypeExpression>,
    },
}

/// Basic type enumeration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BasicType {
    Natural,
    Integer,
    Real,
    Boolean,
    String,
    Symbol,
    VectorSpace(usize),
    RealVector,
    DirectSum,
    MathematicalStructure(String),
    Custom(String),
}

/// Meta entry with structured data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetaEntry {
    pub key: String,
    pub value: MetaValue,
    pub span: Option<Span>,
}

/// Meta value types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MetaValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Constraint(LogicalExpression),
}

/// Logical rule with quantifiers and expressions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogicalRule {
    pub quantifier: Option<Quantifier>,
    pub expression: LogicalExpression,
    pub raw_text: String,
    pub span: Option<Span>,
}

/// Quantifier (∀, ∃)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Quantifier {
    pub kind: QuantifierKind,
    pub variable: String,
    pub domain: Option<String>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum QuantifierKind {
    /// Universal quantifier (∀)
    Universal,
    /// Existential quantifier (∃)
    Existential,
}

/// Logical expression
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LogicalExpression {
    /// Variable reference
    Variable(String),
    /// Constant value
    Constant(ConstantValue),
    /// Binary operation
    Binary {
        op: BinaryOperator,
        left: Box<LogicalExpression>,
        right: Box<LogicalExpression>,
    },
    /// Unary operation
    Unary {
        op: UnaryOperator,
        operand: Box<LogicalExpression>,
    },
    /// Function application
    Application {
        function: String,
        arguments: Vec<LogicalExpression>,
    },
    /// Set membership
    Membership {
        element: Box<LogicalExpression>,
        set: Box<LogicalExpression>,
    },
    /// Temporal operator
    Temporal {
        op: TemporalOperator,
        operand: Box<LogicalExpression>,
    },
    /// Raw text (for fallback parsing)
    Raw(String),
}

/// Constant values
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConstantValue {
    Number(f64),
    String(String),
    Boolean(bool),
}

/// Binary logical operators
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BinaryOperator {
    /// Definition (≜)
    Definition,
    /// Assignment (≔)
    Assignment,
    /// Equivalence (≡)
    Equivalence,
    /// Implication (⇒, →)
    Implication,
    /// Biconditional (⇔, ↔)
    Biconditional,
    /// Conjunction (∧)
    And,
    /// Disjunction (∨)
    Or,
    /// Exclusive or (⊕)
    Xor,
    /// Equality (=)
    Equals,
    /// Inequality (≠)
    NotEquals,
    /// Less than (<)
    LessThan,
    /// Less than or equal (≤)
    LessEqual,
    /// Greater than (>)
    GreaterThan,
    /// Greater than or equal (≥)
    GreaterEqual,
    /// Set union (∪)
    Union,
    /// Set intersection (∩)
    Intersection,
}

/// Unary logical operators
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UnaryOperator {
    /// Negation (¬)
    Not,
    /// Power set (𝒫)
    PowerSet,
}

/// Temporal operators for temporal logic
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TemporalOperator {
    /// Always/Globally (□, G)
    Always,
    /// Eventually/Finally (◊, F)
    Eventually,
    /// Next (X)
    Next,
    /// Until (U)
    Until,
    /// Weak until (W)
    WeakUntil,
    /// Release (R)
    Release,
}

/// Function definition with lambda expressions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionDefinition {
    pub name: String,
    pub lambda: LambdaExpression,
    pub raw_text: String,
    pub span: Option<Span>,
}

/// Lambda expression
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LambdaExpression {
    pub parameters: Vec<String>,
    pub body: LogicalExpression,
    pub span: Option<Span>,
}

impl Default for CanonicalAispDocument {
    fn default() -> Self {
        Self {
            header: DocumentHeader {
                version: "5.1".to_string(),
                name: "default".to_string(),
                date: "2026-01-27".to_string(),
                metadata: None,
            },
            metadata: DocumentMetadata {
                domain: None,
                protocol: None,
            },
            blocks: Vec::new(),
            span: None,
        }
    }
}

impl CanonicalAispDocument {
    /// Create new document with header
    pub fn new(name: String, version: String, date: String) -> Self {
        Self {
            header: DocumentHeader {
                version,
                name,
                date,
                metadata: None,
            },
            metadata: DocumentMetadata {
                domain: None, 
                protocol: None,
            },
            blocks: Vec::new(),
            span: None,
        }
    }
    
    /// Add block to document
    pub fn add_block(&mut self, block: CanonicalAispBlock) {
        self.blocks.push(block);
    }
    
    /// Get all blocks of a specific type
    pub fn get_blocks_by_type<T>(&self, block_type: fn(&CanonicalAispBlock) -> Option<&T>) -> Vec<&T> {
        self.blocks.iter().filter_map(block_type).collect()
    }
    
    /// Get first block of a specific type
    pub fn get_first_block<T>(&self, block_type: fn(&CanonicalAispBlock) -> Option<&T>) -> Option<&T> {
        self.blocks.iter().find_map(block_type)
    }
    
    /// Parse structured data from raw strings (called after construction)
    pub fn parse_structured_data(&mut self) {
        for block in &mut self.blocks {
            match block {
                CanonicalAispBlock::Meta(meta) => {
                    meta.parse_entries();
                }
                CanonicalAispBlock::Types(types) => {
                    types.parse_definitions();
                }
                CanonicalAispBlock::Rules(rules) => {
                    rules.parse_rules();
                }
                CanonicalAispBlock::Functions(functions) => {
                    functions.parse_functions();
                }
                CanonicalAispBlock::Evidence(evidence) => {
                    evidence.parse_evidence();
                }
            }
        }
    }
}

/// Conversion trait for migrating from legacy AST types
pub trait IntoCanonical<T> {
    fn into_canonical(self) -> T;
}

/// Convert from robust_parser types to canonical types
impl IntoCanonical<CanonicalAispDocument> for crate::parser::robust_parser::AispDocument {
    fn into_canonical(self) -> CanonicalAispDocument {
        CanonicalAispDocument {
            header: DocumentHeader {
                version: self.header.version,
                name: self.header.name,
                date: self.header.date,
                metadata: self.header.metadata.map(|m| HeaderMetadata {
                    author: None, // Map from robust_parser metadata if available
                    description: None,
                    tags: Vec::new(),
                }),
            },
            metadata: DocumentMetadata {
                domain: self.metadata.domain,
                protocol: self.metadata.protocol,
            },
            blocks: self.blocks.into_iter().map(|b| b.into_canonical()).collect(),
            span: None, // Could map from robust_parser span if available
        }
    }
}

/// Convert from robust_parser block types to canonical block types  
impl IntoCanonical<CanonicalAispBlock> for crate::parser::robust_parser::AispBlock {
    fn into_canonical(self) -> CanonicalAispBlock {
        match self {
            crate::parser::robust_parser::AispBlock::Meta(meta) => {
                CanonicalAispBlock::Meta(MetaBlock {
                    entries: HashMap::new(), // Parse structured entries from raw strings
                    raw_entries: meta.entries,
                    span: None,
                })
            }
            crate::parser::robust_parser::AispBlock::Types(types) => {
                CanonicalAispBlock::Types(TypesBlock {
                    definitions: HashMap::new(), // Parse structured types from raw strings
                    raw_definitions: types.definitions,
                    span: None,
                })
            }
            crate::parser::robust_parser::AispBlock::Rules(rules) => {
                CanonicalAispBlock::Rules(RulesBlock {
                    rules: Vec::new(), // Parse structured rules from raw strings
                    raw_rules: rules.rules,
                    span: None,
                })
            }
            crate::parser::robust_parser::AispBlock::Functions(functions) => {
                CanonicalAispBlock::Functions(FunctionsBlock {
                    functions: Vec::new(), // Parse structured functions from raw strings
                    raw_functions: functions.functions,
                    span: None,
                })
            }
            crate::parser::robust_parser::AispBlock::Evidence(evidence) => {
                CanonicalAispBlock::Evidence(EvidenceBlock {
                    delta: None, // Parse from evidence strings
                    phi: None,
                    tau: None,
                    metrics: HashMap::new(),
                    raw_evidence: evidence.evidence,
                    span: None,
                })
            }
            // Handle additional block types
            crate::parser::robust_parser::AispBlock::Errors(errors) => {
                // Map errors to meta block for now
                CanonicalAispBlock::Meta(MetaBlock {
                    entries: HashMap::new(),
                    raw_entries: errors.errors,
                    span: None,
                })
            }
        }
    }
}



impl MetaBlock {
    /// Parse raw entries into structured MetaEntry objects
    pub fn parse_entries(&mut self) {
        for raw_entry in &self.raw_entries {
            if let Some((key, value)) = Self::parse_meta_entry(raw_entry) {
                self.entries.insert(key.clone(), MetaEntry {
                    key,
                    value,
                    span: None,
                });
            }
        }
    }
    
    fn parse_meta_entry(entry: &str) -> Option<(String, MetaValue)> {
        // Simple parsing - extend as needed
        if let Some(pos) = entry.find('≜') { // ≜ definition symbol
            let key = entry[..pos].trim().to_string();
            let value_str = entry[pos + '≜'.len_utf8()..].trim();
            
            let value = if let Ok(num) = value_str.parse::<f64>() {
                MetaValue::Number(num)
            } else if value_str == "true" || value_str == "false" {
                MetaValue::Boolean(value_str == "true")
            } else {
                MetaValue::String(value_str.to_string())
            };
            
            Some((key, value))
        } else {
            None
        }
    }
}

impl TypesBlock {
    /// Parse raw definitions into structured TypeDefinition objects
    pub fn parse_definitions(&mut self) {
        for (i, raw_def) in self.raw_definitions.iter().enumerate() {
            if let Some((name, type_expr)) = Self::parse_type_definition(raw_def) {
                self.definitions.insert(name.clone(), TypeDefinition {
                    name,
                    type_expr,
                    span: None,
                });
            }
        }
    }
    
    fn parse_type_definition(def: &str) -> Option<(String, TypeExpression)> {
        // Simple parsing - extend as needed
        if let Some(pos) = def.find('≜') {
            let name = def[..pos].trim().to_string();
            let type_str = def[pos + '≜'.len_utf8()..].trim();
            
            let type_expr = if type_str.starts_with('{') && type_str.ends_with('}') {
                // Set type
                let inner = &type_str[1..type_str.len()-1];
                let elements: Vec<String> = inner.split(',').map(|s| s.trim().to_string()).collect();
                TypeExpression::Union(elements.into_iter().map(|e| 
                    TypeExpression::Basic(BasicType::Custom(e))
                ).collect())
            } else {
                // Basic type
                TypeExpression::Basic(BasicType::Custom(type_str.to_string()))
            };
            
            Some((name, type_expr))
        } else {
            None
        }
    }
}

impl RulesBlock {
    /// Parse raw rules into structured LogicalRule objects
    pub fn parse_rules(&mut self) {
        for raw_rule in &self.raw_rules {
            if let Some(rule) = Self::parse_logical_rule(raw_rule) {
                self.rules.push(rule);
            }
        }
    }
    
    fn parse_logical_rule(rule_str: &str) -> Option<LogicalRule> {
        // Simple parsing - for now just wrap as raw expression
        Some(LogicalRule {
            quantifier: None, // TODO: Parse quantifiers
            expression: LogicalExpression::Raw(rule_str.to_string()),
            raw_text: rule_str.to_string(),
            span: None,
        })
    }
}

impl FunctionsBlock {
    /// Parse raw functions into structured FunctionDefinition objects
    pub fn parse_functions(&mut self) {
        for raw_func in &self.raw_functions {
            if let Some(func) = Self::parse_function_definition(raw_func) {
                self.functions.push(func);
            }
        }
    }
    
    fn parse_function_definition(func_str: &str) -> Option<FunctionDefinition> {
        // Simple parsing - for now just wrap as raw
        if let Some(pos) = func_str.find('≜') {
            let name = func_str[..pos].trim().to_string();
            let lambda_str = func_str[pos + '≜'.len_utf8()..].trim();
            
            Some(FunctionDefinition {
                name,
                lambda: LambdaExpression {
                    parameters: Vec::new(), // TODO: Parse parameters
                    body: LogicalExpression::Raw(lambda_str.to_string()),
                    span: None,
                },
                raw_text: func_str.to_string(),
                span: None,
            })
        } else {
            None
        }
    }
}

impl EvidenceBlock {
    /// Parse raw evidence into structured metrics
    pub fn parse_evidence(&mut self) {
        let raw_evidence_clone = self.raw_evidence.clone();
        for raw_ev in &raw_evidence_clone {
            self.parse_evidence_entry(raw_ev);
        }
    }
    
    fn parse_evidence_entry(&mut self, evidence_str: &str) {
        // Parse δ, φ, τ values
        if evidence_str.contains('δ') {
            if let Some(delta_val) = Self::extract_numeric_value(evidence_str, 'δ') {
                self.delta = Some(delta_val);
            }
        }
        if evidence_str.contains('φ') {
            if let Some(phi_val) = Self::extract_numeric_value(evidence_str, 'φ') {
                self.phi = Some(phi_val as u64);
            }
        }
        if evidence_str.contains('τ') {
            if let Some(tau_val) = Self::extract_string_value(evidence_str, 'τ') {
                self.tau = Some(tau_val);
            }
        }
    }
    
    fn extract_numeric_value(text: &str, symbol: char) -> Option<f64> {
        if let Some(pos) = text.find(symbol) {
            let after_symbol = &text[pos + symbol.len_utf8()..];
            if let Some(eq_pos) = after_symbol.find('≜') {
                let value_str = after_symbol[eq_pos + '≜'.len_utf8()..]
                    .split_whitespace().next()?;
                value_str.parse().ok()
            } else {
                None
            }
        } else {
            None
        }
    }
    
    fn extract_string_value(text: &str, symbol: char) -> Option<String> {
        if let Some(pos) = text.find(symbol) {
            let after_symbol = &text[pos + symbol.len_utf8()..];
            if let Some(eq_pos) = after_symbol.find('≜') {
                let value_str = after_symbol[eq_pos + '≜'.len_utf8()..]
                    .split_whitespace().next()?;
                Some(value_str.to_string())
            } else {
                None
            }
        } else {
            None
        }
    }
}

/// Type alias for backward compatibility during migration
pub type AispDocument = CanonicalAispDocument;
pub type AispBlock = CanonicalAispBlock;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_canonical_document_creation() {
        let doc = CanonicalAispDocument::new(
            "test".to_string(),
            "5.1".to_string(),
            "2026-01-27".to_string(),
        );
        
        assert_eq!(doc.header.name, "test");
        assert_eq!(doc.header.version, "5.1");
        assert_eq!(doc.blocks.len(), 0);
    }
    
    #[test]
    fn test_block_type_identification() {
        let meta_block = CanonicalAispBlock::Meta(MetaBlock {
            entries: vec!["test".to_string()],
            span: None,
        });
        
        assert_eq!(meta_block.block_type(), "Meta");
        assert!(meta_block.as_meta().is_some());
        assert!(meta_block.as_types().is_none());
    }
    
    #[test]
    fn test_block_filtering() {
        let mut doc = CanonicalAispDocument::default();
        
        doc.add_block(CanonicalAispBlock::Meta(MetaBlock {
            entries: vec!["meta1".to_string()],
            span: None,
        }));
        
        doc.add_block(CanonicalAispBlock::Types(TypesBlock {
            definitions: HashMap::new(),
            span: None,
        }));
        
        let meta_blocks = doc.get_blocks_by_type(|b| b.as_meta());
        let type_blocks = doc.get_blocks_by_type(|b| b.as_types());
        
        assert_eq!(meta_blocks.len(), 1);
        assert_eq!(type_blocks.len(), 1);
    }
}