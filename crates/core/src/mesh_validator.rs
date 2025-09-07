use std::collections::HashMap;

#[derive(Debug)]
pub struct MeshValidator {
    pub clearance: f64,  // Minimum clearance between teeth
}

impl Default for MeshValidator {
    fn default() -> Self {
        Self {
            clearance: 0.1,  // 0.1mm minimum clearance
        }
    }
}

#[derive(Debug)]
pub struct GearInfo {
    pub center: (f64, f64),
    pub teeth: u32,
    pub module: f64,
    pub phase: f64,
    pub internal: bool,
}

#[derive(Debug)]
pub struct MeshIssue {
    pub gear1: String,
    pub gear2: String,
    pub issue_type: IssueType,
    pub details: String,
}

#[derive(Debug)]
pub enum IssueType {
    TeethCollision,
    IncorrectDistance,
    PhaseAlignment,
}

impl MeshValidator {
    pub fn new(clearance: f64) -> Self {
        Self { clearance }
    }
    
    pub fn validate_mesh(
        &self,
        gears: &HashMap<String, GearInfo>,
        mesh_pairs: &[(String, String)],
    ) -> Vec<MeshIssue> {
        let mut issues = Vec::new();
        
        for (gear1_id, gear2_id) in mesh_pairs {
            if let (Some(gear1), Some(gear2)) = (gears.get(gear1_id), gears.get(gear2_id)) {
                // Check distance
                if let Some(issue) = self.check_distance(gear1_id, gear1, gear2_id, gear2) {
                    issues.push(issue);
                }
                
                // Check phase alignment
                if let Some(issue) = self.check_phase_alignment(gear1_id, gear1, gear2_id, gear2) {
                    issues.push(issue);
                }
            }
        }
        
        // Check for non-meshing collisions
        for (id1, gear1) in gears {
            for (id2, gear2) in gears {
                if id1 >= id2 {
                    continue;
                }
                
                // Skip if they're supposed to mesh
                let should_mesh = mesh_pairs.iter().any(|(a, b)| {
                    (a == id1 && b == id2) || (a == id2 && b == id1)
                });
                
                if !should_mesh {
                    if let Some(issue) = self.check_non_meshing_collision(id1, gear1, id2, gear2) {
                        issues.push(issue);
                    }
                }
            }
        }
        
        issues
    }
    
    fn check_distance(
        &self,
        id1: &str,
        gear1: &GearInfo,
        id2: &str,
        gear2: &GearInfo,
    ) -> Option<MeshIssue> {
        let dx = gear2.center.0 - gear1.center.0;
        let dy = gear2.center.1 - gear1.center.1;
        let actual_distance = (dx * dx + dy * dy).sqrt();
        
        let pitch_r1 = (gear1.teeth as f64 * gear1.module) / 2.0;
        let pitch_r2 = (gear2.teeth as f64 * gear2.module) / 2.0;
        
        let expected_distance = if gear1.internal || gear2.internal {
            (pitch_r1 - pitch_r2).abs()
        } else {
            pitch_r1 + pitch_r2
        };
        
        let error = (actual_distance - expected_distance).abs();
        
        if error > self.clearance {
            Some(MeshIssue {
                gear1: id1.to_string(),
                gear2: id2.to_string(),
                issue_type: IssueType::IncorrectDistance,
                details: format!(
                    "Distance {:.2}mm, expected {:.2}mm (error: {:.2}mm)",
                    actual_distance, expected_distance, error
                ),
            })
        } else {
            None
        }
    }
    
