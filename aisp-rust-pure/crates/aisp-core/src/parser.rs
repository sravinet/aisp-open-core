//! High-performance AISP parser with zero-copy parsing where possible

use crate::ast::*;
use crate::error::*;
use crate::symbols::*;
use std::collections::HashMap;

/// AISP parser with support for incremental parsing
pub struct AispParser {
    /// Source text being parsed
    source: String,
    /// Current parsing position
    position: usize,
    /// Current line number (1-indexed)
    line: usize,
    /// Current column number (1-indexed)  
    column: usize,
    /// Collected warnings during parsing
    warnings: Vec<AispWarning>,
}

impl AispParser {
    /// Create a new parser for the given source
    pub fn new(source: String) -> Self {
        Self {
            source,
            position: 0,
            line: 1,
            column: 1,
            warnings: Vec::new(),
        }
    }

    /// Parse complete AISP document
    pub fn parse(&mut self) -> AispResult<AispDocument> {
        // Parse document header
        let header = self.parse_header()?;
        
        // Parse metadata (γ and ρ)
        let metadata = self.parse_metadata()?;
        
        // Parse all blocks
        let mut blocks = Vec::new();
        
        while !self.is_at_end() {
            self.skip_whitespace_and_comments();
            if self.is_at_end() {
                break;
            }
            
            let block = self.parse_block()?;
            blocks.push(block);
        }
        
        // Validate required blocks are present
        self.validate_required_blocks(&blocks)?;
        
        let span = Span::new(1, 1, self.line, self.column);
        
        Ok(AispDocument {
            header,
            metadata,
            blocks,
            span,
        })
    }

    /// Get collected warnings
    pub fn warnings(&self) -> &[AispWarning] {
        &self.warnings
    }

    /// Parse document header (𝔸5.1.name@date)
    fn parse_header(&mut self) -> AispResult<DocumentHeader> {
        self.skip_whitespace_and_comments();
        
        // Expect 𝔸 symbol
        if !self.match_char('𝔸') {
            return Err(AispError::parse_error(
                self.line,
                self.column,
                "Expected AISP header starting with 𝔸",
            ));
        }
        
        // Parse version
        let version = self.parse_version()?;
        
        // Expect dot
        if !self.match_char('.') {
            return Err(AispError::parse_error(
                self.line,
                self.column,
                "Expected '.' after version",
            ));
        }
        
        // Parse name
        let name = self.parse_identifier()?;
        
        // Parse optional metadata and date
        let (metadata, date) = if self.match_char('@') {
            let date = self.parse_date()?;
            (None, date)
        } else if self.peek() == Some('#') {
            self.advance(); // consume #
            let meta = self.parse_identifier()?;
            if self.match_char('@') {
                let date = self.parse_date()?;
                (Some(meta), date)
            } else {
                return Err(AispError::parse_error(
                    self.line,
                    self.column,
                    "Expected '@' after metadata",
                ));
            }
        } else {
            return Err(AispError::parse_error(
                self.line,
                self.column,
                "Expected '@' or '#' after name",
            ));
        };
        
        Ok(DocumentHeader {
            version,
            name,
            date,
            metadata,
        })
    }

    /// Parse version number (e.g., "5.1")
    fn parse_version(&mut self) -> AispResult<String> {
        let mut version = String::new();
        
        // Parse major version
        if !self.peek().map_or(false, |c| c.is_ascii_digit()) {
            return Err(AispError::parse_error(
                self.line,
                self.column,
                "Expected digit in version number",
            ));
        }
        
        while self.peek().map_or(false, |c| c.is_ascii_digit()) {
            version.push(self.advance().unwrap());
        }
        
        // Expect dot
        if !self.match_char('.') {
            return Err(AispError::parse_error(
                self.line,
                self.column,
                "Expected '.' in version number",
            ));
        }
        version.push('.');
        
        // Parse minor version
        if !self.peek().map_or(false, |c| c.is_ascii_digit()) {
            return Err(AispError::parse_error(
                self.line,
                self.column,
                "Expected digit after '.' in version number",
            ));
        }
        
        while self.peek().map_or(false, |c| c.is_ascii_digit()) {
            version.push(self.advance().unwrap());
        }
        
        Ok(version)
    }

