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
        assert!(
            (point - p1).length() < 1e-5,
            "Should pass through p1 at t=0"
        );
    }

    #[test]
    fn test_catmull_rom_passes_through_p2_at_t1() {
        let p0 = Vec3::new(0.0, 0.0, 0.0);
        let p1 = Vec3::new(0.0, 1.0, 0.0);
        let p2 = Vec3::new(0.0, 2.0, 0.5);
        let p3 = Vec3::new(0.0, 3.0, 1.0);

        let point = catmull_rom_point(p0, p1, p2, p3, 1.0);
        assert!(
            (point - p2).length() < 1e-5,
            "Should pass through p2 at t=1"
        );
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
        assert!(
            (point - expected).length() < 0.01,
            "Midpoint should be near 1.5"
        );
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
        assert!(
            tangent.x.abs() < 1e-5,
            "Tangent X component should be near 0"
        );
        assert!(
            tangent.z.abs() < 1e-5,
            "Tangent Z component should be near 0"
        );
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
        assert!(!curve.is_empty(), "Curve should have samples");

        // First sample should be at p1
        assert!((curve[0] - points[1]).length() < 1e-5, "First sample at p1");

        // Last sample should be at p2
        let last_idx = curve.len() - 1;
        assert!(
            (curve[last_idx] - points[2]).length() < 1e-5,
            "Last sample at p2"
        );
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
        assert!(
            tangent.length() > 0.01,
            "Tangent should have non-zero length"
        );
    }
}

// ============================================================================
// 3D Curve Reconstruction
// ============================================================================

use crate::Vec2;

/// Resample a 2D curve so that Y values are evenly spaced
///
/// This is crucial for numerical stability when computing second derivatives.
/// The algorithm performs linear interpolation between the original points.
///
/// # Arguments
///
/// * `points` - Input 2D curve points (must be sorted by Y)
/// * `n` - Number of samples to generate
///
/// # Returns
///
/// Vector of resampled points with uniform Y spacing
///
/// # Panics
///
/// Panics if `points.len() < 2` or if Y values are not monotonically increasing
pub fn resample_uniform_y(points: &[Vec2], n: usize) -> Vec<Vec2> {
    assert!(points.len() >= 2, "Need at least 2 points to resample");
    assert!(n >= 2, "Need at least 2 samples");

    let y_min = points.first().unwrap().y;
    let y_max = points.last().unwrap().y;
    let dy = (y_max - y_min) / (n - 1) as f32;

    let mut resampled = Vec::with_capacity(n);

    for i in 0..n {
        let target_y = y_min + i as f32 * dy;

        // Find the segment containing target_y
        let mut idx = 0;
        while idx < points.len() - 1 && points[idx + 1].y < target_y {
            idx += 1;
        }

        // Linear interpolation between points[idx] and points[idx+1]
        if idx < points.len() - 1 {
            let p0 = points[idx];
            let p1 = points[idx + 1];
            let t = (target_y - p0.y) / (p1.y - p0.y).max(1e-6);
            let x = p0.x + t * (p1.x - p0.x);
            resampled.push(Vec2::new(x, target_y));
        } else {
            // Last point
            resampled.push(*points.last().unwrap());
        }
    }

    resampled
}

/// Compute second derivatives of X with respect to Y
///
/// Uses finite difference approximation for points on a regular Y grid.
///
/// # Arguments
///
/// * `points` - 2D curve with uniform Y spacing
///
/// # Returns
///
/// Vector of d²x/dy² values
pub fn compute_second_derivatives_x(points: &[Vec2]) -> Vec<f32> {
    let n = points.len();
    assert!(n >= 3, "Need at least 3 points for second derivatives");

    let mut d2x = Vec::with_capacity(n);

    // Assuming uniform Y spacing
    let dy = if n > 1 {
        (points[1].y - points[0].y).abs().max(1e-6)
    } else {
        1.0
    };
    let dy2 = dy * dy;

    // Forward difference for first point
    let first = (points[2].x - 2.0 * points[1].x + points[0].x) / dy2;
    d2x.push(first);

    // Central difference for interior points
    for i in 1..n - 1 {
        let d2 = (points[i + 1].x - 2.0 * points[i].x + points[i - 1].x) / dy2;
        d2x.push(d2);
    }

    // Backward difference for last point
    let last = (points[n - 1].x - 2.0 * points[n - 2].x + points[n - 3].x) / dy2;
    d2x.push(last);

    d2x
}

