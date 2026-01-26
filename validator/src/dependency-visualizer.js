/**
 * Dependency Graph Visualizer for AISP Relational Logic
 * 
 * Generates ASCII art visualizations of rule dependencies,
 * consistency chains, and logical flow analysis.
 */

export class DependencyVisualizer {
    constructor() {
        this.symbols = {
            node: '●',
            edge: '→',
            cycle: '↻',
            strong: '⇒',
            weak: '⇀',
            bidirectional: '⇔',
            conflict: '⚡'
        };
    }
    
    /**
     * Generate ASCII visualization of dependency graph
     */
    visualizeDependencyGraph(graph, options = {}) {
        const { maxWidth = 80, showStrength = true, showCycles = true } = options;
        
        let output = '';
        output += '🕸️  DEPENDENCY GRAPH\n';
        output += '═'.repeat(maxWidth) + '\n\n';
        
        // Show nodes
        output += 'NODES:\n';
        graph.nodes.forEach(node => {
            const strength = this.getNodeStrength(node, graph);
            const symbol = strength > 0.7 ? '🔴' : strength > 0.4 ? '🟡' : '🟢';
            output += `  ${symbol} ${node.id} (${node.type}) [in:${node.inDegree}, out:${node.outDegree}]\n`;
        });
        
        output += '\n';
        
        // Show edges
        if (graph.edges.length > 0) {
            output += 'DEPENDENCIES:\n';
            graph.edges.forEach(edge => {
                const arrow = showStrength && edge.strength > 0.7 ? this.symbols.strong :
                             showStrength && edge.strength < 0.3 ? this.symbols.weak :
                             this.symbols.edge;
                const strengthStr = showStrength ? ` [${edge.strength.toFixed(2)}]` : '';
                output += `  ${edge.source} ${arrow} ${edge.target}${strengthStr}\n`;
            });
            output += '\n';
        }
        
        // Show cycles
        if (showCycles && graph.cycles.length > 0) {
            output += 'CYCLES DETECTED:\n';
            graph.cycles.forEach((cycle, index) => {
                output += `  ${index + 1}. ${cycle.join(' → ')} → ${cycle[0]} ${this.symbols.cycle}\n`;
            });
            output += '\n';
        }
        
        return output;
    }
    
    /**
     * Visualize consistency chains
     */
    visualizeConsistencyChains(chains, options = {}) {
        const { maxWidth = 80 } = options;
        
        let output = '';
        output += '🔗 CONSISTENCY CHAINS\n';
        output += '═'.repeat(maxWidth) + '\n\n';
        
        if (chains.length === 0) {
            output += '  No consistency chains detected.\n\n';
            return output;
        }
        
        chains.forEach((chain, index) => {
            const status = chain.consistent ? '✅' : '❌';
            const strengthBar = this.generateStrengthBar(chain.strength);
            
            output += `${index + 1}. ${chain.type.toUpperCase()} CHAIN ${status}\n`;
            output += `   Strength: ${strengthBar} (${chain.strength.toFixed(2)})\n`;
            output += `   Rules: ${chain.rules.join(' ⇒ ')}\n\n`;
        });
        
        return output;
    }
    
    /**
     * Visualize logical invariants
     */
    visualizeInvariants(invariants, options = {}) {
        const { maxWidth = 80 } = options;
        
        let output = '';
        output += '🛡️  LOGICAL INVARIANTS\n';
        output += '═'.repeat(maxWidth) + '\n\n';
        
        if (invariants.length === 0) {
            output += '  No invariants detected.\n\n';
            return output;
        }
        
        const groupedInvariants = this.groupInvariantsByType(invariants);
        
        Object.entries(groupedInvariants).forEach(([type, invs]) => {
            output += `${type.toUpperCase().replace('_', ' ')}:\n`;
            invs.forEach(inv => {
                output += `  ▪ ${inv.description} (${inv.rule})\n`;
            });
            output += '\n';
        });
        
        return output;
    }
    