    /// Parse document metadata (γ and ρ declarations)
    fn parse_metadata(&mut self) -> AispResult<DocumentMetadata> {
        self.skip_whitespace_and_comments();
        
        let mut domain = None;
        let mut protocol = None;
        
        // Look for γ (domain) and ρ (protocol) declarations
        while !self.is_at_end() && (self.peek() == Some('γ') || self.peek() == Some('ρ')) {
            let var = self.advance().unwrap();
            
            if !self.match_char('≔') {
                return Err(AispError::parse_error(
                    self.line,
                    self.column,
                    format!("Expected '≔' after '{}'", var),
                ));
            }
            
            let value = self.parse_metadata_value()?;
            
            match var {
                'γ' => domain = Some(value),
                'ρ' => protocol = Some(value),
                _ => unreachable!(),
            }
            
            self.skip_whitespace_and_comments();
        }
        
        Ok(DocumentMetadata { domain, protocol })
    }

    /// Parse metadata value
    fn parse_metadata_value(&mut self) -> AispResult<String> {
        self.skip_whitespace_and_comments();
        
        if self.match_char('⟨') {
            // Parse tuple value like ⟨game,turn-based⟩
            let mut value = String::new();
            while !self.check('⟩') && !self.is_at_end() {
                if let Some(ch) = self.advance() {
                    value.push(ch);
                }
            }
            
            if !self.match_char('⟩') {
                return Err(AispError::parse_error(
                    self.line,
                    self.column,
                    "Expected '⟩' to close tuple",
                ));
            }
            
            Ok(value.trim().to_string())
        } else {
            // Parse simple identifier
            self.parse_identifier()
        }
    }

    /// Parse a block (⟦Type:Name⟧{...})
    fn parse_block(&mut self) -> AispResult<AispBlock> {
        // Expect ⟦
        if !self.match_char('⟦') {
            return Err(AispError::parse_error(
                self.line,
                self.column,
                "Expected '⟦' to start block",
            ));
        }
        
        // Parse block identifier
        let block_id = self.parse_block_identifier()?;
        
        // Expect ⟧
        if !self.match_char('⟧') {
            return Err(AispError::parse_error(
                self.line,
                self.column,
                "Expected '⟧' after block identifier",
            ));
        }
        
        // Parse block content based on type
        let block = match block_id.as_str() {
            "Ε" => {
                // Evidence block uses ⟨⟩ instead of {}
                AispBlock::Evidence(self.parse_evidence_block()?)
            }
            _ => {
                // Other blocks use {}
                if !self.match_char('{') {
                    return Err(AispError::parse_error(
                        self.line,
                        self.column,
                        "Expected '{' after block header",
                    ));
                }
                
                let block = match block_id.as_str() {
                    "Ω:Meta" => AispBlock::Meta(self.parse_meta_block()?),
                    "Σ:Types" => AispBlock::Types(self.parse_types_block()?),
                    "Γ:Rules" => AispBlock::Rules(self.parse_rules_block()?),
                    "Λ:Funcs" => AispBlock::Functions(self.parse_functions_block()?),
                    _ => {
                        return Err(AispError::parse_error(
                            self.line,
                            self.column,
                            format!("Unknown block type: {}", block_id),
                        ));
                    }
                };
                
                // Expect }
                if !self.match_char('}') {
                    return Err(AispError::parse_error(
                        self.line,
                        self.column,
                        "Expected '}' to close block",
                    ));
                }
                
                block
            }
        };
        
        Ok(block)
    }

    /// Parse meta block content
    fn parse_meta_block(&mut self) -> AispResult<MetaBlock> {
        let start_line = self.line;
        let mut entries = HashMap::new();
        
        while !self.check('}') && !self.is_at_end() {
            self.skip_whitespace_and_comments();
            if self.check('}') {
                break;
            }
            
            // Check if this line starts with a quantifier (logical constraint)
            if self.peek() == Some('∀') || self.peek() == Some('∃') {
                // Parse as logical constraint and store with special key
                let constraint = self.parse_logical_expression()?;
                let span = Span::new(self.line, 1, self.line, self.column);
                
                // Use a special key pattern for constraints
                let constraint_key = format!("_constraint_{}", entries.len());
                entries.insert(
                    constraint_key.clone(),
                    MetaEntry { 
                        key: constraint_key, 
                        value: MetaValue::Constraint(constraint), 
                        span 
                    },
                );
            } else {
                // Parse as regular key=value pair
                let key = self.parse_identifier()?;
                
                if !self.match_char('≜') {
                    return Err(AispError::parse_error(
                        self.line,
                        self.column,
                        "Expected '≜' in meta entry",
                    ));
                }
                
                let value = self.parse_meta_value()?;
                let span = Span::new(self.line, 1, self.line, self.column);
                
                entries.insert(
                    key.clone(),
                    MetaEntry { key, value, span },
                );
            }
        }
        
        Ok(MetaBlock {
            entries,
            span: Span::new(start_line, 1, self.line, self.column),
        })
    }

