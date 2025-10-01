//! 3D curve utilities for procedural flower generation
//!
//! This module provides curve interpolation and evaluation functions,
//! particularly Catmull-Rom splines for smooth curves through control points.

use crate::Vec3;

/// Evaluate a Catmull-Rom spline at parameter t ∈ [0, 1]
///
/// Catmull-Rom splines pass through all control points (except the first and last)
/// and provide C1 continuity. This implementation uses the standard uniform
/// Catmull-Rom formulation with tension = 0.5.
///
/// # Arguments
///
/// * `p0` - Control point before the segment
/// * `p1` - Start point of the segment
/// * `p2` - End point of the segment
/// * `p3` - Control point after the segment
/// * `t` - Parameter in range [0, 1] within segment [p1, p2]
///
/// # Returns
///
/// Point on the curve at parameter t
///
/// # Example
///
/// ```
/// use floraison_core::math::curves::catmull_rom_point;
/// use floraison_core::Vec3;
///
/// let p0 = Vec3::new(0.0, 0.0, 0.0);
/// let p1 = Vec3::new(0.0, 1.0, 0.0);
/// let p2 = Vec3::new(0.0, 2.0, 0.5);
/// let p3 = Vec3::new(0.0, 3.0, 1.0);
///
/// // Evaluate at midpoint of segment [p1, p2]
/// let point = catmull_rom_point(p0, p1, p2, p3, 0.5);
/// assert!((point - Vec3::new(0.0, 1.5, 0.21875)).length() < 0.01);
/// ```
pub fn catmull_rom_point(p0: Vec3, p1: Vec3, p2: Vec3, p3: Vec3, t: f32) -> Vec3 {
    // Catmull-Rom basis matrix with tension = 0.5
    // P(t) = 0.5 * [1 t t² t³] * M * [p0 p1 p2 p3]ᵀ
    //
    // where M =  [ 0  2  0  0]
    //            [-1  0  1  0]
    //            [ 2 -5  4 -1]
    //            [-1  3 -3  1]

    let t2 = t * t;
    let t3 = t2 * t;

    // Basis functions
    let b0 = -t + 2.0 * t2 - t3;
    let b1 = 2.0 - 5.0 * t2 + 3.0 * t3;
    let b2 = t + 4.0 * t2 - 3.0 * t3;
    let b3 = -t2 + t3;

    0.5 * (b0 * p0 + b1 * p1 + b2 * p2 + b3 * p3)
}

/// Compute tangent vector at parameter t along a Catmull-Rom segment
///
/// The tangent is the derivative of the curve at parameter t.
///
/// # Arguments
///
/// * `p0` - Control point before the segment
/// * `p1` - Start point of the segment
/// * `p2` - End point of the segment
/// * `p3` - Control point after the segment
/// * `t` - Parameter in range [0, 1] within segment [p1, p2]
///
/// # Returns
///
/// Tangent vector at parameter t (not normalized)
///
/// # Example
///
/// ```
/// use floraison_core::math::curves::catmull_rom_tangent;
/// use floraison_core::Vec3;
///
/// let p0 = Vec3::new(0.0, 0.0, 0.0);
/// let p1 = Vec3::new(0.0, 1.0, 0.0);
/// let p2 = Vec3::new(0.0, 2.0, 0.0);
/// let p3 = Vec3::new(0.0, 3.0, 0.0);
///
/// let tangent = catmull_rom_tangent(p0, p1, p2, p3, 0.5);
/// // Tangent should point primarily in +Y direction
/// assert!(tangent.y > 0.0);
/// ```
pub fn catmull_rom_tangent(p0: Vec3, p1: Vec3, p2: Vec3, p3: Vec3, t: f32) -> Vec3 {
    // Derivative of Catmull-Rom curve
    // P'(t) = 0.5 * [0 1 2t 3t²] * M * [p0 p1 p2 p3]ᵀ

    let t2 = t * t;

    // Derivative basis functions
    let b0 = -1.0 + 4.0 * t - 3.0 * t2;
    let b1 = -10.0 * t + 9.0 * t2;
    let b2 = 1.0 + 8.0 * t - 9.0 * t2;
    let b3 = -2.0 * t + 3.0 * t2;

    0.5 * (b0 * p0 + b1 * p1 + b2 * p2 + b3 * p3)
}

