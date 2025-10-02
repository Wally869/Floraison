//! Bézier curve evaluation and utilities
//!
//! This module provides functions for evaluating and sampling Bézier curves,
//! which are essential for creating smooth profiles, petal outlines, and
//! deformation curves in procedural flower generation.
//!
//! # Bézier Curves
//!
//! Bézier curves are parametric curves defined by control points:
//! - **Quadratic** (3 control points): Simple curves with one interior control point
//! - **Cubic** (4 control points): More flexible curves with two interior control points
//!
//! The parameter `t` ranges from 0.0 to 1.0, where:
//! - `t = 0.0` → start point (first control point)
//! - `t = 1.0` → end point (last control point)
//!
//! # Examples
//!
//! ```
//! use floraison_core::math::bezier::{cubic_bezier_2d, sample_cubic_2d};
//! use floraison_core::Vec2;
//!
//! // Define a smooth S-curve
//! let p0 = Vec2::new(0.0, 0.0);
//! let p1 = Vec2::new(0.0, 1.0);
//! let p2 = Vec2::new(1.0, 0.0);
//! let p3 = Vec2::new(1.0, 1.0);
//!
//! // Evaluate at midpoint
//! let mid = cubic_bezier_2d(p0, p1, p2, p3, 0.5);
//!
//! // Sample 10 points along the curve
//! let samples = sample_cubic_2d(p0, p1, p2, p3, 10);
//! assert_eq!(samples.len(), 10);
//! ```

use crate::{Vec2, Vec3};

/// Evaluate a quadratic Bézier curve at parameter t (2D)
///
/// A quadratic Bézier curve is defined by 3 control points and uses the formula:
/// B(t) = (1-t)²·P₀ + 2(1-t)t·P₁ + t²·P₂
///
/// # Arguments
///
/// * `p0` - Start point
/// * `p1` - Control point (influences curve shape)
/// * `p2` - End point
/// * `t` - Parameter in range [0, 1]
///
/// # Example
///
/// ```
/// use floraison_core::math::bezier::quadratic_bezier_2d;
/// use floraison_core::Vec2;
///
/// let p0 = Vec2::new(0.0, 0.0);
/// let p1 = Vec2::new(0.5, 1.0);
/// let p2 = Vec2::new(1.0, 0.0);
///
/// let start = quadratic_bezier_2d(p0, p1, p2, 0.0);
/// assert_eq!(start, p0);
///
/// let end = quadratic_bezier_2d(p0, p1, p2, 1.0);
/// assert_eq!(end, p2);
/// ```
pub fn quadratic_bezier_2d(p0: Vec2, p1: Vec2, p2: Vec2, t: f32) -> Vec2 {
    let t2 = t * t;
    let mt = 1.0 - t;
    let mt2 = mt * mt;

    p0 * mt2 + p1 * (2.0 * mt * t) + p2 * t2
}

/// Evaluate a cubic Bézier curve at parameter t (2D)
///
/// A cubic Bézier curve is defined by 4 control points and uses the formula:
/// B(t) = (1-t)³·P₀ + 3(1-t)²t·P₁ + 3(1-t)t²·P₂ + t³·P₃
///
/// This is the most common form of Bézier curve, offering good control over shape
/// while remaining computationally efficient.
///
/// # Arguments
///
/// * `p0` - Start point
/// * `p1` - First control point
/// * `p2` - Second control point
/// * `p3` - End point
/// * `t` - Parameter in range [0, 1]
///
/// # Example
///
/// ```
/// use floraison_core::math::bezier::cubic_bezier_2d;
/// use floraison_core::Vec2;
///
/// let p0 = Vec2::new(0.0, 0.0);
/// let p1 = Vec2::new(0.0, 1.0);
/// let p2 = Vec2::new(1.0, 1.0);
/// let p3 = Vec2::new(1.0, 0.0);
///
/// let mid = cubic_bezier_2d(p0, p1, p2, p3, 0.5);
/// assert!(mid.x > 0.0 && mid.x < 1.0);
/// ```
pub fn cubic_bezier_2d(p0: Vec2, p1: Vec2, p2: Vec2, p3: Vec2, t: f32) -> Vec2 {
    let t2 = t * t;
    let t3 = t2 * t;
    let mt = 1.0 - t;
    let mt2 = mt * mt;
    let mt3 = mt2 * mt;

    p0 * mt3 + p1 * (3.0 * mt2 * t) + p2 * (3.0 * mt * t2) + p3 * t3
}

