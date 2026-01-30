# ADR-023: Multi-Format Parser Architecture

**Status:** Proposed  
**Date:** 2026-01-30  
**Authors:** Software Architecture Analysis  
**Supersedes:** ADR-022 (extends, does not replace)  

## Context

Our consolidated SRP-compliant parser architecture (ADR-022) successfully handles pure AISP documents but fails to parse `reference.md`, which contains mixed Markdown + embedded AISP content. This represents a critical capability gap for the AISP ecosystem.

### Current Capability Assessment

```
✅ Pure AISP Documents:
   𝔸5.1.name@date
   ⟦Ω:Meta⟧{...}

❌ Mixed Format Documents (reference.md):
   # Markdown Header
   ```aisp
   ⟦Ω:Core⟧{...}
   ```
```

### Architectural Analysis

1. **SRP Compliance Preserved** - Current specialized content parsers remain correct
2. **Format Detection Missing** - No mechanism to identify document types
3. **Multi-Format Orchestration Missing** - No coordination between Markdown and AISP parsers
4. **Content Extraction Missing** - No extraction of AISP blocks from Markdown fences

## Decision

We will extend the parser architecture with a **Multi-Format Parser Orchestrator** that maintains SRP compliance while supporting mixed-format documents.

### Architecture Design

```
┌─────────────────────────────────────────────────────────────┐
│ Parser Facade (Entry Point)                                │
├─────────────────────────────────────────────────────────────┤
│ Format Detection Layer                                      │
│   • Document Type Analysis                                  │
│   • Format Classification                                   │
│   • Parser Selection Strategy                               │
├─────────────────────────────────────────────────────────────┤
│ Format-Specific Orchestrators                               │
│                                                             │
│ ┌─────────────────┐ ┌─────────────────┐ ┌─────────────────┐ │
│ │ Pure AISP       │ │ Mixed Format    │ │ Future Formats  │ │
│ │ Orchestrator    │ │ Orchestrator    │ │ (JSON, XML...)  │ │
│ │                 │ │                 │ │                 │ │
│ │ ┌─────────────┐ │ │ ┌─────────────┐ │ │ ┌─────────────┐ │ │
│ │ │RobustParser │ │ │ │MarkdownPar.│ │ │ │   Future    │ │ │
│ │ └─────────────┘ │ │ └─────────────┘ │ │ └─────────────┘ │ │
│ │        │        │ │        │        │ │                 │ │
│ │        ▼        │ │        ▼        │ │                 │ │
│ │ ┌─────────────┐ │ │ ┌─────────────┐ │ │                 │ │
│ │ │SRP Content  │ │ │ │AISP Extract.│ │ │                 │ │
│ │ │ Parsers     │ │ │ │   +         │ │ │                 │ │
│ │ │             │ │ │ │SRP Content  │ │ │                 │ │
│ │ │             │ │ │ │ Parsers     │ │ │                 │ │
│ │ └─────────────┘ │ │ └─────────────┘ │ │                 │ │
│ └─────────────────┘ └─────────────────┘ └─────────────────┘ │
└─────────────────────────────────────────────────────────────┘
```

## Implementation Strategy

### Phase 1: Format Detection Layer

```rust
// New format detection module
pub mod format_detection {
    use crate::error::AispResult;
    
    #[derive(Debug, Clone, PartialEq)]
    pub enum DocumentFormat {
        PureAisp,           // 𝔸5.1.name@date
        MarkdownWithAisp,   // # Header + ```aisp blocks
        MixedFormat,        // Multiple embedded formats
        Unknown,            // Unrecognized format
    }
    
    pub struct FormatDetector;
    
    impl FormatDetector {
        pub fn detect(content: &str) -> DocumentFormat {
            let trimmed = content.trim();
            
            // Pure AISP: starts with 𝔸
            if trimmed.starts_with('𝔸') {
                return DocumentFormat::PureAisp;
            }
            
            // Markdown with AISP: has # headers and ```aisp blocks
            if trimmed.starts_with('#') && content.contains("```aisp") {
                return DocumentFormat::MarkdownWithAisp;
            }
            
            // Mixed format: contains multiple format indicators
            if content.contains("```aisp") || content.contains("⟦") {
                return DocumentFormat::MixedFormat;
            }
            
            DocumentFormat::Unknown
        }
    }
}
```

### Phase 2: Multi-Format Orchestrator

```rust
// New multi-format orchestrator
pub mod multi_format {
    use super::format_detection::{DocumentFormat, FormatDetector};
    use super::robust_parser::AispParser;
    use crate::ast::canonical::AispDocument;
    use crate::error::AispResult;
    
