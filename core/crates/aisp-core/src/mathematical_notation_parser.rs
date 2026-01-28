//! Enhanced Mathematical Notation Parser for AISP 5.1
//!
//! This module provides sophisticated parsing capabilities for complex Unicode
//! mathematical symbols and expressions found in the AISP 5.1 reference
//! specification, including category theory notation, advanced mathematical
//! operators, and complex mathematical formulas.

use crate::{
    error::{AispError, AispResult},
    ast::canonical::*,
};
use std::collections::HashMap;
use std::str::Chars;
use std::iter::Peekable;
use thiserror::Error;

/// Mathematical notation parsing errors
#[derive(Debug, Error)]
pub enum MathNotationError {
    #[error("Unknown mathematical symbol: '{symbol}' at position {position}")]
    UnknownSymbol { symbol: String, position: usize },
    
    #[error("Invalid mathematical expression: {expression} - {reason}")]
    InvalidExpression { expression: String, reason: String },
    
    #[error("Unsupported Unicode mathematical block: U+{codepoint:04X}")]
    UnsupportedUnicodeBlock { codepoint: u32 },
    
    #[error("Complex mathematical structure parsing failed: {structure_type}")]
    ComplexStructureFailure { structure_type: String },
}

/// Enhanced mathematical expression
#[derive(Debug, Clone, PartialEq)]
pub enum EnhancedMathExpression {
    /// Basic mathematical symbols
    BasicSymbol(String),
    /// Unicode mathematical operators
    UnicodeOperator { symbol: String, unicode_name: String, category: String },
    /// Category theory constructs
    CategoryTheory { construct: CategoryConstruct },
    /// Complex mathematical structures
    ComplexStructure { structure_type: String, components: Vec<EnhancedMathExpression> },
    /// Lambda calculus expressions
    Lambda { parameter: String, body: Box<EnhancedMathExpression> },
    /// Quantified expressions
    Quantified { quantifier: Quantifier, variable: String, domain: String, body: Box<EnhancedMathExpression> },
    /// Function application
    Application { function: Box<EnhancedMathExpression>, argument: Box<EnhancedMathExpression> },
}

/// Category theory constructs
#[derive(Debug, Clone, PartialEq)]
pub enum CategoryConstruct {
    /// Functor: F: C ⇒ D
    Functor { name: String, source: String, target: String },
    /// Natural transformation: η: F ⇒ G
    NaturalTransformation { name: String, source_functor: String, target_functor: String },
    /// Adjunction: L ⊣ R
    Adjunction { left_adjoint: String, right_adjoint: String },
    /// Category: ⟨Objects, Morphisms, ∘, id⟩
    Category { name: String, objects: String, morphisms: String, composition: String, identity: String },
    /// Monad: ⟨T, η, μ⟩
    Monad { endofunctor: String, unit: String, multiplication: String },
}

/// Quantifier types
#[derive(Debug, Clone, PartialEq)]
pub enum Quantifier {
    /// Universal quantifier: ∀
    Forall,
    /// Existential quantifier: ∃
    Exists,
    /// Unique existence: ∃!
    ExistsUnique,
    /// Lambda abstraction: λ
    Lambda,
}

/// Enhanced mathematical notation parser
pub struct MathematicalNotationParser {
    /// Unicode mathematical symbol registry
    symbol_registry: HashMap<String, UnicodeSymbolInfo>,
    /// Category theory symbol registry
    category_registry: HashMap<String, CategorySymbolInfo>,
    /// Parsing configuration
    config: MathParsingConfig,
}

/// Information about Unicode mathematical symbols
#[derive(Debug, Clone)]
pub struct UnicodeSymbolInfo {
    /// Unicode symbol
    pub symbol: String,
    /// Official Unicode name
    pub unicode_name: String,
    /// Mathematical category
    pub category: String,
    /// LaTeX equivalent
    pub latex_equivalent: Option<String>,
    /// Parsing precedence
    pub precedence: i32,
    /// Associativity
    pub associativity: Associativity,
}

