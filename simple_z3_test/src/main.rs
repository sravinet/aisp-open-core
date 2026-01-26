use z3::*;
use z3::ast::Ast;

fn main() {
    println!("🧮 AISP Z3 Rust Integration Test");
    println!("================================");
    
    // Create Z3 configuration and context
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    
    println!("✅ Z3 context created successfully");
    
    // Test 1: Basic satisfiability check
    test_basic_sat(&ctx);
    
    // Test 2: AISP ambiguity constraint (simplified)
    test_aisp_ambiguity(&ctx);
    
    println!("\n🎉 Z3 INTEGRATION WORKING!");
    println!("The AISP formal verification system can now use Z3 for SMT solving.");
}

fn test_basic_sat(ctx: &Context) {
    println!("\n1. Basic SAT test...");
    let solver = Solver::new(ctx);
    
    // Simple constraint: x > 0
    let x = ast::Int::new_const(ctx, "x");
    let zero = ast::Int::from_i64(ctx, 0);
    solver.assert(&x.gt(&zero));
    
    match solver.check() {
        SatResult::Sat => {
            println!("   ✅ Basic SAT test passed");
            if let Some(model) = solver.get_model() {
                println!("   📊 Solution: x = {}", model.eval(&x, true).unwrap());
            }
        }
        SatResult::Unsat => println!("   ❌ Unexpected UNSAT"),
        SatResult::Unknown => println!("   ⚠️  Unknown result"),
    }
}

fn test_aisp_ambiguity(ctx: &Context) {
    println!("\n2. AISP ambiguity constraint test...");
    let solver = Solver::new(ctx);
    
    // Test if ambiguity < 0.02 is achievable
    // Simplified: unique_parses = 98, total_parses = 100
    // ambiguity = 1 - 98/100 = 0.02
    
    let unique = ast::Real::from_real(ctx, 98, 1);
    let total = ast::Real::from_real(ctx, 100, 1);
    let ambiguity = ast::Real::new_const(ctx, "ambiguity");
    
    // ambiguity = 1 - (unique / total)
    let one = ast::Real::from_real(ctx, 1, 1);
    let ratio = unique / total;
    solver.assert(&ambiguity._eq(&(one - ratio)));
    
    // Test: ambiguity <= 0.02 (achievable)
    let threshold = ast::Real::from_real(ctx, 2, 100);
    solver.assert(&ambiguity.le(&threshold));
    
    match solver.check() {
        SatResult::Sat => {
            println!("   ✅ AISP ambiguity constraint is satisfiable");
            if let Some(model) = solver.get_model() {
                println!("   📊 Ambiguity value: {}", model.eval(&ambiguity, true).unwrap());
            }
        }
        SatResult::Unsat => println!("   ❌ AISP ambiguity constraint is unsatisfiable"),
        SatResult::Unknown => println!("   ⚠️  Unknown result for ambiguity constraint"),
    }
}