/// Evaluate a quadratic Bézier curve at parameter t (3D)
///
/// Same as [`quadratic_bezier_2d`] but for 3D points.
///
/// # Example
///
/// ```
/// use floraison_core::math::bezier::quadratic_bezier_3d;
/// use floraison_core::Vec3;
///
/// let p0 = Vec3::new(0.0, 0.0, 0.0);
/// let p1 = Vec3::new(0.5, 1.0, 0.5);
/// let p2 = Vec3::new(1.0, 0.0, 1.0);
///
/// let mid = quadratic_bezier_3d(p0, p1, p2, 0.5);
/// assert!(mid.y > 0.0); // Curve peaks in the middle
/// ```
pub fn quadratic_bezier_3d(p0: Vec3, p1: Vec3, p2: Vec3, t: f32) -> Vec3 {
    let t2 = t * t;
    let mt = 1.0 - t;
    let mt2 = mt * mt;

    p0 * mt2 + p1 * (2.0 * mt * t) + p2 * t2
}

/// Evaluate a cubic Bézier curve at parameter t (3D)
///
/// Same as [`cubic_bezier_2d`] but for 3D points. Useful for 3D stem curves.
///
/// # Example
///
/// ```
/// use floraison_core::math::bezier::cubic_bezier_3d;
/// use floraison_core::Vec3;
///
/// let p0 = Vec3::ZERO;
/// let p1 = Vec3::new(0.0, 1.0, 0.0);
/// let p2 = Vec3::new(1.0, 2.0, 0.0);
/// let p3 = Vec3::new(1.0, 3.0, 0.0);
///
/// let start = cubic_bezier_3d(p0, p1, p2, p3, 0.0);
/// assert_eq!(start, p0);
/// ```
pub fn cubic_bezier_3d(p0: Vec3, p1: Vec3, p2: Vec3, p3: Vec3, t: f32) -> Vec3 {
    let t2 = t * t;
    let t3 = t2 * t;
    let mt = 1.0 - t;
    let mt2 = mt * mt;
    let mt3 = mt2 * mt;

    p0 * mt3 + p1 * (3.0 * mt2 * t) + p2 * (3.0 * mt * t2) + p3 * t3
}

/// Calculate the derivative (tangent) of a quadratic Bézier curve at parameter t (2D)
///
/// The derivative gives the tangent direction at any point on the curve.
/// Formula: B'(t) = 2(1-t)·(P₁-P₀) + 2t·(P₂-P₁)
///
/// # Arguments
///
/// * `p0` - Start point
/// * `p1` - Control point
/// * `p2` - End point
/// * `t` - Parameter in range [0, 1]
///
/// # Example
///
/// ```
/// use floraison_core::math::bezier::quadratic_bezier_derivative_2d;
/// use floraison_core::Vec2;
///
/// let p0 = Vec2::new(0.0, 0.0);
/// let p1 = Vec2::new(0.5, 1.0);
/// let p2 = Vec2::new(1.0, 0.0);
///
/// let tangent = quadratic_bezier_derivative_2d(p0, p1, p2, 0.5);
/// let normalized = tangent.normalize();
/// assert!((normalized.length() - 1.0).abs() < 0.001);
/// ```
pub fn quadratic_bezier_derivative_2d(p0: Vec2, p1: Vec2, p2: Vec2, t: f32) -> Vec2 {
    let mt = 1.0 - t;
    (p1 - p0) * (2.0 * mt) + (p2 - p1) * (2.0 * t)
}

/// Calculate the derivative (tangent) of a cubic Bézier curve at parameter t (2D)
///
/// The derivative gives the tangent direction at any point on the curve.
/// Formula: B'(t) = 3(1-t)²·(P₁-P₀) + 6(1-t)t·(P₂-P₁) + 3t²·(P₃-P₂)
///
/// # Arguments
///
/// * `p0` - Start point
/// * `p1` - First control point
/// * `p2` - Second control point
/// * `p3` - End point
/// * `t` - Parameter in range [0, 1]
///
/// # Example
///
/// ```
/// use floraison_core::math::bezier::cubic_bezier_derivative_2d;
/// use floraison_core::Vec2;
///
/// let p0 = Vec2::ZERO;
/// let p1 = Vec2::new(0.0, 1.0);
/// let p2 = Vec2::new(1.0, 1.0);
/// let p3 = Vec2::X;
///
/// let tangent_start = cubic_bezier_derivative_2d(p0, p1, p2, p3, 0.0);
/// let tangent_end = cubic_bezier_derivative_2d(p0, p1, p2, p3, 1.0);
/// ```
pub fn cubic_bezier_derivative_2d(p0: Vec2, p1: Vec2, p2: Vec2, p3: Vec2, t: f32) -> Vec2 {
    let t2 = t * t;
    let mt = 1.0 - t;
    let mt2 = mt * mt;

    (p1 - p0) * (3.0 * mt2) + (p2 - p1) * (6.0 * mt * t) + (p3 - p2) * (3.0 * t2)
}