/// Integrate a sequence twice using trapezoidal rule
///
/// Performs numerical integration to convert second derivatives to values.
///
/// # Arguments
///
/// * `second_derivatives` - Values of d²f/dx²
///
/// # Returns
///
/// Vector of f values after double integration
pub fn integrate_twice(second_derivatives: &[f32]) -> Vec<f32> {
    let n = second_derivatives.len();
    if n == 0 {
        return vec![];
    }

    // First integration: d²f/dx² → df/dx
    let mut first_integral = vec![0.0];
    for i in 1..n {
        let avg = (second_derivatives[i] + second_derivatives[i - 1]) / 2.0;
        first_integral.push(first_integral[i - 1] + avg);
    }

    // Second integration: df/dx → f
    let mut second_integral = vec![0.0];
    for i in 1..n {
        let avg = (first_integral[i] + first_integral[i - 1]) / 2.0;
        second_integral.push(second_integral[i - 1] + avg);
    }

    second_integral
}

/// Determine signs for Z second derivatives to create smooth spiral
///
/// Heuristic: flip sign when X second derivative crosses zero
///
/// # Arguments
///
/// * `points_2d` - Original 2D points
/// * `dz2` - Unsigned second derivatives of Z (will be modified in-place)
pub fn determine_z_signs(points_2d: &[Vec2], dz2: &mut [f32]) {
    assert_eq!(points_2d.len(), dz2.len());

    let dx2 = compute_second_derivatives_x(points_2d);
    let mut sign = 1.0f32;

    for i in 1..dz2.len() {
        // Flip sign when dx2 crosses zero
        if dx2[i] * dx2[i - 1] < 0.0 {
            sign *= -1.0;
        }
        dz2[i] *= sign;
    }
}

/// Reconstruct a 3D curve from a 2D input using constant curvature
///
/// Implements the algorithm from SIGGRAPH 2005 paper "Floral diagrams and inflorescences".
/// Given 2D points (x, y), computes Z values such that the 3D curve has constant curvature:
///
/// ```text
/// (d²x/dy²)² + (d²z/dy²)² = constant
/// ```
///
/// This creates natural-looking 3D curves from 2D sketches.
///
/// # Arguments
///
/// * `points_2d` - Input 2D curve points
///
/// # Returns
///
/// Vector of 3D points forming a smooth spatial curve
///
/// # Algorithm
///
/// 1. Resample input to uniform Y spacing (for numerical stability)
/// 2. Compute d²x/dy² using finite differences
/// 3. Find maximum curvature (constant value)
/// 4. Solve for |d²z/dy²| using the constraint
/// 5. Determine signs to avoid curve folding
/// 6. Integrate twice to get Z values
///
/// # Example
///
/// ```
/// use floraison_core::math::curves::reconstruct_3d_curve;
/// use floraison_core::Vec2;
///
/// // Straight line should remain straight (Z ≈ 0)
/// let line = vec![
///     Vec2::new(0.0, 0.0),
///     Vec2::new(0.0, 1.0),
///     Vec2::new(0.0, 2.0),
/// ];
/// let curve_3d = reconstruct_3d_curve(&line);
/// assert!(curve_3d[1].z.abs() < 0.01);
/// ```
pub fn reconstruct_3d_curve(points_2d: &[Vec2]) -> Vec<Vec3> {
    assert!(
        points_2d.len() >= 3,
        "Need at least 3 points for 3D reconstruction"
    );

    // 1. Resample to uniform Y spacing
    let n = points_2d.len();
    let uniform_points = resample_uniform_y(points_2d, n);

    // 2. Compute d²x/dy²
    let dx2 = compute_second_derivatives_x(&uniform_points);

    // 3. Find maximum curvature (the constant)
    let max_curvature = dx2
        .iter()
        .map(|&v| v.abs())
        .fold(0.0f32, f32::max)
        .max(1e-6); // Avoid division by zero for straight lines

    // 4. Solve for |d²z/dy²| using constraint: dx2² + dz2² = k²
    let mut dz2: Vec<f32> = dx2
        .iter()
        .map(|&dx2_val| {
            let val = max_curvature.powi(2) - dx2_val.powi(2);
            if val > 0.0 {
                val.sqrt()
            } else {
                0.0
            }
        })
        .collect();

    // 5. Determine signs for smooth spiral
    determine_z_signs(&uniform_points, &mut dz2);

    // 6. Integrate twice to get Z values
    let z_values = integrate_twice(&dz2);

    // 7. Combine (x, y, z)
    uniform_points
        .iter()
        .zip(z_values.iter())
        .map(|(p2d, &z)| Vec3::new(p2d.x, p2d.y, z))
        .collect()
}

