// SLVSX Visualizer - TypeScript/JavaScript visualization for constraint solutions

class GearSystem {
    constructor() {
        this.canvas = document.getElementById('canvas');
        this.ctx = this.canvas.getContext('2d');
        this.constraints = null;
        this.solution = null;
    }

    generateConstraints() {
        const systemType = document.getElementById('systemType').value;
        const sunTeeth = parseInt(document.getElementById('sunTeeth').value);
        const planetTeeth = parseInt(document.getElementById('planetTeeth').value);
        const ringTeeth = parseInt(document.getElementById('ringTeeth').value);
        const numPlanets = parseInt(document.getElementById('numPlanets').value);
        const module = parseFloat(document.getElementById('module').value);

        // Validate assembly constraint
        if (systemType === 'planetary' || systemType === 'double') {
            if ((sunTeeth + ringTeeth) % numPlanets !== 0) {
                this.showError(`Assembly constraint failed: (${sunTeeth} + ${ringTeeth}) / ${numPlanets} must be integer`);
                return null;
            }
        }

        let constraints = {
            schema: "slvs-json/1",
            units: "mm",
            parameters: {
                module: module,
                pressure_angle: 20.0,
                sun_teeth: sunTeeth,
                planet_teeth: planetTeeth,
                ring_teeth: ringTeeth
            },
            entities: [],
            constraints: []
        };

        // Add sun gear
        constraints.entities.push({
            type: "gear",
            id: "sun",
            center: [0, 0, 0],
            teeth: "$sun_teeth",
            module: "$module",
            pressure_angle: "$pressure_angle",
            phase: 0.0,
            internal: false
        });

        if (systemType === 'planetary' || systemType === 'double') {
            // Add ring gear
            constraints.entities.push({
                type: "gear",
                id: "ring",
                center: [0, 0, 0],
                teeth: "$ring_teeth",
                module: "$module",
                pressure_angle: "$pressure_angle",
                phase: 0.0,
                internal: true
            });

            // Add planets
            const orbitRadius = (sunTeeth + planetTeeth) * module / 2;
            for (let i = 0; i < numPlanets; i++) {
                const angle = (i * 360 / numPlanets) * Math.PI / 180;
                const x = orbitRadius * Math.cos(angle);
                const y = orbitRadius * Math.sin(angle);

                constraints.entities.push({
                    type: "gear",
                    id: `planet${i + 1}`,
                    center: [x, y, 0],
                    teeth: "$planet_teeth",
                    module: "$module",
                    pressure_angle: "$pressure_angle",
                    phase: 0.0,
                    internal: false
                });

                // Add mesh constraints
                constraints.constraints.push({
                    type: "mesh",
                    gear1: "sun",
                    gear2: `planet${i + 1}`
                });

                constraints.constraints.push({
                    type: "mesh",
                    gear1: "ring",
                    gear2: `planet${i + 1}`
                });
            }
        }

        this.constraints = constraints;
        this.showSuccess('Constraints generated successfully!');
        
        // Display JSON
        document.getElementById('output').textContent = JSON.stringify(constraints, null, 2);
        
        return constraints;
    }

    async callSolver(constraints) {
        // In a real implementation, this would call the SLVSX CLI
        // For demo purposes, we'll simulate a solution
        return this.simulateSolution(constraints);
    }

    simulateSolution(constraints) {
        // Simulate solver output
        let solution = {
            status: "success",
            entities: {}
        };

        // Copy entity positions from constraints
        for (let entity of constraints.entities) {
            solution.entities[entity.id] = {
                center: entity.center,
                teeth: this.resolveParameter(entity.teeth, constraints.parameters),
                module: this.resolveParameter(entity.module, constraints.parameters),
                phase: entity.phase || 0,
                internal: entity.internal || false
            };
        }

        return solution;
    }

    resolveParameter(value, parameters) {
        if (typeof value === 'string' && value.startsWith('$')) {
            const paramName = value.substring(1);
            return parameters[paramName];
        }
        return value;
    }

    visualize() {
        if (!this.constraints) {
            this.constraints = this.generateConstraints();
            if (!this.constraints) return;
        }

        const solution = this.simulateSolution(this.constraints);
        this.drawGearSystem(solution);
    }

    drawGearSystem(solution) {
        const ctx = this.ctx;
        const width = this.canvas.width;
        const height = this.canvas.height;
        
        // Clear canvas
        ctx.clearRect(0, 0, width, height);
        
        // Set up transformation to center coordinate system
        ctx.save();
        ctx.translate(width / 2, height / 2);
        ctx.scale(2, -2); // Scale up and flip Y axis
        
        // Draw each gear
        for (let [id, gear] of Object.entries(solution.entities)) {
            this.drawGear(gear, id);
        }
        
        ctx.restore();
        
        this.showSuccess('Visualization complete!');
    }

