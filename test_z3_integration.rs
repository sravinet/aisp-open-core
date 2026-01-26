#!/usr/bin/env rust-script
//! Z3 Integration Test for macOS Homebrew Setup
//! 
//! This script verifies that Z3 is properly configured and accessible
//! from Rust on macOS using Homebrew installation.

use std::process::Command;

fn main() {
    println!("🧪 AISP Z3 Integration Test");
    println!("============================");
    
    // Test 1: Check Z3 binary availability
    println!("\n1. Testing Z3 binary...");
    match Command::new("z3").arg("--version").output() {
        Ok(output) => {
            println!("   ✅ Z3 binary found:");
            println!("   {}", String::from_utf8_lossy(&output.stdout).trim());
        }
        Err(e) => {
            println!("   ❌ Z3 binary not found: {}", e);
        }
    }
    
    // Test 2: Check pkg-config
    println!("\n2. Testing pkg-config for Z3...");
    match Command::new("pkg-config").args(&["--exists", "z3"]).output() {
        Ok(output) => {
            if output.status.success() {
                println!("   ✅ pkg-config finds Z3");
                
                // Get compile flags
                if let Ok(cflags) = Command::new("pkg-config").args(&["--cflags", "z3"]).output() {
                    println!("   CFLAGS: {}", String::from_utf8_lossy(&cflags.stdout).trim());
                }
                
                // Get library flags
                if let Ok(libs) = Command::new("pkg-config").args(&["--libs", "z3"]).output() {
                    println!("   LIBS: {}", String::from_utf8_lossy(&libs.stdout).trim());
                }
            } else {
                println!("   ❌ pkg-config doesn't find Z3");
            }
        }
        Err(e) => {
            println!("   ❌ pkg-config error: {}", e);
        }
    }
    
    // Test 3: Check header files
    println!("\n3. Testing Z3 header files...");
    let header_paths = [
        "/opt/homebrew/include/z3.h",
        "/opt/homebrew/Cellar/z3/4.15.4/include/z3.h",
    ];
    
    for path in &header_paths {
        if std::path::Path::new(path).exists() {
            println!("   ✅ Found header: {}", path);
        } else {
            println!("   ⚠️  Missing header: {}", path);
        }
    }
    
    // Test 4: Check library files
    println!("\n4. Testing Z3 library files...");
    let lib_paths = [
        "/opt/homebrew/lib/libz3.dylib",
        "/opt/homebrew/Cellar/z3/4.15.4/lib/libz3.dylib",
    ];
    
    for path in &lib_paths {
        if std::path::Path::new(path).exists() {
            println!("   ✅ Found library: {}", path);
        } else {
            println!("   ⚠️  Missing library: {}", path);
        }
    }
    
    // Test 5: Check clang/LLVM
    println!("\n5. Testing clang/LLVM for bindgen...");
    let clang_lib_paths = [
        "/opt/homebrew/opt/llvm/lib/libclang.dylib",
        "/opt/homebrew/Cellar/llvm/21.1.8/lib/libclang.dylib",
    ];
    
    for path in &clang_lib_paths {
        if std::path::Path::new(path).exists() {
            println!("   ✅ Found libclang: {}", path);
        } else {
            println!("   ⚠️  Missing libclang: {}", path);
        }
    }
    
    // Test 6: Environment variables
    println!("\n6. Checking environment variables...");
    let env_vars = [
        "LIBCLANG_PATH",
        "Z3_SYS_Z3_HEADER", 
        "C_INCLUDE_PATH",
        "LIBRARY_PATH",
        "PKG_CONFIG_PATH",
    ];
    
    for var in &env_vars {
        match std::env::var(var) {
            Ok(value) => println!("   ✅ {}: {}", var, value),
            Err(_) => println!("   ⚠️  {} not set", var),
        }
    }
    
    // Test 7: Simple SMT query via command line
    println!("\n7. Testing Z3 SMT solving...");
    let smt_query = "(assert (> (+ 1 2) 2))\n(check-sat)\n";
    
    match Command::new("z3")
        .arg("-in")
        .arg("-smt2")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
    {
        Ok(mut child) => {
            if let Some(stdin) = child.stdin.as_mut() {
                use std::io::Write;
                let _ = stdin.write_all(smt_query.as_bytes());
            }
            
            match child.wait_with_output() {
                Ok(output) => {
                    let result_string = String::from_utf8_lossy(&output.stdout);
                    let result = result_string.trim();
                    if result.contains("sat") {
                        println!("   ✅ Z3 SMT solving works: {}", result);
                    } else {
                        println!("   ⚠️  Z3 SMT unexpected result: {}", result);
                    }
                }
                Err(e) => {
                    println!("   ❌ Z3 SMT error: {}", e);
                }
            }
        }
        Err(e) => {
            println!("   ❌ Failed to spawn Z3: {}", e);
        }
    }
    
    println!("\n🏁 RECOMMENDED ENVIRONMENT SETUP");
    println!("=================================");
    println!("Add these to your shell profile (~/.zshrc, ~/.bashrc):");
    println!();
    println!("export LIBCLANG_PATH=\"/opt/homebrew/opt/llvm/lib\"");
    println!("export Z3_SYS_Z3_HEADER=\"/opt/homebrew/include/z3.h\"");
    println!("export C_INCLUDE_PATH=\"/opt/homebrew/include\"");
    println!("export LIBRARY_PATH=\"/opt/homebrew/lib\"");
    println!("export PKG_CONFIG_PATH=\"/opt/homebrew/lib/pkgconfig:$PKG_CONFIG_PATH\"");
    println!();
    println!("Then reload your shell and try:");
    println!("cd aisp-formal-verification && cargo check --features z3-verification");
}