    fn check_phase_alignment(
        &self,
        id1: &str,
        gear1: &GearInfo,
        id2: &str,
        gear2: &GearInfo,
    ) -> Option<MeshIssue> {
        // Calculate the angle between gear centers
        let dx = gear2.center.0 - gear1.center.0;
        let dy = gear2.center.1 - gear1.center.1;
        let angle_rad = dy.atan2(dx);
        let angle_deg = angle_rad.to_degrees();
        
        // Calculate expected phase difference
        let gear_ratio = gear1.teeth as f64 / gear2.teeth as f64;
        
        // For proper meshing, the phase relationship should be:
        // phase2 = phase1 - angle * gear_ratio (for external)
        // phase2 = phase1 + angle * gear_ratio (for internal)
        let expected_phase2 = if gear1.internal != gear2.internal {
            gear1.phase + angle_deg * gear_ratio
        } else {
            gear1.phase - angle_deg * gear_ratio + 180.0
        };
        
        // Normalize phases to 0-360 range
        let normalized_expected = expected_phase2 % 360.0;
        let normalized_actual = gear2.phase % 360.0;
        
        // Calculate phase error (considering tooth periodicity)
        let tooth_angle1 = 360.0 / gear1.teeth as f64;
        let tooth_angle2 = 360.0 / gear2.teeth as f64;
        let max_tooth_angle = tooth_angle1.max(tooth_angle2);
        
        let mut phase_error = (normalized_actual - normalized_expected).abs();
        if phase_error > 180.0 {
            phase_error = 360.0 - phase_error;
        }
        
        // Check if phase error is more than half a tooth
        if phase_error > max_tooth_angle / 2.0 {
            Some(MeshIssue {
                gear1: id1.to_string(),
                gear2: id2.to_string(),
                issue_type: IssueType::PhaseAlignment,
                details: format!(
                    "Phase error {:.1}° (exceeds {:.1}° tooth spacing)",
                    phase_error, max_tooth_angle / 2.0
                ),
            })
        } else {
            None
        }
    }
    
    fn check_non_meshing_collision(
        &self,
        id1: &str,
        gear1: &GearInfo,
        id2: &str,
        gear2: &GearInfo,
    ) -> Option<MeshIssue> {
        let dx = gear2.center.0 - gear1.center.0;
        let dy = gear2.center.1 - gear1.center.1;
        let center_distance = (dx * dx + dy * dy).sqrt();
        
        // Calculate outer radii (including teeth)
        let addendum = gear1.module;  // Standard addendum
        let outer_r1 = if gear1.internal {
            (gear1.teeth as f64 * gear1.module) / 2.0 - addendum
        } else {
            (gear1.teeth as f64 * gear1.module) / 2.0 + addendum
        };
        
        let outer_r2 = if gear2.internal {
            (gear2.teeth as f64 * gear2.module) / 2.0 - addendum
        } else {
            (gear2.teeth as f64 * gear2.module) / 2.0 + addendum
        };
        
        // Check if gears overlap
        let min_safe_distance = outer_r1 + outer_r2 + self.clearance;
        
        if center_distance < min_safe_distance {
            Some(MeshIssue {
                gear1: id1.to_string(),
                gear2: id2.to_string(),
                issue_type: IssueType::TeethCollision,
                details: format!(
                    "Gears too close: {:.2}mm < {:.2}mm minimum",
                    center_distance, min_safe_distance
                ),
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_correct_mesh() {
        let mut gears = HashMap::new();
        gears.insert("sun".to_string(), GearInfo {
            center: (0.0, 0.0),
            teeth: 24,
            module: 2.0,
            phase: 0.0,
            internal: false,
        });
        gears.insert("planet".to_string(), GearInfo {
            center: (36.0, 0.0),  // Correct distance: (24+12)*2/2 = 36
            teeth: 12,
            module: 2.0,
            phase: 0.0,
            internal: false,
        });
        
        let validator = MeshValidator::default();
        let issues = validator.validate_mesh(&gears, &[("sun".to_string(), "planet".to_string())]);
        
        assert!(issues.is_empty(), "Should have no issues for correct mesh");
    }
    
    #[test]
    fn test_incorrect_distance() {
        let mut gears = HashMap::new();
        gears.insert("sun".to_string(), GearInfo {
            center: (0.0, 0.0),
            teeth: 24,
            module: 2.0,
            phase: 0.0,
            internal: false,
        });
        gears.insert("planet".to_string(), GearInfo {
            center: (30.0, 0.0),  // Wrong distance
            teeth: 12,
            module: 2.0,
            phase: 0.0,
            internal: false,
        });
        
        let validator = MeshValidator::default();
        let issues = validator.validate_mesh(&gears, &[("sun".to_string(), "planet".to_string())]);
        
        assert!(!issues.is_empty(), "Should detect incorrect distance");
        assert!(matches!(issues[0].issue_type, IssueType::IncorrectDistance));
    }
}