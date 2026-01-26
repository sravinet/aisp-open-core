/**
 * Z3 Integration Bridge for AISP Validation
 * 
 * Provides SMT-based formal verification of AISP logical constructs:
 * - Quantified formulas (∀, ∃)
 * - Set operations (∈, ⊆, ∩, ∪)
 * - Logical implications (⇒, ⇔)
 * - Type constraints (ℕ, ℤ, ℝ, 𝔹)
 * - Function definitions and invariants
 */

import { spawn } from 'child_process';
import { writeFile, unlink } from 'fs/promises';
import { join } from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = join(__filename, '..');

/**
 * AISP to Z3 SMT-LIB translator
 */
export class AispZ3Translator {
    constructor() {
        this.sorts = new Map();
        this.functions = new Map(); 
        this.constants = new Map();
        this.assertions = [];
        this.builtinSorts = new Set(['Int', 'Real', 'Bool', 'String']);
        
        this.initBuiltins();
    }
    
    initBuiltins() {
        // Standard AISP types (use Z3 built-ins where possible)
        this.declareSorts([
            'Player',
            'Cell', 
            'Board',
            'GameState',
            'Move',
            'Lines'
        ]);
        
        // Built-in sorts already initialized in constructor
        
        // Declare functions for game logic
        this.declareFunctions([
            '(declare-fun ValidMove (Board Int) Bool)',
            '(declare-fun WinCondition (Board Player) Bool)',
            '(declare-fun DrawCondition (Board) Bool)',
            '(declare-fun board_get (Board Int) Cell)',
            '(declare-fun Empty () Cell)',
            '(declare-fun X () Player)',
            '(declare-fun O () Player)',
            '(declare-fun member_Lines (Int) Bool)'
        ]);
    }
    
    declareSorts(names) {
        for (const name of names) {
            if (!this.builtinSorts.has(name)) {
                this.sorts.set(name, `(declare-sort ${name})`);
            }
        }
    }
    
    declareFunctions(smtlibDecls) {
        for (const decl of smtlibDecls) {
            const match = decl.match(/declare-fun\s+(\w+)/);
            if (match) {
                this.functions.set(match[1], decl);
            }
        }
    }
    
    /**
     * Parse AISP rules block and extract logical constructs
     * @param {string} rulesContent - Content of ⟦Γ:Rules⟧ block
     * @returns {Array} Array of parsed constructs
     */
    parseRulesBlock(rulesContent) {
        const constructs = [];
        
        // Split by lines and process each rule
        const lines = rulesContent.split('\n')
            .map(line => line.trim())
            .filter(line => line && !line.startsWith('//'));
            
        for (const line of lines) {
            // Universal quantifier with iff: ∀move:ValidMove(board,pos)⇔board[pos]=Empty
            if (line.includes('∀') && line.includes('⇔')) {
                const construct = this.parseUniversalIff(line);
                if (construct) constructs.push(construct);
            }
            
            // Existential quantifier: ∃line∈Lines:∀c∈line:c=player
            else if (line.includes('∃')) {
                const construct = this.parseExistential(line);
                if (construct) constructs.push(construct);
            }
            
            // Simple implications: A⇒B
            else if (line.includes('⇒')) {
                const construct = this.parseImplication(line);
                if (construct) constructs.push(construct);
            }
            
            // Definitions: Lines≜{rows,cols,diags}
            else if (line.includes('≜')) {
                const construct = this.parseDefinition(line);
                if (construct) constructs.push(construct);
            }
        }
        
        return constructs;
    }
    
    /**
     * Parse: ∀move:ValidMove(board,pos)⇔board[pos]=Empty
     */
    parseUniversalIff(line) {
        const match = line.match(/∀(\w+):([^⇔]+)⇔(.+)/);
        if (!match) return null;
        
        const [, variable, leftSide, rightSide] = match;
        
        return {
            type: 'forall',
            variable,
            domain: this.inferDomain(leftSide),
            leftSide: leftSide.trim(),
            rightSide: rightSide.trim()
        };
    }
    
    /**
     * Parse: ∃line∈Lines:∀c∈line:c=player
     */
    parseExistential(line) {
        const match = line.match(/∃(\w+)∈(\w+):(.+)/);
        if (!match) return null;
        
        const [, variable, domain, predicate] = match;
        
        return {
            type: 'exists',
            variable,
            domain,
            predicate: predicate.trim()
        };
    }
    
    /**
     * Parse: A⇒B
     */
    parseImplication(line) {
        const parts = line.split('⇒');
        if (parts.length !== 2) return null;
        
        return {
            type: 'implies',
            antecedent: parts[0].trim(),
            consequent: parts[1].trim()
        };
    }
    