    /// Parse meta value (string, number, boolean, or constraint)
    fn parse_meta_value(&mut self) -> AispResult<MetaValue> {
        self.skip_whitespace_and_comments();
        
        // Try to parse as logical constraint first (contains AISP symbols)
        let start_pos = self.position;
        let remaining: String = self.source.chars().skip(self.position).collect();
        
        // Look ahead for AISP symbols that indicate a logical expression
        if remaining.chars().any(|c| is_aisp_symbol(c)) {
            return Ok(MetaValue::Constraint(self.parse_logical_expression()?));
        }
        
        // Reset position and try other value types
        self.position = start_pos;
        
        if let Some(ch) = self.peek() {
            match ch {
                '"' => {
                    self.advance(); // consume "
                    let mut value = String::new();
                    while let Some(ch) = self.peek() {
                        if ch == '"' {
                            self.advance();
                            break;
                        }
                        value.push(self.advance().unwrap());
                    }
                    Ok(MetaValue::String(value))
                }
                ch if ch.is_ascii_digit() || ch == '.' || ch == '-' => {
                    Ok(MetaValue::Number(self.parse_number()?))
                }
                't' | 'f' => {
                    // Try to parse boolean
                    if remaining.starts_with("true") {
                        // Advance 4 characters
                        for _ in 0..4 { self.advance(); }
                        Ok(MetaValue::Boolean(true))
                    } else if remaining.starts_with("false") {
                        // Advance 5 characters
                        for _ in 0..5 { self.advance(); }
                        Ok(MetaValue::Boolean(false))
                    } else {
                        Ok(MetaValue::String(self.parse_identifier()?))
                    }
                }
                _ => Ok(MetaValue::String(self.parse_identifier()?)),
            }
        } else {
            Err(AispError::parse_error(
                self.line,
                self.column,
                "Unexpected end of input in meta value",
            ))
        }
    }

    /// Parse types block content  
    fn parse_types_block(&mut self) -> AispResult<TypesBlock> {
        let start_line = self.line;
        let mut definitions = HashMap::new();
        
        while !self.check('}') && !self.is_at_end() {
            self.skip_whitespace_and_comments();
            if self.check('}') {
                break;
            }
            
            let name = self.parse_identifier()?;
            
            if !self.match_char('≜') {
                return Err(AispError::parse_error(
                    self.line,
                    self.column,
                    "Expected '≜' in type definition",
                ));
            }
            
            let type_expr = self.parse_type_expression()?;
            let span = Span::new(self.line, 1, self.line, self.column);
            
            definitions.insert(
                name.clone(),
                TypeDefinition {
                    name: name.clone(),
                    type_expr,
                    span,
                },
            );
        }
        
        Ok(TypesBlock {
            definitions,
            span: Span::new(start_line, 1, self.line, self.column),
        })
    }