/// Sample a quadratic Bézier curve at evenly-spaced parameter values (2D)
///
/// Generates `count` points along the curve by evaluating at t = 0, 1/(count-1), 2/(count-1), ..., 1.
///
/// # Arguments
///
/// * `p0` - Start point
/// * `p1` - Control point
/// * `p2` - End point
/// * `count` - Number of samples (must be ≥ 2)
///
/// # Returns
///
/// Vector of sampled points, including start and end points
///
/// # Panics
///
/// Panics if count < 2
///
/// # Example
///
/// ```
/// use floraison_core::math::bezier::sample_quadratic_2d;
/// use floraison_core::Vec2;
///
/// let p0 = Vec2::new(0.0, 0.0);
/// let p1 = Vec2::new(0.5, 1.0);
/// let p2 = Vec2::new(1.0, 0.0);
///
/// let samples = sample_quadratic_2d(p0, p1, p2, 5);
/// assert_eq!(samples.len(), 5);
/// assert_eq!(samples[0], p0);
/// assert_eq!(samples[4], p2);
/// ```
pub fn sample_quadratic_2d(p0: Vec2, p1: Vec2, p2: Vec2, count: usize) -> Vec<Vec2> {
    assert!(count >= 2, "Need at least 2 samples");

    (0..count)
        .map(|i| {
            let t = i as f32 / (count - 1) as f32;
            quadratic_bezier_2d(p0, p1, p2, t)
        })
        .collect()
}

/// Sample a cubic Bézier curve at evenly-spaced parameter values (2D)
///
/// Generates `count` points along the curve by evaluating at t = 0, 1/(count-1), 2/(count-1), ..., 1.
///
/// # Arguments
///
/// * `p0` - Start point
/// * `p1` - First control point
/// * `p2` - Second control point
/// * `p3` - End point
/// * `count` - Number of samples (must be ≥ 2)
///
/// # Returns
///
/// Vector of sampled points, including start and end points
///
/// # Panics
///
/// Panics if count < 2
///
/// # Example
///
/// ```
/// use floraison_core::math::bezier::sample_cubic_2d;
/// use floraison_core::Vec2;
///
/// let p0 = Vec2::new(0.0, 0.0);
/// let p1 = Vec2::new(0.0, 1.0);
/// let p2 = Vec2::new(1.0, 0.0);
/// let p3 = Vec2::new(1.0, 1.0);
///
/// let samples = sample_cubic_2d(p0, p1, p2, p3, 10);
/// assert_eq!(samples.len(), 10);
/// ```
pub fn sample_cubic_2d(p0: Vec2, p1: Vec2, p2: Vec2, p3: Vec2, count: usize) -> Vec<Vec2> {
    assert!(count >= 2, "Need at least 2 samples");

    (0..count)
        .map(|i| {
            let t = i as f32 / (count - 1) as f32;
            cubic_bezier_2d(p0, p1, p2, p3, t)
        })
        .collect()
}