/// Information about category theory symbols
#[derive(Debug, Clone)]
pub struct CategorySymbolInfo {
    /// Symbol representation
    pub symbol: String,
    /// Category theory meaning
    pub meaning: String,
    /// Usage context
    pub context: String,
}

/// Operator associativity
#[derive(Debug, Clone, PartialEq)]
pub enum Associativity {
    Left,
    Right,
    None,
}

/// Configuration for mathematical parsing
#[derive(Debug, Clone)]
pub struct MathParsingConfig {
    /// Enable category theory parsing
    pub enable_category_theory: bool,
    /// Enable advanced Unicode support
    pub enable_advanced_unicode: bool,
    /// Enable lambda calculus parsing
    pub enable_lambda_calculus: bool,
    /// Maximum parsing depth for complex expressions
    pub max_parsing_depth: usize,
}

impl Default for MathParsingConfig {
    fn default() -> Self {
        Self {
            enable_category_theory: true,
            enable_advanced_unicode: true,
            enable_lambda_calculus: true,
            max_parsing_depth: 50,
        }
    }
}

impl MathematicalNotationParser {
    /// Create new mathematical notation parser
    pub fn new() -> Self {
        Self {
            symbol_registry: Self::create_symbol_registry(),
            category_registry: Self::create_category_registry(),
            config: MathParsingConfig::default(),
        }
    }
    
    /// Create with custom configuration
    pub fn with_config(config: MathParsingConfig) -> Self {
        let mut parser = Self::new();
        parser.config = config;
        parser
    }
    
    /// Parse enhanced mathematical expression from string
    pub fn parse_mathematical_expression(&self, input: &str) -> AispResult<EnhancedMathExpression> {
        let mut chars = input.chars().peekable();
        self.parse_expression(&mut chars, 0)
    }
    
    /// Parse expression with precedence handling
    fn parse_expression(&self, chars: &mut Peekable<Chars>, depth: usize) -> AispResult<EnhancedMathExpression> {
        if depth > self.config.max_parsing_depth {
            return Err(AispError::validation_error(
                "Maximum parsing depth exceeded".to_string()
            ));
        }
        
        self.skip_whitespace(chars);
        
        if chars.peek().is_none() {
            return Err(AispError::validation_error(
                "Unexpected end of mathematical expression".to_string()
            ));
        }
        
        // Try parsing different types of mathematical expressions
        if let Some(&ch) = chars.peek() {
            match ch {
                // Quantifiers: ∀, ∃, ∃!, λ
                '∀' | '∃' | 'λ' => self.parse_quantified_expression(chars, depth),
                // Category theory symbols
                '𝔽' | '𝔾' | '⟨' | '⇒' | '⊣' | '∘' => self.parse_category_theory_construct(chars, depth),
                // Greek letters and mathematical symbols
                'α'..='ω' | 'Α'..='Ω' => self.parse_unicode_symbol(chars, depth),
                // Mathematical operators
                '≜' | '≔' | '≡' | '⇒' | '↔' | '⊢' | '⊨' | '⊕' | '⊖' | '⊗' => self.parse_mathematical_operator(chars, depth),
                // Parentheses and brackets
                '(' | '[' | '{' => self.parse_bracketed_expression(chars, depth),
                // Regular identifiers and numbers
                _ => self.parse_basic_expression(chars, depth),
            }
        } else {
            Err(AispError::validation_error(
                "Empty mathematical expression".to_string()
            ))
        }
    }
    
