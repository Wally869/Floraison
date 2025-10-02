//! Umbel pattern generator
//!
//! Umbel: All pedicels originate from single point (umbrella-like).
//! Blooming pattern: Determinate (all flowers same age).
//!
//! Examples: Onion, Carrot

use floraison_core::math::curves::AxisCurve;
use glam::Quat;

use crate::{apply_age_distribution, BranchPoint, InflorescenceParams};

/// Generate branch points for an umbel pattern
///
/// # Arguments
/// * `params` - Inflorescence parameters defining axis, branch count, angles, etc.
/// * `axis` - The main axis curve (umbel uses only the top point)
///
/// # Returns
/// Vector of branch points, each representing a flower attachment location
///
/// # Pattern Characteristics
/// - All branches originate from single point (top of axis)
/// - Spread out in umbrella shape
/// - Rotation angle determines angular spacing
/// - Down angle determines spread
/// - **Determinate**: All flowers same age (bloom together)
pub fn generate_branch_points(params: &InflorescenceParams, axis: &AxisCurve) -> Vec<BranchPoint> {
    let mut branches = Vec::with_capacity(params.branch_count);

    // All branches from top of axis
    let sample = axis.sample_at_t(1.0);

    for i in 0..params.branch_count {
        // Rotation around axis
        let rotation = params.rotation_angle * i as f32;

        // Compute direction (down and rotated around axis)
        let down_rotation = Quat::from_axis_angle(sample.binormal, -params.angle_top.to_radians());
        let spiral_rotation = Quat::from_axis_angle(sample.tangent, rotation.to_radians());
        let direction = (spiral_rotation * down_rotation * sample.normal).normalize();

        // Branch endpoint
        let position = sample.position + direction * params.branch_length_top;

        // Age: determinate (all flowers same age, bloom together)
        let base_age = 1.0;
        let age = apply_age_distribution(base_age, params.age_distribution);

        branches.push(BranchPoint {
            position,
            direction,
            length: params.branch_length_top,
            flower_scale: params.flower_size_top,
            age,
        });
    }

    branches
}

#[cfg(test)]
mod tests {
    use super::*;
    use floraison_core::Vec3;

    #[test]
    fn test_umbel_branch_count() {
        let params = InflorescenceParams {
            branch_count: 12,
            ..Default::default()
        };

        let axis = AxisCurve::new(vec![Vec3::ZERO, Vec3::new(0.0, 10.0, 0.0)]);
        let branches = generate_branch_points(&params, &axis);

        assert_eq!(branches.len(), 12);
    }

    #[test]
    fn test_umbel_single_origin() {
        let params = InflorescenceParams {
            branch_count: 8,
            axis_length: 10.0,
            branch_length_top: 2.0,
            angle_top: 45.0,
            ..Default::default()
        };

        let axis = AxisCurve::new(vec![Vec3::ZERO, Vec3::new(0.0, 10.0, 0.0)]);
        let branches = generate_branch_points(&params, &axis);

        // Calculate base positions (where branches attach to axis)
        // Base = position - direction * length
        for branch in &branches {
            let base = branch.position - branch.direction * branch.length;

            // All bases should be at same point (top of axis, y=10)
            assert!(
                (base.y - 10.0).abs() < 1e-3,
                "All branches should originate from top of axis (y=10), got y={}",
                base.y
            );

            // X and Z should be near 0 (on axis)
            assert!(
                base.x.abs() < 1e-3,
                "Branch base should be on axis (x≈0), got x={}",
                base.x
            );
            assert!(
                base.z.abs() < 1e-3,
                "Branch base should be on axis (z≈0), got z={}",
                base.z
            );
        }
    }