    /// Parse type expression
    fn parse_type_expression(&mut self) -> AispResult<TypeExpression> {
        self.skip_whitespace_and_comments();
        
        if let Some(ch) = self.peek() {
            match ch {
                '{' => {
                    // Enumeration type {A, B, C}
                    self.advance(); // consume {
                    let mut values = Vec::new();
                    
                    while !self.check('}') && !self.is_at_end() {
                        self.skip_whitespace_and_comments();
                        if self.check('}') {
                            break;
                        }
                        
                        values.push(self.parse_identifier()?);
                        
                        self.skip_whitespace_and_comments();
                        if self.match_char(',') {
                            continue;
                        } else if self.check('}') {
                            break;
                        } else {
                            return Err(AispError::parse_error(
                                self.line,
                                self.column,
                                "Expected ',' or '}' in enumeration",
                            ));
                        }
                    }
                    
                    if !self.match_char('}') {
                        return Err(AispError::parse_error(
                            self.line,
                            self.column,
                            "Expected '}' to close enumeration",
                        ));
                    }
                    
                    Ok(TypeExpression::Enumeration(values))
                }
                'ℕ' => {
                    self.advance();
                    self.parse_type_suffix(TypeExpression::Basic(BasicType::Natural))
                }
                'ℤ' => {
                    self.advance();
                    self.parse_type_suffix(TypeExpression::Basic(BasicType::Integer))
                }
                'ℝ' => {
                    self.advance();
                    self.parse_type_suffix(TypeExpression::Basic(BasicType::Real))
                }
                '𝔹' => {
                    self.advance();
                    self.parse_type_suffix(TypeExpression::Basic(BasicType::Boolean))
                }
                '𝕊' => {
                    self.advance();
                    self.parse_type_suffix(TypeExpression::Basic(BasicType::String))
                }
                _ => {
                    // Type reference
                    let name = self.parse_identifier()?;
                    self.parse_type_suffix(TypeExpression::Reference(name))
                }
            }
        } else {
            Err(AispError::parse_error(
                self.line,
                self.column,
                "Expected type expression",
            ))
        }
    }

    /// Parse type suffix (array, function arrow, etc.)
    fn parse_type_suffix(&mut self, base_type: TypeExpression) -> AispResult<TypeExpression> {
        if self.match_char('[') {
            // Array type Type[n]
            let size = if self.check(']') {
                None
            } else {
                Some(self.parse_number()? as usize)
            };
            
            if !self.match_char(']') {
                return Err(AispError::parse_error(
                    self.line,
                    self.column,
                    "Expected ']' after array size",
                ));
            }
            
            Ok(TypeExpression::Array {
                element_type: Box::new(base_type),
                size,
            })
        } else if self.match_char('→') || self.match_str("->") {
            // Function type A → B
            let output = self.parse_type_expression()?;
            Ok(TypeExpression::Function {
                input: Box::new(base_type),
                output: Box::new(output),
            })
        } else {
            Ok(base_type)
        }
    }

    /// Parse rules block (logical rules)
    fn parse_rules_block(&mut self) -> AispResult<RulesBlock> {
        let start_line = self.line;
        let mut rules = Vec::new();
        
        while !self.check('}') && !self.is_at_end() {
            self.skip_whitespace_and_comments();
            if self.check('}') {
                break;
            }
            
            rules.push(self.parse_logical_rule()?);
        }
        
        Ok(RulesBlock {
            rules,
            span: Span::new(start_line, 1, self.line, self.column),
        })
    }

    /// Parse a logical rule
    fn parse_logical_rule(&mut self) -> AispResult<LogicalRule> {
        let start_line = self.line;
        
        // Check for quantifier
        let quantifier = if self.peek() == Some('∀') || self.peek() == Some('∃') {
            Some(self.parse_quantifier()?)
        } else {
            None
        };
        
        let expression = self.parse_logical_expression()?;
        
        Ok(LogicalRule {
            quantifier,
            expression,
            span: Span::new(start_line, 1, self.line, self.column),
        })
    }

    /// Parse quantifier (∀ or ∃)
    fn parse_quantifier(&mut self) -> AispResult<Quantifier> {
        let start_line = self.line;
        
        let kind = match self.advance() {
            Some('∀') => QuantifierKind::Universal,
            Some('∃') => QuantifierKind::Existential,
            _ => {
                return Err(AispError::parse_error(
                    self.line,
                    self.column,
                    "Expected quantifier",
                ));
            }
        };
        
        let variable = self.parse_identifier()?;
        
        // Optional domain specification
        let domain = if self.match_char('∈') || self.match_char(':') {
            Some(self.parse_identifier()?)
        } else {
            None
        };
        
        Ok(Quantifier {
            kind,
            variable,
            domain,
            span: Span::new(start_line, 1, self.line, self.column),
        })
    }