    pub struct MultiFormatParser;
    
    impl MultiFormatParser {
        pub fn parse(content: &str) -> AispResult<ParsedDocument> {
            let format = FormatDetector::detect(content);
            
            match format {
                DocumentFormat::PureAisp => {
                    Self::parse_pure_aisp(content)
                }
                DocumentFormat::MarkdownWithAisp => {
                    Self::parse_markdown_with_aisp(content)
                }
                DocumentFormat::MixedFormat => {
                    Self::parse_mixed_format(content)
                }
                DocumentFormat::Unknown => {
                    Err(AispError::UnknownFormat)
                }
            }
        }
        
        fn parse_pure_aisp(content: &str) -> AispResult<ParsedDocument> {
            let mut parser = AispParser::new(content.to_string());
            let aisp_doc = parser.parse()?;
            
            Ok(ParsedDocument::Pure(aisp_doc))
        }
        
        fn parse_markdown_with_aisp(content: &str) -> AispResult<ParsedDocument> {
            let extractor = AispCodeBlockExtractor::new();
            let aisp_blocks = extractor.extract_aisp_blocks(content)?;
            let markdown_content = extractor.extract_markdown_content(content)?;
            
            let mut aisp_documents = Vec::new();
            for block_content in aisp_blocks {
                let mut parser = AispParser::new(block_content);
                aisp_documents.push(parser.parse()?);
            }
            
            Ok(ParsedDocument::Mixed {
                markdown: markdown_content,
                aisp_documents,
                format: DocumentFormat::MarkdownWithAisp,
            })
        }
        
        fn parse_mixed_format(content: &str) -> AispResult<ParsedDocument> {
            // Implementation for complex mixed formats
            todo!("Implement mixed format parsing")
        }
    }
    
    #[derive(Debug)]
    pub enum ParsedDocument {
        Pure(AispDocument),
        Mixed {
            markdown: String,
            aisp_documents: Vec<AispDocument>,
            format: DocumentFormat,
        },
    }
}
```

### Phase 3: AISP Code Block Extractor

```rust
// AISP code block extraction from Markdown
pub mod aisp_extractor {
    use crate::error::AispResult;
    use regex::Regex;
    
    pub struct AispCodeBlockExtractor {
        aisp_block_regex: Regex,
    }
    
    impl AispCodeBlockExtractor {
        pub fn new() -> Self {
            Self {
                aisp_block_regex: Regex::new(
                    r"```aisp\s*\n(.*?)\n```"
                ).expect("Valid regex"),
            }
        }
        
        pub fn extract_aisp_blocks(&self, content: &str) -> AispResult<Vec<String>> {
            let mut blocks = Vec::new();
            
            for captures in self.aisp_block_regex.captures_iter(content) {
                if let Some(block_content) = captures.get(1) {
                    blocks.push(block_content.as_str().to_string());
                }
            }
            
            Ok(blocks)
        }
        
        pub fn extract_markdown_content(&self, content: &str) -> AispResult<String> {
            // Remove AISP code blocks, keep markdown
            Ok(self.aisp_block_regex.replace_all(content, "").to_string())
        }
    }
}
```

### Phase 4: Updated Parser Module Structure

```rust
// Updated parser/mod.rs
pub mod robust_parser;      // Existing - pure AISP
pub mod unicode_support;    // Existing - Unicode handling
pub mod content;           // Existing - SRP content parsers
pub mod format_detection;  // New - format identification
pub mod multi_format;      // New - multi-format orchestration
pub mod aisp_extractor;    // New - code block extraction

// Public API - backward compatible
pub use robust_parser::{AispParser, ParseResult, ParseError};
pub use multi_format::{MultiFormatParser, ParsedDocument};