    drawGear(gear, id) {
        const ctx = this.ctx;
        const x = gear.center[0];
        const y = gear.center[1];
        const pitchRadius = (gear.teeth * gear.module) / 2;
        const toothHeight = gear.module;
        
        ctx.save();
        ctx.translate(x, y);
        ctx.rotate(gear.phase * Math.PI / 180);
        
        if (gear.internal) {
            // Draw ring gear (internal)
            ctx.strokeStyle = '#764ba2';
            ctx.lineWidth = 0.5;
            
            // Outer circle
            ctx.beginPath();
            ctx.arc(0, 0, pitchRadius + toothHeight * 2, 0, 2 * Math.PI);
            ctx.stroke();
            
            // Draw internal teeth
            this.drawInternalTeeth(ctx, gear.teeth, pitchRadius, toothHeight);
        } else {
            // Draw external gear
            ctx.strokeStyle = '#667eea';
            ctx.lineWidth = 0.5;
            
            // Draw teeth
            this.drawExternalTeeth(ctx, gear.teeth, pitchRadius, toothHeight);
            
            // Center hole
            ctx.beginPath();
            ctx.arc(0, 0, pitchRadius * 0.2, 0, 2 * Math.PI);
            ctx.stroke();
        }
        
        // Label
        ctx.restore();
        ctx.save();
        ctx.translate(x, -y); // Flip Y back for text
        ctx.scale(0.5, -0.5);
        ctx.fillStyle = '#333';
        ctx.font = '12px Arial';
        ctx.textAlign = 'center';
        ctx.fillText(id, 0, 0);
        ctx.restore();
    }

    drawExternalTeeth(ctx, numTeeth, pitchRadius, toothHeight) {
        const toothAngle = (2 * Math.PI) / numTeeth;
        const outerRadius = pitchRadius + toothHeight;
        const innerRadius = pitchRadius - toothHeight * 0.5;
        
        ctx.beginPath();
        for (let i = 0; i < numTeeth; i++) {
            const angle = i * toothAngle;
            const nextAngle = (i + 1) * toothAngle;
            const midAngle = angle + toothAngle / 2;
            
            // Tooth tip
            ctx.lineTo(
                outerRadius * Math.cos(angle + toothAngle * 0.2),
                outerRadius * Math.sin(angle + toothAngle * 0.2)
            );
            ctx.lineTo(
                outerRadius * Math.cos(angle + toothAngle * 0.3),
                outerRadius * Math.sin(angle + toothAngle * 0.3)
            );
            
            // Tooth valley
            ctx.lineTo(
                innerRadius * Math.cos(midAngle),
                innerRadius * Math.sin(midAngle)
            );
        }
        ctx.closePath();
        ctx.stroke();
    }

    drawInternalTeeth(ctx, numTeeth, pitchRadius, toothHeight) {
        const toothAngle = (2 * Math.PI) / numTeeth;
        const outerRadius = pitchRadius - toothHeight;
        const innerRadius = pitchRadius;
        
        ctx.beginPath();
        for (let i = 0; i < numTeeth; i++) {
            const angle = i * toothAngle;
            const midAngle = angle + toothAngle / 2;
            
            // Valley
            ctx.lineTo(
                innerRadius * Math.cos(angle),
                innerRadius * Math.sin(angle)
            );
            
            // Tooth (pointing inward)
            ctx.lineTo(
                outerRadius * Math.cos(midAngle),
                outerRadius * Math.sin(midAngle)
            );
            
            ctx.lineTo(
                innerRadius * Math.cos(angle + toothAngle),
                innerRadius * Math.sin(angle + toothAngle)
            );
        }
        ctx.closePath();
        ctx.stroke();
    }

    exportSVG() {
        if (!this.constraints) {
            this.showError('Please generate constraints first');
            return;
        }

        // Generate SVG string
        const solution = this.simulateSolution(this.constraints);
        let svg = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="-200 -200 400 400" width="800" height="800">\n`;
        
        for (let [id, gear] of Object.entries(solution.entities)) {
            svg += this.gearToSVG(gear, id);
        }
        
        svg += '</svg>';
        
        // Download SVG
        const blob = new Blob([svg], { type: 'image/svg+xml' });
        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = 'gear_system.svg';
        a.click();
        URL.revokeObjectURL(url);
        
        this.showSuccess('SVG exported successfully!');
    }

    gearToSVG(gear, id) {
        const x = gear.center[0];
        const y = gear.center[1];
        const pitchRadius = (gear.teeth * gear.module) / 2;
        
        return `  <g id="${id}" transform="translate(${x}, ${y})">
    <circle cx="0" cy="0" r="${pitchRadius}" fill="none" stroke="black" stroke-width="0.5"/>
    <text x="0" y="0" text-anchor="middle" font-size="8">${id}</text>
  </g>\n`;
    }

    showError(message) {
        const output = document.getElementById('output');
        output.innerHTML = `<div class="error">${message}</div>`;
    }

    showSuccess(message) {
        const output = document.getElementById('output');
        output.innerHTML = `<div class="success">${message}</div>`;
    }
}

// Initialize
const gearSystem = new GearSystem();

// Global functions for button clicks
function generateConstraints() {
    gearSystem.generateConstraints();
}

function visualize() {
    gearSystem.visualize();
}

function exportSVG() {
    gearSystem.exportSVG();
}