    /// Parse logical expression (simplified - consumes rest of rule)
    fn parse_logical_expression(&mut self) -> AispResult<LogicalExpression> {
        self.skip_whitespace_and_comments();
        
        // Simplified: just consume the rest of the rule as a single expression
        let mut expression = String::new();
        
        while !self.is_at_end() && !self.check('\n') && !self.check('}') {
            if let Some(ch) = self.advance() {
                expression.push(ch);
            }
        }
        
        let expression = expression.trim().to_string();
        
        if expression.is_empty() {
            Err(AispError::parse_error(
                self.line,
                self.column,
                "Expected logical expression",
            ))
        } else {
            // For now, represent as a variable with the full expression text
            Ok(LogicalExpression::Variable(expression))
        }
    }

    /// Parse functions block
    fn parse_functions_block(&mut self) -> AispResult<FunctionsBlock> {
        let start_line = self.line;
        let mut functions = HashMap::new();
        
        while !self.check('}') && !self.is_at_end() {
            self.skip_whitespace_and_comments();
            if self.check('}') {
                break;
            }
            
            let name = self.parse_identifier()?;
            
            if !self.match_char('≜') {
                return Err(AispError::parse_error(
                    self.line,
                    self.column,
                    "Expected '≜' in function definition",
                ));
            }
            
            let lambda = self.parse_lambda_expression()?;
            let span = Span::new(self.line, 1, self.line, self.column);
            
            functions.insert(
                name.clone(),
                FunctionDefinition {
                    name: name.clone(),
                    lambda,
                    span,
                },
            );
        }
        
        Ok(FunctionsBlock {
            functions,
            span: Span::new(start_line, 1, self.line, self.column),
        })
    }

    /// Parse lambda expression
    fn parse_lambda_expression(&mut self) -> AispResult<LambdaExpression> {
        let start_line = self.line;
        
        if !self.match_char('λ') {
            return Err(AispError::parse_error(
                self.line,
                self.column,
                "Expected 'λ' in lambda expression",
            ));
        }
        
        // Parse parameters - can be with or without parentheses
        let mut parameters = Vec::new();
        
        if self.check('(') {
            // Parse parameters with parentheses: λ(x,y)
            self.advance(); // consume '('
            while !self.check(')') && !self.is_at_end() {
                parameters.push(self.parse_identifier()?);
                if self.match_char(',') {
                    continue;
                } else if self.check(')') {
                    break;
                } else {
                    return Err(AispError::parse_error(
                        self.line,
                        self.column,
                        "Expected ',' or ')' in parameter list",
                    ));
                }
            }
            
            if !self.match_char(')') {
                return Err(AispError::parse_error(
                    self.line,
                    self.column,
                    "Expected ')' after parameter list",
                ));
            }
        } else {
            // Parse single parameter without parentheses: λx
            // Read until we hit '.'
            while !self.check('.') && !self.is_at_end() {
                let param_char = self.advance().unwrap();
                if param_char.is_alphanumeric() {
                    // Simple single-letter parameter
                    parameters.push(param_char.to_string());
                    break;
                }
            }
        }
        
        if !self.match_char('.') {
            return Err(AispError::parse_error(
                self.line,
                self.column,
                "Expected '.' after lambda parameters",
            ));
        }
        
        let body = self.parse_logical_expression()?;
        
        Ok(LambdaExpression {
            parameters,
            body,
            span: Span::new(start_line, 1, self.line, self.column),
        })
    }

    /// Parse evidence block
    fn parse_evidence_block(&mut self) -> AispResult<EvidenceBlock> {
        let start_line = self.line;
        let mut delta = None;
        let mut phi = None;
        let mut tau = None;
        let mut metrics = HashMap::new();
        
        self.skip_whitespace_and_comments();
        
        if self.match_char('⟨') {
            // Parse evidence tuple ⟨δ≜0.65;φ≜100;τ≜◊⁺⟩
            while !self.check('⟩') && !self.is_at_end() {
                let key = self.parse_identifier()?;
                
                if !self.match_char('≜') {
                    return Err(AispError::parse_error(
                        self.line,
                        self.column,
                        "Expected '≜' in evidence entry",
                    ));
                }
                
                match key.as_str() {
                    "δ" => delta = Some(self.parse_number()?),
                    "φ" => phi = Some(self.parse_number()?),
                    "τ" => tau = Some(self.parse_tier_symbol()?),
                    _ => {
                        metrics.insert(key, self.parse_number()?);
                    }
                }
                
                if self.match_char(';') {
                    continue;
                } else if self.check('⟩') {
                    break;
                } else {
                    return Err(AispError::parse_error(
                        self.line,
                        self.column,
                        "Expected ';' or '⟩' in evidence block",
                    ));
                }
            }
            
            if !self.match_char('⟩') {
                return Err(AispError::parse_error(
                    self.line,
                    self.column,
                    "Expected '⟩' to close evidence block",
                ));
            }
        }
        
        Ok(EvidenceBlock {
            delta,
            phi,
            tau,
            metrics,
            span: Span::new(start_line, 1, self.line, self.column),
        })
    }

