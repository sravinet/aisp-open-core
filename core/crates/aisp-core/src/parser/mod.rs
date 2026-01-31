// Parser module for security-hardened AISP parsing
// Implements ADR-022: Pest Parser Migration for Robustness  
// Extended by ADR-023: Multi-Format Parser Architecture
// Consolidated SRP-compliant parser architecture with multi-format support

pub mod robust_parser;
pub mod unicode_support;
pub mod content;
pub mod format_detection;
pub mod aisp_extractor;
pub mod multi_format;

// Main parser exports (single source of truth)
pub use robust_parser::{
    RobustAispParser, 
    ParseResult, 
    ParseError, 
    SecurityIssue,
    SecuritySeverity,
    AispParser,
};

// Unicode support
pub use unicode_support::{
    UnicodeSymbolRegistry,
    MathematicalSymbol,
    SecurityReport,
    SecurityLevel,
};

// SRP content parsers (for internal use by robust_parser)
pub use content::{
    MetaContentParser,
    TypeContentParser,
    LogicContentParser,
    LambdaContentParser,
    EvidenceContentParser,
};

// Multi-format parser architecture (ADR-023)
pub use format_detection::{
    DocumentFormat,
    FormatDetector,
    FormatAnalysis,
    FormatComplexity,
};

pub use aisp_extractor::{
    AispCodeBlockExtractor,
    ExtractedAispBlock,
    ExtractionContext,
};

pub use multi_format::{
    MultiFormatParser,
    ParsedDocument,
    ParsedAispDocument,
    MixedFormatMetadata,
    ParsingMetrics,
};

/// Primary parse function with automatic format detection (ADR-023)
/// 
/// This function automatically detects document format and routes to the
/// appropriate parser. Supports pure AISP documents and mixed-format
/// documents like reference.md.
pub fn parse(source: &str) -> crate::error::AispResult<ParsedDocument> {
    let parser = MultiFormatParser::new();
    parser.parse(source)
}

/// Legacy compatibility function for pure AISP parsing only
/// 
/// This function maintains backward compatibility for existing code
/// that expects only pure AISP document parsing.
pub fn parse_aisp_only(source: &str) -> crate::error::AispResult<crate::ast::canonical::AispDocument> {
    let parser = RobustAispParser::new();
    let parse_result = parser.parse(source);
    parse_result.document.ok_or_else(|| {
        crate::error::AispError::ParseError {
            message: "Failed to parse AISP document".to_string(),
            line: 0,
            column: 0,
        }
    })
}

/// Convenience function to detect document format without parsing
pub fn detect_format(source: &str) -> DocumentFormat {
    FormatDetector::detect(source)
}

/// Convenience function for detailed format analysis
pub fn analyze_format(source: &str) -> FormatAnalysis {
    FormatDetector::analyze(source)
}