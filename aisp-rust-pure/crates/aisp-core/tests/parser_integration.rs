//! Parser integration tests
//!
//! This module tests the complete parsing pipeline from lexing through AST
//! construction, ensuring all parser components work together correctly.

use aisp_core::{
    AispParser, AispDocument, AispBlock, TypesBlock, MetaBlock, 
    RulesBlock, FunctionsBlock, EvidenceBlock, AispHeader,
    TypeExpression, BasicType, MetaValue, LogicalExpression,
    EvidenceMetric, QualityTier
};

/// Builder for creating parser test cases
pub struct ParserTestBuilder {
    input: String,
    expected_blocks: usize,
    should_fail: bool,
}

impl ParserTestBuilder {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
            expected_blocks: 0,
            should_fail: false,
        }
    }

    pub fn expecting_blocks(mut self, count: usize) -> Self {
        self.expected_blocks = count;
        self
    }

    pub fn should_fail(mut self) -> Self {
        self.should_fail = true;
        self
    }

    pub fn test_parse(self) -> ParseResult {
        let parser = AispParser::new();
        let result = parser.parse(&self.input);

        if self.should_fail {
            assert!(result.is_err(), "Expected parsing to fail but it succeeded");
            ParseResult::Failed
        } else {
            let document = result.expect("Parsing should succeed");
            assert_eq!(document.blocks.len(), self.expected_blocks, 
                "Expected {} blocks but got {}", self.expected_blocks, document.blocks.len());
            ParseResult::Success(document)
        }
    }
}

pub enum ParseResult {
    Success(AispDocument),
    Failed,
}

impl ParseResult {
    pub fn document(self) -> AispDocument {
        match self {
            ParseResult::Success(doc) => doc,
            ParseResult::Failed => panic!("Cannot get document from failed parse result"),
        }
    }
}

/// Helper for asserting document properties
pub struct DocumentAssertion {
    document: AispDocument,
}

impl DocumentAssertion {
    pub fn new(document: AispDocument) -> Self {
        Self { document }
    }

    pub fn has_header_version(self, version: &str) -> Self {
        assert_eq!(self.document.header.version, version);
        self
    }

    pub fn has_header_name(self, name: &str) -> Self {
        assert_eq!(self.document.header.name, name);
        self
    }

    pub fn has_block_count(self, count: usize) -> Self {
        assert_eq!(self.document.blocks.len(), count);
        self
    }

    pub fn has_meta_block(self) -> MetaBlockAssertion {
        let meta_block = self.document.blocks.iter()
            .find_map(|block| match block {
                AispBlock::Meta(meta) => Some(meta),
                _ => None,
            })
            .expect("Document should have meta block");
        
        MetaBlockAssertion::new(meta_block.clone())
    }

    pub fn has_types_block(self) -> TypesBlockAssertion {
        let types_block = self.document.blocks.iter()
            .find_map(|block| match block {
                AispBlock::Types(types) => Some(types),
                _ => None,
            })
            .expect("Document should have types block");
        
        TypesBlockAssertion::new(types_block.clone())
    }

    pub fn has_evidence_block(self) -> EvidenceBlockAssertion {
        let evidence_block = self.document.blocks.iter()
            .find_map(|block| match block {
                AispBlock::Evidence(evidence) => Some(evidence),
                _ => None,
            })
            .expect("Document should have evidence block");
        
        EvidenceBlockAssertion::new(evidence_block.clone())
    }
}

pub struct MetaBlockAssertion {
    meta: MetaBlock,
}

impl MetaBlockAssertion {
    pub fn new(meta: MetaBlock) -> Self {
        Self { meta }
    }

    pub fn has_entry(self, key: &str, expected_value: &str) -> Self {
        let entry = self.meta.entries.get(key)
            .expect(&format!("Meta block should have entry '{}'", key));
        
        match &entry.value {
            MetaValue::String(value) => assert_eq!(value, expected_value),
            _ => panic!("Expected string value for meta entry '{}'", key),
        }
        self
    }

    pub fn has_entry_count(self, count: usize) -> Self {
        assert_eq!(self.meta.entries.len(), count);
        self
    }
}

pub struct TypesBlockAssertion {
    types: TypesBlock,
}

impl TypesBlockAssertion {
    pub fn new(types: TypesBlock) -> Self {
        Self { types }
    }

