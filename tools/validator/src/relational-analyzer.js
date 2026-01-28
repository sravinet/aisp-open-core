/**
 * Level 4 Relational Logic Analyzer for AISP
 * 
 * Analyzes cross-rule dependencies, logical consistency chains,
 * invariant detection, and completeness analysis.
 */

import { AispZ3Translator } from './z3-bridge.js';

/**
 * Represents a logical dependency between AISP constructs
 */
export class LogicalDependency {
    constructor(source, target, type, strength = 1.0) {
        this.source = source;      // Source rule/definition
        this.target = target;      // Target rule/definition  
        this.type = type;          // 'uses', 'defines', 'contradicts', 'implies'
        this.strength = strength;  // Dependency strength [0,1]
    }
}

/**
 * Represents an AISP rule with metadata
 */
export class AispRule {
    constructor(id, content, type, variables = [], predicates = []) {
        this.id = id;
        this.content = content;
        this.type = type;           // 'definition', 'quantified', 'implication'
        this.variables = variables;  // Variables used in rule
        this.predicates = predicates; // Predicates referenced
        this.dependencies = [];     // Other rules this depends on
        this.dependents = [];       // Rules that depend on this
    }
    
    addDependency(dependency) {
        this.dependencies.push(dependency);
    }
    
    addDependent(dependent) {
        this.dependents.push(dependent);
    }
}

/**
 * Level 4 Relational Logic Analyzer
 */
export class RelationalAnalyzer {
    constructor() {
        this.rules = [];
        this.dependencies = [];
        this.definitions = new Map();
        this.predicates = new Map();
        this.variables = new Set();
    }
    
    /**
     * Analyze AISP document for relational logic patterns
     */
    async analyze(aispContent, options = {}) {
        const startTime = Date.now();
        
        // Extract and parse all blocks
        const blocks = this.extractBlocks(aispContent);
        
        // Build rule database
        this.buildRuleDatabase(blocks);
        
        // Perform relational analysis
        const dependencyGraph = this.buildDependencyGraph();
        const consistencyChains = this.findConsistencyChains();
        const invariants = this.detectInvariants();
        const completeness = this.analyzeCompleteness();
        const complexity = this.analyzeComplexity();
        
        const runtime = Date.now() - startTime;
        
        return {
            level: 4,
            name: 'Relational Logic Analysis',
            runtime,
            rules: this.rules.length,
            dependencies: this.dependencies.length,
            dependencyGraph,
            consistencyChains,
            invariants,
            completeness,
            complexity,
            metrics: this.calculateMetrics()
        };
    }
    
    /**
     * Extract all logical blocks from AISP document
     */
    extractBlocks(content) {
        const blocks = {
            types: this.extractBlock(content, 'Σ:Types'),
            rules: this.extractBlock(content, 'Γ:Rules'),
            functions: this.extractBlock(content, 'Λ:Funcs'),
            meta: this.extractBlock(content, 'Ω:Meta')
        };
        
        return blocks;
    }
    
    /**
     * Extract specific block content
     */
    extractBlock(content, blockName) {
        const blockStart = content.indexOf(`⟦${blockName}⟧{`);
        if (blockStart === -1) return null;
        
        let braceCount = 0;
        let blockEnd = -1;
        
        for (let i = blockStart + blockName.length + 3; i < content.length; i++) {
            if (content[i] === '{') braceCount++;
            else if (content[i] === '}') {
                if (braceCount === 0) {
                    blockEnd = i;
                    break;
                }
                braceCount--;
            }
        }
        
        if (blockEnd === -1) return null;
        
        return content.substring(blockStart + blockName.length + 3, blockEnd).trim();
    }
    