    /**
     * Generate comprehensive analysis report
     */
    generateAnalysisReport(relationalResult, options = {}) {
        const { includeGraph = true, includeChains = true, includeInvariants = true } = options;
        
        let report = '';
        
        // Header
        report += '📊 AISP RELATIONAL LOGIC ANALYSIS REPORT\n';
        report += '═'.repeat(80) + '\n';
        report += `Generated: ${new Date().toISOString()}\n`;
        report += `Analysis Level: ${relationalResult.level}\n`;
        report += `Runtime: ${relationalResult.runtime}ms\n\n`;
        
        // Summary metrics
        report += '📈 SUMMARY METRICS\n';
        report += '─'.repeat(50) + '\n';
        report += `Rules Analyzed: ${relationalResult.rules}\n`;
        report += `Dependencies Found: ${relationalResult.dependencies}\n`;
        report += `Dependency Density: ${relationalResult.metrics.dependencyDensity.toFixed(3)}\n`;
        report += `Cohesion Score: ${relationalResult.metrics.cohesion.toFixed(3)}\n`;
        report += `Coupling Score: ${relationalResult.metrics.coupling.toFixed(3)}\n`;
        report += `Cyclomatic Complexity: ${relationalResult.metrics.cyclomaticComplexity}\n\n`;
        
        // Complexity analysis
        report += '🧮 COMPLEXITY ANALYSIS\n';
        report += '─'.repeat(50) + '\n';
        report += `Complexity Class: ${relationalResult.complexity.class}\n`;
        report += `Decidable: ${relationalResult.complexity.decidable ? 'Yes' : 'No'}\n`;
        report += `Tractable: ${relationalResult.complexity.tractable ? 'Yes' : 'No'}\n`;
        report += `Max Quantifier Depth: ${relationalResult.complexity.maxQuantifierDepth}\n`;
        report += `Quantifier Alternations: ${relationalResult.complexity.quantifierAlternations}\n\n`;
        
        // Completeness analysis
        report += '✅ COMPLETENESS ANALYSIS\n';
        report += '─'.repeat(50) + '\n';
        report += `Completeness Score: ${(relationalResult.completeness.score * 100).toFixed(1)}%\n`;
        report += `Coverage: ${relationalResult.completeness.coverage.percentage}\n`;
        report += `Gaps Detected: ${relationalResult.completeness.gaps.length}\n`;
        report += `Redundancies: ${relationalResult.completeness.redundancies.length}\n\n`;
        
        // Detailed visualizations
        if (includeGraph && relationalResult.dependencyGraph) {
            report += this.visualizeDependencyGraph(relationalResult.dependencyGraph);
        }
        
        if (includeChains && relationalResult.consistencyChains) {
            report += this.visualizeConsistencyChains(relationalResult.consistencyChains);
        }
        
        if (includeInvariants && relationalResult.invariants) {
            report += this.visualizeInvariants(relationalResult.invariants);
        }
        
        // Recommendations
        report += '💡 RECOMMENDATIONS\n';
        report += '─'.repeat(50) + '\n';
        report += this.generateRecommendations(relationalResult);
        
        return report;
    }
    
    /**
     * Generate actionable recommendations based on analysis
     */
    generateRecommendations(result) {
        const recommendations = [];
        
        // Completeness recommendations
        if (result.completeness.score < 0.8) {
            recommendations.push('• Consider defining missing predicates to improve completeness');
        }
        
        if (result.completeness.gaps.length > 0) {
            recommendations.push(`• Address ${result.completeness.gaps.length} logical gaps in rule coverage`);
        }
        
        // Complexity recommendations
        if (result.complexity.class === 'HIGHER_ORDER') {
            recommendations.push('• Consider simplifying higher-order constructs for better tractability');
        }
        
        if (result.complexity.maxQuantifierDepth > 3) {
            recommendations.push('• Deep quantifier nesting may impact verification performance');
        }
        
        // Coupling recommendations
        if (result.metrics.coupling > 0.7) {
            recommendations.push('• High coupling detected - consider modularizing rules');
        }
        
        if (result.metrics.cohesion < 0.3) {
            recommendations.push('• Low cohesion detected - rules may lack thematic unity');
        }
        
        // Dependency recommendations
        if (result.dependencyGraph?.cycles?.length > 0) {
            recommendations.push('• Circular dependencies detected - review rule relationships');
        }
        
        if (recommendations.length === 0) {
            recommendations.push('• Logical structure appears well-formed - no major issues detected');
        }
        
        return recommendations.join('\n') + '\n\n';
    }
    
    // Helper methods
    
    getNodeStrength(node, graph) {
        const totalDegree = node.inDegree + node.outDegree;
        const maxDegree = Math.max(...graph.nodes.map(n => n.inDegree + n.outDegree));
        return maxDegree > 0 ? totalDegree / maxDegree : 0;
    }
    
    generateStrengthBar(strength, width = 10) {
        const filled = Math.round(strength * width);
        const empty = width - filled;
        return '█'.repeat(filled) + '░'.repeat(empty);
    }
    
    groupInvariantsByType(invariants) {
        const grouped = {};
        
        invariants.forEach(inv => {
            const type = inv.type || 'unknown';
            if (!grouped[type]) grouped[type] = [];
            grouped[type].push(inv);
        });
        
        return grouped;
    }
}