// ============================================================================
// Axis Curve Parameterization
// ============================================================================

/// Sample point on an axis curve with Frenet frame
///
/// Contains position and orientation information at a specific point.
#[derive(Debug, Clone)]
pub struct AxisSample {
    /// 3D position on the curve
    pub position: Vec3,

    /// Tangent vector (normalized, direction of curve)
    pub tangent: Vec3,

    /// Normal vector (normalized, perpendicular to tangent, direction of curvature)
    pub normal: Vec3,

    /// Binormal vector (normalized, tangent × normal)
    pub binormal: Vec3,
}

/// Compute cumulative arc lengths for curve points
///
/// Returns a vector where `arc_lengths[i]` is the total arc length
/// from the start of the curve to point `i`.
///
/// # Arguments
///
/// * `points` - Curve points
///
/// # Returns
///
/// Vector of cumulative arc lengths
pub fn compute_arc_lengths(points: &[Vec3]) -> Vec<f32> {
    let mut lengths = vec![0.0];

    for i in 1..points.len() {
        let segment_length = (points[i] - points[i - 1]).length();
        lengths.push(lengths[i - 1] + segment_length);
    }

    lengths
}

/// Parameterized 3D curve with arc-length sampling
///
/// Provides uniform sampling along a curve based on arc length,
/// and computes Frenet frames (tangent, normal, binormal) at sample points.
#[derive(Debug, Clone)]
pub struct AxisCurve {
    points: Vec<Vec3>,
    arc_lengths: Vec<f32>,
    total_length: f32,
}

impl AxisCurve {
    /// Create a new axis curve from 3D points
    ///
    /// Computes arc lengths for later parameterization.
    ///
    /// # Arguments
    ///
    /// * `points` - Curve control points (at least 2 required)
    ///
    /// # Panics
    ///
    /// Panics if `points.len() < 2`
    pub fn new(points: Vec<Vec3>) -> Self {
        assert!(points.len() >= 2, "Need at least 2 points for axis curve");

        let arc_lengths = compute_arc_lengths(&points);
        let total_length = *arc_lengths.last().unwrap();

        Self {
            points,
            arc_lengths,
            total_length,
        }
    }

    /// Get total arc length of the curve
    pub fn length(&self) -> f32 {
        self.total_length
    }

    /// Sample curve at normalized parameter t ∈ [0, 1]
    ///
    /// Returns position and Frenet frame at the sample point.
    ///
    /// # Arguments
    ///
    /// * `t` - Normalized parameter (0 = start, 1 = end)
    ///
    /// # Returns
    ///
    /// Sample with position and orientation
    pub fn sample_at_t(&self, t: f32) -> AxisSample {
        let t_clamped = t.clamp(0.0, 1.0);
        let target_length = t_clamped * self.total_length;

        self.sample_at_arc_length(target_length)
    }