    pub fn has_definition(self, name: &str) -> Self {
        assert!(self.types.definitions.contains_key(name),
            "Types block should have definition for '{}'", name);
        self
    }

    pub fn has_definition_count(self, count: usize) -> Self {
        assert_eq!(self.types.definitions.len(), count);
        self
    }

    pub fn has_enumeration(self, name: &str, values: &[&str]) -> Self {
        let definition = self.types.definitions.get(name)
            .expect(&format!("Should have type definition for '{}'", name));
        
        match &definition.type_expr {
            TypeExpression::Enumeration(enum_values) => {
                assert_eq!(enum_values.len(), values.len());
                for (i, value) in values.iter().enumerate() {
                    assert_eq!(enum_values[i], *value);
                }
            }
            _ => panic!("Expected enumeration type for '{}'", name),
        }
        self
    }
}

pub struct EvidenceBlockAssertion {
    evidence: EvidenceBlock,
}

impl EvidenceBlockAssertion {
    pub fn new(evidence: EvidenceBlock) -> Self {
        Self { evidence }
    }

    pub fn has_delta(self, expected: f64) -> Self {
        assert!((self.evidence.delta - expected).abs() < 0.001,
            "Expected delta {} but got {}", expected, self.evidence.delta);
        self
    }

    pub fn has_quality_tier(self, expected: QualityTier) -> Self {
        assert_eq!(self.evidence.quality_tier, expected);
        self
    }
}

#[test]
fn test_parse_minimal_document() {
    let input = r#"𝔸5.1.TestDoc@2026-01-25

⟦Ω:Meta⟧{
  domain≜test
  version≜"1.0.0"
}

⟦Ε⟧⟨δ≜0.8⟩"#;

    let document = ParserTestBuilder::new(input)
        .expecting_blocks(2)
        .test_parse()
        .document();

    DocumentAssertion::new(document)
        .has_header_version("5.1")
        .has_header_name("TestDoc")
        .has_block_count(2)
        .has_meta_block()
        .has_entry("domain", "test")
        .has_entry("version", "1.0.0")
        .has_entry_count(2);
}

#[test]
fn test_parse_document_with_types() {
    let input = r#"𝔸5.1.GameLogic@2026-01-25

⟦Σ:Types⟧{
  GameState≜{Start,Playing,GameOver}
  Player≜{PlayerA,PlayerB}
  Move≜ℕ
}

⟦Ε⟧⟨δ≜0.85⟩"#;

    let document = ParserTestBuilder::new(input)
        .expecting_blocks(2)
        .test_parse()
        .document();

    DocumentAssertion::new(document)
        .has_types_block()
        .has_definition_count(3)
        .has_definition("GameState")
        .has_definition("Player")
        .has_definition("Move")
        .has_enumeration("GameState", &["Start", "Playing", "GameOver"])
        .has_enumeration("Player", &["PlayerA", "PlayerB"]);
}

#[test]
fn test_parse_document_with_metadata() {
    let input = r#"𝔸5.1.GameLogic@2026-01-25

γ≔⟨game,turn-based⟩
ρ≔⟨protocol,state-transition⟩

⟦Ω:Meta⟧{
  domain≜game_logic
}

⟦Ε⟧⟨δ≜0.8⟩"#;

    let document = ParserTestBuilder::new(input)
        .expecting_blocks(2)
        .test_parse()
        .document();

    DocumentAssertion::new(document)
        .has_header_version("5.1")
        .has_header_name("GameLogic")
        .has_block_count(2);

    // Check that metadata was parsed
    assert_eq!(document.header.metadata.len(), 2);
    assert_eq!(document.header.metadata[0].key, "γ");
    assert_eq!(document.header.metadata[1].key, "ρ");
}

#[test]
fn test_parse_complete_document() {
    let input = r#"𝔸5.1.CompleteDoc@2026-01-25

γ≔⟨test,complete⟩

⟦Ω:Meta⟧{
  domain≜test_complete
  version≜"2.0.0"
  description≜"Complete test document"
}

⟦Σ:Types⟧{
  State≜{A,B,C}
  Transition≜State→State
  Value≜ℕ
}

⟦Γ:Rules⟧{
  ∀s:State→Valid(s)
  ∀t:Transition→Consistent(t)
}

⟦Λ:Funcs⟧{
  next≜λx.Next(x)
  valid≜λs.IsValid(s)
}

⟦Ε⟧⟨δ≜0.9;φ≜100;τ≜◊⁺⟩"#;

    let document = ParserTestBuilder::new(input)
        .expecting_blocks(5)
        .test_parse()
        .document();

    DocumentAssertion::new(document)
        .has_header_version("5.1")
        .has_header_name("CompleteDoc")
        .has_block_count(5)
        .has_meta_block()
        .has_entry_count(3)
        .has_evidence_block()
        .has_delta(0.9)
        .has_quality_tier(QualityTier::Platinum);
}

