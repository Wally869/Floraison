//! Corymb pattern generator
//!
//! Corymb: Pedicels of varying length create flat-topped appearance.
//! Blooming pattern: Indeterminate (bottom flowers bloom first).
//!
//! Examples: Hawthorn, Yarrow

use floraison_core::math::curves::AxisCurve;
use glam::Quat;

use crate::{apply_age_distribution, BranchPoint, InflorescenceParams};

/// Linear interpolation between two values
#[inline]
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Generate branch points for a corymb pattern
///
/// # Arguments
/// * `params` - Inflorescence parameters defining axis, branch count, angles, etc.
/// * `axis` - The main axis curve along which flowers are arranged
///
/// # Returns
/// Vector of branch points, each representing a flower attachment location
///
/// # Pattern Characteristics
/// - Flowers evenly spaced along axis (like raceme)
/// - Pedicel lengths adjusted so all flowers reach same height (flat top)
/// - Rotation follows Fibonacci spiral (golden angle)
/// - Age increases from bottom (oldest) to top (youngest) - indeterminate
pub fn generate_branch_points(params: &InflorescenceParams, axis: &AxisCurve) -> Vec<BranchPoint> {
    let mut branches = Vec::with_capacity(params.branch_count);

    // Target height: all flowers should reach the top of the axis
    let target_height = axis.sample_at_t(1.0).position.y;

    for i in 0..params.branch_count {
        // Normalize position along axis (0.0 at bottom, 1.0 at top)
        let t = if params.branch_count > 1 {
            i as f32 / (params.branch_count - 1) as f32
        } else {
            0.5 // Single flower at middle
        };

        // Sample axis to get position and Frenet frame
        let sample = axis.sample_at_t(t);

        // Interpolate parameters from bottom to top
        let angle = lerp(params.angle_bottom, params.angle_top, t);
        let flower_scale = lerp(params.flower_size_bottom, params.flower_size_top, t);

        // Compute rotation around axis (phyllotaxis spiral)
        let rotation = params.rotation_angle * i as f32;

        // Compute branch direction
        let down_rotation = Quat::from_axis_angle(sample.binormal, -angle.to_radians());
        let spiral_rotation = Quat::from_axis_angle(sample.tangent, rotation.to_radians());
        let direction = (spiral_rotation * down_rotation * sample.normal).normalize();

        // Calculate pedicel length to reach target height
        // position.y + direction.y * length = target_height
        // length = (target_height - position.y) / direction.y
        //
        // Clamp direction.y to avoid division by zero when nearly horizontal
        let length = if direction.y.abs() > 0.01 {
            ((target_height - sample.position.y) / direction.y).max(0.0)
        } else {
            // If direction is nearly horizontal, use default length
            lerp(params.branch_length_bottom, params.branch_length_top, t)
        };

        // Branch endpoint
        let position = sample.position + direction * length;

        // Age: indeterminate (bottom = oldest = 1.0, top = youngest = 0.0)
        let base_age = 1.0 - t;
        let age = apply_age_distribution(base_age, params.age_distribution);

        branches.push(BranchPoint {
            position,
            direction,
            length,
            flower_scale,
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
    fn test_corymb_branch_count() {
        let params = InflorescenceParams {
            branch_count: 10,
            ..Default::default()
        };

        let axis = AxisCurve::new(vec![Vec3::ZERO, Vec3::new(0.0, 10.0, 0.0)]);
        let branches = generate_branch_points(&params, &axis);

        assert_eq!(branches.len(), 10);
    }

    #[test]
    fn test_corymb_flat_top() {
        let params = InflorescenceParams {
            branch_count: 5,
            axis_length: 10.0,
            angle_top: 30.0,
            angle_bottom: 60.0,
            ..Default::default()
        };

        let axis = AxisCurve::new(vec![Vec3::ZERO, Vec3::new(0.0, 10.0, 0.0)]);
        let branches = generate_branch_points(&params, &axis);

        // All flower positions should have approximately same Y coordinate
        let target_y = 10.0; // Top of axis
        for branch in &branches {
            assert!(
                (branch.position.y - target_y).abs() < 0.1,
                "Corymb should create flat top, flower at y={}, expected y≈{}",
                branch.position.y,
                target_y
            );
        }
    }

    #[test]
    fn test_corymb_length_variation() {
        let params = InflorescenceParams {
            branch_count: 4,
            axis_length: 10.0,
            angle_top: 45.0,
            angle_bottom: 45.0, // Same angle for simpler test
            ..Default::default()
        };

        let axis = AxisCurve::new(vec![Vec3::ZERO, Vec3::new(0.0, 10.0, 0.0)]);
        let branches = generate_branch_points(&params, &axis);

        // Bottom branches should have longer pedicels to reach top
        // Top branches should have shorter pedicels
        assert!(
            branches[0].length > branches.last().unwrap().length,
            "Bottom branches should have longer pedicels than top branches"
        );

        // Pedicels should generally decrease in length
        for i in 1..branches.len() {
            assert!(
                branches[i].length <= branches[i - 1].length + 0.1,
                "Pedicel length should decrease or stay similar from bottom to top"
            );
        }
    }

    #[test]
    fn test_corymb_age_gradient() {
        let params = InflorescenceParams {
            branch_count: 4,
            ..Default::default()
        };

        let axis = AxisCurve::new(vec![Vec3::ZERO, Vec3::new(0.0, 10.0, 0.0)]);
        let branches = generate_branch_points(&params, &axis);

        // First (bottom) should be oldest (age = 1.0)
        assert!((branches[0].age - 1.0).abs() < 1e-5);

        // Last (top) should be youngest (age = 0.0)
        assert!(branches.last().unwrap().age.abs() < 1e-5);

        // Age should decrease monotonically (indeterminate)
        for i in 1..branches.len() {
            assert!(
                branches[i].age < branches[i - 1].age,
                "Age should decrease from bottom to top"
            );
        }
    }

    #[test]
    fn test_corymb_direction_normalized() {
        let params = InflorescenceParams {
            branch_count: 6,
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
    fn test_corymb_top_branch() {
        let params = InflorescenceParams {
            branch_count: 5,
            axis_length: 10.0,
            ..Default::default()
        };

        let axis = AxisCurve::new(vec![Vec3::ZERO, Vec3::new(0.0, 10.0, 0.0)]);
        let branches = generate_branch_points(&params, &axis);

        // Top branch (at axis top) should have very short pedicel
        let top_branch = branches.last().unwrap();
        assert!(
            top_branch.length < 0.5,
            "Top branch should have short pedicel, got length={}",
            top_branch.length
        );
    }

    #[test]
    fn test_corymb_fibonacci_rotation() {
        let params = InflorescenceParams {
            branch_count: 3,
            rotation_angle: 137.5, // Golden angle
            angle_top: 45.0,
            angle_bottom: 45.0,
            ..Default::default()
        };

        let axis = AxisCurve::new(vec![Vec3::ZERO, Vec3::new(0.0, 10.0, 0.0)]);
        let branches = generate_branch_points(&params, &axis);

        // Check that flowers are rotated around axis
        assert!(
            (branches[0].direction.x - branches[1].direction.x).abs() > 0.1
                || (branches[0].direction.z - branches[1].direction.z).abs() > 0.1,
            "Fibonacci rotation should create different X/Z directions"
        );
    }
}
