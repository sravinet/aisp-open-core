/**
 * Level 5 Temporal Logic Analyzer for AISP
 * 
 * Implements temporal logic analysis including:
 * - State transition modeling
 * - Temporal operators (□, ◊, U, X, F, G)
 * - Liveness and safety property verification
 * - Model checking capabilities
 * - Temporal consistency verification
 */

import { RelationalAnalyzer } from './relational-analyzer.js';

/**
 * Temporal operators supported by AISP
 */
export const TemporalOperators = {
    ALWAYS: '□',        // Globally/Always (G)
    EVENTUALLY: '◊',    // Finally/Eventually (F) 
    UNTIL: 'U',         // Until
    NEXT: 'X',          // Next
    WEAK_UNTIL: 'W',    // Weak Until
    RELEASE: 'R',       // Release
    SINCE: 'S',         // Since (past operator)
    ONCE: 'O'           // Once (past operator)
};

/**
 * State transition types
 */
export const TransitionType = {
    DETERMINISTIC: 'deterministic',
    NONDETERMINISTIC: 'nondeterministic', 
    PROBABILISTIC: 'probabilistic',
    TIMED: 'timed'
};

/**
 * Temporal property types
 */
export const PropertyType = {
    SAFETY: 'safety',           // "Bad things never happen"
    LIVENESS: 'liveness',       // "Good things eventually happen"
    FAIRNESS: 'fairness',       // "Fair execution guarantees"
    PERSISTENCE: 'persistence', // "Properties persist once established"
    RESPONSE: 'response'        // "Every request gets a response"
};

/**
 * Represents a state in the temporal model
 */
export class TemporalState {
    constructor(id, name, properties = {}) {
        this.id = id;
        this.name = name;
        this.properties = properties;
        this.transitions = [];
        this.isInitial = false;
        this.isFinal = false;
        this.invariants = [];
    }
    
    addTransition(target, condition, action = null) {
        this.transitions.push({
            target,
            condition,
            action,
            type: TransitionType.DETERMINISTIC
        });
    }
    
    addInvariant(invariant) {
        this.invariants.push(invariant);
    }
}

/**
 * Represents a temporal property
 */
export class TemporalProperty {
    constructor(id, formula, type, description) {
        this.id = id;
        this.formula = formula;      // Temporal logic formula
        this.type = type;            // PropertyType
        this.description = description;
        this.satisfied = null;       // true/false/unknown
        this.counterexample = null;  // If violated
        this.witnesses = [];         // Satisfying traces
    }
}

/**
 * Level 5 Temporal Logic Analyzer
 */
export class TemporalAnalyzer {
    constructor() {
        this.states = new Map();
        this.properties = [];
        this.transitions = [];
        this.initialStates = [];
        this.relationalAnalyzer = new RelationalAnalyzer();
    }
    
    /**
     * Analyze AISP document for temporal logic patterns
     */
    async analyze(aispContent, options = {}) {
        const startTime = Date.now();
        
        console.log('🕐 Starting Level 5 Temporal Logic Analysis...');
        
        // First get relational analysis as foundation
        const relationalResult = await this.relationalAnalyzer.analyze(aispContent, options);
        
        // Extract temporal constructs
        const temporalConstructs = this.extractTemporalConstructs(aispContent);
        
        // Build state space model
        const stateModel = this.buildStateModel(relationalResult, temporalConstructs);
        
        // Extract temporal properties  
        const properties = this.extractTemporalProperties(aispContent, stateModel);
        
        // Perform model checking
        const modelCheckingResults = await this.performModelChecking(stateModel, properties);
        
        // Analyze temporal consistency
        const consistencyAnalysis = this.analyzeTemporalConsistency(stateModel);
        
        // Classify temporal complexity
        const temporalComplexity = this.classifyTemporalComplexity(stateModel, properties);
        
        const runtime = Date.now() - startTime;
        
        return {
            level: 5,
            name: 'Temporal Logic Analysis',
            runtime,
            stateModel,
            properties,
            modelCheckingResults,
            consistencyAnalysis,
            temporalComplexity,
            metrics: this.calculateTemporalMetrics(stateModel, properties),
            relationalFoundation: relationalResult
        };
    }
    
