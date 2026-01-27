// Robust AISP Parser with Security Hardening and Error Recovery
// Implements ADR-022: Pest Parser Migration for Robustness

use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;
use std::fmt;
use crate::error::{AispError, AispResult};

// Simplified AST types for parser
#[derive(Debug, Clone, Default)]
pub struct AispDocument {
    pub header: DocumentHeader,
    pub metadata: DocumentMetadata,
    pub blocks: Vec<AispBlock>,
}

#[derive(Debug, Clone, Default)]
pub struct DocumentHeader {
    pub version: String,
    pub name: String,
    pub date: String,
    pub metadata: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct DocumentMetadata {
    pub domain: Option<String>,
    pub protocol: Option<String>,
}

#[derive(Debug, Clone)]
pub enum AispBlock {
    Meta(MetaBlock),
    Types(TypesBlock),
    Rules(RulesBlock),
    Functions(FunctionsBlock),
    Errors(ErrorsBlock),
    Evidence(EvidenceBlock),
}

#[derive(Debug, Clone, Default)]
pub struct MetaBlock {
    pub entries: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct TypesBlock {
    pub definitions: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct RulesBlock {
    pub rules: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct FunctionsBlock {
    pub functions: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ErrorsBlock {
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct EvidenceBlock {
    pub evidence: Vec<String>,
}


/// Re-export pest types  
pub use pest::iterators::{Pair, Pairs};

// Create a module to properly export the generated Rule enum
pub mod parser_impl {
    use super::*;
    use pest::Parser;
    use pest_derive::Parser;
    
    #[derive(Parser)]
    #[grammar_inline = r#"
WHITESPACE = _{ " " | "\t" | "\n" | "\r" }
COMMENT = _{ "//" ~ (!"\n" ~ ANY)* }

// Top-level document structure
aisp_document = { 
    SOI ~ 
    header ~ 
    domain_protocol_decl? ~
    aisp_blocks ~ 
    EOI 
}

// Header with version, identifier, and date
header = { "𝔸" ~ version ~ "." ~ doc_identifier ~ "@" ~ date }
version = { ASCII_DIGIT+ ~ "." ~ ASCII_DIGIT+ }
doc_identifier = { (ASCII_ALPHANUMERIC | "-" | "_" | ".")+ }
date = { ASCII_DIGIT{4} ~ "-" ~ ASCII_DIGIT{2} ~ "-" ~ ASCII_DIGIT{2} }

// Domain and protocol declarations
domain_protocol_decl = { 
    (gamma_decl ~ rho_decl?) | 
    (rho_decl ~ gamma_decl?) 
}
gamma_decl = { "γ" ~ "≔" ~ domain_path }
rho_decl = { "ρ" ~ "≔" ~ "⟨" ~ tag_list ~ "⟩" }
domain_path = { (ASCII_ALPHANUMERIC | "." | "-" | "_")+ }
tag_list = { tag ~ ("," ~ tag)* }
tag = { (ASCII_ALPHANUMERIC | "-" | "_")+ }

// AISP block structure
aisp_blocks = { aisp_block* }
aisp_block = { 
    omega_block | 
    sigma_block | 
    gamma_block | 
    lambda_block | 
    chi_block | 
    epsilon_block |
    malformed_block
}

// Block definitions
omega_block = { "⟦" ~ "Ω" ~ ":" ~ "Meta" ~ "⟧" ~ "{" ~ meta_entries ~ "}" }
sigma_block = { "⟦" ~ "Σ" ~ ":" ~ "Types" ~ "⟧" ~ "{" ~ type_definitions ~ "}" }
gamma_block = { "⟦" ~ "Γ" ~ ":" ~ "Rules" ~ "⟧" ~ "{" ~ rule_definitions ~ "}" }
lambda_block = { "⟦" ~ "Λ" ~ ":" ~ ("Funcs" | "Functions") ~ "⟧" ~ "{" ~ function_definitions ~ "}" }
chi_block = { "⟦" ~ "Χ" ~ ":" ~ "Errors" ~ "⟧" ~ "{" ~ error_definitions ~ "}" }
epsilon_block = { "⟦" ~ "Ε" ~ (":" ~ "Evidence")? ~ "⟧" ~ "⟨" ~ evidence_entries ~ "⟩" }

// Block content
meta_entries = { meta_entry* }
meta_entry = { identifier ~ "≜" ~ string_literal ~ ";"? }

type_definitions = { type_definition* }
type_definition = { identifier ~ "≜" ~ type_expression ~ ";"? }

rule_definitions = { rule_definition* }
rule_definition = { logical_expr ~ ";"? }

function_definitions = { function_definition* }
function_definition = { identifier ~ "≜" ~ lambda_expression ~ ";"? }

error_definitions = { error_definition* }
error_definition = { identifier ~ "≜" ~ logical_expr ~ ";"? }

evidence_entries = { evidence_entry* }
evidence_entry = { evidence_symbol ~ "≜" ~ evidence_value ~ ";"? }

// Expression types
type_expression = { identifier | basic_type }
basic_type = { "ℕ" | "ℝ" | "ℂ" | "ℚ" | "ℤ" | "𝕊" | "𝔹" }

lambda_expression = { "λ" ~ identifier ~ "." ~ identifier | identifier }

logical_expr = { identifier ~ ("∈" | "≡" | "⊆") ~ identifier | identifier }

evidence_symbol = { "δ" | "φ" | "τ" | identifier }
evidence_value = { number | string_literal }

// Primitives
identifier = { (ASCII_ALPHA | "_") ~ (ASCII_ALPHANUMERIC | "_" | "-")* }
number = { ASCII_DIGIT+ ~ ("." ~ ASCII_DIGIT+)? }
string_literal = { "\"" ~ (!"\"" ~ ANY)* ~ "\"" }

// Error recovery
malformed_block = { "⟦" ~ (!"⟧" ~ ANY)* ~ ("⟧" | &EOI) }
    "#]
    pub struct InternalParser;
    
    pub use InternalParser as AispParser;
}

// Re-export the parser and rule types
pub use parser_impl::{AispParser, Rule};

/// Security-hardened parser with error recovery capabilities
pub struct RobustAispParser {
    strict_mode: bool,
    recovery_enabled: bool,
    max_error_count: usize,
    unicode_normalization: bool,
    security_validation: bool,
}

/// Parse result with comprehensive error reporting
#[derive(Debug, Clone)]
pub struct ParseResult {
    pub document: Option<AispDocument>,
    pub errors: Vec<ParseError>,
    pub warnings: Vec<ParseWarning>,
    pub recovery_applied: bool,
    pub partial_success: bool,
    pub security_issues: Vec<SecurityIssue>,
}

/// Enhanced parse error with security context
#[derive(Debug, Clone)]
pub struct ParseError {
    pub error_type: ParseErrorType,
    pub line: usize,
    pub column: usize,
    pub message: String,
    pub context: String,
    pub security_impact: SecurityImpact,
    pub suggestions: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseErrorType {
    SyntaxError,
    UnicodeError,
    StructuralError,
    SecurityViolation,
    RecoveryFailure,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SecurityImpact {
    None,
    Low,
    Medium,
    High,
    Critical,
}

/// Parse warnings for non-fatal issues
#[derive(Debug, Clone)]
pub struct ParseWarning {
    pub warning_type: WarningType,
    pub line: usize,
    pub column: usize,
    pub message: String,
    pub recommendation: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WarningType {
    DeprecatedSyntax,
    AmbiguousConstruct,
    SecurityRisk,
    PerformanceIssue,
}

/// Security issues detected during parsing
#[derive(Debug, Clone)]
pub struct SecurityIssue {
    pub issue_type: SecurityIssueType,
    pub severity: SecuritySeverity,
    pub description: String,
    pub location: (usize, usize),
    pub mitigation: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SecurityIssueType {
    UnicodeNormalizationAttack,
    ExcessiveNesting,
    SuspiciousPattern,
    ResourceExhaustion,
    EncodingManipulation,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SecuritySeverity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

/// Block boundary information for error recovery
#[derive(Debug, Clone)]
pub struct BlockBoundary {
    pub block_type: String,
    pub start_pos: usize,
    pub end_pos: usize,
    pub content: String,
    pub is_well_formed: bool,
}

impl Default for RobustAispParser {
    fn default() -> Self {
        Self {
            strict_mode: false,
            recovery_enabled: true,
            max_error_count: 100,
            unicode_normalization: true,
            security_validation: true,
        }
    }
}

impl RobustAispParser {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn strict() -> Self {
        Self {
            strict_mode: true,
            recovery_enabled: false,
            max_error_count: 1,
            unicode_normalization: true,
            security_validation: true,
        }
    }

    pub fn with_security_validation(mut self, enabled: bool) -> Self {
        self.security_validation = enabled;
        self
    }

    pub fn with_error_recovery(mut self, enabled: bool) -> Self {
        self.recovery_enabled = enabled;
        self
    }

    /// Main parsing entry point with comprehensive error handling
    pub fn parse(&self, input: &str) -> ParseResult {
        // Pre-parse security validation
        if self.security_validation {
            if let Some(security_issue) = self.detect_pre_parse_security_issues(input) {
                return ParseResult {
                    document: None,
                    errors: vec![],
                    warnings: vec![],
                    recovery_applied: false,
                    partial_success: false,
                    security_issues: vec![security_issue],
                };
            }
        }

        // Attempt primary parsing
        match AispParser::parse(Rule::aisp_document, input) {
            Ok(pairs) => {
                match self.build_ast_from_pairs(pairs, input) {
                    Ok(document) => ParseResult {
                        document: Some(document),
                        errors: vec![],
                        warnings: vec![],
                        recovery_applied: false,
                        partial_success: false,
                        security_issues: vec![],
                    },
                    Err(ast_error) => {
                        if self.recovery_enabled {
                            self.attempt_error_recovery(input, ast_error)
                        } else {
                            ParseResult {
                                document: None,
                                errors: vec![self.convert_ast_error_to_parse_error(ast_error)],
                                warnings: vec![],
                                recovery_applied: false,
                                partial_success: false,
                                security_issues: vec![],
                            }
                        }
                    }
                }
            }
            Err(pest_error) => {
                if self.recovery_enabled {
                    self.parse_with_error_recovery(input, pest_error)
                } else {
                    ParseResult {
                        document: None,
                        errors: vec![self.convert_pest_error_to_parse_error(pest_error)],
                        warnings: vec![],
                        recovery_applied: false,
                        partial_success: false,
                        security_issues: vec![],
                    }
                }
            }
        }
    }

    /// Error recovery parsing when primary parsing fails
    fn parse_with_error_recovery(&self, input: &str, original_error: pest::error::Error<Rule>) -> ParseResult {
        let mut document = AispDocument::default();
        let mut errors = vec![self.convert_pest_error_to_parse_error(original_error)];
        let mut warnings = vec![];
        let mut security_issues = vec![];

        // Extract block boundaries for partial parsing
        let block_boundaries = self.extract_block_boundaries(input);
        
        for boundary in &block_boundaries {
            match self.parse_single_block(&boundary) {
                Ok(block) => {
                    document.blocks.push(block);
                    if !boundary.is_well_formed {
                        warnings.push(ParseWarning {
                            warning_type: WarningType::AmbiguousConstruct,
                            line: self.position_to_line(input, boundary.start_pos),
                            column: self.position_to_column(input, boundary.start_pos),
                            message: format!("Recovered malformed {} block", boundary.block_type),
                            recommendation: "Verify block syntax and content".to_string(),
                        });
                    }
                }
                Err(block_error) => {
                    errors.push(ParseError {
                        error_type: ParseErrorType::RecoveryFailure,
                        line: self.position_to_line(input, boundary.start_pos),
                        column: self.position_to_column(input, boundary.start_pos),
                        message: format!("Failed to recover {} block: {}", boundary.block_type, block_error),
                        context: boundary.content.chars().take(50).collect(),
                        security_impact: SecurityImpact::Medium,
                        suggestions: vec![
                            "Check block syntax".to_string(),
                            "Verify Unicode encoding".to_string(),
                        ],
                    });
                }
            }
        }

        // Security validation on recovered content
        if self.security_validation {
            security_issues.extend(self.validate_recovered_content(&document, &block_boundaries));
        }

        ParseResult {
            document: if document.blocks.is_empty() { None } else { Some(document.clone()) },
            errors,
            warnings,
            recovery_applied: true,
            partial_success: !document.blocks.is_empty(),
            security_issues,
        }
    }

    /// Build AST from successfully parsed Pest pairs
    fn build_ast_from_pairs(&self, pairs: Pairs<Rule>, input: &str) -> AispResult<AispDocument> {
        let mut document = AispDocument::default();
        
        for pair in pairs {
            match pair.as_rule() {
                Rule::aisp_document => {
                    for inner_pair in pair.into_inner() {
                        match inner_pair.as_rule() {
                            Rule::header => {
                                document.header = self.parse_header(inner_pair)?;
                            }
                            Rule::domain_protocol_decl => {
                                document.metadata = self.parse_metadata(inner_pair)?;
                            }
                            Rule::aisp_blocks => {
                                for block_pair in inner_pair.into_inner() {
                                    if let Ok(block) = self.parse_block(block_pair) {
                                        document.blocks.push(block);
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(document)
    }

    /// Parse document header
    fn parse_header(&self, pair: Pair<Rule>) -> AispResult<DocumentHeader> {
        let mut version = String::new();
        let mut name = String::new();
        let mut date = String::new();

        for inner in pair.into_inner() {
            match inner.as_rule() {
                Rule::version => version = inner.as_str().to_string(),
                Rule::doc_identifier => name = inner.as_str().to_string(),
                Rule::date => date = inner.as_str().to_string(),
                _ => {}
            }
        }

        Ok(DocumentHeader {
            version,
            name,
            date,
            metadata: None,
        })
    }

    /// Parse document metadata
    fn parse_metadata(&self, pair: Pair<Rule>) -> AispResult<DocumentMetadata> {
        let mut domain = None;
        let mut protocol = None;

        for inner in pair.into_inner() {
            match inner.as_rule() {
                Rule::gamma_decl => {
                    for gamma_inner in inner.into_inner() {
                        if matches!(gamma_inner.as_rule(), Rule::domain_path) {
                            domain = Some(gamma_inner.as_str().to_string());
                        }
                    }
                }
                Rule::rho_decl => {
                    for rho_inner in inner.into_inner() {
                        if matches!(rho_inner.as_rule(), Rule::tag_list) {
                            protocol = Some(rho_inner.as_str().to_string());
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(DocumentMetadata { domain, protocol })
    }

    /// Parse individual block with type detection
    fn parse_block(&self, pair: Pair<Rule>) -> AispResult<AispBlock> {
        match pair.as_rule() {
            Rule::omega_block => self.parse_omega_block(pair),
            Rule::sigma_block => self.parse_sigma_block(pair),
            Rule::gamma_block => self.parse_gamma_block(pair),
            Rule::lambda_block => self.parse_lambda_block(pair),
            Rule::chi_block => self.parse_chi_block(pair),
            Rule::epsilon_block => self.parse_epsilon_block(pair),
            Rule::malformed_block => {
                // Handle malformed blocks gracefully
                Err(AispError::ParseError {
                    line: 0,
                    column: 0,
                    message: "Malformed block detected".to_string(),
                })
            }
            _ => Err(AispError::ParseError {
                line: 0,
                column: 0,
                message: format!("Unexpected block type: {:?}", pair.as_rule()),
            }),
        }
    }

    // Block parsing methods (simplified for space - full implementation would handle each block type)
    fn parse_omega_block(&self, pair: Pair<Rule>) -> AispResult<AispBlock> {
        let mut meta_block = MetaBlock::default();
        
        for inner in pair.into_inner() {
            match inner.as_rule() {
                Rule::meta_entries => {
                    for entry in inner.into_inner() {
                        meta_block.entries.push(entry.as_str().to_string());
                    }
                }
                _ => {}
            }
        }
        
        Ok(AispBlock::Meta(meta_block))
    }

    fn parse_sigma_block(&self, pair: Pair<Rule>) -> AispResult<AispBlock> {
        let mut types_block = TypesBlock::default();
        
        for inner in pair.into_inner() {
            match inner.as_rule() {
                Rule::type_definitions => {
                    for def in inner.into_inner() {
                        types_block.definitions.push(def.as_str().to_string());
                    }
                }
                _ => {}
            }
        }
        
        Ok(AispBlock::Types(types_block))
    }

    fn parse_gamma_block(&self, pair: Pair<Rule>) -> AispResult<AispBlock> {
        let mut rules_block = RulesBlock::default();
        
        for inner in pair.into_inner() {
            match inner.as_rule() {
                Rule::rule_definitions => {
                    for rule in inner.into_inner() {
                        rules_block.rules.push(rule.as_str().to_string());
                    }
                }
                _ => {}
            }
        }
        
        Ok(AispBlock::Rules(rules_block))
    }

    fn parse_lambda_block(&self, pair: Pair<Rule>) -> AispResult<AispBlock> {
        let mut functions_block = FunctionsBlock::default();
        
        for inner in pair.into_inner() {
            match inner.as_rule() {
                Rule::function_definitions => {
                    for func in inner.into_inner() {
                        functions_block.functions.push(func.as_str().to_string());
                    }
                }
                _ => {}
            }
        }
        
        Ok(AispBlock::Functions(functions_block))
    }

    fn parse_chi_block(&self, pair: Pair<Rule>) -> AispResult<AispBlock> {
        let mut errors_block = ErrorsBlock::default();
        
        for inner in pair.into_inner() {
            match inner.as_rule() {
                Rule::error_definitions => {
                    for error in inner.into_inner() {
                        errors_block.errors.push(error.as_str().to_string());
                    }
                }
                _ => {}
            }
        }
        
        Ok(AispBlock::Errors(errors_block))
    }

    fn parse_epsilon_block(&self, pair: Pair<Rule>) -> AispResult<AispBlock> {
        let mut evidence_block = EvidenceBlock::default();
        
        for inner in pair.into_inner() {
            match inner.as_rule() {
                Rule::evidence_entries => {
                    for evidence in inner.into_inner() {
                        evidence_block.evidence.push(evidence.as_str().to_string());
                    }
                }
                _ => {}
            }
        }
        
        Ok(AispBlock::Evidence(evidence_block))
    }

    /// Extract block boundaries for error recovery
    fn extract_block_boundaries(&self, input: &str) -> Vec<BlockBoundary> {
        let mut boundaries = Vec::new();
        let block_patterns = vec![
            ("Omega", "⟦Ω:Meta⟧"),
            ("Sigma", "⟦Σ:Types⟧"),
            ("Gamma", "⟦Γ:Rules⟧"),
            ("Lambda", "⟦Λ:Funcs⟧"),
            ("Lambda", "⟦Λ:Functions⟧"),
            ("Chi", "⟦Χ:Errors⟧"),
            ("Epsilon", "⟦Ε:Evidence⟧"),
            ("Epsilon", "⟦Ε⟧"),
        ];

        for (block_type, pattern) in block_patterns {
            if let Some(start) = input.find(pattern) {
                // Find matching closing delimiter
                let search_start = start + pattern.len();
                if let Some(end) = self.find_block_end(input, search_start, block_type) {
                    boundaries.push(BlockBoundary {
                        block_type: block_type.to_string(),
                        start_pos: start,
                        end_pos: end,
                        content: input[start..=end].to_string(),
                        is_well_formed: self.validate_block_structure(&input[start..=end]),
                    });
                }
            }
        }

        boundaries.sort_by_key(|b| b.start_pos);
        boundaries
    }

    /// Find the end of a block based on balanced delimiters
    fn find_block_end(&self, input: &str, start: usize, block_type: &str) -> Option<usize> {
        let remaining = &input[start..];
        
        if block_type == "Epsilon" {
            // Evidence blocks use ⟨ ⟩ delimiters
            self.find_balanced_delimiter(remaining, '⟨', '⟩').map(|pos| start + pos)
        } else {
            // Other blocks use { } delimiters
            self.find_balanced_delimiter(remaining, '{', '}').map(|pos| start + pos)
        }
    }

    /// Find balanced delimiter pairs
    fn find_balanced_delimiter(&self, text: &str, open: char, close: char) -> Option<usize> {
        let mut depth = 0i32;
        let mut start_found = false;

        for (i, ch) in text.char_indices() {
            if ch == open {
                depth += 1;
                start_found = true;
            } else if ch == close && start_found {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
        }

        None
    }

    /// Validate block structure for well-formedness
    fn validate_block_structure(&self, block_content: &str) -> bool {
        // Basic structural validation
        let open_braces = block_content.matches('{').count();
        let close_braces = block_content.matches('}').count();
        let open_angles = block_content.matches('⟨').count();
        let close_angles = block_content.matches('⟩').count();

        (open_braces == close_braces) && (open_angles == close_angles)
    }

    /// Parse individual block during error recovery
    fn parse_single_block(&self, boundary: &BlockBoundary) -> AispResult<AispBlock> {
        match AispParser::parse(Rule::aisp_block, &boundary.content) {
            Ok(mut pairs) => {
                if let Some(pair) = pairs.next() {
                    self.parse_block(pair)
                } else {
                    Err(AispError::ParseError {
                        line: 0,
                        column: 0,
                        message: "Empty block content".to_string(),
                    })
                }
            }
            Err(_) => {
                // Attempt graceful degradation
                Ok(self.create_placeholder_block(&boundary.block_type))
            }
        }
    }

    /// Create placeholder block for recovery
    fn create_placeholder_block(&self, block_type: &str) -> AispBlock {
        match block_type {
            "Omega" => AispBlock::Meta(Default::default()),
            "Sigma" => AispBlock::Types(Default::default()),
            "Gamma" => AispBlock::Rules(Default::default()),
            "Lambda" => AispBlock::Functions(Default::default()),
            "Chi" => AispBlock::Errors(Default::default()),
            "Epsilon" => AispBlock::Evidence(Default::default()),
            _ => AispBlock::Meta(Default::default()),
        }
    }

    /// Security validation methods
    fn detect_pre_parse_security_issues(&self, input: &str) -> Option<SecurityIssue> {
        // Check for excessive size (potential DoS)
        if input.len() > 1_000_000 {  // 1MB limit
            return Some(SecurityIssue {
                issue_type: SecurityIssueType::ResourceExhaustion,
                severity: SecuritySeverity::High,
                description: "Input exceeds maximum size limit".to_string(),
                location: (0, 0),
                mitigation: "Reduce input size or increase limits with caution".to_string(),
            });
        }

        // Check for excessive nesting depth
        let max_depth = self.calculate_nesting_depth(input);
        if max_depth > 50 {
            return Some(SecurityIssue {
                issue_type: SecurityIssueType::ExcessiveNesting,
                severity: SecuritySeverity::Medium,
                description: format!("Excessive nesting depth: {}", max_depth),
                location: (0, 0),
                mitigation: "Limit nesting depth to prevent stack overflow".to_string(),
            });
        }

        // Check for Unicode normalization attacks
        if self.has_unicode_normalization_issues(input) {
            return Some(SecurityIssue {
                issue_type: SecurityIssueType::UnicodeNormalizationAttack,
                severity: SecuritySeverity::Medium,
                description: "Potential Unicode normalization attack detected".to_string(),
                location: (0, 0),
                mitigation: "Normalize Unicode input before processing".to_string(),
            });
        }

        None
    }

    fn calculate_nesting_depth(&self, input: &str) -> usize {
        let mut depth = 0i32;
        let mut max_depth = 0usize;

        for ch in input.chars() {
            match ch {
                '{' | '⟨' | '(' => {
                    depth += 1;
                    max_depth = max_depth.max(depth as usize);
                }
                '}' | '⟩' | ')' => {
                    depth = depth.saturating_sub(1);
                }
                _ => {}
            }
        }

        max_depth
    }

    fn has_unicode_normalization_issues(&self, input: &str) -> bool {
        // Check for mixed normalization forms
        // This is a simplified check - production would use proper Unicode normalization
        input.contains('\u{200D}') || // Zero Width Joiner
        input.contains('\u{200C}') || // Zero Width Non-Joiner  
        input.contains('\u{FEFF}')    // Byte Order Mark
    }

    fn validate_recovered_content(&self, document: &AispDocument, boundaries: &[BlockBoundary]) -> Vec<SecurityIssue> {
        let mut issues = Vec::new();

        // Check for suspicious patterns in recovered content
        for boundary in boundaries {
            if !boundary.is_well_formed {
                issues.push(SecurityIssue {
                    issue_type: SecurityIssueType::SuspiciousPattern,
                    severity: SecuritySeverity::Low,
                    description: format!("Malformed {} block recovered", boundary.block_type),
                    location: (self.position_to_line("", boundary.start_pos), 0),
                    mitigation: "Verify block content integrity".to_string(),
                });
            }
        }

        issues
    }

    /// Utility methods
    fn convert_pest_error_to_parse_error(&self, error: pest::error::Error<Rule>) -> ParseError {
        let (line, column) = match error.line_col {
            pest::error::LineColLocation::Pos((line, col)) => (line, col),
            pest::error::LineColLocation::Span((line, col), _) => (line, col),
        };

        ParseError {
            error_type: ParseErrorType::SyntaxError,
            line,
            column,
            message: error.to_string(),
            context: String::new(),
            security_impact: SecurityImpact::Medium,
            suggestions: vec![
                "Check syntax near error location".to_string(),
                "Verify Unicode characters are valid".to_string(),
            ],
        }
    }

    fn convert_ast_error_to_parse_error(&self, error: AispError) -> ParseError {
        match error {
            AispError::ParseError { line, column, message } => ParseError {
                error_type: ParseErrorType::StructuralError,
                line,
                column,
                message,
                context: String::new(),
                security_impact: SecurityImpact::Low,
                suggestions: vec!["Check document structure".to_string()],
            },
            _ => ParseError {
                error_type: ParseErrorType::StructuralError,
                line: 0,
                column: 0,
                message: error.to_string(),
                context: String::new(),
                security_impact: SecurityImpact::Low,
                suggestions: vec![],
            }
        }
    }

    fn attempt_error_recovery(&self, input: &str, error: AispError) -> ParseResult {
        // Simplified error recovery - delegate to main recovery method
        let fake_pest_error = pest::error::Error::new_from_pos(
            pest::error::ErrorVariant::CustomError { message: error.to_string() },
            pest::Position::from_start(input),
        );
        self.parse_with_error_recovery(input, fake_pest_error)
    }

    fn position_to_line(&self, _input: &str, _pos: usize) -> usize {
        // Simplified - would implement actual line counting
        1
    }

    fn position_to_column(&self, _input: &str, _pos: usize) -> usize {
        // Simplified - would implement actual column counting
        1
    }
}

// Implement Display traits for error reporting
impl fmt::Display for ParseResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref doc) = self.document {
            write!(f, "Successfully parsed document: {}", doc.header.name)?;
        } else {
            write!(f, "Failed to parse document")?;
        }

        if !self.errors.is_empty() {
            write!(f, "\nErrors ({}): ", self.errors.len())?;
            for error in &self.errors {
                write!(f, "\n  - {}", error)?;
            }
        }

        if !self.warnings.is_empty() {
            write!(f, "\nWarnings ({}): ", self.warnings.len())?;
            for warning in &self.warnings {
                write!(f, "\n  - {}", warning)?;
            }
        }

        if !self.security_issues.is_empty() {
            write!(f, "\nSecurity Issues ({}): ", self.security_issues.len())?;
            for issue in &self.security_issues {
                write!(f, "\n  - {}", issue)?;
            }
        }

        Ok(())
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}:{}] {}: {}", self.line, self.column, self.error_type, self.message)
    }
}

impl fmt::Display for ParseErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseErrorType::SyntaxError => write!(f, "Syntax Error"),
            ParseErrorType::UnicodeError => write!(f, "Unicode Error"),
            ParseErrorType::StructuralError => write!(f, "Structural Error"),
            ParseErrorType::SecurityViolation => write!(f, "Security Violation"),
            ParseErrorType::RecoveryFailure => write!(f, "Recovery Failure"),
        }
    }
}

impl fmt::Display for SecurityIssue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:?}] {}: {}", self.severity, self.issue_type, self.description)
    }
}

impl fmt::Display for SecurityIssueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SecurityIssueType::UnicodeNormalizationAttack => write!(f, "Unicode Normalization Attack"),
            SecurityIssueType::ExcessiveNesting => write!(f, "Excessive Nesting"),
            SecurityIssueType::SuspiciousPattern => write!(f, "Suspicious Pattern"),
            SecurityIssueType::ResourceExhaustion => write!(f, "Resource Exhaustion"),
            SecurityIssueType::EncodingManipulation => write!(f, "Encoding Manipulation"),
        }
    }
}

impl fmt::Display for ParseWarning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}:{}] {}: {}", self.line, self.column, self.warning_type, self.message)
    }
}

impl fmt::Display for WarningType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WarningType::DeprecatedSyntax => write!(f, "Deprecated Syntax"),
            WarningType::AmbiguousConstruct => write!(f, "Ambiguous Construct"),
            WarningType::SecurityRisk => write!(f, "Security Risk"),
            WarningType::PerformanceIssue => write!(f, "Performance Issue"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_robust_parser_creation() {
        let parser = RobustAispParser::new();
        assert!(parser.recovery_enabled);
        assert!(parser.security_validation);
    }

    #[test]
    fn test_strict_parser_creation() {
        let parser = RobustAispParser::strict();
        assert!(!parser.recovery_enabled);
        assert!(parser.strict_mode);
    }

    #[test]
    fn test_parse_valid_document() {
        let parser = RobustAispParser::new();
        let input = r#"𝔸5.1.test-document@2026-01-27
⟦Ω:Meta⟧{
  Vision≜"Test document"
}
⟦Ε⟧⟨δ≜0.01⟩"#;

        let result = parser.parse(input);
        assert!(result.partial_success || result.document.is_some());
    }

    #[test]
    fn test_parse_malformed_document_with_recovery() {
        let parser = RobustAispParser::new();
        let input = r#"𝔸5.1.malformed@2026-01-27
⟦Ω:Meta⟧{
  Vision≜"Missing close brace"
⟦Ε⟧⟨δ≜0.01⟩"#;

        let result = parser.parse(input);
        assert!(result.recovery_applied);
        assert!(!result.errors.is_empty());
    }

    #[test]
    fn test_security_validation() {
        let parser = RobustAispParser::new().with_security_validation(true);
        
        // Test excessive size
        let large_input = "a".repeat(2_000_000);
        let result = parser.parse(&large_input);
        assert!(!result.security_issues.is_empty());
    }

    #[test]
    fn test_error_recovery_disabled() {
        let parser = RobustAispParser::new().with_error_recovery(false);
        let malformed_input = "invalid aisp document";
        
        let result = parser.parse(malformed_input);
        assert!(!result.recovery_applied);
        assert!(result.document.is_none());
    }
}