    /// Parse quantified expressions (∀, ∃, λ)
    fn parse_quantified_expression(&self, chars: &mut Peekable<Chars>, depth: usize) -> AispResult<EnhancedMathExpression> {
        let quantifier_char = chars.next().unwrap();
        let quantifier = match quantifier_char {
            '∀' => Quantifier::Forall,
            '∃' => {
                // Check for unique existence ∃!
                if chars.peek() == Some(&'!') {
                    chars.next();
                    Quantifier::ExistsUnique
                } else {
                    Quantifier::Exists
                }
            },
            'λ' => Quantifier::Lambda,
            _ => return Err(AispError::validation_error(
                format!("Invalid quantifier: {}", quantifier_char)
            )),
        };
        
        self.skip_whitespace(chars);
        
        // Parse variable
        let variable = self.parse_identifier(chars)?;
        
        // Parse domain (if present)
        let domain = if chars.peek() == Some(&'∈') || chars.peek() == Some(&':') {
            chars.next(); // consume ∈ or :
            self.skip_whitespace(chars);
            self.parse_identifier(chars)?
        } else {
            String::new()
        };
        
        // Parse body
        self.skip_whitespace(chars);
        if chars.peek() == Some(&':') || chars.peek() == Some(&'.') {
            chars.next(); // consume separator
            self.skip_whitespace(chars);
        }
        
        let body = Box::new(self.parse_expression(chars, depth + 1)?);
        
        Ok(EnhancedMathExpression::Quantified {
            quantifier,
            variable,
            domain,
            body,
        })
    }
    
    /// Parse category theory constructs
    fn parse_category_theory_construct(&self, chars: &mut Peekable<Chars>, depth: usize) -> AispResult<EnhancedMathExpression> {
        if !self.config.enable_category_theory {
            return self.parse_basic_expression(chars, depth);
        }
        
        let start_char = chars.peek().cloned().unwrap();
        
        match start_char {
            '𝔽' | '𝔾' => self.parse_functor(chars, depth),
            '⟨' => self.parse_categorical_tuple(chars, depth),
            '⇒' => self.parse_natural_transformation(chars, depth),
            '⊣' => self.parse_adjunction(chars, depth),
            '∘' => self.parse_composition(chars, depth),
            _ => self.parse_unicode_symbol(chars, depth),
        }
    }
    
    /// Parse functor notation: 𝔽:𝐁𝐥𝐤⇒𝐕𝐚𝐥
    fn parse_functor(&self, chars: &mut Peekable<Chars>, _depth: usize) -> AispResult<EnhancedMathExpression> {
        let functor_name = chars.next().unwrap().to_string();
        self.skip_whitespace(chars);
        
        if chars.peek() == Some(&':') {
            chars.next(); // consume ':'
            self.skip_whitespace(chars);
            
            let source = self.parse_category_name(chars)?;
            self.skip_whitespace(chars);
            
            if chars.peek() == Some(&'⇒') {
                chars.next(); // consume '⇒'
                self.skip_whitespace(chars);
                
                let target = self.parse_category_name(chars)?;
                
                return Ok(EnhancedMathExpression::CategoryTheory {
                    construct: CategoryConstruct::Functor {
                        name: functor_name,
                        source,
                        target,
                    },
                });
            }
        }
        
        // Fallback to basic symbol
        Ok(EnhancedMathExpression::BasicSymbol(functor_name))
    }
    
    /// Parse category name (like 𝐁𝐥𝐤, 𝐕𝐚𝐥)
    fn parse_category_name(&self, chars: &mut Peekable<Chars>) -> AispResult<String> {
        let mut name = String::new();
        
        while let Some(&ch) = chars.peek() {
            if ch.is_whitespace() || "⇒⊣∘:".contains(ch) {
                break;
            }
            name.push(chars.next().unwrap());
        }
        
        if name.is_empty() {
            Err(AispError::validation_error("Empty category name".to_string()))
        } else {
            Ok(name)
        }
    }
    
