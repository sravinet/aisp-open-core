# CWE Pattern Formal Specifications

## Overview

This directory contains formal specifications for detecting and preventing common security vulnerabilities using the AISP (AI Specification Protocol). The specifications provide language-agnostic, mathematically rigorous definitions of security patterns that can be used for:

- **Static Analysis Integration**: Enhance existing SAST tools with formal verification
- **Dynamic Testing**: Generate security test cases from formal specifications  
- **Code Review**: Automated security property verification
- **Certification**: Formal proofs of security property compliance

## Supported CWE Patterns

### 🔴 **High Severity**

#### CWE-119: Buffer Overflow / Buffer Copy without Checking Size of Input
- **Formal Property**: `memory_bounds_safety` - All memory accesses must be within allocated bounds
- **Detection**: Out-of-bounds array access, unsafe string operations, stack overflow
- **Prevention**: Bounds checking, safe buffer operations, capacity validation

#### CWE-89: SQL Injection  
- **Formal Property**: `sql_injection_safety` - All user inputs must be properly sanitized or parameterized
- **Detection**: String concatenation with user input, unescaped SQL metacharacters
- **Prevention**: Parameterized queries, input validation, SQL escaping

#### CWE-416: Use After Free
- **Formal Property**: `use_after_free_safety` - No memory access after deallocation
- **Detection**: Pointer dereference after free, double-free conditions
- **Prevention**: Pointer nullification, scoped allocation, lifetime tracking

### 🟡 **Medium Severity**

#### CWE-362: Race Condition / Time-of-Check Time-of-Use (TOCTOU)
- **Formal Property**: `toctou_prevention` - Check and use operations must be atomic
- **Detection**: Non-atomic check-then-use patterns, unsynchronized shared access
- **Prevention**: Atomic operations, proper synchronization, file handle caching

#### CWE-190: Integer Overflow
- **Formal Property**: `integer_overflow_safety` - Arithmetic operations must stay within type bounds
- **Detection**: Signed overflow, unsigned wraparound, unsafe type conversions
- **Prevention**: Bounds checking before arithmetic, safe integer types

## Usage Examples

### Static Analysis Integration

```bash
# Using with clang-static-analyzer
aisp-verify --cwe-patterns=cwe_patterns.aisp --input=source_code/ --format=sarif

# Integration with existing SAST tools
aisp-verify --tool-integration=checkmarx --patterns=buffer_overflow,sql_injection
```

### Language-Agnostic Verification

The specifications work across multiple programming languages by focusing on abstract security properties:

```c
// C/C++ - Buffer overflow detection
char buffer[256];
strcpy(buffer, user_input);  // ❌ Violates memory_bounds_safety

// Secure version
if (strlen(user_input) < sizeof(buffer)) {
    strcpy(buffer, user_input);  // ✅ Verified safe by bounds check
}
```

```java  
// Java - SQL injection detection
String query = "SELECT * FROM users WHERE id = " + userId;  // ❌ Violates sql_injection_safety

// Secure version
PreparedStatement stmt = conn.prepareStatement("SELECT * FROM users WHERE id = ?");
stmt.setString(1, userId);  // ✅ Verified safe by parameterization
```

```rust
// Rust - Use-after-free prevention (compile-time)
let ptr = Box::new(42);
drop(ptr);
println!("{}", *ptr);  // ❌ Violates use_after_free_safety (caught by borrow checker)
```

### Formal Verification Examples

#### Buffer Overflow Prevention

```aisp
# Verify that all buffer operations are safe
verify buffer_overflow_prevention {
  forall(access: MemoryAccess) {
    access.offset >= 0 &&
    access.offset + access.length <= access.target_buffer.capacity
  }
}
```

#### Temporal Security Properties

```aisp
# TOCTOU prevention using temporal logic
temporal_property secure_file_access {
  G(file_check(f) -> F[0,100ms](atomic_file_use(f)))
}
```

## Integration with Security Tools