    /**
     * Build comprehensive rule database
     */
    buildRuleDatabase(blocks) {
        let ruleId = 1;
        
        // Process type definitions
        if (blocks.types) {
            this.parseTypeDefinitions(blocks.types, ruleId);
            ruleId += this.countLines(blocks.types);
        }
        
        // Process rules
        if (blocks.rules) {
            this.parseRules(blocks.rules, ruleId);
            ruleId += this.countLines(blocks.rules);
        }
        
        // Process function definitions
        if (blocks.functions) {
            this.parseFunctionDefinitions(blocks.functions, ruleId);
        }
    }
    
    /**
     * Parse type definitions and extract dependencies
     */
    parseTypeDefinitions(typeContent, startId) {
        const lines = typeContent.split('\n')
            .map(line => line.trim())
            .filter(line => line && !line.startsWith('//'));
            
        lines.forEach((line, index) => {
            if (line.includes('≜')) {
                const [name, definition] = line.split('≜').map(s => s.trim());
                
                const rule = new AispRule(
                    `type_${startId + index}`,
                    line,
                    'definition',
                    [name],
                    this.extractPredicates(definition)
                );
                
                this.rules.push(rule);
                this.definitions.set(name, rule);
            }
        });
    }
    
    /**
     * Parse logical rules and extract relationships
     */
    parseRules(rulesContent, startId) {
        const translator = new AispZ3Translator();
        const constructs = translator.parseRulesBlock(rulesContent);
        
        constructs.forEach((construct, index) => {
            const rule = new AispRule(
                `rule_${startId + index}`,
                this.constructToString(construct),
                construct.type
            );
            
            // Extract variables and predicates
            if (construct.variable) {
                rule.variables.push(construct.variable);
                this.variables.add(construct.variable);
            }
            
            // Extract predicates from left and right sides
            const allText = [construct.leftSide, construct.rightSide, construct.predicate, construct.definition]
                .filter(Boolean).join(' ');
            
            rule.predicates = this.extractPredicates(allText);
            
            this.rules.push(rule);
        });
    }
    
    /**
     * Parse function definitions
     */
    parseFunctionDefinitions(funcContent, startId) {
        const lines = funcContent.split('\n')
            .map(line => line.trim())
            .filter(line => line && !line.startsWith('//'));
            
        lines.forEach((line, index) => {
            if (line.includes('≜')) {
                const [name, definition] = line.split('≜').map(s => s.trim());
                
                const rule = new AispRule(
                    `func_${startId + index}`,
                    line,
                    'function',
                    [name],
                    this.extractPredicates(definition)
                );
                
                this.rules.push(rule);
                this.definitions.set(name, rule);
            }
        });
    }
    
    /**
     * Build dependency graph between rules
     */
    buildDependencyGraph() {
        const graph = {
            nodes: [],
            edges: [],
            cycles: [],
            stronglyConnectedComponents: []
        };
        
        // Create nodes
        this.rules.forEach(rule => {
            graph.nodes.push({
                id: rule.id,
                type: rule.type,
                content: rule.content,
                inDegree: 0,
                outDegree: 0
            });
        });
        
        // Create edges based on dependencies
        this.rules.forEach(sourceRule => {
            sourceRule.predicates.forEach(predicate => {
                // Find rules that define this predicate
                this.rules.forEach(targetRule => {
                    if (targetRule !== sourceRule && 
                        (targetRule.variables.includes(predicate) || 
                         targetRule.content.includes(predicate))) {
                        
                        const dependency = new LogicalDependency(
                            sourceRule.id,
                            targetRule.id,
                            'uses',
                            this.calculateDependencyStrength(sourceRule, targetRule)
                        );
                        
                        this.dependencies.push(dependency);
                        sourceRule.addDependency(dependency);
                        targetRule.addDependent(dependency);
                        
                        graph.edges.push({
                            source: sourceRule.id,
                            target: targetRule.id,
                            type: 'uses',
                            strength: dependency.strength
                        });
                    }
                });
            });
        });
        
        // Calculate degrees
        graph.edges.forEach(edge => {
            const sourceNode = graph.nodes.find(n => n.id === edge.source);
            const targetNode = graph.nodes.find(n => n.id === edge.target);
            if (sourceNode) sourceNode.outDegree++;
            if (targetNode) targetNode.inDegree++;
        });
        
        // Detect cycles
        graph.cycles = this.detectCycles(graph);
        
        return graph;
    }
    
