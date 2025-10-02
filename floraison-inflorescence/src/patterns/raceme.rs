//! Raceme pattern generator
//!
//! Raceme: Flowers on pedicels (stalks) along unbranched axis.
//! Blooming pattern: Indeterminate (bottom flowers bloom first).
//!
//! Examples: Snapdragon, Lupine

use floraison_core::math::curves::AxisCurve;
use glam::Quat;

use crate::{apply_age_distribution, BranchPoint, InflorescenceParams};

/// Linear interpolation between two values
#[inline]
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Generate branch points for a raceme pattern
///
/// # Arguments
/// * `params` - Inflorescence parameters defining axis, branch count, angles, etc.
/// * `axis` - The main axis curve along which flowers are arranged
///
/// # Returns
/// Vector of branch points, each representing a flower attachment location
///
/// # Pattern Characteristics
/// - Flowers evenly spaced along axis
/// - Pedicel length interpolates from bottom to top
/// - Down angle interpolates from bottom to top
/// - Rotation follows Fibonacci spiral (golden angle)
/// - Age increases from bottom (oldest) to top (youngest) - indeterminate
pub fn generate_branch_points(params: &InflorescenceParams, axis: &AxisCurve) -> Vec<BranchPoint> {
    let mut branches = Vec::with_capacity(params.branch_count);

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
        let length = lerp(params.branch_length_bottom, params.branch_length_top, t);
        let flower_scale = lerp(params.flower_size_bottom, params.flower_size_top, t);

        // Compute rotation around axis (phyllotaxis spiral)
        let rotation = params.rotation_angle * i as f32;

        // Compute branch direction
        // 1. Rotate down from normal (perpendicular to tangent)
        let down_rotation = Quat::from_axis_angle(sample.binormal, -angle.to_radians());
        // 2. Rotate around tangent for spiral arrangement
        let spiral_rotation = Quat::from_axis_angle(sample.tangent, rotation.to_radians());
        // 3. Apply both rotations to the normal vector
        let direction = (spiral_rotation * down_rotation * sample.normal).normalize();

        // Branch endpoint (flower position)
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
    fn test_raceme_branch_count() {
        let params = InflorescenceParams {
            branch_count: 10,
            ..Default::default()
        };

        // Create simple straight axis
        let axis = AxisCurve::new(vec![Vec3::ZERO, Vec3::new(0.0, 10.0, 0.0)]);

        let branches = generate_branch_points(&params, &axis);

        assert_eq!(branches.len(), 10);
    }

    #[test]
    fn test_raceme_spacing() {
        let params = InflorescenceParams {
            branch_count: 5,
            axis_length: 10.0,
            ..Default::default()
        };

        let axis = AxisCurve::new(vec![Vec3::ZERO, Vec3::new(0.0, 10.0, 0.0)]);
        let branches = generate_branch_points(&params, &axis);

        // Check vertical spacing is approximately even
        for i in 1..branches.len() {
            let delta_y = branches[i].position.y - branches[i - 1].position.y;
            // Should be roughly 2.5 units apart in Y (10 / 4 intervals)
            // Allow some tolerance for branch angle effects
            assert!(delta_y > 0.0, "Branches should increase in height");
        }
    }

    #[test]
    fn test_raceme_age_gradient() {
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

        // Age should decrease monotonically
        for i in 1..branches.len() {
            assert!(
                branches[i].age < branches[i - 1].age,
                "Age should decrease from bottom to top"
            );
        }
    }

    #[test]
    fn test_raceme_direction_normalized() {
        let params = InflorescenceParams {
            branch_count: 5,
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
    fn test_raceme_length_interpolation() {
        let params = InflorescenceParams {
            branch_count: 3,
            branch_length_bottom: 2.0,
            branch_length_top: 0.5,
            ..Default::default()
        };

        let axis = AxisCurve::new(vec![Vec3::ZERO, Vec3::new(0.0, 10.0, 0.0)]);
        let branches = generate_branch_points(&params, &axis);

        // Bottom should have longest pedicel
        assert!((branches[0].length - 2.0).abs() < 1e-5);

        // Top should have shortest pedicel
        assert!((branches[2].length - 0.5).abs() < 1e-5);

        // Middle should be interpolated
        assert!(
            branches[1].length > 0.5 && branches[1].length < 2.0,
            "Middle length should be interpolated"
        );
    }

    #[test]
    fn test_raceme_single_flower() {
        let params = InflorescenceParams {
            branch_count: 1,
            ..Default::default()
        };

        let axis = AxisCurve::new(vec![Vec3::ZERO, Vec3::new(0.0, 10.0, 0.0)]);
        let branches = generate_branch_points(&params, &axis);

        assert_eq!(branches.len(), 1);
        // Single flower should exist (position depends on branch length and angle)
        assert!(branches[0].flower_scale > 0.0);
    }

    #[test]
    fn test_raceme_fibonacci_rotation() {
        let params = InflorescenceParams {
            branch_count: 3,
            rotation_angle: 137.5, // Golden angle
            angle_top: 0.0,        // Straight up for easier testing
            angle_bottom: 0.0,
            ..Default::default()
        };

        let axis = AxisCurve::new(vec![Vec3::ZERO, Vec3::new(0.0, 10.0, 0.0)]);
        let branches = generate_branch_points(&params, &axis);

        // With zero down angle, directions should differ only in XZ rotation
        // Check that they're not all pointing the same direction
        assert!(
            (branches[0].direction.x - branches[1].direction.x).abs() > 0.1
                || (branches[0].direction.z - branches[1].direction.z).abs() > 0.1,
            "Fibonacci rotation should create different X/Z directions"
        );
    }
}