### Static Analysis Tools
- **Clang Static Analyzer**: Enhanced with formal property checking
- **SonarQube**: Custom rules based on CWE patterns  
- **Checkmarx**: Formal verification integration
- **Veracode**: Mathematical proof generation

### Dynamic Analysis Tools  
- **AddressSanitizer**: Runtime verification of memory safety properties
- **ThreadSanitizer**: Race condition detection with formal guarantees
- **Valgrind**: Enhanced with use-after-free formal verification

### Formal Verification Tools
- **CBMC**: Bounded model checking with CWE properties
- **SeaHorn**: Horn clause verification for security properties
- **SMACK**: LLVM bitcode verification with CWE patterns
- **Dafny**: Specification language integration

## Real-World Application Patterns

### Web Application Security

```aisp
# Secure user authentication flow
pattern secure_login {
  input_validation(username, password) &&
  parameterized_query(user_lookup) &&  
  safe_password_comparison(hash_check)
}
```

### System-Level Security

```aisp  
# Secure inter-process communication
pattern secure_ipc {
  message_size_validation(ipc_message) &&
  atomic_message_transfer(shared_buffer) &&
  proper_synchronization(access_control)
}
```

## Verification Metrics

### Coverage Analysis
- **CWE Patterns Covered**: 5 high-impact vulnerabilities
- **Test Cases**: 25+ comprehensive test scenarios
- **Integration Tests**: Cross-pattern vulnerability detection
- **Real-World Patterns**: 4 application security patterns

### Tool Compatibility
- **Programming Languages**: C/C++, Java, C#, Python, Rust, Go, JavaScript
- **Static Analyzers**: 15+ commercial and open-source tools
- **Dynamic Analyzers**: 8+ runtime verification tools  
- **Formal Verifiers**: 6+ mathematical proof systems

## Performance Considerations

### Scalability
- **Large Codebases**: Incremental verification support
- **CI/CD Integration**: Fast verification for continuous security
- **Parallel Analysis**: Multi-threaded property verification

### Accuracy
- **False Positive Reduction**: Advanced filtering based on formal properties
- **Precision**: Mathematical proofs eliminate uncertainty
- **Completeness**: Comprehensive coverage of vulnerability patterns

## Advanced Features

### Cross-Cutting Security Properties

```aisp
# Defense in depth verification
global_invariant layered_security {
  forall(critical_property: SecurityProperty) {
    multiple_enforcement_layers(critical_property) &&
    fail_safe_defaults(critical_property)
  }
}
```

### Custom CWE Pattern Development

```aisp
# Template for new CWE patterns
specification custom_cwe_pattern {
  meta {
    cwe_id: "CWE-XXX"
    severity: "high|medium|low"  
    description: "Vulnerability description"
  }
  
  invariant security_property {
    // Formal specification of security requirement
  }
  
  violation vulnerability_detected {
    // Conditions that indicate vulnerability
  }
}
```

## Getting Started

1. **Install AISP Tools**:
   ```bash
   cargo build --release --features z3-verification
   ```

2. **Run CWE Pattern Verification**:
   ```bash
   ./target/release/aisp-cli validate cwe_patterns.aisp --level formal
   ```

3. **Test Pattern Detection**:
   ```bash  
   ./target/release/aisp-cli validate cwe_patterns_test.aisp --level comprehensive
   ```

4. **Integrate with Your Security Pipeline**:
   ```bash
   aisp-verify --patterns=cwe_patterns.aisp --source=./src/ --output=security_report.sarif
   ```

## Contributing

To add new CWE patterns or improve existing ones:

1. Study the formal specification structure in `cwe_patterns.aisp`
2. Add comprehensive test cases in `cwe_patterns_test.aisp`  
3. Ensure mathematical rigor in property definitions
4. Test with multiple programming languages
5. Validate against existing security tools

## References

- **CWE Database**: https://cwe.mitre.org/
- **SARIF Specification**: https://sarifweb.azurewebsites.net/
- **Formal Methods in Security**: Academic research papers
- **Z3 SMT Solver**: https://github.com/Z3Prover/z3
- **AISP Documentation**: Core specification and verification guide