    /**
     * Find logical consistency chains
     */
    findConsistencyChains() {
        const chains = [];
        
        // Find implication chains: A⇒B⇒C
        this.rules.forEach(rule => {
            if (rule.content.includes('⇒') || rule.content.includes('⇔')) {
                const chain = this.traceImplicationChain(rule);
                if (chain.length > 1) {
                    chains.push({
                        type: 'implication',
                        rules: chain,
                        strength: this.calculateChainStrength(chain),
                        consistent: this.checkChainConsistency(chain)
                    });
                }
            }
        });
        
        // Find equivalence chains: A⇔B⇔C
        const equivalenceGroups = this.findEquivalenceGroups();
        equivalenceGroups.forEach(group => {
            if (group.length > 1) {
                chains.push({
                    type: 'equivalence',
                    rules: group,
                    strength: 1.0,
                    consistent: this.checkEquivalenceConsistency(group)
                });
            }
        });
        
        return chains;
    }
    
    /**
     * Detect logical invariants
     */
    detectInvariants() {
        const invariants = [];
        
        // Type invariants: Properties that must hold for all instances
        this.rules.forEach(rule => {
            if (rule.type === 'definition' && rule.content.includes('{')) {
                const invariant = this.extractTypeInvariant(rule);
                if (invariant) invariants.push(invariant);
            }
        });
        
        // Universal quantifier invariants
        this.rules.forEach(rule => {
            if (rule.content.includes('∀')) {
                const invariant = this.extractUniversalInvariant(rule);
                if (invariant) invariants.push(invariant);
            }
        });
        
        // Cross-rule invariants: Properties maintained across multiple rules
        const crossInvariants = this.findCrossRuleInvariants();
        invariants.push(...crossInvariants);
        
        return invariants;
    }
    
    /**
     * Analyze logical completeness
     */
    analyzeCompleteness() {
        const completeness = {
            score: 0,
            coverage: {},
            gaps: [],
            redundancies: []
        };
        
        // Check if all referenced predicates are defined
        const undefinedPredicates = this.findUndefinedPredicates();
        completeness.gaps.push(...undefinedPredicates.map(p => ({
            type: 'undefined_predicate',
            name: p,
            severity: 'error'
        })));
        
        // Check for case coverage
        const incompleteCases = this.findIncompleteCases();
        completeness.gaps.push(...incompleteCases);
        
        // Check for redundant rules
        const redundancies = this.findRedundantRules();
        completeness.redundancies.push(...redundancies);
        
        // Calculate completeness score
        const totalPredicates = this.getAllPredicates().length;
        const definedPredicates = totalPredicates - undefinedPredicates.length;
        completeness.score = totalPredicates > 0 ? definedPredicates / totalPredicates : 1.0;
        
        completeness.coverage = {
            predicates: `${definedPredicates}/${totalPredicates}`,
            percentage: (completeness.score * 100).toFixed(1) + '%'
        };
        
        return completeness;
    }
    
