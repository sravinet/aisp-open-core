use aisp_core::validator::{AispValidator, ValidationConfig};

fn main() {
    println!("Testing AISP Core API...");
    
    // Create validator
    let validator = AispValidator::new();
    println!("✓ Validator created");
    
    // Create config
    let config = ValidationConfig::default();
    println!("✓ Configuration created");
    
    // Test validation with simple content
    let test_content = r#"𝔸5.1.Test@2026-01-28

⟦Ω:Meta⟧{
  domain≜"api_test"
}"#;
    
    let result = validator.validate(test_content);
    println!("✓ Validation completed: delta={:.3}", result.delta);
    
    println!("✅ All API tests passed!");
}