    /// Parse categorical tuple: ⟨Objects, Morphisms, ∘, id⟩
    fn parse_categorical_tuple(&self, chars: &mut Peekable<Chars>, depth: usize) -> AispResult<EnhancedMathExpression> {
        chars.next(); // consume '⟨'
        let mut components = Vec::new();
        
        loop {
            self.skip_whitespace(chars);
            
            if chars.peek() == Some(&'⟩') {
                chars.next(); // consume '⟩'
                break;
            }
            
            let component = self.parse_expression(chars, depth + 1)?;
            components.push(component);
            
            self.skip_whitespace(chars);
            if chars.peek() == Some(&',') {
                chars.next(); // consume ','
            } else if chars.peek() != Some(&'⟩') {
                return Err(AispError::validation_error(
                    "Expected ',' or '⟩' in categorical tuple".to_string()
                ));
            }
        }
        
        Ok(EnhancedMathExpression::ComplexStructure {
            structure_type: "categorical_tuple".to_string(),
            components,
        })
    }
    
    /// Parse natural transformation: η: F ⇒ G
    fn parse_natural_transformation(&self, chars: &mut Peekable<Chars>, _depth: usize) -> AispResult<EnhancedMathExpression> {
        // This would be called when we see ⇒ in a natural transformation context
        // For now, treat as basic symbol
        let symbol = chars.next().unwrap().to_string();
        Ok(EnhancedMathExpression::BasicSymbol(symbol))
    }
    
    /// Parse adjunction: L ⊣ R
    fn parse_adjunction(&self, chars: &mut Peekable<Chars>, _depth: usize) -> AispResult<EnhancedMathExpression> {
        let symbol = chars.next().unwrap().to_string();
        Ok(EnhancedMathExpression::BasicSymbol(symbol))
    }
    
    /// Parse composition: f ∘ g
    fn parse_composition(&self, chars: &mut Peekable<Chars>, _depth: usize) -> AispResult<EnhancedMathExpression> {
        let symbol = chars.next().unwrap().to_string();
        Ok(EnhancedMathExpression::BasicSymbol(symbol))
    }
    
    /// Parse Unicode mathematical symbols
    fn parse_unicode_symbol(&self, chars: &mut Peekable<Chars>, _depth: usize) -> AispResult<EnhancedMathExpression> {
        let symbol = chars.next().unwrap().to_string();
        
        if let Some(symbol_info) = self.symbol_registry.get(&symbol) {
            Ok(EnhancedMathExpression::UnicodeOperator {
                symbol: symbol.clone(),
                unicode_name: symbol_info.unicode_name.clone(),
                category: symbol_info.category.clone(),
            })
        } else {
            Ok(EnhancedMathExpression::BasicSymbol(symbol))
        }
    }
    
    /// Parse mathematical operators
    fn parse_mathematical_operator(&self, chars: &mut Peekable<Chars>, _depth: usize) -> AispResult<EnhancedMathExpression> {
        let mut operator = String::new();
        
        // Handle multi-character operators
        while let Some(&ch) = chars.peek() {
            if "≜≔≡⇒↔⊢⊨⊕⊖⊗".contains(ch) {
                operator.push(chars.next().unwrap());
            } else {
                break;
            }
        }
        
        if let Some(symbol_info) = self.symbol_registry.get(&operator) {
            Ok(EnhancedMathExpression::UnicodeOperator {
                symbol: operator.clone(),
                unicode_name: symbol_info.unicode_name.clone(),
                category: symbol_info.category.clone(),
            })
        } else {
            Ok(EnhancedMathExpression::BasicSymbol(operator))
        }
    }
    
    /// Parse bracketed expressions
    fn parse_bracketed_expression(&self, chars: &mut Peekable<Chars>, depth: usize) -> AispResult<EnhancedMathExpression> {
        let open = chars.next().unwrap();
        let close = match open {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            _ => return Err(AispError::validation_error(
                format!("Unknown bracket type: {}", open)
            )),
        };
        
        let inner = self.parse_expression(chars, depth + 1)?;
        
        self.skip_whitespace(chars);
        if chars.next() != Some(close) {
            return Err(AispError::validation_error(
                format!("Expected closing bracket '{}'", close)
            ));
        }
        
        Ok(inner)
    }
    
    /// Parse basic expressions (identifiers, numbers)
    fn parse_basic_expression(&self, chars: &mut Peekable<Chars>, _depth: usize) -> AispResult<EnhancedMathExpression> {
        let identifier = self.parse_identifier(chars)?;
        Ok(EnhancedMathExpression::BasicSymbol(identifier))
    }
    