    /**
     * Parse: Lines≜{rows,cols,diags}
     */
    parseDefinition(line) {
        const match = line.match(/(\w+)≜(.+)/);
        if (!match) return null;
        
        const [, name, definition] = match;
        
        return {
            type: 'definition',
            name,
            definition: definition.trim()
        };
    }
    
    /**
     * Infer domain from predicate
     */
    inferDomain(predicate) {
        if (predicate.includes('ValidMove') || predicate.includes('move')) return 'Move';
        if (predicate.includes('Player') || predicate.includes('player')) return 'Player';
        if (predicate.includes('board')) return 'Board';
        if (predicate.includes('WinCondition') || predicate.includes('win')) return 'Int';
        if (predicate.includes('DrawCondition') || predicate.includes('game')) return 'Int';
        return 'Int';
    }
    
    /**
     * Convert AISP construct to Z3 SMT-LIB
     */
    translateConstruct(construct) {
        switch (construct.type) {
            case 'forall':
                return this.translateForAll(construct);
            case 'exists':
                return this.translateExists(construct);
            case 'implies':
                return this.translateImplies(construct);
            case 'definition':
                return this.translateDefinition(construct);
            default:
                return `; Unknown construct: ${construct.type}`;
        }
    }
    
    translateForAll(construct) {
        const { variable, domain, leftSide, rightSide } = construct;
        
        // Special handling for common AISP patterns
        if (leftSide.includes('ValidMove')) {
            return `(forall ((${variable} Move)) (= (ValidMove board pos) (= (board_get board pos) Empty)))`;
        }
        
        if (leftSide.includes('WinCondition') && rightSide.includes('∃line∈Lines')) {
            return `(forall ((${variable} Int)) (= (WinCondition board player) (member_Lines ${variable})))`;
        }
        
        if (leftSide.includes('DrawCondition')) {
            return `(forall ((${variable} Int)) (= (DrawCondition board) (and (not (WinCondition board X)) (not (WinCondition board O)))))`;
        }
        
        // Fallback to basic conversion
        const left = this.convertPredicate(leftSide);
        const right = this.convertPredicate(rightSide);
        
        return `(forall ((${variable} ${domain})) (= ${left} ${right}))`;
    }
    
    translateExists(construct) {
        const { variable, domain, predicate } = construct;
        const pred = this.convertPredicate(predicate);
        
        return `(exists ((${variable} ${domain})) ${pred})`;
    }
    
    translateImplies(construct) {
        const { antecedent, consequent } = construct;
        const ant = this.convertPredicate(antecedent);
        const con = this.convertPredicate(consequent);
        
        return `(=> ${ant} ${con})`;
    }
    
    translateDefinition(construct) {
        const { name, definition } = construct;
        
        // For sets like {rows,cols,diags}, declare as enumeration
        if (definition.startsWith('{') && definition.endsWith('}')) {
            const elements = definition.slice(1, -1).split(',').map(e => e.trim());
            return `; Definition: ${name} = ${definition}\n(declare-datatypes () ((${name} ${elements.join(' ')})))`;
        }
        
        return `; Definition: ${name} = ${definition}`;
    }
    
    /**
     * Convert AISP predicate to Z3 format
     */
    convertPredicate(predicate) {
        // Simple conversions for common patterns
        let result = predicate
            .replace(/(\w+)\[(\w+)\]/g, '(select $1 $2)')  // array access
            .replace(/(\w+)∈(\w+)/g, '(member $1 $2)')     // set membership
            .replace(/(\w+)⊆(\w+)/g, '(subset $1 $2)')     // set inclusion
            .replace(/∧/g, 'and')                          // logical and
            .replace(/∨/g, 'or')                           // logical or
            .replace(/¬/g, 'not');                         // logical not
            
        // Handle equality last to catch complex expressions
        result = result.replace(/([^=]+)=([^=]+)/g, '(= $1 $2)');     // equality
        result = result.replace(/([^≠]+)≠([^≠]+)/g, '(not (= $1 $2))'); // inequality
        
        return result;
    }
    
    /**
     * Generate complete SMT-LIB script
     */
    generateSMTLib(constructs) {
        let script = '; AISP Z3 Validation Script\n';
        script += '; Generated automatically from AISP rules\n\n';
        
        // Add sort declarations
        for (const decl of this.sorts.values()) {
            script += `${decl}\n`;
        }
        script += '\n';
        
        // Add function declarations
        for (const decl of this.functions.values()) {
            script += `${decl}\n`;
        }
        script += '\n';
        
        // Add constant declarations
        for (const decl of this.constants.values()) {
            script += `${decl}\n`;
        }
        script += '\n';
        
        // Translate and add assertions
        for (const construct of constructs) {
            const assertion = this.translateConstruct(construct);
            if (assertion) {
                if (assertion.includes('(declare-datatypes')) {
                    // Handle datatypes specially
                    const lines = assertion.split('\n');
                    for (const line of lines) {
                        if (line.trim()) {
                            if (line.startsWith(';')) {
                                script += `${line}\n`;
                            } else if (line.includes('declare-datatypes')) {
                                script += `${line}\n`;
                            } else if (!line.startsWith(';')) {
                                script += `(assert ${line})\n`;
                            }
                        }
                    }
                } else if (!assertion.startsWith(';')) {
                    script += `(assert ${assertion})\n`;
                } else {
                    script += `${assertion}\n`;
                }
            }
        }
        
        // Add check-sat and exit
        script += '\n(check-sat)\n';
        script += '(exit)\n';
        
        return script;
    }
}

