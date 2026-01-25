//! Symbol analysis for AISP documents
//!
//! This module tracks symbol definitions, usage, and scope resolution
//! across the document to detect undefined symbols and scope violations.

use crate::ast::*;
use crate::error::*;
use std::collections::{HashMap, HashSet};

/// Symbol analyzer for AISP documents
pub struct SymbolAnalyzer {
    /// Symbol definitions by scope
    symbols: HashMap<SymbolScope, HashMap<String, SymbolInfo>>,
    /// Symbol usage tracking
    usage: Vec<SymbolUsage>,
    /// Errors found during analysis
    errors: Vec<AispError>,
}

/// Symbol scope in AISP documents
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SymbolScope {
    /// Global scope (types, functions)
    Global,
    /// Meta block scope
    Meta,
    /// Rules block scope
    Rules,
    /// Function parameter scope
    Function(String),
}

/// Information about a symbol
#[derive(Debug, Clone)]
pub struct SymbolInfo {
    /// Symbol name
    pub name: String,
    /// Symbol type
    pub symbol_type: SymbolType,
    /// Where the symbol is defined
    pub definition_span: Span,
    /// Whether the symbol has been used
    pub used: bool,
}

/// Type of symbol
#[derive(Debug, Clone)]
pub enum SymbolType {
    /// Type definition
    Type,
    /// Function definition
    Function,
    /// Meta entry
    MetaEntry,
    /// Function parameter
    Parameter,
    /// Enumeration value
    EnumValue,
}

/// Symbol usage tracking
#[derive(Debug, Clone)]
pub struct SymbolUsage {
    /// Name of the symbol used
    pub name: String,
    /// Scope where it was used
    pub scope: SymbolScope,
    /// Location of usage
    pub span: Span,
}

/// Result of symbol analysis
#[derive(Debug)]
pub struct SymbolAnalysisResult {
    /// Errors found
    pub errors: Vec<AispError>,
    /// Warnings (unused symbols, etc.)
    pub warnings: Vec<AispWarning>,
    /// Symbol statistics
    pub stats: SymbolStats,
}

/// Symbol statistics
#[derive(Debug)]
pub struct SymbolStats {
    /// Total symbols defined
    pub total_symbols: usize,
    /// Used symbols
    pub used_symbols: usize,
    /// Undefined references
    pub undefined_references: usize,
    /// Symbol complexity score
    pub complexity_score: f64,
}