    /// Parse identifier
    fn parse_identifier(&self, chars: &mut Peekable<Chars>) -> AispResult<String> {
        let mut identifier = String::new();
        
        while let Some(&ch) = chars.peek() {
            if ch.is_alphanumeric() || "_₀₁₂₃₄₅₆₇₈₉".contains(ch) {
                identifier.push(chars.next().unwrap());
            } else {
                break;
            }
        }
        
        if identifier.is_empty() {
            Err(AispError::validation_error("Empty identifier".to_string()))
        } else {
            Ok(identifier)
        }
    }
    
    /// Skip whitespace characters
    fn skip_whitespace(&self, chars: &mut Peekable<Chars>) {
        while chars.peek().map_or(false, |c| c.is_whitespace()) {
            chars.next();
        }
    }
    
    /// Create Unicode symbol registry
    fn create_symbol_registry() -> HashMap<String, UnicodeSymbolInfo> {
        let mut registry = HashMap::new();
        
        // Logic and proof symbols
        registry.insert("≜".to_string(), UnicodeSymbolInfo {
            symbol: "≜".to_string(),
            unicode_name: "DELTA EQUAL TO".to_string(),
            category: "logic".to_string(),
            latex_equivalent: Some("\\triangleq".to_string()),
            precedence: 1,
            associativity: Associativity::None,
        });
        
        registry.insert("≔".to_string(), UnicodeSymbolInfo {
            symbol: "≔".to_string(),
            unicode_name: "COLON EQUALS".to_string(),
            category: "logic".to_string(),
            latex_equivalent: Some("\\coloneqq".to_string()),
            precedence: 1,
            associativity: Associativity::None,
        });
        
        registry.insert("≡".to_string(), UnicodeSymbolInfo {
            symbol: "≡".to_string(),
            unicode_name: "IDENTICAL TO".to_string(),
            category: "logic".to_string(),
            latex_equivalent: Some("\\equiv".to_string()),
            precedence: 2,
            associativity: Associativity::None,
        });
        
        // Quantifiers
        registry.insert("∀".to_string(), UnicodeSymbolInfo {
            symbol: "∀".to_string(),
            unicode_name: "FOR ALL".to_string(),
            category: "quantifier".to_string(),
            latex_equivalent: Some("\\forall".to_string()),
            precedence: 5,
            associativity: Associativity::Right,
        });
        
        registry.insert("∃".to_string(), UnicodeSymbolInfo {
            symbol: "∃".to_string(),
            unicode_name: "THERE EXISTS".to_string(),
            category: "quantifier".to_string(),
            latex_equivalent: Some("\\exists".to_string()),
            precedence: 5,
            associativity: Associativity::Right,
        });
        
        // Category theory symbols
        registry.insert("⇒".to_string(), UnicodeSymbolInfo {
            symbol: "⇒".to_string(),
            unicode_name: "RIGHTWARDS DOUBLE ARROW".to_string(),
            category: "category_theory".to_string(),
            latex_equivalent: Some("\\Rightarrow".to_string()),
            precedence: 3,
            associativity: Associativity::Right,
        });
        
        registry.insert("⊣".to_string(), UnicodeSymbolInfo {
            symbol: "⊣".to_string(),
            unicode_name: "LEFT TACK".to_string(),
            category: "category_theory".to_string(),
            latex_equivalent: Some("\\dashv".to_string()),
            precedence: 4,
            associativity: Associativity::None,
        });
        
        registry.insert("∘".to_string(), UnicodeSymbolInfo {
            symbol: "∘".to_string(),
            unicode_name: "RING OPERATOR".to_string(),
            category: "category_theory".to_string(),
            latex_equivalent: Some("\\circ".to_string()),
            precedence: 6,
            associativity: Associativity::Left,
        });
        
        // Mathematical operators
        registry.insert("⊕".to_string(), UnicodeSymbolInfo {
            symbol: "⊕".to_string(),
            unicode_name: "CIRCLED PLUS".to_string(),
            category: "operator".to_string(),
            latex_equivalent: Some("\\oplus".to_string()),
            precedence: 4,
            associativity: Associativity::Left,
        });
        
        registry.insert("⊖".to_string(), UnicodeSymbolInfo {
            symbol: "⊖".to_string(),
            unicode_name: "CIRCLED MINUS".to_string(),
            category: "operator".to_string(),
            latex_equivalent: Some("\\ominus".to_string()),
            precedence: 4,
            associativity: Associativity::Left,
        });
        
        registry.insert("⊗".to_string(), UnicodeSymbolInfo {
            symbol: "⊗".to_string(),
            unicode_name: "CIRCLED TIMES".to_string(),
            category: "operator".to_string(),
            latex_equivalent: Some("\\otimes".to_string()),
            precedence: 5,
            associativity: Associativity::Left,
        });
        
        registry
    }
    