    /**
     * Analyze computational complexity
     */
    analyzeComplexity() {
        const complexity = {
            class: 'UNKNOWN',
            decidable: null,
            tractable: null,
            quantifierAlternations: 0,
            maxQuantifierDepth: 0,
            logicalOperators: 0
        };
        
        // Count quantifier alternations (∀∃∀ patterns)
        this.rules.forEach(rule => {
            const alternations = this.countQuantifierAlternations(rule.content);
            complexity.quantifierAlternations = Math.max(complexity.quantifierAlternations, alternations);
            
            const depth = this.getQuantifierDepth(rule.content);
            complexity.maxQuantifierDepth = Math.max(complexity.maxQuantifierDepth, depth);
            
            complexity.logicalOperators += (rule.content.match(/[⇒⇔∧∨¬]/g) || []).length;
        });
        
        // Determine complexity class
        if (complexity.maxQuantifierDepth === 0) {
            complexity.class = 'PROPOSITIONAL';
            complexity.decidable = true;
            complexity.tractable = true;
        } else if (complexity.quantifierAlternations === 0) {
            complexity.class = 'FIRST_ORDER_HORN';
            complexity.decidable = true;
            complexity.tractable = true;
        } else if (complexity.quantifierAlternations <= 1) {
            complexity.class = 'FIRST_ORDER';
            complexity.decidable = false; // Semi-decidable
            complexity.tractable = false;
        } else {
            complexity.class = 'HIGHER_ORDER';
            complexity.decidable = false;
            complexity.tractable = false;
        }
        
        return complexity;
    }
    
    /**
     * Calculate relational analysis metrics
     */
    calculateMetrics() {
        const totalRules = this.rules.length;
        const totalDeps = this.dependencies.length;
        
        return {
            ruleCount: totalRules,
            dependencyCount: totalDeps,
            dependencyDensity: totalRules > 0 ? totalDeps / (totalRules * totalRules) : 0,
            averageInDegree: totalRules > 0 ? totalDeps / totalRules : 0,
            cyclomaticComplexity: this.calculateCyclomaticComplexity(),
            cohesion: this.calculateCohesion(),
            coupling: this.calculateCoupling()
        };
    }
    
    // Helper methods
    
    extractPredicates(text) {
        const predicates = [];
        
        // Extract function calls: Function(args)
        const functionCalls = text.match(/\b[A-Z][a-zA-Z]*\([^)]*\)/g) || [];
        predicates.push(...functionCalls.map(f => f.split('(')[0]));
        
        // Extract simple identifiers
        const identifiers = text.match(/\b[a-zA-Z][a-zA-Z0-9]*\b/g) || [];
        predicates.push(...identifiers.filter(id => 
            !['if', 'then', 'else', 'and', 'or', 'not'].includes(id.toLowerCase())
        ));
        