    // Helper methods

    fn parse_identifier(&mut self) -> AispResult<String> {
        self.skip_whitespace_and_comments();
        let mut identifier = String::new();
        
        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() || ch == '_' || ch == '-' {
                identifier.push(self.advance().unwrap());
            } else {
                break;
            }
        }
        
        if identifier.is_empty() {
            return Err(AispError::parse_error(
                self.line,
                self.column,
                "Expected identifier",
            ));
        }
        
        Ok(identifier)
    }

    fn parse_tier_symbol(&mut self) -> AispResult<String> {
        self.skip_whitespace_and_comments();
        let mut symbol = String::new();
        
        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() || ch == '_' || ch == '-' || 
               ch == '◊' || ch == '⊘' || ch == '⁺' || ch == '⁻' {
                symbol.push(self.advance().unwrap());
            } else {
                break;
            }
        }
        
        if symbol.is_empty() {
            return Err(AispError::parse_error(
                self.line,
                self.column,
                "Expected tier symbol",
            ));
        }
        
        Ok(symbol)
    }

    fn parse_date(&mut self) -> AispResult<String> {
        let mut date = String::new();
        
        // Parse YYYY-MM-DD format (flexible length)
        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() || ch == '-' {
                date.push(self.advance().unwrap());
            } else {
                break;
            }
        }
        
        // Basic validation - at least 4 digits at start
        if date.len() >= 4 && date.chars().take(4).all(|c| c.is_ascii_digit()) {
            Ok(date)
        } else {
            Err(AispError::parse_error(
                self.line,
                self.column,
                "Invalid date format",
            ))
        }
    }

    fn parse_number(&mut self) -> AispResult<f64> {
        let mut number = String::new();
        
        // Handle negative numbers
        if self.peek() == Some('-') {
            number.push(self.advance().unwrap());
        }
        
        // Parse digits
        while self.peek().map_or(false, |c| c.is_ascii_digit()) {
            number.push(self.advance().unwrap());
        }
        
        // Handle decimal point
        if self.peek() == Some('.') {
            number.push(self.advance().unwrap());
            while self.peek().map_or(false, |c| c.is_ascii_digit()) {
                number.push(self.advance().unwrap());
            }
        }
        
        if number.is_empty() || number == "-" {
            return Err(AispError::parse_error(
                self.line,
                self.column,
                "Expected number",
            ));
        }
        
        number.parse::<f64>().map_err(|_| {
            AispError::parse_error(self.line, self.column, "Invalid number format")
        })
    }

    fn parse_block_identifier(&mut self) -> AispResult<String> {
        let mut identifier = String::new();
        
        while let Some(ch) = self.peek() {
            if ch != '⟧' {
                identifier.push(self.advance().unwrap());
            } else {
                break;
            }
        }
        
        Ok(identifier.trim().to_string())
    }

    fn validate_required_blocks(&self, blocks: &[AispBlock]) -> AispResult<()> {
        let mut has_meta = false;
        let mut has_types = false;
        let mut has_rules = false;
        let mut has_functions = false;
        let mut has_evidence = false;
        
        for block in blocks {
            match block {
                AispBlock::Meta(_) => has_meta = true,
                AispBlock::Types(_) => has_types = true,
                AispBlock::Rules(_) => has_rules = true,
                AispBlock::Functions(_) => has_functions = true,
                AispBlock::Evidence(_) => has_evidence = true,
            }
        }
        
        if !has_meta {
            return Err(AispError::MissingBlock {
                block_name: "Meta".to_string(),
            });
        }
        if !has_types {
            return Err(AispError::MissingBlock {
                block_name: "Types".to_string(),
            });
        }
        if !has_rules {
            return Err(AispError::MissingBlock {
                block_name: "Rules".to_string(),
            });
        }
        if !has_functions {
            return Err(AispError::MissingBlock {
                block_name: "Functions".to_string(),
            });
        }
        if !has_evidence {
            return Err(AispError::MissingBlock {
                block_name: "Evidence".to_string(),
            });
        }
        
        Ok(())
    }

    // Utility methods

    fn is_at_end(&self) -> bool {
        self.position >= self.source.chars().count()
    }

    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.position)
    }

    fn advance(&mut self) -> Option<char> {
        if let Some(ch) = self.peek() {
            // Find the byte position of the character at self.position
            let mut char_indices = self.source.char_indices().skip(self.position);
            if let Some((_, current_char)) = char_indices.next() {
                if let Some((next_byte_idx, _)) = char_indices.next() {
                    // Update position to next character index
                    self.position += 1;
                } else {
                    // This is the last character
                    self.position += 1;
                }
            }
            
            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            Some(ch)
        } else {
            None
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.peek() == Some(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn match_str(&mut self, expected: &str) -> bool {
        let start_pos = self.position;
        let chars_from_pos: Vec<char> = self.source.chars().skip(start_pos).collect();
        let expected_chars: Vec<char> = expected.chars().collect();
        
        if chars_from_pos.len() < expected_chars.len() {
            return false;
        }
        
        // Check if the expected string matches at current position
        for (i, &expected_char) in expected_chars.iter().enumerate() {
            if i >= chars_from_pos.len() || chars_from_pos[i] != expected_char {
                return false;
            }
        }
        
        // If we got here, the string matches - now consume it
        for _ in expected.chars() {
            self.advance();
        }
        true
    }

    fn check(&self, expected: char) -> bool {
        self.peek() == Some(expected)
    }

    fn skip_whitespace_and_comments(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.advance();
            } else if ch == '/' {
                // Check if next character is also '/'
                let chars_from_pos: Vec<char> = self.source.chars().skip(self.position).collect();
                if chars_from_pos.len() > 1 && chars_from_pos[1] == '/' {
                    // Skip line comment
                    while let Some(ch) = self.peek() {
                        self.advance();
                        if ch == '\n' {
                            break;
                        }
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_header() {
        let mut parser = AispParser::new("𝔸5.1.test@2026-01-25".to_string());
        let header = parser.parse_header().unwrap();
        
        assert_eq!(header.version, "5.1");
        assert_eq!(header.name, "test");
        assert_eq!(header.date, "2026-01-25");
        assert_eq!(header.metadata, None);
    }

    #[test]
    fn test_parse_minimal_document() {
        let source = r#"
𝔸5.1.test@2026-01-25

⟦Ω:Meta⟧{
  domain≜test
}

⟦Σ:Types⟧{
  State≜{A,B}
}

⟦Γ:Rules⟧{
  ∀x:State→Next(x)
}

⟦Λ:Funcs⟧{
  next≜λx.Next(x)
}

⟦Ε⟧⟨δ≜0.8⟩
        "#.trim();

        let mut parser = AispParser::new(source.to_string());
        match parser.parse() {
            Ok(doc) => {
                assert_eq!(doc.blocks.len(), 5);
                assert_eq!(doc.header.name, "test");
            }
            Err(e) => {
                panic!("Parse failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_parse_simple_document() {
        let source = r#"
𝔸5.1.test@2026-01-25

⟦Ω:Meta⟧{
  domain≜test
}

⟦Σ:Types⟧{
  State≜{Start,End}
}

⟦Γ:Rules⟧{
  ∀x:State→NextState(x)
}

⟦Λ:Funcs⟧{
  next≜λx.NextState(x)
}

⟦Ε⟧⟨δ≜0.8;φ≜100;τ≜◊⁺⟩
        "#.trim();

        let mut parser = AispParser::new(source.to_string());
        let doc = parser.parse();
        
        assert!(doc.is_ok(), "Parse failed: {:?}", doc.err());
        let doc = doc.unwrap();
        
        assert_eq!(doc.header.name, "test");
        assert_eq!(doc.blocks.len(), 5);
    }
}