    /// Create category theory symbol registry
    fn create_category_registry() -> HashMap<String, CategorySymbolInfo> {
        let mut registry = HashMap::new();
        
        registry.insert("𝔽".to_string(), CategorySymbolInfo {
            symbol: "𝔽".to_string(),
            meaning: "Functor from blocks to validation".to_string(),
            context: "AISP validation functor".to_string(),
        });
        
        registry.insert("𝔾".to_string(), CategorySymbolInfo {
            symbol: "𝔾".to_string(),
            meaning: "Functor from pockets to signals".to_string(),
            context: "AISP signal extraction functor".to_string(),
        });
        
        registry.insert("𝐁𝐥𝐤".to_string(), CategorySymbolInfo {
            symbol: "𝐁𝐥𝐤".to_string(),
            meaning: "Category of AISP blocks".to_string(),
            context: "AISP block category".to_string(),
        });
        
        registry.insert("𝐕𝐚𝐥".to_string(), CategorySymbolInfo {
            symbol: "𝐕𝐚𝐥".to_string(),
            meaning: "Category of validation results".to_string(),
            context: "AISP validation category".to_string(),
        });
        
        registry
    }
}

impl Default for MathematicalNotationParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_symbol_parsing() {
        let parser = MathematicalNotationParser::new();
        let result = parser.parse_mathematical_expression("𝔽").unwrap();
        
        match result {
            EnhancedMathExpression::BasicSymbol(s) => assert_eq!(s, "𝔽"),
            _ => panic!("Expected basic symbol"),
        }
    }
    
    #[test]
    fn test_quantified_expression_parsing() {
        let parser = MathematicalNotationParser::new();
        let result = parser.parse_mathematical_expression("∀x:P(x)").unwrap();
        
        match result {
            EnhancedMathExpression::Quantified { quantifier, variable, domain, .. } => {
                assert_eq!(quantifier, Quantifier::Forall);
                assert_eq!(variable, "x");
                assert_eq!(domain, "P(x)");
            },
            _ => panic!("Expected quantified expression"),
        }
    }
    
    #[test]
    fn test_mathematical_operator_parsing() {
        let parser = MathematicalNotationParser::new();
        let result = parser.parse_mathematical_expression("≜").unwrap();
        
        match result {
            EnhancedMathExpression::UnicodeOperator { symbol, unicode_name, category } => {
                assert_eq!(symbol, "≜");
                assert_eq!(unicode_name, "DELTA EQUAL TO");
                assert_eq!(category, "logic");
            },
            _ => panic!("Expected Unicode operator"),
        }
    }
    
    #[test]
    fn test_symbol_registry() {
        let registry = MathematicalNotationParser::create_symbol_registry();
        
        assert!(registry.contains_key("≜"));
        assert!(registry.contains_key("∀"));
        assert!(registry.contains_key("⊕"));
        assert_eq!(registry["≜"].category, "logic");
        assert_eq!(registry["∀"].precedence, 5);
    }
}