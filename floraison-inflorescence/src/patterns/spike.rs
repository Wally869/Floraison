//! Spike pattern generator
//!
//! Spike: Flowers sessile (no pedicels) on unbranched axis.
//! Blooming pattern: Indeterminate (bottom flowers bloom first).
//!
//! Examples: Wheat, Plantain

use floraison_core::math::curves::AxisCurve;
use glam::Quat;

use crate::{apply_age_distribution, BranchPoint, InflorescenceParams};

/// Linear interpolation between two values
#[inline]
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Generate branch points for a spike pattern
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
/// - **Sessile**: Flowers attach directly to axis (pedicel length ≈ 0)
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

        // Interpolate flower scale from bottom to top
        let flower_scale = lerp(params.flower_size_bottom, params.flower_size_top, t);

        // Compute rotation around axis (phyllotaxis spiral)
        let rotation = params.rotation_angle * i as f32;

        // Compute flower direction (sessile, so just rotated normal)
        // Still apply down angle for flower orientation, but no pedicel extension
        let angle = lerp(params.angle_bottom, params.angle_top, t);
        let down_rotation = Quat::from_axis_angle(sample.binormal, -angle.to_radians());
        let spiral_rotation = Quat::from_axis_angle(sample.tangent, rotation.to_radians());
        let direction = (spiral_rotation * down_rotation * sample.normal).normalize();

        // Flower position directly on axis (sessile - no pedicel)
        let position = sample.position;

        // Age: indeterminate (bottom = oldest = 1.0, top = youngest = 0.0)
        let base_age = 1.0 - t;
        let age = apply_age_distribution(base_age, params.age_distribution);

        branches.push(BranchPoint {
            position,
            direction,
            length: 0.0, // Sessile: no pedicel
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
    fn test_spike_branch_count() {
        let params = InflorescenceParams {
            branch_count: 8,
            ..Default::default()
        };

        let axis = AxisCurve::new(vec![Vec3::ZERO, Vec3::new(0.0, 10.0, 0.0)]);
        let branches = generate_branch_points(&params, &axis);

        assert_eq!(branches.len(), 8);
    }

    #[test]
    fn test_spike_sessile_attachment() {
        let params = InflorescenceParams {
            branch_count: 5,
            ..Default::default()
        };

        let axis = AxisCurve::new(vec![Vec3::ZERO, Vec3::new(0.0, 10.0, 0.0)]);
        let branches = generate_branch_points(&params, &axis);

        // All flowers should be sessile (length = 0)
        for branch in &branches {
            assert_eq!(
                branch.length, 0.0,
                "Spike pattern should have sessile flowers (length = 0)"
            );
        }
    }

    #[test]
    fn test_spike_on_axis() {
        let params = InflorescenceParams {
            branch_count: 4,
            ..Default::default()
        };

        let axis = AxisCurve::new(vec![Vec3::ZERO, Vec3::new(0.0, 12.0, 0.0)]);
        let branches = generate_branch_points(&params, &axis);

        // Flowers should be positioned directly on axis
        // For straight vertical axis: X and Z should be near 0
        for branch in &branches {
            assert!(
                branch.position.x.abs() < 1e-5,
                "Spike flowers should be on axis (x ≈ 0)"
            );
            assert!(
                branch.position.z.abs() < 1e-5,
                "Spike flowers should be on axis (z ≈ 0)"
            );
        }

        // Y positions should increase
        for i in 1..branches.len() {
            assert!(
                branches[i].position.y > branches[i - 1].position.y,
                "Positions should ascend along axis"
            );
        }
    }

    #[test]
    fn test_spike_age_gradient() {
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
    fn test_spike_direction_normalized() {
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
    fn test_spike_spiral_rotation() {
        let params = InflorescenceParams {
            branch_count: 3,
            rotation_angle: 137.5, // Golden angle
            angle_top: 0.0,        // Straight out for easier testing
            angle_bottom: 0.0,
            ..Default::default()
        };

        let axis = AxisCurve::new(vec![Vec3::ZERO, Vec3::new(0.0, 10.0, 0.0)]);
        let branches = generate_branch_points(&params, &axis);

        // With zero down angle, directions should differ only in XZ rotation
        assert!(
            (branches[0].direction.x - branches[1].direction.x).abs() > 0.1
                || (branches[0].direction.z - branches[1].direction.z).abs() > 0.1,
            "Fibonacci rotation should create different X/Z directions"
        );
    }
}