        return [...new Set(predicates)];
    }
    
    constructToString(construct) {
        if (construct.type === 'definition') {
            return `${construct.name}≜${construct.definition}`;
        } else if (construct.type === 'forall') {
            return `∀${construct.variable}:${construct.leftSide}⇔${construct.rightSide}`;
        } else if (construct.type === 'exists') {
            return `∃${construct.variable}∈${construct.domain}:${construct.predicate}`;
        }
        return construct.content || JSON.stringify(construct);
    }
    
    calculateDependencyStrength(source, target) {
        // Simple heuristic based on textual overlap
        const sourceWords = new Set(source.content.toLowerCase().split(/\W+/));
        const targetWords = new Set(target.content.toLowerCase().split(/\W+/));
        
        const intersection = new Set([...sourceWords].filter(w => targetWords.has(w)));
        const union = new Set([...sourceWords, ...targetWords]);
        
        return union.size > 0 ? intersection.size / union.size : 0;
    }
    
    detectCycles(graph) {
        const cycles = [];
        const visited = new Set();
        const recursionStack = new Set();
        
        const dfs = (nodeId, path) => {
            if (recursionStack.has(nodeId)) {
                const cycleStart = path.indexOf(nodeId);
                if (cycleStart !== -1) {
                    cycles.push(path.slice(cycleStart));
                }
                return;
            }
            
            if (visited.has(nodeId)) return;
            
            visited.add(nodeId);
            recursionStack.add(nodeId);
            path.push(nodeId);
            
            const outgoingEdges = graph.edges.filter(e => e.source === nodeId);
            outgoingEdges.forEach(edge => {
                dfs(edge.target, [...path]);
            });
            
            recursionStack.delete(nodeId);
        };
        
        graph.nodes.forEach(node => {
            if (!visited.has(node.id)) {
                dfs(node.id, []);
            }
        });
        
        return cycles;
    }
    
    traceImplicationChain(startRule) {
        // Simplified implementation
        return [startRule.id];
    }
    
    findEquivalenceGroups() {
        // Simplified implementation
        return [];
    }
    
    checkChainConsistency(chain) {
        // Placeholder - would need sophisticated logic checking
        return true;
    }
    
    checkEquivalenceConsistency(group) {
        // Placeholder - would need sophisticated logic checking  
        return true;
    }
    
    calculateChainStrength(chain) {
        return chain.length > 0 ? 1.0 / chain.length : 0;
    }
    
    extractTypeInvariant(rule) {
        // Simplified type invariant extraction
        return {
            type: 'type_constraint',
            rule: rule.id,
            property: 'enumeration_completeness',
            description: `All values in ${rule.variables[0]} must be from defined set`
        };
    }
    
    extractUniversalInvariant(rule) {
        // Simplified universal invariant extraction
        return {
            type: 'universal_property',
            rule: rule.id,
            property: 'universal_constraint', 
            description: `Property holds for all instances in domain`
        };
    }
    
    findCrossRuleInvariants() {
        // Placeholder for cross-rule invariant detection
        return [];
    }
    
    findUndefinedPredicates() {
        const defined = new Set(this.definitions.keys());
        const used = new Set();
        
        this.rules.forEach(rule => {
            rule.predicates.forEach(pred => used.add(pred));
        });
        
        return [...used].filter(pred => !defined.has(pred));
    }
    
    findIncompleteCases() {
        // Placeholder for case completeness analysis
        return [];
    }
    
    findRedundantRules() {
        // Placeholder for redundancy detection
        return [];
    }
    
    getAllPredicates() {
        const all = new Set();
        this.rules.forEach(rule => {
            rule.predicates.forEach(pred => all.add(pred));
        });
        return [...all];
    }
    
    countQuantifierAlternations(text) {
        // Count ∀∃ or ∃∀ patterns
        return (text.match(/[∀∃][^∀∃]*[∃∀]/g) || []).length;
    }
    
    getQuantifierDepth(text) {
        // Count maximum nesting depth of quantifiers
        let depth = 0;
        let maxDepth = 0;
        
        for (const char of text) {
            if (char === '∀' || char === '∃') {
                depth++;
                maxDepth = Math.max(maxDepth, depth);
            } else if (char === ')' || char === '}') {
                depth = Math.max(0, depth - 1);
            }
        }
        
        return maxDepth;
    }
    
    calculateCyclomaticComplexity() {
        // Simplified cyclomatic complexity based on logical operators
        let complexity = 1; // Base complexity
        
        this.rules.forEach(rule => {
            const operators = (rule.content.match(/[∧∨⇒⇔]/g) || []).length;
            complexity += operators;
        });
        
        return complexity;
    }
    
    calculateCohesion() {
        // Measure how closely related the rules are
        if (this.rules.length === 0) return 0;
        
        let totalSimilarity = 0;
        let pairs = 0;
        
        for (let i = 0; i < this.rules.length; i++) {
            for (let j = i + 1; j < this.rules.length; j++) {
                totalSimilarity += this.calculateDependencyStrength(this.rules[i], this.rules[j]);
                pairs++;
            }
        }
        
        return pairs > 0 ? totalSimilarity / pairs : 0;
    }
    
    calculateCoupling() {
        // Measure inter-rule dependencies
        const totalPossibleDeps = this.rules.length * (this.rules.length - 1);
        return totalPossibleDeps > 0 ? this.dependencies.length / totalPossibleDeps : 0;
    }
    
    countLines(text) {
        return text.split('\n').filter(line => line.trim()).length;
    }
}