    #[test]
    fn test_umbel_same_age() {
        let params = InflorescenceParams {
            branch_count: 6,
            ..Default::default()
        };

        let axis = AxisCurve::new(vec![Vec3::ZERO, Vec3::new(0.0, 10.0, 0.0)]);
        let branches = generate_branch_points(&params, &axis);

        // All flowers should have same age (determinate)
        for branch in &branches {
            assert_eq!(
                branch.age, 1.0,
                "Umbel pattern is determinate: all flowers same age (1.0)"
            );
        }
    }

    #[test]
    fn test_umbel_same_length() {
        let params = InflorescenceParams {
            branch_count: 5,
            branch_length_top: 3.0,
            ..Default::default()
        };

        let axis = AxisCurve::new(vec![Vec3::ZERO, Vec3::new(0.0, 10.0, 0.0)]);
        let branches = generate_branch_points(&params, &axis);

        // All pedicels should have same length
        for branch in &branches {
            assert_eq!(
                branch.length, 3.0,
                "All umbel branches should have same length (from params.branch_length_top)"
            );
        }
    }

    #[test]
    fn test_umbel_same_flower_size() {
        let params = InflorescenceParams {
            branch_count: 4,
            flower_size_top: 1.5,
            ..Default::default()
        };

        let axis = AxisCurve::new(vec![Vec3::ZERO, Vec3::new(0.0, 10.0, 0.0)]);
        let branches = generate_branch_points(&params, &axis);

        // All flowers should have same size
        for branch in &branches {
            assert_eq!(
                branch.flower_scale, 1.5,
                "All umbel flowers should have same size (from params.flower_size_top)"
            );
        }
    }

    #[test]
    fn test_umbel_direction_normalized() {
        let params = InflorescenceParams {
            branch_count: 8,
            ..Default::default()
        };

        let axis = AxisCurve::new(vec![Vec3::ZERO, Vec3::new(0.0, 10.0, 0.0)]);
        let branches = generate_branch_points(&params, &axis);

        for branch in &branches {
            let length = branch.direction.length();
            assert!(
                (length - 1.0).abs() < 1e-5,
                "Direction should be normalized, got length {}",
                length
            );
        }
    }

    #[test]
    fn test_umbel_radial_spread() {
        let params = InflorescenceParams {
            branch_count: 4,
            rotation_angle: 90.0, // Evenly spaced at 90° intervals
            angle_top: 45.0,
            branch_length_top: 2.0,
            ..Default::default()
        };

        let axis = AxisCurve::new(vec![Vec3::ZERO, Vec3::new(0.0, 10.0, 0.0)]);
        let branches = generate_branch_points(&params, &axis);

        // Check that flowers spread radially
        // With 90° rotation, should get roughly 4 cardinal directions
        let radii: Vec<f32> = branches
            .iter()
            .map(|b| (b.position.x.powi(2) + b.position.z.powi(2)).sqrt())
            .collect();

        // All should be roughly same distance from axis in XZ plane
        let avg_radius = radii.iter().sum::<f32>() / radii.len() as f32;
        for r in &radii {
            assert!(
                (r - avg_radius).abs() < 0.1,
                "Umbel flowers should be equidistant from axis in XZ plane"
            );
        }
    }

    #[test]
    fn test_umbel_golden_angle() {
        let params = InflorescenceParams {
            branch_count: 8,
            rotation_angle: 137.5, // Golden angle
            angle_top: 30.0,
            ..Default::default()
        };

        let axis = AxisCurve::new(vec![Vec3::ZERO, Vec3::new(0.0, 10.0, 0.0)]);
        let branches = generate_branch_points(&params, &axis);

        // With golden angle, branches should not align
        // Check that no two branches have same X or Z coordinate
        for i in 0..branches.len() {
            for j in (i + 1)..branches.len() {
                let same_x = (branches[i].position.x - branches[j].position.x).abs() < 0.01;
                let same_z = (branches[i].position.z - branches[j].position.z).abs() < 0.01;
                assert!(
                    !(same_x && same_z),
                    "Golden angle should prevent branches from overlapping"
                );
            }
        }
    }
}