#[test]
fn test_parse_document_with_unicode_symbols() {
    let input = r#"𝔸5.1.UnicodeTest@2026-01-25

⟦Σ:Types⟧{
  Natural≜ℕ
  Integer≜ℤ
  Real≜ℝ
  Boolean≜𝔹
  String≜𝕊
}

⟦Γ:Rules⟧{
  ∀x:ℕ→x≥0
  ∃y:ℝ→y>0
  □(P→◊Q)
}

⟦Ε⟧⟨δ≜0.8⟩"#;

    let document = ParserTestBuilder::new(input)
        .expecting_blocks(3)
        .test_parse()
        .document();

    DocumentAssertion::new(document)
        .has_types_block()
        .has_definition_count(5)
        .has_definition("Natural")
        .has_definition("Integer")
        .has_definition("Real")
        .has_definition("Boolean")
        .has_definition("String");
}

#[test]
fn test_parse_malformed_header() {
    let input = r#"INVALID_HEADER

⟦Ω:Meta⟧{
  domain≜test
}

⟦Ε⟧⟨δ≜0.8⟩"#;

    ParserTestBuilder::new(input)
        .should_fail()
        .test_parse();
}

#[test]
fn test_parse_invalid_block_structure() {
    let input = r#"𝔸5.1.TestDoc@2026-01-25

⟦Ω:Meta⟧
  domain≜test
  # Missing closing brace

⟦Ε⟧⟨δ≜0.8⟩"#;

    ParserTestBuilder::new(input)
        .should_fail()
        .test_parse();
}

#[test]
fn test_parse_invalid_evidence_block() {
    let input = r#"𝔸5.1.TestDoc@2026-01-25

⟦Ω:Meta⟧{
  domain≜test
}

⟦Ε⟧⟨invalid_metric⟩"#;

    ParserTestBuilder::new(input)
        .should_fail()
        .test_parse();
}

#[test]
fn test_parse_empty_blocks() {
    let input = r#"𝔸5.1.TestDoc@2026-01-25

⟦Ω:Meta⟧{
}

⟦Σ:Types⟧{
}

⟦Ε⟧⟨δ≜0.8⟩"#;

    let document = ParserTestBuilder::new(input)
        .expecting_blocks(3)
        .test_parse()
        .document();

    DocumentAssertion::new(document)
        .has_meta_block()
        .has_entry_count(0);
}

#[test]
fn test_parse_complex_type_expressions() {
    let input = r#"𝔸5.1.TypeTest@2026-01-25

⟦Σ:Types⟧{
  Array≜ℕ[10]
  Function≜ℕ → 𝔹
  Tuple≜(ℕ,𝔹,𝕊)
  Nested≜ℕ[5] → (𝔹,𝕊)
}

⟦Ε⟧⟨δ≜0.8⟩"#;

    let document = ParserTestBuilder::new(input)
        .expecting_blocks(2)
        .test_parse()
        .document();

    DocumentAssertion::new(document)
        .has_types_block()
        .has_definition_count(4)
        .has_definition("Array")
        .has_definition("Function")
        .has_definition("Tuple")
        .has_definition("Nested");
}

#[test]
fn test_parser_error_recovery() {
    // Test that parser can handle and report multiple errors
    let input = r#"𝔸5.1.ErrorTest@2026-01-25

⟦Ω:Meta⟧{
  domain≜test
  invalid_syntax_here!!!
  version≜"1.0.0"
}

⟦Σ:Types⟧{
  ValidType≜{A,B,C}
  InvalidType≜UnknownSyntax!!!
}

⟦Ε⟧⟨δ≜invalid_number⟩"#;

    // This should fail due to multiple syntax errors
    ParserTestBuilder::new(input)
        .should_fail()
        .test_parse();
}