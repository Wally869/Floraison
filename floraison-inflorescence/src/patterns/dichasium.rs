//! Dichasium pattern generator
//!
//! Dichasium: Recursive Y-shaped branching with two opposite branches at each node.
//! Blooming pattern: Determinate (center/top flowers bloom first).
//!
//! Examples: Carnations, Sweet William

use floraison_core::math::curves::AxisCurve;
use glam::{Quat, Vec3};

use crate::{apply_age_distribution, BranchPoint, InflorescenceParams};

/// Helper structure for recursive branch building
#[derive(Debug, Clone)]
struct BranchNode {
    position: Vec3,
    direction: Vec3,
    length: f32,
    depth: usize,
}

/// Generate branch points for a dichasium pattern
///
/// # Arguments
/// * `params` - Inflorescence parameters
/// * `axis` - The main axis curve (dichasium uses only the top point)
///
/// # Returns
/// Vector of branch points, each representing a flower attachment location
///
/// # Pattern Characteristics
/// - Recursive Y-shaped branching (two branches per node)
/// - Forms binary tree structure
/// - **Determinate**: Top/center flowers oldest
/// - Depth controlled by `params.recursion_depth` (default: 3)
/// - Branch ratio: child length = parent × ratio (default: 0.7)
/// - Angle divergence: angle between Y-branches (default: 30°)
pub fn generate_branch_points(
    params: &InflorescenceParams,
    axis: &AxisCurve,
) -> Vec<BranchPoint> {
    // Extract parameters with defaults
    let max_depth = params.recursion_depth.unwrap_or(1);
    let branch_ratio = params.branch_ratio.unwrap_or(0.7);
    let angle_div = params.angle_divergence.unwrap_or(30.0);

    // Start from top of axis (determinate)
    let sample = axis.sample_at_t(1.0);
    let root = BranchNode {
        position: sample.position,
        direction: sample.normal,
        length: params.branch_length_top,
        depth: 0,
    };

    // Use binormal from Frenet frame for consistent branching plane
    let branching_axis = sample.binormal;

    // Build binary tree recursively
    let nodes = build_tree_recursive(&root, max_depth, branch_ratio, angle_div, branching_axis);

    // Convert nodes to branch points
    nodes_to_branch_points(nodes, max_depth, params)
}