/// Sample a cubic Bézier curve at evenly-spaced parameter values (3D)
///
/// Same as [`sample_cubic_2d`] but for 3D curves.
///
/// # Example
///
/// ```
/// use floraison_core::math::bezier::sample_cubic_3d;
/// use floraison_core::Vec3;
///
/// let p0 = Vec3::ZERO;
/// let p1 = Vec3::new(0.0, 1.0, 0.0);
/// let p2 = Vec3::new(1.0, 2.0, 0.0);
/// let p3 = Vec3::new(1.0, 3.0, 0.0);
///
/// let samples = sample_cubic_3d(p0, p1, p2, p3, 8);
/// assert_eq!(samples.len(), 8);
/// ```
pub fn sample_cubic_3d(p0: Vec3, p1: Vec3, p2: Vec3, p3: Vec3, count: usize) -> Vec<Vec3> {
    assert!(count >= 2, "Need at least 2 samples");

    (0..count)
        .map(|i| {
            let t = i as f32 / (count - 1) as f32;
            cubic_bezier_3d(p0, p1, p2, p3, t)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f32 = 1e-5;

    #[test]
    fn test_quadratic_bezier_2d_endpoints() {
        let p0 = Vec2::new(0.0, 0.0);
        let p1 = Vec2::new(0.5, 1.0);
        let p2 = Vec2::new(1.0, 0.0);

        let start = quadratic_bezier_2d(p0, p1, p2, 0.0);
        assert!((start - p0).length() < EPSILON);

        let end = quadratic_bezier_2d(p0, p1, p2, 1.0);
        assert!((end - p2).length() < EPSILON);
    }

    #[test]
    fn test_quadratic_bezier_2d_midpoint() {
        let p0 = Vec2::new(0.0, 0.0);
        let p1 = Vec2::new(0.5, 1.0);
        let p2 = Vec2::new(1.0, 0.0);

        let mid = quadratic_bezier_2d(p0, p1, p2, 0.5);
        // At t=0.5, quadratic Bezier should be at the average weighted by 1:2:1
        // B(0.5) = 0.25*P0 + 0.5*P1 + 0.25*P2
        let expected = p0 * 0.25 + p1 * 0.5 + p2 * 0.25;
        assert!((mid - expected).length() < EPSILON);
    }

    #[test]
    fn test_cubic_bezier_2d_endpoints() {
        let p0 = Vec2::ZERO;
        let p1 = Vec2::new(0.0, 1.0);
        let p2 = Vec2::new(1.0, 1.0);
        let p3 = Vec2::X;

        let start = cubic_bezier_2d(p0, p1, p2, p3, 0.0);
        assert!((start - p0).length() < EPSILON);

        let end = cubic_bezier_2d(p0, p1, p2, p3, 1.0);
        assert!((end - p3).length() < EPSILON);
    }

    #[test]
    fn test_cubic_bezier_2d_midpoint() {
        let p0 = Vec2::ZERO;
        let p1 = Vec2::new(0.0, 1.0);
        let p2 = Vec2::new(1.0, 1.0);
        let p3 = Vec2::X;

        let mid = cubic_bezier_2d(p0, p1, p2, p3, 0.5);
        // At t=0.5, cubic Bezier should be at the average weighted by 1:3:3:1 / 8
        let expected = (p0 + p1 * 3.0 + p2 * 3.0 + p3) / 8.0;
        assert!((mid - expected).length() < EPSILON);
    }

    #[test]
    fn test_quadratic_bezier_3d() {
        let p0 = Vec3::ZERO;
        let p1 = Vec3::new(0.5, 1.0, 0.5);
        let p2 = Vec3::new(1.0, 0.0, 1.0);

        let start = quadratic_bezier_3d(p0, p1, p2, 0.0);
        assert!((start - p0).length() < EPSILON);

        let end = quadratic_bezier_3d(p0, p1, p2, 1.0);
        assert!((end - p2).length() < EPSILON);

        let mid = quadratic_bezier_3d(p0, p1, p2, 0.5);
        assert!(mid.y > 0.0); // Should peak above the endpoints
    }

    #[test]
    fn test_cubic_bezier_3d() {
        let p0 = Vec3::ZERO;
        let p1 = Vec3::Y;
        let p2 = Vec3::new(1.0, 1.0, 0.0);
        let p3 = Vec3::X;

        let start = cubic_bezier_3d(p0, p1, p2, p3, 0.0);
        assert!((start - p0).length() < EPSILON);

        let end = cubic_bezier_3d(p0, p1, p2, p3, 1.0);
        assert!((end - p3).length() < EPSILON);
    }

    #[test]
    fn test_quadratic_derivative_2d() {
        let p0 = Vec2::ZERO;
        let p1 = Vec2::new(0.5, 1.0);
        let p2 = Vec2::X;

        // Derivative at start should point from p0 towards p1
        let tangent_start = quadratic_bezier_derivative_2d(p0, p1, p2, 0.0);
        let direction_start = (p1 - p0).normalize();
        let tangent_start_normalized = tangent_start.normalize();
        assert!((tangent_start_normalized - direction_start).length() < 0.01);

        // Derivative at end should point from p1 towards p2
        let tangent_end = quadratic_bezier_derivative_2d(p0, p1, p2, 1.0);
        let direction_end = (p2 - p1).normalize();
        let tangent_end_normalized = tangent_end.normalize();
        assert!((tangent_end_normalized - direction_end).length() < 0.01);
    }

    #[test]
    fn test_cubic_derivative_2d() {
        let p0 = Vec2::ZERO;
        let p1 = Vec2::new(0.0, 1.0);
        let p2 = Vec2::new(1.0, 1.0);
        let p3 = Vec2::X;

        // Derivative at start should point from p0 towards p1
        let tangent_start = cubic_bezier_derivative_2d(p0, p1, p2, p3, 0.0);
        let direction_start = (p1 - p0).normalize();
        let tangent_start_normalized = tangent_start.normalize();
        assert!((tangent_start_normalized - direction_start).length() < 0.01);

        // Derivative at end should point from p2 towards p3
        let tangent_end = cubic_bezier_derivative_2d(p0, p1, p2, p3, 1.0);
        let direction_end = (p3 - p2).normalize();
        let tangent_end_normalized = tangent_end.normalize();
        assert!((tangent_end_normalized - direction_end).length() < 0.01);
    }

    #[test]
    fn test_sample_quadratic_2d() {
        let p0 = Vec2::ZERO;
        let p1 = Vec2::new(0.5, 1.0);
        let p2 = Vec2::X;

        let samples = sample_quadratic_2d(p0, p1, p2, 5);
        assert_eq!(samples.len(), 5);
        assert!((samples[0] - p0).length() < EPSILON);
        assert!((samples[4] - p2).length() < EPSILON);

        // Points should progress from p0 to p2
        for i in 1..samples.len() {
            assert!(samples[i].x >= samples[i - 1].x - EPSILON);
        }
    }

    #[test]
    fn test_sample_cubic_2d() {
        let p0 = Vec2::ZERO;
        let p1 = Vec2::new(0.0, 1.0);
        let p2 = Vec2::new(1.0, 1.0);
        let p3 = Vec2::X;

        let samples = sample_cubic_2d(p0, p1, p2, p3, 10);
        assert_eq!(samples.len(), 10);
        assert!((samples[0] - p0).length() < EPSILON);
        assert!((samples[9] - p3).length() < EPSILON);
    }

    #[test]
    fn test_sample_cubic_3d() {
        let p0 = Vec3::ZERO;
        let p1 = Vec3::Y;
        let p2 = Vec3::new(1.0, 2.0, 0.0);
        let p3 = Vec3::new(1.0, 3.0, 0.0);

        let samples = sample_cubic_3d(p0, p1, p2, p3, 8);
        assert_eq!(samples.len(), 8);
        assert!((samples[0] - p0).length() < EPSILON);
        assert!((samples[7] - p3).length() < EPSILON);
    }

    #[test]
    #[should_panic(expected = "Need at least 2 samples")]
    fn test_sample_quadratic_too_few() {
        let p0 = Vec2::ZERO;
        let p1 = Vec2::new(0.5, 1.0);
        let p2 = Vec2::X;
        sample_quadratic_2d(p0, p1, p2, 1);
    }

    #[test]
    #[should_panic(expected = "Need at least 2 samples")]
    fn test_sample_cubic_too_few() {
        let p0 = Vec2::ZERO;
        let p1 = Vec2::new(0.0, 1.0);
        let p2 = Vec2::new(1.0, 1.0);
        let p3 = Vec2::X;
        sample_cubic_2d(p0, p1, p2, p3, 0);
    }

    #[test]
    fn test_linear_degenerate_quadratic() {
        // When p1 is on the line between p0 and p2, curve should be straight
        let p0 = Vec2::ZERO;
        let p1 = Vec2::new(0.5, 0.5);
        let p2 = Vec2::new(1.0, 1.0);

        let samples = sample_quadratic_2d(p0, p1, p2, 5);

        // All points should lie on the line y = x
        for sample in &samples {
            assert!((sample.x - sample.y).abs() < EPSILON);
        }
    }

    #[test]
    fn test_linear_degenerate_cubic() {
        // When all control points are collinear, curve should be straight
        let p0 = Vec2::ZERO;
        let p1 = Vec2::new(0.33, 0.33);
        let p2 = Vec2::new(0.67, 0.67);
        let p3 = Vec2::new(1.0, 1.0);

        let samples = sample_cubic_2d(p0, p1, p2, p3, 5);

        // All points should lie on the line y = x
        for sample in &samples {
            assert!((sample.x - sample.y).abs() < EPSILON);
        }
    }

    #[test]
    fn test_curve_continuity() {
        // Sample curve densely and check for continuity
        let p0 = Vec2::ZERO;
        let p1 = Vec2::new(0.0, 1.0);
        let p2 = Vec2::new(1.0, 1.0);
        let p3 = Vec2::X;

        let samples = sample_cubic_2d(p0, p1, p2, p3, 100);

        // Check that consecutive samples are close together
        for i in 1..samples.len() {
            let dist = (samples[i] - samples[i - 1]).length();
            assert!(
                dist < 0.1,
                "Large gap between samples {} and {}: {}",
                i - 1,
                i,
                dist
            );
        }
    }
}