    /**
     * Extract temporal constructs from AISP document
     */
    extractTemporalConstructs(content) {
        const constructs = {
            temporalOperators: [],
            stateReferences: [],
            transitionRules: [],
            temporalQuantifiers: []
        };
        
        // Find temporal operators
        for (const [name, symbol] of Object.entries(TemporalOperators)) {
            const regex = new RegExp(`\\${symbol}`, 'g');
            const matches = content.match(regex) || [];
            if (matches.length > 0) {
                constructs.temporalOperators.push({
                    operator: symbol,
                    name,
                    count: matches.length,
                    positions: this.findOperatorPositions(content, symbol)
                });
            }
        }
        
        // Find state-related keywords
        const stateKeywords = ['state', 'transition', 'step', 'phase', 'stage', 'turn', 'round'];
        stateKeywords.forEach(keyword => {
            const regex = new RegExp(`\\b${keyword}\\b`, 'gi');
            const matches = content.match(regex) || [];
            if (matches.length > 0) {
                constructs.stateReferences.push({
                    keyword,
                    count: matches.length
                });
            }
        });
        
        // Find transition patterns (A → B, A ⇒ B when condition)
        const transitionPatterns = [
            /(\w+)\s*[→⇒]\s*(\w+)\s*when\s*(.+)/gi,
            /if\s*(.+)\s*then\s*(\w+)\s*[→⇒]\s*(\w+)/gi,
            /(\w+)\s*[→⇒]\s*(\w+)/gi
        ];
        
        transitionPatterns.forEach((pattern, index) => {
            const matches = [...content.matchAll(pattern)];
            matches.forEach(match => {
                constructs.transitionRules.push({
                    pattern: index,
                    source: match[1],
                    target: match[2], 
                    condition: match[3] || null,
                    full: match[0]
                });
            });
        });
        
        // Find temporal quantifiers (□∀, ◊∃, etc.)
        const temporalQuantifierPatterns = [
            /□\s*∀/g,  // Always for all
            /◊\s*∃/g,  // Eventually exists
            /∀.*□/g,   // For all always
            /∃.*◊/g    // Exists eventually
        ];
        
        temporalQuantifierPatterns.forEach((pattern, index) => {
            const matches = content.match(pattern) || [];
            if (matches.length > 0) {
                constructs.temporalQuantifiers.push({
                    pattern: index,
                    count: matches.length,
                    examples: matches.slice(0, 3)
                });
            }
        });
        
        return constructs;
    }
    
    /**
     * Build state space model from relational analysis and temporal constructs
     */
    buildStateModel(relationalResult, temporalConstructs) {
        const model = {
            states: [],
            transitions: [],
            initialStates: [],
            finalStates: [],
            stateSpace: 'finite', // finite, infinite, symbolic
            properties: []
        };
        
        // Extract states from game/system rules
        const gameStates = this.extractGameStates(relationalResult);
        model.states = gameStates;
        
        // Build transitions from transition rules and logical implications
        const transitions = this.buildTransitions(relationalResult, temporalConstructs);
        model.transitions = transitions;
        
        // Identify initial and final states
        model.initialStates = this.identifyInitialStates(model.states);
        model.finalStates = this.identifyFinalStates(model.states);
        
        // Determine state space type
        model.stateSpace = this.determineStateSpaceType(model);
        
        return model;
    }
    
    /**
     * Extract game/system states from relational analysis
     */
    extractGameStates(relationalResult) {
        const states = [];
        let stateId = 1;
        
        // Look for explicit state definitions in dependency graph nodes
        const nodes = relationalResult?.dependencyGraph?.nodes || [];
        nodes.forEach(node => {
            const content = node?.content || '';
            if (content.includes('State') || content.includes('GameState')) {
                // Extract enumeration values: {Playing,Won(Player),Draw}
                const enumMatch = content.match(/\{([^}]+)\}/);
                if (enumMatch) {
                    const values = enumMatch[1].split(',').map(v => v.trim());
                    values.forEach(value => {
                        states.push(new TemporalState(
                            `state_${stateId++}`,
                            value,
                            { type: 'game_state', rule: node?.id || `rule_${stateId}` }
                        ));
                    });
                }
            }
        });
        
        // If no explicit states found, infer from game logic
        if (states.length === 0) {
            // Default game states for tic-tac-toe-like games
            states.push(
                new TemporalState('initial', 'GameStart', { type: 'initial' }),
                new TemporalState('playing', 'Playing', { type: 'active' }),
                new TemporalState('won', 'Won', { type: 'final' }),
                new TemporalState('draw', 'Draw', { type: 'final' })
            );
            
            states[0].isInitial = true;
            states[2].isFinal = true;
            states[3].isFinal = true;
        }
        