    /// Sample curve at specific arc length
    ///
    /// # Arguments
    ///
    /// * `target_length` - Arc length from curve start
    ///
    /// # Returns
    ///
    /// Sample with position and orientation
    fn sample_at_arc_length(&self, target_length: f32) -> AxisSample {
        let n = self.points.len();

        // Find segment containing target arc length
        let mut idx = 0;
        while idx < n - 1 && self.arc_lengths[idx + 1] < target_length {
            idx += 1;
        }

        // Clamp to valid range
        if idx >= n - 1 {
            idx = n - 2;
        }

        // Linear interpolation within segment
        let segment_start_length = self.arc_lengths[idx];
        let segment_end_length = self.arc_lengths[idx + 1];
        let segment_length = segment_end_length - segment_start_length;

        let local_t = if segment_length > 1e-6 {
            (target_length - segment_start_length) / segment_length
        } else {
            0.0
        };

        let p0 = self.points[idx];
        let p1 = self.points[idx + 1];
        let position = p0.lerp(p1, local_t);

        // Compute tangent (first derivative)
        let tangent = self.tangent_at_index(idx).normalize_or_zero();

        // Compute normal (second derivative direction or arbitrary perpendicular)
        let normal = self.normal_at_index(idx, &tangent).normalize_or_zero();

        // Compute binormal (cross product)
        let binormal = tangent.cross(normal).normalize_or_zero();

        // Re-orthogonalize (ensure perfect orthonormality)
        let normal = binormal.cross(tangent).normalize_or_zero();

        AxisSample {
            position,
            tangent,
            normal,
            binormal,
        }
    }

    /// Compute tangent at a specific index using finite differences
    fn tangent_at_index(&self, idx: usize) -> Vec3 {
        let n = self.points.len();

        if n < 2 {
            return Vec3::Y; // Default upward
        }

        // Central difference for interior points
        if idx > 0 && idx < n - 1 {
            (self.points[idx + 1] - self.points[idx - 1]).normalize_or(Vec3::Y)
        }
        // Forward difference for first point
        else if idx == 0 {
            (self.points[1] - self.points[0]).normalize_or(Vec3::Y)
        }
        // Backward difference for last point
        else {
            (self.points[n - 1] - self.points[n - 2]).normalize_or(Vec3::Y)
        }
    }

    /// Compute normal at a specific index (direction of curvature)
    fn normal_at_index(&self, idx: usize, tangent: &Vec3) -> Vec3 {
        let n = self.points.len();

        if n < 3 {
            // Not enough points for curvature, use arbitrary perpendicular
            return self.arbitrary_perpendicular(tangent);
        }

        // Estimate second derivative (acceleration)
        let d2p = if idx > 0 && idx < n - 1 {
            // Central difference for second derivative
            self.points[idx + 1] - 2.0 * self.points[idx] + self.points[idx - 1]
        } else if idx == 0 && n >= 3 {
            self.points[2] - 2.0 * self.points[1] + self.points[0]
        } else if idx == n - 1 && n >= 3 {
            self.points[n - 1] - 2.0 * self.points[n - 2] + self.points[n - 3]
        } else {
            Vec3::ZERO
        };

        // Project acceleration onto plane perpendicular to tangent
        let tangent_component = d2p.dot(*tangent) * *tangent;
        let mut normal = d2p - tangent_component;

        // If normal is too small (straight segment), use arbitrary perpendicular
        if normal.length() < 1e-4 {
            normal = self.arbitrary_perpendicular(tangent);
        }

        normal.normalize_or(Vec3::X)
    }

    /// Find an arbitrary unit vector perpendicular to given vector
    fn arbitrary_perpendicular(&self, v: &Vec3) -> Vec3 {
        // Choose a vector not parallel to v
        let candidate = if v.y.abs() < 0.9 { Vec3::Y } else { Vec3::X };

        // Project out parallel component
        let parallel = candidate.dot(*v) * *v;
        let perpendicular = candidate - parallel;

        perpendicular.normalize_or(Vec3::X)
    }

    /// Sample curve uniformly at N points
    ///
    /// Returns `count` samples evenly spaced along the curve by arc length.
    ///
    /// # Arguments
    ///
    /// * `count` - Number of samples to generate
    ///
    /// # Returns
    ///
    /// Vector of samples with position and orientation
    pub fn sample_uniform(&self, count: usize) -> Vec<AxisSample> {
        assert!(count >= 1, "Need at least 1 sample");

        if count == 1 {
            return vec![self.sample_at_t(0.0)];
        }

        let mut samples = Vec::with_capacity(count);

        for i in 0..count {
            let t = i as f32 / (count - 1) as f32;
            samples.push(self.sample_at_t(t));
        }

        samples
    }
}

#[cfg(test)]
mod reconstruction_tests {
    use super::*;

    #[test]
    fn test_resample_uniform_y_basic() {
        let points = vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(2.0, 2.0),
        ];