/// Sample N points along a Catmull-Rom spline through control points
///
/// This function creates a smooth curve passing through all interior control points.
/// The curve does not pass through the first and last control points - they are
/// used only to determine the tangent at the endpoints.
///
/// # Arguments
///
/// * `points` - Control points defining the curve (minimum 4 points)
/// * `samples_per_segment` - Number of samples between each pair of adjacent points
///
/// # Returns
///
/// Vector of sampled points along the curve
///
/// # Panics
///
/// Panics if `points.len() < 4` or `samples_per_segment < 2`
///
/// # Example
///
/// ```
/// use floraison_core::math::curves::sample_catmull_rom_curve;
/// use floraison_core::Vec3;
///
/// let control_points = vec![
///     Vec3::new(0.0, 0.0, 0.0),
///     Vec3::new(0.0, 1.0, 0.0),
///     Vec3::new(0.0, 2.0, 0.5),
///     Vec3::new(0.0, 3.0, 1.0),
/// ];
///
/// let curve = sample_catmull_rom_curve(&control_points, 10);
/// // Curve passes through p1 and p2, but not p0 and p3
/// assert!(curve.len() > 0);
/// ```
pub fn sample_catmull_rom_curve(points: &[Vec3], samples_per_segment: usize) -> Vec<Vec3> {
    assert!(
        points.len() >= 4,
        "Catmull-Rom spline requires at least 4 control points"
    );
    assert!(
        samples_per_segment >= 2,
        "Need at least 2 samples per segment"
    );

    let num_segments = points.len() - 3;
    let mut curve = Vec::with_capacity(num_segments * samples_per_segment);

    // For each segment between interior points
    for seg_idx in 0..num_segments {
        let p0 = points[seg_idx];
        let p1 = points[seg_idx + 1];
        let p2 = points[seg_idx + 2];
        let p3 = points[seg_idx + 3];

        // Sample points along this segment
        for i in 0..samples_per_segment {
            let t = i as f32 / samples_per_segment as f32;
            let point = catmull_rom_point(p0, p1, p2, p3, t);
            curve.push(point);
        }
    }

    // Add final endpoint
    curve.push(points[points.len() - 2]);

    curve
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catmull_rom_passes_through_p1_at_t0() {
        let p0 = Vec3::new(0.0, 0.0, 0.0);
        let p1 = Vec3::new(0.0, 1.0, 0.0);
        let p2 = Vec3::new(0.0, 2.0, 0.5);
        let p3 = Vec3::new(0.0, 3.0, 1.0);

        let point = catmull_rom_point(p0, p1, p2, p3, 0.0);
        assert!((point - p1).length() < 1e-5, "Should pass through p1 at t=0");
    }

    #[test]
    fn test_catmull_rom_passes_through_p2_at_t1() {
        let p0 = Vec3::new(0.0, 0.0, 0.0);
        let p1 = Vec3::new(0.0, 1.0, 0.0);
        let p2 = Vec3::new(0.0, 2.0, 0.5);
        let p3 = Vec3::new(0.0, 3.0, 1.0);

        let point = catmull_rom_point(p0, p1, p2, p3, 1.0);
        assert!((point - p2).length() < 1e-5, "Should pass through p2 at t=1");
    }

    #[test]
    fn test_catmull_rom_midpoint() {
        let p0 = Vec3::new(0.0, 0.0, 0.0);
        let p1 = Vec3::new(0.0, 1.0, 0.0);
        let p2 = Vec3::new(0.0, 2.0, 0.0);
        let p3 = Vec3::new(0.0, 3.0, 0.0);

        let point = catmull_rom_point(p0, p1, p2, p3, 0.5);
        // For a straight line, midpoint should be average of p1 and p2
        let expected = Vec3::new(0.0, 1.5, 0.0);
        assert!((point - expected).length() < 0.01, "Midpoint should be near 1.5");
    }

    #[test]
    fn test_catmull_rom_tangent_direction() {
        let p0 = Vec3::new(0.0, 0.0, 0.0);
        let p1 = Vec3::new(0.0, 1.0, 0.0);
        let p2 = Vec3::new(0.0, 2.0, 0.0);
        let p3 = Vec3::new(0.0, 3.0, 0.0);

        let tangent = catmull_rom_tangent(p0, p1, p2, p3, 0.5);
        // For points along Y-axis, tangent should point in +Y
        assert!(tangent.y > 0.0, "Tangent should point in +Y direction");
        assert!(tangent.x.abs() < 1e-5, "Tangent X component should be near 0");
        assert!(tangent.z.abs() < 1e-5, "Tangent Z component should be near 0");
    }

    #[test]
    fn test_sample_curve_basic() {
        let points = vec![
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 2.0, 0.5),
            Vec3::new(0.0, 3.0, 1.0),
        ];

        let curve = sample_catmull_rom_curve(&points, 10);
        assert!(curve.len() > 0, "Curve should have samples");

        // First sample should be at p1
        assert!((curve[0] - points[1]).length() < 1e-5, "First sample at p1");

        // Last sample should be at p2
        let last_idx = curve.len() - 1;
        assert!((curve[last_idx] - points[2]).length() < 1e-5, "Last sample at p2");
    }

    #[test]
    fn test_sample_curve_smooth() {
        let points = vec![
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(1.0, 2.0, 0.0),
            Vec3::new(1.0, 3.0, 0.0),
        ];

        let curve = sample_catmull_rom_curve(&points, 20);

        // Check that samples are reasonably spaced (no huge jumps)
        for i in 0..curve.len() - 1 {
            let dist = (curve[i + 1] - curve[i]).length();
            assert!(
                dist < 0.5,
                "Consecutive samples should be close, got distance {}",
                dist
            );
        }
    }

    #[test]
    #[should_panic(expected = "at least 4 control points")]
    fn test_sample_curve_too_few_points() {
        let points = vec![Vec3::ZERO, Vec3::X, Vec3::Y];
        sample_catmull_rom_curve(&points, 10);
    }

    #[test]
    fn test_tangent_length_non_zero() {
        let p0 = Vec3::new(0.0, 0.0, 0.0);
        let p1 = Vec3::new(0.0, 1.0, 0.0);
        let p2 = Vec3::new(0.0, 2.0, 0.0);
        let p3 = Vec3::new(0.0, 3.0, 0.0);

        let tangent = catmull_rom_tangent(p0, p1, p2, p3, 0.5);
        assert!(tangent.length() > 0.01, "Tangent should have non-zero length");
    }
}