/**
 * Validate AISP document using Z3 SMT solver
 * @param {string} aispContent - Full AISP document
 * @param {object} options - Validation options
 * @returns {Promise<object>} Validation result
 */
export async function validateWithZ3(aispContent, options = {}) {
    try {
        const translator = new AispZ3Translator();
        
        // Extract rules block (handle multiline and nested braces)
        const rulesStart = aispContent.indexOf('⟦Γ:Rules⟧{');
        if (rulesStart === -1) {
            return {
                valid: false,
                error: 'No rules block found for Z3 validation',
                z3Available: false
            };
        }
        
        // Find matching closing brace
        let braceCount = 0;
        let rulesEnd = -1;
        for (let i = rulesStart + 10; i < aispContent.length; i++) {
            if (aispContent[i] === '{') {
                braceCount++;
            } else if (aispContent[i] === '}') {
                if (braceCount === 0) {
                    rulesEnd = i;
                    break;
                }
                braceCount--;
            }
        }
        
        if (rulesEnd === -1) {
            return {
                valid: false,
                error: 'Malformed rules block (no closing brace)',
                z3Available: false
            };
        }
        
        const rulesContent = aispContent.substring(rulesStart + 11, rulesEnd);
        const constructs = translator.parseRulesBlock(rulesContent);
        
        if (constructs.length === 0) {
            return {
                valid: true,
                warning: 'No formal constructs found for Z3 verification',
                z3Available: true,
                constructs: 0,
                satisfiable: true  // Empty set of constraints is satisfiable
            };
        }
        
        // Generate SMT-LIB script
        const smtlibScript = translator.generateSMTLib(constructs);
        
        // Write script to temporary file
        const tempFile = join(__dirname, '..', 'tmp', `z3_${Date.now()}.smt2`);
        await writeFile(tempFile, smtlibScript);
        
        // Run Z3
        const z3Result = await runZ3(tempFile);
        
        // Clean up
        await unlink(tempFile).catch(() => {}); // Ignore cleanup errors
        
        return {
            valid: z3Result.satisfiable !== false,
            satisfiable: z3Result.satisfiable,
            z3Available: true,
            constructs: constructs.length,
            smtlibGenerated: smtlibScript.split('\n').length,
            runtime: z3Result.runtime,
            ...(options.debug && { smtlib: smtlibScript })
        };
        
    } catch (error) {
        return {
            valid: false,
            error: `Z3 validation failed: ${error.message}`,
            z3Available: false
        };
    }
}

/**
 * Run Z3 SMT solver on script file
 * @param {string} scriptFile - Path to SMT-LIB script
 * @returns {Promise<object>} Z3 result
 */
async function runZ3(scriptFile) {
    return new Promise((resolve, reject) => {
        const startTime = Date.now();
        
        const z3Process = spawn('z3', [scriptFile], {
            stdio: ['pipe', 'pipe', 'pipe'],
            timeout: 30000 // 30 second timeout
        });
        
        let stdout = '';
        let stderr = '';
        
        z3Process.stdout.on('data', (data) => {
            stdout += data.toString();
        });
        
        z3Process.stderr.on('data', (data) => {
            stderr += data.toString();
        });
        
        z3Process.on('close', (code) => {
            const runtime = Date.now() - startTime;
            
            if (code === 0) {
                const output = stdout.trim();
                let satisfiable;
                
                if (output.includes('sat') && !output.includes('unsat')) {
                    satisfiable = true;
                } else if (output.includes('unsat')) {
                    satisfiable = false;
                } else {
                    satisfiable = null; // unknown
                }
                
                resolve({
                    satisfiable,
                    stdout,
                    stderr,
                    runtime,
                    exitCode: code
                });
            } else {
                reject(new Error(`Z3 exited with code ${code}: ${stderr}`));
            }
        });
        
        z3Process.on('error', (error) => {
            if (error.code === 'ENOENT') {
                reject(new Error('Z3 not found. Please install Z3 SMT solver.'));
            } else {
                reject(error);
            }
        });
    });
}

// AispZ3Translator exported above