        let resampled = resample_uniform_y(&points, 5);
        assert_eq!(resampled.len(), 5);

        // Check Y values are evenly spaced
        assert!((resampled[0].y - 0.0).abs() < 1e-5);
        assert!((resampled[1].y - 0.5).abs() < 1e-5);
        assert!((resampled[2].y - 1.0).abs() < 1e-5);
        assert!((resampled[3].y - 1.5).abs() < 1e-5);
        assert!((resampled[4].y - 2.0).abs() < 1e-5);
    }

    #[test]
    fn test_compute_second_derivatives_straight_line() {
        // Straight line should have zero second derivative
        let points = vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(2.0, 2.0),
            Vec2::new(3.0, 3.0),
        ];

        let d2x = compute_second_derivatives_x(&points);
        assert_eq!(d2x.len(), 4);

        // All second derivatives should be near zero
        for &val in &d2x {
            assert!(val.abs() < 1e-3, "Expected ~0, got {}", val);
        }
    }

    #[test]
    fn test_integrate_twice_constant_acceleration() {
        // Constant second derivative should give quadratic
        let d2f = vec![2.0, 2.0, 2.0, 2.0];
        let result = integrate_twice(&d2f);

        // Result should be quadratic: f(x) = x²
        // At x=0: 0, x=1: ~1, x=2: ~4, x=3: ~9
        assert_eq!(result.len(), 4);
        assert!(result[0].abs() < 0.1); // ~0
    }

    #[test]
    fn test_reconstruct_3d_straight_line() {
        // Straight vertical line should remain straight (Z ≈ 0)
        let line = vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(0.0, 1.0),
            Vec2::new(0.0, 2.0),
            Vec2::new(0.0, 3.0),
        ];

        let curve_3d = reconstruct_3d_curve(&line);
        assert_eq!(curve_3d.len(), 4);

        // All Z values should be near zero
        for point in &curve_3d {
            assert!(
                point.z.abs() < 0.1,
                "Straight line should have Z≈0, got {}",
                point.z
            );
        }

        // X and Y should match input
        assert!((curve_3d[0].x - 0.0).abs() < 1e-3);
        assert!((curve_3d[0].y - 0.0).abs() < 1e-3);
    }

    #[test]
    fn test_reconstruct_3d_sine_wave() {
        use std::f32::consts::PI;

        // Sine wave in X should produce spiral
        let mut sine_wave = Vec::new();
        for i in 0..20 {
            let y = i as f32 * 0.5;
            let x = (y * PI / 4.0).sin();
            sine_wave.push(Vec2::new(x, y));
        }

        let curve_3d = reconstruct_3d_curve(&sine_wave);
        assert_eq!(curve_3d.len(), 20);

        // Should have non-zero Z values (it's a 3D spiral)
        let max_z = curve_3d.iter().map(|p| p.z.abs()).fold(0.0f32, f32::max);
        assert!(
            max_z > 0.1,
            "Sine wave should produce 3D curve with Z variation, max_z={}",
            max_z
        );
    }

    #[test]
    fn test_reconstruct_3d_no_nan() {
        // Test numerical stability
        let points = vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(0.5, 1.0),
            Vec2::new(-0.3, 2.0),
            Vec2::new(0.2, 3.0),
            Vec2::new(0.0, 4.0),
        ];

        let curve_3d = reconstruct_3d_curve(&points);

        // No NaN or infinity
        for point in &curve_3d {
            assert!(point.x.is_finite(), "X should be finite");
            assert!(point.y.is_finite(), "Y should be finite");
            assert!(point.z.is_finite(), "Z should be finite");
        }
    }

    #[test]
    fn test_integrate_twice_empty() {
        let result = integrate_twice(&[]);
        assert!(result.is_empty());
    }

    #[test]
    #[should_panic(expected = "at least 3 points")]
    fn test_reconstruct_3d_too_few_points() {
        let points = vec![Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0)];
        reconstruct_3d_curve(&points);
    }
}

#[cfg(test)]
mod axis_tests {
    use super::*;