// Main entry point with auto-detection
pub fn parse(source: &str) -> AispResult<ParsedDocument> {
    MultiFormatParser::parse(source)
}

// Legacy compatibility for pure AISP
pub fn parse_aisp_only(source: &str) -> AispResult<AispDocument> {
    let mut parser = AispParser::new(source.to_string());
    parser.parse()
}
```

## Benefits

### 1. **SRP Compliance Maintained**
- Existing content parsers unchanged
- Each parser has single responsibility
- Clean separation of concerns

### 2. **Extensible Architecture**
- Easy to add new format support
- Clear extension points
- Modular design

### 3. **Backward Compatibility**
- Pure AISP parsing unchanged
- Existing APIs preserved
- Zero breaking changes

### 4. **Reference.md Support**
- Handles Markdown + embedded AISP
- Extracts and validates AISP blocks
- Preserves prose context

### 5. **Future-Proof Design**
- Ready for JSON/XML/YAML formats
- Pluggable format detection
- Scalable orchestration

## Implementation Plan

### Sprint 1: Format Detection
- [ ] Implement `FormatDetector`
- [ ] Add comprehensive format tests
- [ ] Validate detection accuracy

### Sprint 2: Code Block Extraction  
- [ ] Implement `AispCodeBlockExtractor`
- [ ] Handle edge cases (nested blocks, malformed syntax)
- [ ] Add extraction tests

### Sprint 3: Multi-Format Orchestration
- [ ] Implement `MultiFormatParser`
- [ ] Integrate with existing parsers
- [ ] Add integration tests

### Sprint 4: Reference.md Validation
- [ ] Test against real `reference.md`
- [ ] Validate all AISP blocks parse correctly
- [ ] Performance optimization

## Testing Strategy

### 1. **Format Detection Tests**
```rust
#[cfg(test)]
mod format_detection_tests {
    #[test]
    fn test_pure_aisp_detection() {
        let content = "𝔸5.1.test@2026-01-30\n⟦Ω:Meta⟧{...}";
        assert_eq!(FormatDetector::detect(content), DocumentFormat::PureAisp);
    }
    
    #[test]
    fn test_markdown_with_aisp_detection() {
        let content = "# Header\n```aisp\n𝔸5.1.test@2026-01-30\n```";
        assert_eq!(FormatDetector::detect(content), DocumentFormat::MarkdownWithAisp);
    }
}
```

### 2. **Reference.md Integration Test**
```rust
#[test]
fn test_reference_md_parsing() {
    let reference_content = std::fs::read_to_string("docs/examples/reference.md")?;
    let parsed = MultiFormatParser::parse(&reference_content)?;
    
    match parsed {
        ParsedDocument::Mixed { aisp_documents, .. } => {
            assert!(!aisp_documents.is_empty());
            for doc in aisp_documents {
                assert!(doc.header.name.len() > 0);
            }
        }
        _ => panic!("Expected mixed format document"),
    }
}
```

## Risks and Mitigations

### Risk 1: Performance Overhead
- **Mitigation**: Format detection is O(1) scan, minimal overhead
- **Monitoring**: Add performance benchmarks

### Risk 2: Regex Complexity
- **Mitigation**: Use simple, well-tested regex patterns
- **Alternative**: State machine parser for complex cases

### Risk 3: Markdown Parser Dependencies
- **Mitigation**: Minimal markdown parsing, focus on AISP extraction
- **Scope**: Only handle code block boundaries, not full markdown

## Success Criteria

1. **✅ Reference.md Parsing**: Successfully parse and validate all AISP blocks
2. **✅ Backward Compatibility**: Existing pure AISP parsing unchanged  
3. **✅ Performance**: <10ms additional overhead for format detection
4. **✅ Test Coverage**: >95% coverage for new modules
5. **✅ SRP Compliance**: No responsibility violations in design

## Related ADRs

- **ADR-022**: Pest Parser Migration for Robustness (extended by this ADR)
- **ADR-015**: Security-First Parser Design (security principles maintained)

---

*This ADR extends our proven SRP parser architecture to handle mixed-format documents while preserving architectural integrity and backward compatibility.*