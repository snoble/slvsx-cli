#!/usr/bin/env tsx

import { InputDocument, PointEntity, LineEntity, DistanceConstraint, FixedConstraint } from './types.js';

/**
 * Advanced Parametric Bridge Truss Generator
 * 
 * Generates a Warren Truss bridge structure with configurable parameters:
 * - Number of panels (bays)
 * - Bridge span length
 * - Bridge height
 * - Panel topology (triangular pattern)
 * 
 * This demonstrates complex constraint generation that would be tedious to write by hand.
 */
class ParametricTrussGenerator {
    private panelCount: number;
    private spanLength: number;
    private bridgeHeight: number;
    private entities: (PointEntity | LineEntity)[] = [];
    private constraints: (DistanceConstraint | FixedConstraint)[] = [];
    private nodeCounter = 0;
    private memberCounter = 0;

    constructor(panelCount: number, spanLength: number, bridgeHeight: number) {
        this.panelCount = panelCount;
        this.spanLength = spanLength;
        this.bridgeHeight = bridgeHeight;
    }

    private createNode(id: string, x: number, y: number, z: number = 0): PointEntity {
        return {
            type: "point",
            id,
            at: [x, y, z]
        };
    }

    private createMember(id: string, node1: string, node2: string): LineEntity {
        return {
            type: "line",
            id,
            p1: node1,
            p2: node2
        };
    }

    private addDistanceConstraint(between: [string, string], value: number | string) {
        this.constraints.push({
            type: "distance",
            between,
            value
        });
    }

    private addFixedConstraint(entity: string) {
        this.constraints.push({
            type: "fixed",
            entity
        });
    }

    /**
     * Generate the main truss geometry
     */
    generate(): InputDocument {
        this.generateTopChord();
        this.generateBottomChord();
        this.generateWebMembers();
        this.generateVerticalMembers();
        this.addBoundaryConditions();
        this.addParametricConstraints();

        return {
            schema: "slvs-json/1",
            units: "mm",
            parameters: {
                panel_length: this.spanLength / this.panelCount,
                bridge_height: this.bridgeHeight,
                member_length_diagonal: Math.sqrt(
                    Math.pow(this.spanLength / this.panelCount, 2) + 
                    Math.pow(this.bridgeHeight, 2)
                ),
                support_width: 50
            },
            entities: this.entities,
            constraints: this.constraints
        };
    }

    /**
     * Generate top chord nodes and members
     */
    private generateTopChord() {
        // Create top chord nodes
        for (let i = 0; i <= this.panelCount; i++) {
            const x = (i * this.spanLength) / this.panelCount;
            const nodeId = `T${i}`;
            this.entities.push(this.createNode(nodeId, x, this.bridgeHeight));
        }

        // Create top chord members
        for (let i = 0; i < this.panelCount; i++) {
            const memberId = `TOP${i}`;
            this.entities.push(this.createMember(memberId, `T${i}`, `T${i + 1}`));
        }
    }

    /**
     * Generate bottom chord nodes and members
     */
    private generateBottomChord() {
        // Create bottom chord nodes
        for (let i = 0; i <= this.panelCount; i++) {
            const x = (i * this.spanLength) / this.panelCount;
            const nodeId = `B${i}`;
            this.entities.push(this.createNode(nodeId, x, 0));
        }

        // Create bottom chord members
        for (let i = 0; i < this.panelCount; i++) {
            const memberId = `BOT${i}`;
            this.entities.push(this.createMember(memberId, `B${i}`, `B${i + 1}`));
        }
    }

    /**
     * Generate diagonal web members (Warren truss pattern)
     */
    private generateWebMembers() {
        for (let i = 0; i < this.panelCount; i++) {
            // Diagonal going up-right
            if (i % 2 === 0) {
                const memberId = `WEB${i}_UR`;
                this.entities.push(this.createMember(memberId, `B${i}`, `T${i + 1}`));
            } else {
                // Diagonal going up-left  
                const memberId = `WEB${i}_UL`;
                this.entities.push(this.createMember(memberId, `B${i + 1}`, `T${i}`));
            }
        }
    }

    /**
     * Generate vertical members at quarter points
     */
    private generateVerticalMembers() {
        // Add verticals at quarter spans for additional stiffness
        const quarterPoints = [
            Math.floor(this.panelCount / 4),
            Math.floor(this.panelCount / 2),
            Math.floor(3 * this.panelCount / 4)
        ];

        quarterPoints.forEach((panel, index) => {
            if (panel > 0 && panel < this.panelCount) {
                const memberId = `VERT${index}`;
                this.entities.push(this.createMember(memberId, `B${panel}`, `T${panel}`));
            }
        });
    }

    /**
     * Add boundary conditions (support constraints)
     */
    private addBoundaryConditions() {
        // Fix left support (pin)
        this.addFixedConstraint('B0');
        
        // Add roller support at right end (constrain vertically but allow horizontal movement)
        // For now, we'll fix it - in a more advanced version we'd add horizontal/vertical constraints separately
        this.addFixedConstraint(`B${this.panelCount}`);
    }

