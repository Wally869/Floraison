//! Drepanium pattern generator
//!
//! Drepanium (Scorpioid cyme): Single branch per node, spiraling one direction.
//! Blooming pattern: Determinate (center/top flowers bloom first).
//!
//! Examples: Forget-me-not, Heliotrope

use floraison_core::math::curves::AxisCurve;
use glam::{Quat, Vec3};

use crate::{BranchPoint, InflorescenceParams};

/// Helper structure for recursive branch building
#[derive(Debug, Clone)]
struct BranchNode {
    position: Vec3,
    direction: Vec3,
    length: f32,
    depth: usize,
}

/// Generate branch points for a drepanium (scorpioid cyme) pattern
///
/// # Arguments
/// * `params` - Inflorescence parameters
/// * `axis` - The main axis curve (drepanium uses only the top point)
///
/// # Returns
/// Vector of branch points, each representing a flower attachment location
///
/// # Pattern Characteristics
/// - Recursive single-branch pattern (scorpioid/helical)
/// - Each node has exactly ONE child branch
/// - Spirals in one direction creating curved/coiled appearance
/// - **Determinate**: Top/center flowers oldest
/// - Depth controlled by `params.recursion_depth` (default: 5)
/// - Branch ratio: child length = parent × ratio (default: 0.8)
/// - Spiral angle from `params.rotation_angle` (default: 137.5°)
pub fn generate_branch_points(
    params: &InflorescenceParams,
    axis: &AxisCurve,
) -> Vec<BranchPoint> {
    // Extract parameters with defaults
    let max_depth = params.recursion_depth.unwrap_or(5);
    let branch_ratio = params.branch_ratio.unwrap_or(0.8);
    let spiral_angle = params.rotation_angle; // Golden angle by default

    // Start from top of axis (determinate)
    let sample = axis.sample_at_t(1.0);
    let root = BranchNode {
        position: sample.position,
        direction: sample.normal,
        length: params.branch_length_top,
        depth: 0,
    };

    // Build spiral recursively
    let nodes = build_spiral_recursive(&root, max_depth, branch_ratio, spiral_angle);

    // Convert nodes to branch points
    nodes_to_branch_points(nodes, max_depth, params)
}

/// Recursively build spiral branch structure
fn build_spiral_recursive(
    node: &BranchNode,
    max_depth: usize,
    branch_ratio: f32,
    spiral_angle: f32,
) -> Vec<BranchNode> {
    // Base case: reached maximum depth
    if node.depth >= max_depth {
        return vec![node.clone()];
    }

    // Calculate next branch position
    let branch_end = node.position + node.direction * node.length;

    // Compute child direction (spiral rotation)
    // Rotate around the parent direction axis
    let rotation = Quat::from_axis_angle(node.direction.normalize(), spiral_angle.to_radians());

    // Also apply a slight downward tilt for natural droop
    let perpendicular = if node.direction.y.abs() < 0.9 {
        Vec3::Y.cross(node.direction).normalize()
    } else {
        Vec3::X.cross(node.direction).normalize()
    };
    let tilt_rotation = Quat::from_axis_angle(perpendicular, -15.0_f32.to_radians());

    let child_dir = (tilt_rotation * rotation * node.direction).normalize();

    // Create child node
    let child = BranchNode {
        position: branch_end,
        direction: child_dir,
        length: node.length * branch_ratio,
        depth: node.depth + 1,
    };

    // Recurse on child (single branch only - scorpioid)
    let mut result = vec![node.clone()];
    result.extend(build_spiral_recursive(&child, max_depth, branch_ratio, spiral_angle));

    result
}