impl SymbolAnalyzer {
    /// Create a new symbol analyzer
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
            usage: Vec::new(),
            errors: Vec::new(),
        }
    }

    /// Analyze symbols in the entire document
    pub fn analyze_document(&mut self, document: &AispDocument) -> SymbolAnalysisResult {
        // Initialize scopes
        self.symbols.insert(SymbolScope::Global, HashMap::new());
        self.symbols.insert(SymbolScope::Meta, HashMap::new());
        self.symbols.insert(SymbolScope::Rules, HashMap::new());

        // Collect symbol definitions
        self.collect_symbol_definitions(document);
        
        // Analyze symbol usage
        self.analyze_symbol_usage(document);
        
        // Check for undefined symbols
        self.check_undefined_symbols();
        
        // Generate warnings for unused symbols
        let warnings = self.generate_warnings();
        
        SymbolAnalysisResult {
            errors: self.errors.clone(),
            warnings,
            stats: self.calculate_statistics(),
        }
    }

    /// Collect all symbol definitions from the document
    fn collect_symbol_definitions(&mut self, document: &AispDocument) {
        for block in &document.blocks {
            match block {
                AispBlock::Meta(meta_block) => {
                    self.collect_meta_symbols(meta_block);
                }
                AispBlock::Types(types_block) => {
                    self.collect_type_symbols(types_block);
                }
                AispBlock::Functions(functions_block) => {
                    self.collect_function_symbols(functions_block);
                }
                _ => {} // Rules and Evidence don't define symbols
            }
        }
    }

    /// Collect symbols from meta block
    fn collect_meta_symbols(&mut self, meta_block: &MetaBlock) {
        let scope = &mut self.symbols.get_mut(&SymbolScope::Meta).unwrap();
        
        for (name, entry) in &meta_block.entries {
            scope.insert(name.clone(), SymbolInfo {
                name: name.clone(),
                symbol_type: SymbolType::MetaEntry,
                definition_span: entry.span.clone(),
                used: false,
            });
        }
    }

    /// Collect symbols from types block
    fn collect_type_symbols(&mut self, types_block: &TypesBlock) {
        let scope = &mut self.symbols.get_mut(&SymbolScope::Global).unwrap();
        
        for (name, definition) in &types_block.definitions {
            scope.insert(name.clone(), SymbolInfo {
                name: name.clone(),
                symbol_type: SymbolType::Type,
                definition_span: definition.span.clone(),
                used: false,
            });
            
            // Collect enumeration values as symbols
            if let TypeExpression::Enumeration(values) = &definition.type_expr {
                for value in values {
                    scope.insert(value.clone(), SymbolInfo {
                        name: value.clone(),
                        symbol_type: SymbolType::EnumValue,
                        definition_span: definition.span.clone(),
                        used: false,
                    });
                }
            }
        }
    }

    /// Collect symbols from functions block
    fn collect_function_symbols(&mut self, functions_block: &FunctionsBlock) {
        // First, collect function names for global scope
        let mut function_symbols = HashMap::new();
        let mut function_params = Vec::new();
        
        for (name, function) in &functions_block.functions {
            // Add function to global scope
            function_symbols.insert(name.clone(), SymbolInfo {
                name: name.clone(),
                symbol_type: SymbolType::Function,
                definition_span: function.span.clone(),
                used: false,
            });
            
            // Prepare function parameter scope
            let mut param_scope = HashMap::new();
            for param in &function.lambda.parameters {
                param_scope.insert(param.clone(), SymbolInfo {
                    name: param.clone(),
                    symbol_type: SymbolType::Parameter,
                    definition_span: function.lambda.span.clone(),
                    used: false,
                });
            }
            
            function_params.push((name.clone(), param_scope));
        }
        
        // Now update the symbols map
        if let Some(global_scope) = self.symbols.get_mut(&SymbolScope::Global) {
            global_scope.extend(function_symbols);
        }
        
        for (name, param_scope) in function_params {
            self.symbols.insert(SymbolScope::Function(name), param_scope);
        }
    }

    /// Analyze symbol usage throughout the document
    fn analyze_symbol_usage(&mut self, document: &AispDocument) {
        for block in &document.blocks {
            match block {
                AispBlock::Rules(rules_block) => {
                    self.analyze_rules_usage(rules_block);
                }
                AispBlock::Functions(functions_block) => {
                    self.analyze_functions_usage(functions_block);
                }
                AispBlock::Types(types_block) => {
                    self.analyze_type_references(types_block);
                }
                _ => {}
            }
        }
    }

    /// Analyze symbol usage in rules
    fn analyze_rules_usage(&mut self, _rules_block: &RulesBlock) {
        // TODO: Parse logical expressions to find symbol references
        // For now, this is simplified
    }

    /// Analyze symbol usage in functions
    fn analyze_functions_usage(&mut self, functions_block: &FunctionsBlock) {
        for (func_name, function) in &functions_block.functions {
            // TODO: Parse lambda body to find symbol references
            // For now, mark parameters as used if they appear in simple expressions
            if let LogicalExpression::Variable(expr) = &function.lambda.body {
                for param in &function.lambda.parameters {
                    if expr.contains(param) {
                        self.mark_symbol_used(param, &SymbolScope::Function(func_name.clone()));
                    }
                }
            }
        }
    }

    /// Analyze type references in type expressions
    fn analyze_type_references(&mut self, types_block: &TypesBlock) {
        for (_, definition) in &types_block.definitions {
            self.analyze_type_expression_usage(&definition.type_expr);
        }
    }

    /// Analyze symbol usage in type expressions
    fn analyze_type_expression_usage(&mut self, type_expr: &TypeExpression) {
        match type_expr {
            TypeExpression::Reference(name) => {
                self.mark_symbol_used(name, &SymbolScope::Global);
                self.usage.push(SymbolUsage {
                    name: name.clone(),
                    scope: SymbolScope::Global,
                    span: Span::new(0, 0, 0, 0), // TODO: Get actual span
                });
            }
            TypeExpression::Array { element_type, .. } => {
                self.analyze_type_expression_usage(element_type);
            }
            TypeExpression::Function { input, output } => {
                self.analyze_type_expression_usage(input);
                self.analyze_type_expression_usage(output);
            }
            _ => {} // Basic types and enumerations don't reference other symbols
        }
    }

    /// Mark a symbol as used
    fn mark_symbol_used(&mut self, name: &str, scope: &SymbolScope) {
        if let Some(scope_symbols) = self.symbols.get_mut(scope) {
            if let Some(symbol_info) = scope_symbols.get_mut(name) {
                symbol_info.used = true;
            }
        }
    }

    /// Check for undefined symbol references
    fn check_undefined_symbols(&mut self) {
        for usage in &self.usage {
            let mut found = false;
            
            // Check in the usage scope first
            if let Some(scope_symbols) = self.symbols.get(&usage.scope) {
                if scope_symbols.contains_key(&usage.name) {
                    found = true;
                }
            }
            
            // Check in global scope if not found in local scope
            if !found {
                if let Some(global_symbols) = self.symbols.get(&SymbolScope::Global) {
                    if global_symbols.contains_key(&usage.name) {
                        found = true;
                    }
                }
            }
            
            if !found {
                self.errors.push(AispError::UndefinedSymbol {
                    symbol: usage.name.clone(),
                });
            }
        }
    }

    /// Generate warnings for unused symbols
    fn generate_warnings(&self) -> Vec<AispWarning> {
        let mut warnings = Vec::new();
        
        for (scope, symbols) in &self.symbols {
            for (name, info) in symbols {
                if !info.used && !name.starts_with('_') {
                    warnings.push(AispWarning::warning(
                        format!("Unused symbol '{}' in scope {:?}", name, scope)
                    ));
                }
            }
        }
        
        warnings
    }

    /// Calculate symbol statistics
    fn calculate_statistics(&self) -> SymbolStats {
        let mut total_symbols = 0;
        let mut used_symbols = 0;
        
        for (_, symbols) in &self.symbols {
            for (_, info) in symbols {
                total_symbols += 1;
                if info.used {
                    used_symbols += 1;
                }
            }
        }
        
        let undefined_references = self.errors.iter()
            .filter(|e| matches!(e, AispError::UndefinedSymbol { .. }))
            .count();
        
        let complexity_score = if total_symbols > 0 {
            (used_symbols as f64 / total_symbols as f64) * 100.0
        } else {
            0.0
        };
        
        SymbolStats {
            total_symbols,
            used_symbols,
            undefined_references,
            complexity_score,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol_definition_collection() {
        let mut analyzer = SymbolAnalyzer::new();
        
        // Create a simple types block
        let mut definitions = HashMap::new();
        definitions.insert("State".to_string(), TypeDefinition {
            name: "State".to_string(),
            type_expr: TypeExpression::Enumeration(vec!["A".to_string(), "B".to_string()]),
            span: Span::new(1, 1, 1, 10),
        });
        
        let types_block = TypesBlock {
            definitions,
            span: Span::new(1, 1, 2, 1),
        };
        
        analyzer.symbols.insert(SymbolScope::Global, HashMap::new());
        analyzer.collect_type_symbols(&types_block);
        
        let global_symbols = &analyzer.symbols[&SymbolScope::Global];
        assert!(global_symbols.contains_key("State"));
        assert!(global_symbols.contains_key("A"));
        assert!(global_symbols.contains_key("B"));
    }

    #[test]
    fn test_symbol_usage_marking() {
        let mut analyzer = SymbolAnalyzer::new();
        
        // Add a symbol
        let mut global_symbols = HashMap::new();
        global_symbols.insert("TestType".to_string(), SymbolInfo {
            name: "TestType".to_string(),
            symbol_type: SymbolType::Type,
            definition_span: Span::new(1, 1, 1, 10),
            used: false,
        });
        analyzer.symbols.insert(SymbolScope::Global, global_symbols);
        
        // Mark it as used
        analyzer.mark_symbol_used("TestType", &SymbolScope::Global);
        
        let symbol = &analyzer.symbols[&SymbolScope::Global]["TestType"];
        assert!(symbol.used);
    }

    #[test]
    fn test_statistics_calculation() {
        let mut analyzer = SymbolAnalyzer::new();
        
        let mut global_symbols = HashMap::new();
        global_symbols.insert("UsedSymbol".to_string(), SymbolInfo {
            name: "UsedSymbol".to_string(),
            symbol_type: SymbolType::Type,
            definition_span: Span::new(1, 1, 1, 10),
            used: true,
        });
        global_symbols.insert("UnusedSymbol".to_string(), SymbolInfo {
            name: "UnusedSymbol".to_string(),
            symbol_type: SymbolType::Type,
            definition_span: Span::new(2, 1, 2, 10),
            used: false,
        });
        analyzer.symbols.insert(SymbolScope::Global, global_symbols);
        
        let stats = analyzer.calculate_statistics();
        assert_eq!(stats.total_symbols, 2);
        assert_eq!(stats.used_symbols, 1);
        assert_eq!(stats.complexity_score, 50.0);
    }
}