    /**
     * Add parametric constraints to maintain geometric relationships
     */
    private addParametricConstraints() {
        // Ensure all panels have equal length
        for (let i = 0; i < this.panelCount; i++) {
            this.addDistanceConstraint([`B${i}`, `B${i + 1}`], '$panel_length');
            this.addDistanceConstraint([`T${i}`, `T${i + 1}`], '$panel_length');
        }

        // Ensure constant bridge height
        for (let i = 0; i <= this.panelCount; i++) {
            this.addDistanceConstraint([`B${i}`, `T${i}`], '$bridge_height');
        }

        // Constrain diagonal lengths (this creates the truss geometry)
        for (let i = 0; i < this.panelCount; i++) {
            if (i % 2 === 0) {
                // Up-right diagonal
                this.addDistanceConstraint([`B${i}`, `T${i + 1}`], '$member_length_diagonal');
            } else {
                // Up-left diagonal
                this.addDistanceConstraint([`B${i + 1}`, `T${i}`], '$member_length_diagonal');
            }
        }
    }
}

/**
 * Generate multiple bridge configurations
 */
function generateBridgeVariations() {
    const configurations = [
        { panels: 4, span: 2000, height: 400, name: "small_bridge" },
        { panels: 6, span: 3000, height: 500, name: "medium_bridge" },
        { panels: 8, span: 4000, height: 600, name: "large_bridge" }
    ];

    configurations.forEach(config => {
        console.log(`\\n=== Generating ${config.name} ===`);
        console.log(`Panels: ${config.panels}, Span: ${config.span}mm, Height: ${config.height}mm`);
        
        const generator = new ParametricTrussGenerator(config.panels, config.span, config.height);
        const document = generator.generate();
        
        console.log(`Generated ${document.entities.length} entities and ${document.constraints.length} constraints`);
        
        // Write to file
        const fs = require('fs');
        const filename = `generated_${config.name}.json`;
        fs.writeFileSync(filename, JSON.stringify(document, null, 2));
        console.log(`Saved to ${filename}`);
    });
}

/**
 * Generate a complex mechanical linkage system
 */
function generateComplexLinkage(): InputDocument {
    console.log("\\n=== Generating Complex 6-Bar Linkage System ===");
    
    const entities: (PointEntity | LineEntity)[] = [];
    const constraints: (DistanceConstraint | FixedConstraint)[] = [];
    
    // Create a complex 6-bar linkage with branching
    const points = [
        { id: "ground_A", x: 0, y: 0 },
        { id: "ground_B", x: 200, y: 0 },
        { id: "joint_P1", x: 50, y: 80 },
        { id: "joint_P2", x: 150, y: 80 },
        { id: "joint_P3", x: 100, y: 160 },
        { id: "output_point", x: 180, y: 120 }
    ];
    
    // Add all points
    points.forEach(p => {
        entities.push({
            type: "point",
            id: p.id,
            at: [p.x, p.y, 0]
        });
    });
    
    // Define the linkage structure
    const links = [
        { id: "ground_link", p1: "ground_A", p2: "ground_B" },
        { id: "input_crank", p1: "ground_A", p2: "joint_P1" },
        { id: "coupler_1", p1: "joint_P1", p2: "joint_P3" },
        { id: "coupler_2", p1: "joint_P2", p2: "joint_P3" },
        { id: "output_rocker", p1: "ground_B", p2: "joint_P2" },
        { id: "branch_link", p1: "joint_P2", p2: "output_point" }
    ];
    
    // Add all links
    links.forEach(link => {
        entities.push({
            type: "line",
            id: link.id,
            p1: link.p1,
            p2: link.p2
        });
    });
    
    // Add constraints
    constraints.push({ type: "fixed", entity: "ground_A" });
    constraints.push({ type: "fixed", entity: "ground_B" });
    constraints.push({ type: "distance", between: ["ground_A", "ground_B"], value: "$ground_distance" });
    constraints.push({ type: "distance", between: ["ground_A", "joint_P1"], value: "$input_crank_length" });
    constraints.push({ type: "distance", between: ["joint_P1", "joint_P3"], value: "$coupler1_length" });
    constraints.push({ type: "distance", between: ["joint_P2", "joint_P3"], value: "$coupler2_length" });
    constraints.push({ type: "distance", between: ["ground_B", "joint_P2"], value: "$output_rocker_length" });
    constraints.push({ type: "distance", between: ["joint_P2", "output_point"], value: "$branch_length" });
    
    return {
        schema: "slvs-json/1",
        units: "mm",
        parameters: {
            ground_distance: 200,
            input_crank_length: 60,
            coupler1_length: 120,
            coupler2_length: 100,
            output_rocker_length: 80,
            branch_length: 90
        },
        entities,
        constraints
    };
}

// Main execution
if (require.main === module) {
    console.log("🏗️  SLVSX Advanced Constraint Generator");
    console.log("=====================================");
    
    // Generate bridge variations
    generateBridgeVariations();
    
    // Generate complex linkage
    const linkage = generateComplexLinkage();
    require('fs').writeFileSync('generated_complex_linkage.json', JSON.stringify(linkage, null, 2));
    console.log(`Generated complex linkage with ${linkage.entities.length} entities and ${linkage.constraints.length} constraints`);
    console.log("Saved to generated_complex_linkage.json");
    
    console.log("\\n✅ Generation complete! Test with:");
    console.log("   slvsx solve generated_complex_linkage.json");
    console.log("   slvsx export generated_complex_linkage.json --format svg --output linkage.svg");
}