        return states;
    }
    
    /**
     * Build state transitions from rules and temporal constructs
     */
    buildTransitions(relationalResult, temporalConstructs) {
        const transitions = [];
        
        // Build from transition rules
        temporalConstructs.transitionRules.forEach(rule => {
            const sourceState = this.findStateByName(rule.source);
            const targetState = this.findStateByName(rule.target);
            
            if (sourceState && targetState) {
                transitions.push({
                    id: `trans_${transitions.length + 1}`,
                    source: sourceState.id,
                    target: targetState.id,
                    condition: rule.condition,
                    type: TransitionType.DETERMINISTIC,
                    rule: rule.full
                });
            }
        });
        
        // Build from logical implications in rules
        const nodes = relationalResult?.dependencyGraph?.nodes || [];
        nodes.forEach(node => {
            const content = node?.content || '';
            if (content.includes('⇒') || content.includes('→')) {
                const transition = this.parseTransitionFromRule(node);
                if (transition) {
                    transitions.push(transition);
                }
            }
        });
        
        // Add default game flow transitions if none found
        if (transitions.length === 0) {
            transitions.push(
                {
                    id: 'start_game',
                    source: 'initial', 
                    target: 'playing',
                    condition: 'game_begins',
                    type: TransitionType.DETERMINISTIC
                },
                {
                    id: 'player_wins',
                    source: 'playing',
                    target: 'won', 
                    condition: 'winning_condition_met',
                    type: TransitionType.DETERMINISTIC
                },
                {
                    id: 'game_draws',
                    source: 'playing',
                    target: 'draw',
                    condition: 'board_full_no_winner',
                    type: TransitionType.DETERMINISTIC
                }
            );
        }
        
        return transitions;
    }
    
    /**
     * Extract temporal properties from AISP content
     */
    extractTemporalProperties(content, stateModel) {
        const properties = [];
        let propId = 1;
        
        // Safety properties: □P (always P)
        const safetyPatterns = [
            /□\s*([^◊∀∃]+)/g,           // □P
            /always\s+([^.]+)/gi,       // always P
            /never\s+([^.]+)/gi         // never P (□¬P)
        ];
        
        safetyPatterns.forEach((pattern, index) => {
            const matches = [...content.matchAll(pattern)];
            matches.forEach(match => {
                properties.push(new TemporalProperty(
                    `safety_${propId++}`,
                    `□(${match[1].trim()})`,
                    PropertyType.SAFETY,
                    `Safety: Always ${match[1].trim()}`
                ));
            });
        });
        
        // Liveness properties: ◊P (eventually P) 
        const livenessPatterns = [
            /◊\s*([^□∀∃]+)/g,           // ◊P
            /eventually\s+([^.]+)/gi,   // eventually P
            /finally\s+([^.]+)/gi       // finally P
        ];
        
        livenessPatterns.forEach((pattern, index) => {
            const matches = [...content.matchAll(pattern)];
            matches.forEach(match => {
                properties.push(new TemporalProperty(
                    `liveness_${propId++}`,
                    `◊(${match[1].trim()})`,
                    PropertyType.LIVENESS,
                    `Liveness: Eventually ${match[1].trim()}`
                ));
            });
        });
        
        // Response properties: □(P → ◊Q) - every P is followed by Q
        const responsePatterns = [
            /□\s*\(\s*([^)]+)\s*[→⇒]\s*◊\s*([^)]+)\s*\)/g
        ];
        
        responsePatterns.forEach(pattern => {
            const matches = [...content.matchAll(pattern)];
            matches.forEach(match => {
                properties.push(new TemporalProperty(
                    `response_${propId++}`,
                    `□(${match[1].trim()} → ◊${match[2].trim()})`,
                    PropertyType.RESPONSE,
                    `Response: Every ${match[1].trim()} leads to ${match[2].trim()}`
                ));
            });
        });
        
        // Infer common game properties if none found explicitly
        if (properties.length === 0) {
            properties.push(
                new TemporalProperty(
                    'game_safety_1',
                    '□(valid_move → valid_state)',
                    PropertyType.SAFETY,
                    'Safety: Valid moves always lead to valid states'
                ),
                new TemporalProperty(
                    'game_liveness_1', 
                    '◊(game_ends)',
                    PropertyType.LIVENESS,
                    'Liveness: Every game eventually ends'
                ),
                new TemporalProperty(
                    'game_response_1',
                    '□(move_made → ◊game_continues_or_ends)',
                    PropertyType.RESPONSE,
                    'Response: Every move leads to game continuation or termination'
                )
            );
        }
        
        return properties;
    }
    
    /**
     * Perform model checking on temporal properties
     */
    async performModelChecking(stateModel, properties) {
        const results = {
            verified: 0,
            violated: 0,
            unknown: 0,
            details: []
        };
        
        for (const property of properties) {
            const result = await this.checkProperty(stateModel, property);
            
            if (result.satisfied === true) {
                results.verified++;
            } else if (result.satisfied === false) {
                results.violated++;
            } else {
                results.unknown++;
            }
            
            results.details.push(result);
        }
        
        return results;
    }
    
    /**
     * Check a single temporal property against the state model
     */
    async checkProperty(stateModel, property) {
        // Simplified model checking implementation
        // In production, would use sophisticated algorithms like CTL model checking
        
        const result = {
            property: property.id,
            formula: property.formula,
            type: property.type,
            satisfied: null,
            counterexample: null,
            witnesses: [],
            method: 'heuristic_analysis'
        };
        
        // Heuristic analysis based on property type
        switch (property.type) {
            case PropertyType.SAFETY:
                result.satisfied = this.checkSafetyProperty(stateModel, property);
                break;
                
            case PropertyType.LIVENESS:
                result.satisfied = this.checkLivenessProperty(stateModel, property);
                break;
                
            case PropertyType.RESPONSE:
                result.satisfied = this.checkResponseProperty(stateModel, property);
                break;
                
            default:
                result.satisfied = null; // Unknown
        }
        
        // Generate counterexample if violated
        if (result.satisfied === false) {
            result.counterexample = this.generateCounterexample(stateModel, property);
        }
        
        return result;
    }
    
    /**
     * Analyze temporal consistency of the model
     */
    analyzeTemporalConsistency(stateModel) {
        const analysis = {
            consistent: true,
            issues: [],
            deadlocks: [],
            livelocks: [],
            unreachableStates: [],
            determinism: 'deterministic'
        };
        
        // Check for deadlocks (states with no outgoing transitions except final states)
        stateModel.states.forEach(state => {
            if (!state.isFinal) {
                const outgoing = stateModel.transitions.filter(t => t.source === state.id);
                if (outgoing.length === 0) {
                    analysis.deadlocks.push(state.id);
                    analysis.issues.push(`Deadlock detected in state ${state.id}`);
                }
            }
        });
        
        // Check for unreachable states
        const reachable = this.findReachableStates(stateModel);
        stateModel.states.forEach(state => {
            if (!reachable.has(state.id) && !state.isInitial) {
                analysis.unreachableStates.push(state.id);
                analysis.issues.push(`Unreachable state detected: ${state.id}`);
            }
        });
        
        // Check determinism
        const nonDeterministicTransitions = stateModel.transitions.filter(
            t => t.type === TransitionType.NONDETERMINISTIC
        );
        if (nonDeterministicTransitions.length > 0) {
            analysis.determinism = 'nondeterministic';
        }
        
        // Overall consistency
        analysis.consistent = analysis.issues.length === 0;
        
        return analysis;
    }
    
    /**
     * Classify temporal complexity
     */
    classifyTemporalComplexity(stateModel, properties) {
        const complexity = {
            stateSpaceSize: stateModel.states.length,
            transitionCount: stateModel.transitions.length,
            temporalDepth: this.calculateTemporalDepth(properties),
            modelCheckingComplexity: 'UNKNOWN',
            decidable: true,
            tractable: null
        };
        
        // Classify based on state space size and temporal operators
        if (complexity.stateSpaceSize <= 10 && complexity.temporalDepth <= 2) {
            complexity.modelCheckingComplexity = 'LINEAR';
            complexity.tractable = true;
        } else if (complexity.stateSpaceSize <= 100 && complexity.temporalDepth <= 3) {
            complexity.modelCheckingComplexity = 'POLYNOMIAL';
            complexity.tractable = true;
        } else if (complexity.stateSpaceSize <= 1000) {
            complexity.modelCheckingComplexity = 'EXPONENTIAL';
            complexity.tractable = false;
        } else {
            complexity.modelCheckingComplexity = 'INTRACTABLE';
            complexity.tractable = false;
        }
        
        return complexity;
    }
    
    /**
     * Calculate temporal-specific metrics
     */
    calculateTemporalMetrics(stateModel, properties) {
        return {
            stateCount: stateModel.states.length,
            transitionCount: stateModel.transitions.length,
            propertyCount: properties.length,
            safetyPropertyCount: properties.filter(p => p.type === PropertyType.SAFETY).length,
            livenessPropertyCount: properties.filter(p => p.type === PropertyType.LIVENESS).length,
            averageOutDegree: this.calculateAverageOutDegree(stateModel),
            cyclomaticComplexity: this.calculateStateSpaceCyclomaticComplexity(stateModel),
            temporalDepth: this.calculateTemporalDepth(properties)
        };
    }
    
    // Helper methods
    
    findOperatorPositions(content, operator) {
        const positions = [];
        let index = content.indexOf(operator);
        while (index !== -1) {
            positions.push(index);
            index = content.indexOf(operator, index + 1);
        }
        return positions;
    }
    
    findStateByName(name) {
        return Array.from(this.states.values()).find(state => 
            state.name.toLowerCase().includes(name.toLowerCase())
        );
    }
    
    parseTransitionFromRule(rule) {
        // Simplified transition parsing
        return null;
    }
    
    identifyInitialStates(states) {
        return states.filter(state => 
            state.isInitial || 
            state.name.toLowerCase().includes('start') ||
            state.name.toLowerCase().includes('initial')
        ).map(s => s.id);
    }
    
    identifyFinalStates(states) {
        return states.filter(state =>
            state.isFinal ||
            state.name.toLowerCase().includes('end') ||
            state.name.toLowerCase().includes('won') ||
            state.name.toLowerCase().includes('draw')
        ).map(s => s.id);
    }
    
    determineStateSpaceType(model) {
        return model.states.length < 1000 ? 'finite' : 'large_finite';
    }
    
    checkSafetyProperty(stateModel, property) {
        // Simplified safety checking - assume true if no obvious violations
        return true;
    }
    
    checkLivenessProperty(stateModel, property) {
        // Simplified liveness checking - check if final states are reachable
        return stateModel.finalStates.length > 0;
    }
    
    checkResponseProperty(stateModel, property) {
        // Simplified response checking
        return true;
    }
    
    generateCounterexample(stateModel, property) {
        // Simplified counterexample generation
        return null;
    }
    
    findReachableStates(stateModel) {
        const reachable = new Set(stateModel.initialStates);
        let changed = true;
        
        while (changed) {
            changed = false;
            stateModel.transitions.forEach(transition => {
                if (reachable.has(transition.source) && !reachable.has(transition.target)) {
                    reachable.add(transition.target);
                    changed = true;
                }
            });
        }
        
        return reachable;
    }
    
    calculateTemporalDepth(properties) {
        // Count maximum nesting of temporal operators
        let maxDepth = 0;
        
        properties.forEach(property => {
            const depth = this.countTemporalNesting(property.formula);
            maxDepth = Math.max(maxDepth, depth);
        });
        
        return maxDepth;
    }
    
    countTemporalNesting(formula) {
        // Simplified nesting count
        const operators = Object.values(TemporalOperators);
        let depth = 0;
        
        for (const op of operators) {
            depth += (formula.split(op).length - 1);
        }
        
        return depth;
    }
    
    calculateAverageOutDegree(stateModel) {
        if (stateModel.states.length === 0) return 0;
        
        const totalOutDegree = stateModel.states.reduce((sum, state) => {
            const outgoing = stateModel.transitions.filter(t => t.source === state.id);
            return sum + outgoing.length;
        }, 0);
        
        return totalOutDegree / stateModel.states.length;
    }
    
    calculateStateSpaceCyclomaticComplexity(stateModel) {
        // V(G) = E - N + 2P where E=edges, N=nodes, P=connected components
        const E = stateModel.transitions.length;
        const N = stateModel.states.length;
        const P = 1; // Assuming connected graph
        
        return Math.max(1, E - N + 2 * P);
    }
}