/// Convert branch nodes to branch points with age information
fn nodes_to_branch_points(
    nodes: Vec<BranchNode>,
    max_depth: usize,
    params: &InflorescenceParams,
) -> Vec<BranchPoint> {
    nodes
        .into_iter()
        .map(|node| {
            // Calculate flower position at end of branch
            let position = node.position + node.direction * node.length;

            // Age: Determinate (top/center = oldest)
            // depth=0 (root) -> age=1.0
            // depth=max -> age=0.0
            let age = if max_depth > 0 {
                1.0 - (node.depth as f32 / max_depth as f32)
            } else {
                1.0
            };

            // Interpolate flower scale based on depth
            let t = node.depth as f32 / max_depth.max(1) as f32;
            let flower_scale = params.flower_size_top * (1.0 - t * 0.3); // Slightly smaller at tips

            BranchPoint {
                position,
                direction: node.direction,
                length: node.length,
                flower_scale,
                age,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drepanium_branch_count() {
        let params = InflorescenceParams {
            recursion_depth: Some(4),
            ..Default::default()
        };

        let axis = AxisCurve::new(vec![Vec3::ZERO, Vec3::new(0.0, 10.0, 0.0)]);
        let branches = generate_branch_points(&params, &axis);

        // Drepanium: linear chain, branch_count = depth + 1
        assert_eq!(branches.len(), 5, "depth=4 should give 5 branch points");
    }

    #[test]
    fn test_drepanium_age_determinate() {
        let params = InflorescenceParams {
            recursion_depth: Some(3),
            ..Default::default()
        };

        let axis = AxisCurve::new(vec![Vec3::ZERO, Vec3::new(0.0, 10.0, 0.0)]);
        let branches = generate_branch_points(&params, &axis);

        // Determinate: top/center oldest
        // First (depth=0) should have age=1.0
        assert!(
            (branches[0].age - 1.0).abs() < 1e-5,
            "Root should have age=1.0 (oldest)"
        );

        // Last (depth=3) should have age=0.0
        assert!(
            branches.last().unwrap().age.abs() < 1e-5,
            "Tip should have age=0.0 (youngest)"
        );

        // Age should decrease along spiral
        for i in 1..branches.len() {
            assert!(
                branches[i].age < branches[i - 1].age,
                "Age should decrease from root to tip"
            );
        }
    }

    #[test]
    fn test_drepanium_spiral_curvature() {
        let params = InflorescenceParams {
            recursion_depth: Some(5),
            rotation_angle: 90.0, // Exaggerated for testing
            ..Default::default()
        };

        let axis = AxisCurve::new(vec![Vec3::ZERO, Vec3::new(0.0, 10.0, 0.0)]);
        let branches = generate_branch_points(&params, &axis);

        // Positions should not all be collinear (should curve)
        // Check that there's variation in X or Z coordinates
        let has_x_variation = branches.iter().any(|b| b.position.x.abs() > 0.1);
        let has_z_variation = branches.iter().any(|b| b.position.z.abs() > 0.1);

        assert!(
            has_x_variation || has_z_variation,
            "Spiral should curve in XZ plane"
        );
    }

    #[test]
    fn test_drepanium_branch_ratio() {
        let params = InflorescenceParams {
            recursion_depth: Some(4),
            branch_ratio: Some(0.7),
            branch_length_top: 2.0,
            ..Default::default()
        };

        let axis = AxisCurve::new(vec![Vec3::ZERO, Vec3::new(0.0, 10.0, 0.0)]);
        let branches = generate_branch_points(&params, &axis);

        // First branch should have length ≈ 2.0
        assert!((branches[0].length - 2.0).abs() < 0.1);

        // Each subsequent branch should be ~70% of previous
        for i in 1..branches.len() {
            let expected_ratio = branches[i].length / branches[i - 1].length;
            assert!(
                (expected_ratio - 0.7).abs() < 0.1,
                "Branch ratio should be ~0.7, got {}",
                expected_ratio
            );
        }
    }

    #[test]
    fn test_drepanium_depth_zero() {
        let params = InflorescenceParams {
            recursion_depth: Some(0),
            ..Default::default()
        };

        let axis = AxisCurve::new(vec![Vec3::ZERO, Vec3::new(0.0, 10.0, 0.0)]);
        let branches = generate_branch_points(&params, &axis);

        // Depth 0 should give single branch point
        assert_eq!(branches.len(), 1);
        assert_eq!(branches[0].age, 1.0);
    }

    #[test]
    fn test_drepanium_depth_one() {
        let params = InflorescenceParams {
            recursion_depth: Some(1),
            ..Default::default()
        };

        let axis = AxisCurve::new(vec![Vec3::ZERO, Vec3::new(0.0, 10.0, 0.0)]);
        let branches = generate_branch_points(&params, &axis);

        // Depth 1 should give 2 branch points (root + 1 child)
        assert_eq!(branches.len(), 2);
    }

    #[test]
    fn test_drepanium_direction_normalized() {
        let params = InflorescenceParams {
            recursion_depth: Some(3),
            ..Default::default()
        };

        let axis = AxisCurve::new(vec![Vec3::ZERO, Vec3::new(0.0, 10.0, 0.0)]);
        let branches = generate_branch_points(&params, &axis);

        for branch in &branches {
            let len = branch.direction.length();
            assert!(
                (len - 1.0).abs() < 1e-3,
                "Direction should be normalized, got length {}",
                len
            );
        }
    }

    #[test]
    fn test_drepanium_default_params() {
        let params = InflorescenceParams::default();
        let axis = AxisCurve::new(vec![Vec3::ZERO, Vec3::new(0.0, 10.0, 0.0)]);

        let branches = generate_branch_points(&params, &axis);

        // With default recursion_depth=None, should use 5
        assert_eq!(branches.len(), 6, "Default depth should be 5");
    }

    #[test]
    fn test_drepanium_flower_scale_progression() {
        let params = InflorescenceParams {
            recursion_depth: Some(4),
            flower_size_top: 1.0,
            ..Default::default()
        };

        let axis = AxisCurve::new(vec![Vec3::ZERO, Vec3::new(0.0, 10.0, 0.0)]);
        let branches = generate_branch_points(&params, &axis);

        // Flower scale should decrease slightly along spiral
        for i in 1..branches.len() {
            assert!(
                branches[i].flower_scale <= branches[i - 1].flower_scale,
                "Flower scale should decrease or stay same"
            );
        }
    }
}