/// Recursively build binary tree structure
fn build_tree_recursive(
    node: &BranchNode,
    max_depth: usize,
    branch_ratio: f32,
    angle_divergence: f32,
    branching_axis: Vec3,
) -> Vec<BranchNode> {
    // Base case: reached maximum depth (leaf node)
    if node.depth >= max_depth {
        return vec![node.clone()];
    }

    // Calculate branch endpoint
    let branch_end = node.position + node.direction * node.length;

    // Use fixed branching axis from Frenet frame (consistent branching plane)
    let perpendicular = branching_axis;

    // Left branch: rotate +angle_divergence around perpendicular
    let left_rotation = Quat::from_axis_angle(perpendicular, angle_divergence.to_radians());
    let left_dir = (left_rotation * node.direction).normalize();

    let left_child = BranchNode {
        position: branch_end,
        direction: left_dir,
        length: node.length * branch_ratio,
        depth: node.depth + 1,
    };

    // Right branch: rotate -angle_divergence around perpendicular
    let right_rotation = Quat::from_axis_angle(perpendicular, -angle_divergence.to_radians());
    let right_dir = (right_rotation * node.direction).normalize();

    let right_child = BranchNode {
        position: branch_end,
        direction: right_dir,
        length: node.length * branch_ratio,
        depth: node.depth + 1,
    };

    // Recurse on both children
    let mut result = vec![node.clone()];
    result.extend(build_tree_recursive(&left_child, max_depth, branch_ratio, angle_divergence, branching_axis));
    result.extend(build_tree_recursive(&right_child, max_depth, branch_ratio, angle_divergence, branching_axis));

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
            let base_age = if max_depth > 0 {
                1.0 - (node.depth as f32 / max_depth as f32)
            } else {
                1.0
            };
            let age = apply_age_distribution(base_age, params.age_distribution);

            // Interpolate flower scale based on depth
            let t = node.depth as f32 / max_depth.max(1) as f32;
            let flower_scale = params.flower_size_top * (1.0 - t * 0.4); // Smaller at periphery

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
    fn test_dichasium_branch_count() {
        let params = InflorescenceParams {
            recursion_depth: Some(3),
            ..Default::default()
        };

        let axis = AxisCurve::new(vec![Vec3::ZERO, Vec3::new(0.0, 10.0, 0.0)]);
        let branches = generate_branch_points(&params, &axis);

        // Binary tree: (2^(depth+1) - 1) nodes total
        // depth=3: 2^4 - 1 = 15 nodes
        assert_eq!(branches.len(), 15, "depth=3 should give 15 nodes");
    }

    #[test]
    fn test_dichasium_depth_counts() {
        // Test various depths
        let test_cases = vec![
            (0, 1),   // 2^1 - 1 = 1
            (1, 3),   // 2^2 - 1 = 3
            (2, 7),   // 2^3 - 1 = 7
            (3, 15),  // 2^4 - 1 = 15
            (4, 31),  // 2^5 - 1 = 31
        ];

        for (depth, expected_count) in test_cases {
            let params = InflorescenceParams {
                recursion_depth: Some(depth),
                ..Default::default()
            };
            let axis = AxisCurve::new(vec![Vec3::ZERO, Vec3::new(0.0, 10.0, 0.0)]);
            let branches = generate_branch_points(&params, &axis);

            assert_eq!(
                branches.len(),
                expected_count,
                "depth={} should give {} nodes",
                depth,
                expected_count
            );
        }
    }

    #[test]
    fn test_dichasium_age_determinate() {
        let params = InflorescenceParams {
            recursion_depth: Some(2),
            ..Default::default()
        };

        let axis = AxisCurve::new(vec![Vec3::ZERO, Vec3::new(0.0, 10.0, 0.0)]);
        let branches = generate_branch_points(&params, &axis);

        // First (depth=0, root) should have age=1.0
        assert!(
            (branches[0].age - 1.0).abs() < 1e-5,
            "Root should have age=1.0 (oldest)"
        );

        // Check that leaves (depth=2) have age=0.0
        let leaves: Vec<_> = branches.iter().filter(|b| b.age.abs() < 1e-5).collect();
        assert_eq!(leaves.len(), 4, "Should have 4 leaves at depth=2");
    }

    #[test]
    fn test_dichasium_branch_divergence() {
        let params = InflorescenceParams {
            recursion_depth: Some(1),
            angle_divergence: Some(45.0),
            branch_length_top: 1.0,
            ..Default::default()
        };

        let axis = AxisCurve::new(vec![Vec3::ZERO, Vec3::new(0.0, 10.0, 0.0)]);
        let branches = generate_branch_points(&params, &axis);

        // Should have 3 branches (root + 2 children)
        assert_eq!(branches.len(), 3);

        // Children should diverge from parent direction
        let parent_dir = branches[0].direction;
        let child1_dir = branches[1].direction;
        let child2_dir = branches[2].direction;

        // Angle between parent and each child
        let angle1 = parent_dir.dot(child1_dir).acos().to_degrees();
        let angle2 = parent_dir.dot(child2_dir).acos().to_degrees();

        // Should be approximately angle_divergence (45°)
        assert!(
            (angle1 - 45.0).abs() < 5.0,
            "Child 1 should diverge ~45°, got {}°",
            angle1
        );
        assert!(
            (angle2 - 45.0).abs() < 5.0,
            "Child 2 should diverge ~45°, got {}°",
            angle2
        );

        // Children should diverge in opposite directions
        // Angle between the two children should be ~90° (2× divergence)
        let angle_between = child1_dir.dot(child2_dir).acos().to_degrees();
        assert!(
            (angle_between - 90.0).abs() < 10.0,
            "Children should be ~90° apart, got {}°",
            angle_between
        );
    }

    #[test]
    fn test_dichasium_branch_ratio() {
        let params = InflorescenceParams {
            recursion_depth: Some(2),
            branch_ratio: Some(0.6),
            branch_length_top: 3.0,
            ..Default::default()
        };

        let axis = AxisCurve::new(vec![Vec3::ZERO, Vec3::new(0.0, 10.0, 0.0)]);
        let branches = generate_branch_points(&params, &axis);

        // Root should have length 3.0
        assert!((branches[0].length - 3.0).abs() < 0.1);

        // Depth 1 should have length 3.0 * 0.6 = 1.8
        let depth1_branches: Vec<_> = branches.iter().filter(|b| (b.age - 0.5).abs() < 0.1).collect();
        for branch in depth1_branches {
            assert!(
                (branch.length - 1.8).abs() < 0.1,
                "Depth 1 should have length ~1.8, got {}",
                branch.length
            );
        }
    }

    #[test]
    fn test_dichasium_direction_normalized() {
        let params = InflorescenceParams {
            recursion_depth: Some(2),
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
    fn test_dichasium_y_structure() {
        let params = InflorescenceParams {
            recursion_depth: Some(1),
            angle_divergence: Some(30.0),
            ..Default::default()
        };

        let axis = AxisCurve::new(vec![Vec3::ZERO, Vec3::new(0.0, 10.0, 0.0)]);
        let branches = generate_branch_points(&params, &axis);

        // With depth=1, should have Y-shape: 1 root + 2 children
        assert_eq!(branches.len(), 3);

        // Children positions should be at different locations (diverging)
        let child1_pos = branches[1].position;
        let child2_pos = branches[2].position;

        let distance_apart = (child1_pos - child2_pos).length();
        assert!(
            distance_apart > 0.1,
            "Children should be spatially separated, got distance={}",
            distance_apart
        );
    }

    #[test]
    fn test_dichasium_default_params() {
        let params = InflorescenceParams::default();
        let axis = AxisCurve::new(vec![Vec3::ZERO, Vec3::new(0.0, 10.0, 0.0)]);

        let branches = generate_branch_points(&params, &axis);

        // With default recursion_depth=None, should use 3
        // 2^4 - 1 = 15 nodes
        assert_eq!(branches.len(), 15, "Default depth should be 3");
    }

    #[test]
    fn test_dichasium_flower_scale_progression() {
        let params = InflorescenceParams {
            recursion_depth: Some(2),
            flower_size_top: 1.0,
            ..Default::default()
        };

        let axis = AxisCurve::new(vec![Vec3::ZERO, Vec3::new(0.0, 10.0, 0.0)]);
        let branches = generate_branch_points(&params, &axis);

        // Root (depth=0) should have largest scale
        assert_eq!(branches[0].flower_scale, 1.0);

        // Leaves (depth=2) should have smallest scale
        let min_scale = branches.iter().map(|b| b.flower_scale).fold(f32::INFINITY, f32::min);
        assert!(min_scale < 1.0, "Leaf flowers should be smaller");
    }

    #[test]
    fn test_dichasium_positions_increase_outward() {
        let params = InflorescenceParams {
            recursion_depth: Some(2),
            ..Default::default()
        };

        let axis = AxisCurve::new(vec![Vec3::ZERO, Vec3::new(0.0, 10.0, 0.0)]);
        let branches = generate_branch_points(&params, &axis);

        // Root should be closest to axis origin
        let root_dist = branches[0].position.length();

        // Leaves should be farther from origin
        let max_dist = branches.iter().map(|b| b.position.length()).fold(0.0f32, f32::max);

        assert!(
            max_dist > root_dist,
            "Outer branches should be farther from center"
        );
    }
}