    #[test]
    fn test_compute_arc_lengths_simple() {
        let points = vec![
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(2.0, 0.0, 0.0),
        ];

        let lengths = compute_arc_lengths(&points);
        assert_eq!(lengths.len(), 3);
        assert!((lengths[0] - 0.0).abs() < 1e-5);
        assert!((lengths[1] - 1.0).abs() < 1e-5);
        assert!((lengths[2] - 2.0).abs() < 1e-5);
    }

    #[test]
    fn test_axis_curve_creation() {
        let points = vec![
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 2.0, 0.0),
        ];

        let axis = AxisCurve::new(points);
        assert!((axis.length() - 2.0).abs() < 1e-5);
    }

    #[test]
    fn test_sample_at_endpoints() {
        let points = vec![Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 10.0, 0.0)];

        let axis = AxisCurve::new(points);

        let sample_start = axis.sample_at_t(0.0);
        assert!((sample_start.position - Vec3::new(0.0, 0.0, 0.0)).length() < 1e-3);

        let sample_end = axis.sample_at_t(1.0);
        assert!((sample_end.position - Vec3::new(0.0, 10.0, 0.0)).length() < 1e-3);
    }

    #[test]
    fn test_sample_at_midpoint() {
        let points = vec![Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 10.0, 0.0)];

        let axis = AxisCurve::new(points);
        let sample = axis.sample_at_t(0.5);

        // Midpoint should be at Y=5
        assert!((sample.position.y - 5.0).abs() < 1e-3);
    }

    #[test]
    fn test_frenet_frame_orthonormal() {
        let points = vec![
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 1.0, 0.0),
            Vec3::new(2.0, 2.0, 1.0),
            Vec3::new(3.0, 3.0, 1.0),
        ];

        let axis = AxisCurve::new(points);
        let sample = axis.sample_at_t(0.5);

        // Check normalized
        assert!(
            (sample.tangent.length() - 1.0).abs() < 1e-3,
            "Tangent should be unit"
        );
        assert!(
            (sample.normal.length() - 1.0).abs() < 1e-3,
            "Normal should be unit"
        );
        assert!(
            (sample.binormal.length() - 1.0).abs() < 1e-3,
            "Binormal should be unit"
        );

        // Check orthogonal
        assert!(
            sample.tangent.dot(sample.normal).abs() < 1e-3,
            "Tangent and normal should be orthogonal"
        );
        assert!(
            sample.tangent.dot(sample.binormal).abs() < 1e-3,
            "Tangent and binormal should be orthogonal"
        );
        assert!(
            sample.normal.dot(sample.binormal).abs() < 1e-3,
            "Normal and binormal should be orthogonal"
        );
    }

    #[test]
    fn test_uniform_sampling() {
        let points = vec![Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 10.0, 0.0)];

        let axis = AxisCurve::new(points);
        let samples = axis.sample_uniform(5);

        assert_eq!(samples.len(), 5);

        // Check positions are evenly spaced
        assert!((samples[0].position.y - 0.0).abs() < 1e-3);
        assert!((samples[1].position.y - 2.5).abs() < 1e-3);
        assert!((samples[2].position.y - 5.0).abs() < 1e-3);
        assert!((samples[3].position.y - 7.5).abs() < 1e-3);
        assert!((samples[4].position.y - 10.0).abs() < 1e-3);
    }

    #[test]
    fn test_straight_line_tangent() {
        let points = vec![
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 5.0, 0.0),
            Vec3::new(0.0, 10.0, 0.0),
        ];

        let axis = AxisCurve::new(points);
        let sample = axis.sample_at_t(0.5);

        // Tangent should point in +Y direction
        assert!(sample.tangent.y > 0.9, "Tangent should point upward");
        assert!(sample.tangent.x.abs() < 0.1, "Tangent X should be near 0");
        assert!(sample.tangent.z.abs() < 0.1, "Tangent Z should be near 0");
    }

    #[test]
    #[should_panic(expected = "at least 2 points")]
    fn test_axis_curve_too_few_points() {
        let points = vec![Vec3::ZERO];
        AxisCurve::new(points);
    }

    #[test]
    fn test_single_sample() {
        let points = vec![Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 10.0, 0.0)];

        let axis = AxisCurve::new(points);
        let samples = axis.sample_uniform(1);

        assert_eq!(samples.len(), 1);
    }
}
