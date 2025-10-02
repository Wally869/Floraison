//! B-spline curves and surfaces
//!
//! This module provides B-spline basis function evaluation and tensor product
//! surface evaluation for advanced petal geometry with deformations.
//!
//! # B-Splines
//!
//! B-splines are piecewise polynomial curves defined by:
//! - **Control points**: Define the shape
//! - **Degree** (p): Polynomial degree (cubic = 3)
//! - **Knot vector**: Parameter values determining curve segments
//!
//! Unlike Bézier curves, B-splines offer:
//! - Local control (moving one control point affects only nearby curve)
//! - C² continuity (smooth curvature)
//! - Flexible degree and continuity control
//!
//! # Examples
//!
//! ```
//! use floraison_core::math::bspline::{basis_function, generate_knot_vector};
//!
//! // Create knot vector for 5 control points, degree 3
//! let knots = generate_knot_vector(5, 3, true);
//!
//! // Evaluate basis function at parameter u = 0.5
//! let n_i = basis_function(1, 3, 0.5, &knots);
//! ```

use crate::Vec3;

/// Evaluate a B-spline basis function using Cox-de Boor recursion
///
/// The Cox-de Boor formula computes the i-th basis function of degree p
/// at parameter value u, using the given knot vector.
///
/// # Recursive Definition
///
/// ```text
/// Nᵢ,₀(u) = 1 if uᵢ ≤ u < uᵢ₊₁, else 0
///
/// Nᵢ,ₚ(u) = (u - uᵢ)/(uᵢ₊ₚ - uᵢ) · Nᵢ,ₚ₋₁(u)
///         + (uᵢ₊ₚ₊₁ - u)/(uᵢ₊ₚ₊₁ - uᵢ₊₁) · Nᵢ₊₁,ₚ₋₁(u)
/// ```
///
/// Division by zero is handled by treating 0/0 as 0.
///
/// # Arguments
///
/// * `i` - Basis function index (0 to n-1 for n control points)
/// * `p` - Degree of the basis function
/// * `u` - Parameter value to evaluate at
/// * `knots` - Knot vector (must have length n + p + 1)
///
/// # Returns
///
/// The value of the i-th basis function of degree p at parameter u
///
/// # Example
///
/// ```
/// use floraison_core::math::bspline::{basis_function, generate_knot_vector};
///
/// let knots = generate_knot_vector(4, 2, true); // 4 points, degree 2
/// let value = basis_function(1, 2, 0.5, &knots);
/// assert!(value >= 0.0 && value <= 1.0);
/// ```
pub fn basis_function(i: usize, p: usize, u: f32, knots: &[f32]) -> f32 {
    // Bounds checking
    if i + p + 1 >= knots.len() {
        return 0.0;
    }

    // Base case: degree 0 (step function)
    if p == 0 {
        // N_{i,0}(u) = 1 if u_i <= u < u_{i+1}, else 0
        // Exception: if knots[i] == knots[i+1] (degenerate interval), return 0
        // Special case: if u equals the maximum knot value, use closed interval for last valid span

        if (knots[i + 1] - knots[i]).abs() < 1e-10 {
            // Degenerate interval (knots[i] == knots[i+1])
            return 0.0;
        }

        // Normal case: u in [knots[i], knots[i+1])
        if u >= knots[i] && u < knots[i + 1] {
            return 1.0;
        }

        // Special case: u at maximum knot value
        // Find the maximum knot value
        let max_knot = knots[knots.len() - 1];

        // If u equals max knot AND knots[i+1] equals max knot AND this is a valid (non-degenerate) interval
        if (u - max_knot).abs() < 1e-10 && (knots[i + 1] - max_knot).abs() < 1e-10 {
            // This is the last valid interval, use closed interval [knots[i], knots[i+1]]
            if u >= knots[i] && u <= knots[i + 1] {
                return 1.0;
            }
        }

        return 0.0;
    }

    // Recursive case: degree p
    // N_{i,p}(u) = left_term + right_term

    // Left term: (u - u_i) / (u_{i+p} - u_i) * N_{i,p-1}(u)
    let left_num = u - knots[i];
    let left_denom = knots[i + p] - knots[i];

    let left = if left_denom.abs() < 1e-10 {
        0.0 // Treat 0/0 as 0
    } else {
        (left_num / left_denom) * basis_function(i, p - 1, u, knots)
    };

    // Right term: (u_{i+p+1} - u) / (u_{i+p+1} - u_{i+1}) * N_{i+1,p-1}(u)
    let right_num = knots[i + p + 1] - u;
    let right_denom = knots[i + p + 1] - knots[i + 1];

    let right = if right_denom.abs() < 1e-10 {
        0.0 // Treat 0/0 as 0
    } else {
        (right_num / right_denom) * basis_function(i + 1, p - 1, u, knots)
    };

    left + right
}

/// Generate an open uniform B-spline knot vector
///
/// An open uniform knot vector has:
/// - First (p+1) knots = 0
/// - Middle knots uniformly spaced
/// - Last (p+1) knots = 1
///
/// This causes the curve to interpolate the first and last control points.
///
/// # Arguments
///
/// * `n` - Number of control points
/// * `p` - Degree of the B-spline
/// * `uniform` - If true, use uniform spacing; if false, clamped at ends only
///
/// # Returns
///
/// Knot vector of length m = n + p + 1
///
/// # Example
///
/// ```
/// use floraison_core::math::bspline::generate_knot_vector;
///
/// // For 5 control points, degree 3 (cubic)
/// let knots = generate_knot_vector(5, 3, true);
/// assert_eq!(knots.len(), 5 + 3 + 1);
/// assert_eq!(knots[0], 0.0);
/// assert_eq!(knots[knots.len() - 1], 1.0);
/// ```
pub fn generate_knot_vector(n: usize, p: usize, uniform: bool) -> Vec<f32> {
    let m = n + p + 1;
    let mut knots = vec![0.0; m];

    // First p+1 knots are 0
    for item in knots.iter_mut().take(p + 1) {
        *item = 0.0;
    }

    // Middle knots (if any)
    if uniform && n > p {
        for (i, item) in knots.iter_mut().enumerate().take(n).skip(p + 1) {
            *item = (i - p) as f32 / (n - p) as f32;
        }
    }

    // Last p+1 knots are 1
    for item in knots.iter_mut().take(m).skip(n) {
        *item = 1.0;
    }

    knots
}

/// A B-spline surface using tensor product evaluation
///
/// A tensor product surface is defined by a 2D grid of control points
/// and two sets of basis functions (one for each parametric direction).
///
/// The surface is evaluated as:
/// ```text
/// S(u,v) = ΣᵢΣⱼ Pᵢⱼ · Nᵢ,ₚ(u) · Nⱼ,q(v)
/// ```
///
/// # Example
///
/// ```
/// use floraison_core::math::bspline::{BSplineSurface, generate_knot_vector};
/// use floraison_core::Vec3;
///
/// // Create a simple 3x3 control grid
/// let control_points = vec![
///     vec![Vec3::new(-1.0, 0.0, -1.0), Vec3::new(0.0, 0.0, -1.0), Vec3::new(1.0, 0.0, -1.0)],
///     vec![Vec3::new(-1.0, 1.0,  0.0), Vec3::new(0.0, 2.0,  0.0), Vec3::new(1.0, 1.0,  0.0)],
///     vec![Vec3::new(-1.0, 0.0,  1.0), Vec3::new(0.0, 0.0,  1.0), Vec3::new(1.0, 0.0,  1.0)],
/// ];
///
/// let surface = BSplineSurface {
///     control_points,
///     degree_u: 2,
///     degree_v: 2,
///     knots_u: generate_knot_vector(3, 2, true),
///     knots_v: generate_knot_vector(3, 2, true),
/// };
///
/// let point = surface.evaluate(0.5, 0.5);
/// assert!(point.y > 0.0); // Should be above XZ plane
/// ```
#[derive(Debug, Clone)]
pub struct BSplineSurface {
    /// Control points in a 2D grid: control_points[i][j]
    /// Rows (i) correspond to u direction, columns (j) to v direction
    pub control_points: Vec<Vec<Vec3>>,

    /// Degree in u direction (typically 3 for cubic)
    pub degree_u: usize,

    /// Degree in v direction (typically 3 for cubic)
    pub degree_v: usize,

    /// Knot vector in u direction
    pub knots_u: Vec<f32>,

    /// Knot vector in v direction
    pub knots_v: Vec<f32>,
}

impl BSplineSurface {
    /// Evaluate the surface at parameters (u, v)
    ///
    /// # Arguments
    ///
    /// * `u` - Parameter in u direction (typically 0.0 to 1.0)
    /// * `v` - Parameter in v direction (typically 0.0 to 1.0)
    ///
    /// # Returns
    ///
    /// 3D point on the surface
    pub fn evaluate(&self, u: f32, v: f32) -> Vec3 {
        let n = self.control_points.len();
        let m = self.control_points[0].len();

        let mut point = Vec3::ZERO;

        for i in 0..n {
            let basis_u = basis_function(i, self.degree_u, u, &self.knots_u);

            // Skip if basis is zero (optimization)
            if basis_u.abs() < 1e-10 {
                continue;
            }

            for j in 0..m {
                let basis_v = basis_function(j, self.degree_v, v, &self.knots_v);

                // Skip if basis is zero (optimization)
                if basis_v.abs() < 1e-10 {
                    continue;
                }

                point += self.control_points[i][j] * basis_u * basis_v;
            }
        }

        point
    }

    /// Evaluate the partial derivative with respect to u
    ///
    /// Returns the tangent vector in the u direction.
    ///
    /// # Arguments
    ///
    /// * `u` - Parameter in u direction
    /// * `v` - Parameter in v direction
    ///
    /// # Returns
    ///
    /// Tangent vector ∂S/∂u
    pub fn evaluate_derivative_u(&self, u: f32, v: f32) -> Vec3 {
        // Numerical derivative using finite differences
        let h = 0.001;
        let u_plus = (u + h).min(1.0);
        let u_minus = (u - h).max(0.0);

        let p_plus = self.evaluate(u_plus, v);
        let p_minus = self.evaluate(u_minus, v);

        (p_plus - p_minus) / (u_plus - u_minus)
    }

    /// Evaluate the partial derivative with respect to v
    ///
    /// Returns the tangent vector in the v direction.
    ///
    /// # Arguments
    ///
    /// * `u` - Parameter in u direction
    /// * `v` - Parameter in v direction
    ///
    /// # Returns
    ///
    /// Tangent vector ∂S/∂v
    pub fn evaluate_derivative_v(&self, u: f32, v: f32) -> Vec3 {
        // Numerical derivative using finite differences
        let h = 0.001;
        let v_plus = (v + h).min(1.0);
        let v_minus = (v - h).max(0.0);

        let p_plus = self.evaluate(u, v_plus);
        let p_minus = self.evaluate(u, v_minus);

        (p_plus - p_minus) / (v_plus - v_minus)
    }

    /// Compute the surface normal at parameters (u, v)
    ///
    /// The normal is computed as the cross product of the two tangent vectors.
    ///
    /// # Arguments
    ///
    /// * `u` - Parameter in u direction
    /// * `v` - Parameter in v direction
    ///
    /// # Returns
    ///
    /// Unit normal vector perpendicular to the surface
    pub fn normal(&self, u: f32, v: f32) -> Vec3 {
        let tangent_u = self.evaluate_derivative_u(u, v);
        let tangent_v = self.evaluate_derivative_v(u, v);

        let normal = tangent_u.cross(tangent_v);

        // Normalize (handle degenerate case)
        let length = normal.length();
        if length > 1e-6 {
            normal / length
        } else {
            Vec3::Y // Fallback to Y-up
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f32 = 1e-5;

    // Tests for basis_function

    #[test]
    fn test_basis_function_degree_0() {
        let knots = vec![0.0, 0.5, 1.0];

        // At u=0.25, should be in first interval [0.0, 0.5)
        assert_eq!(basis_function(0, 0, 0.25, &knots), 1.0);
        assert_eq!(basis_function(1, 0, 0.25, &knots), 0.0);

        // At u=0.75, should be in second interval [0.5, 1.0)
        assert_eq!(basis_function(0, 0, 0.75, &knots), 0.0);
        assert_eq!(basis_function(1, 0, 0.75, &knots), 1.0);
    }

    #[test]
    fn test_basis_function_partition_of_unity() {
        // For any u, the sum of all basis functions should equal 1
        let n = 5;
        let p = 3;
        let knots = generate_knot_vector(n, p, true);

        let test_values = [0.0, 0.1, 0.25, 0.5, 0.75, 0.9, 1.0];

        for &u in &test_values {
            let sum: f32 = (0..n).map(|i| basis_function(i, p, u, &knots)).sum();
            assert!(
                (sum - 1.0).abs() < EPSILON,
                "Partition of unity failed at u={}: sum={}",
                u,
                sum
            );
        }
    }

    #[test]
    fn test_basis_function_local_support() {
        // N_{i,p}(u) should be zero outside [u_i, u_{i+p+1}]
        let n = 5;
        let p = 2;
        let knots = generate_knot_vector(n, p, true);

        // Basis function 0 should have support on [knots[0], knots[3]]
        // With open uniform knots for n=5, p=2: [0, 0, 0, 0.5, 1, 1, 1]
        // N_{0,2} has support on [u_0, u_3] = [0, 0.5]

        // Test at u=0.6, which is outside [0, 0.5]
        let value_outside = basis_function(0, p, 0.6, &knots);
        assert!(
            value_outside.abs() < EPSILON,
            "Basis should be zero outside support: got {}",
            value_outside
        );

        // Test at u=0.3, which is inside [0, 0.5]
        let value_inside = basis_function(0, p, 0.3, &knots);
        assert!(
            value_inside > EPSILON,
            "Basis should be non-zero inside support"
        );
    }

    #[test]
    fn test_basis_function_non_negative() {
        let n = 5;
        let p = 3;
        let knots = generate_knot_vector(n, p, true);

        // All basis functions should be non-negative
        for u_steps in 0..=20 {
            let u = u_steps as f32 / 20.0;
            for i in 0..n {
                let value = basis_function(i, p, u, &knots);
                assert!(value >= -EPSILON, "Basis function should be non-negative");
            }
        }
    }

    // Tests for generate_knot_vector

    #[test]
    fn test_generate_knot_vector_length() {
        let n = 5;
        let p = 3;
        let knots = generate_knot_vector(n, p, true);

        assert_eq!(knots.len(), n + p + 1);
    }

    #[test]
    fn test_generate_knot_vector_clamped() {
        let n = 5;
        let p = 3;
        let knots = generate_knot_vector(n, p, true);

        // First p+1 knots should be 0
        for item in knots.iter().take(p + 1) {
            assert_eq!(*item, 0.0);
        }

        // Last p+1 knots should be 1
        for item in knots.iter().take(n + p + 1).skip(n) {
            assert_eq!(*item, 1.0);
        }
    }

    #[test]
    fn test_generate_knot_vector_uniform_spacing() {
        let n = 7;
        let p = 3;
        let knots = generate_knot_vector(n, p, true);

        // Check that middle knots are uniformly spaced
        // For n=7, p=3: knots should be [0,0,0,0, 0.25, 0.5, 0.75, 1,1,1,1]
        assert_eq!(knots[4], 0.25);
        assert_eq!(knots[5], 0.5);
        assert_eq!(knots[6], 0.75);
    }

    #[test]
    fn test_generate_knot_vector_monotonic() {
        let n = 10;
        let p = 3;
        let knots = generate_knot_vector(n, p, true);

        // Knot vector must be non-decreasing
        for i in 1..knots.len() {
            assert!(
                knots[i] >= knots[i - 1],
                "Knot vector must be non-decreasing"
            );
        }
    }

    // Tests for BSplineSurface

    #[test]
    fn test_bspline_surface_corner_interpolation() {
        // For open uniform knots, surface should interpolate corner control points
        let control_points = vec![
            vec![
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(2.0, 0.0, 0.0),
            ],
            vec![
                Vec3::new(0.0, 1.0, 1.0),
                Vec3::new(1.0, 2.0, 1.0),
                Vec3::new(2.0, 1.0, 1.0),
            ],
            vec![
                Vec3::new(0.0, 0.0, 2.0),
                Vec3::new(1.0, 0.0, 2.0),
                Vec3::new(2.0, 0.0, 2.0),
            ],
        ];

        let surface = BSplineSurface {
            control_points: control_points.clone(),
            degree_u: 2,
            degree_v: 2,
            knots_u: generate_knot_vector(3, 2, true),
            knots_v: generate_knot_vector(3, 2, true),
        };

        // Test corner interpolation
        let p00 = surface.evaluate(0.0, 0.0);
        assert!(
            (p00 - control_points[0][0]).length() < 0.1,
            "Should interpolate corner (0,0)"
        );

        let p01 = surface.evaluate(0.0, 1.0);
        assert!(
            (p01 - control_points[0][2]).length() < 0.1,
            "Should interpolate corner (0,1)"
        );

        let p10 = surface.evaluate(1.0, 0.0);
        assert!(
            (p10 - control_points[2][0]).length() < 0.1,
            "Should interpolate corner (1,0)"
        );

        let p11 = surface.evaluate(1.0, 1.0);
        assert!(
            (p11 - control_points[2][2]).length() < 0.1,
            "Should interpolate corner (1,1)"
        );
    }

    #[test]
    fn test_bspline_surface_normals_non_zero() {
        let control_points = vec![
            vec![
                Vec3::new(-1.0, 0.0, -1.0),
                Vec3::new(0.0, 0.0, -1.0),
                Vec3::new(1.0, 0.0, -1.0),
            ],
            vec![
                Vec3::new(-1.0, 1.0, 0.0),
                Vec3::new(0.0, 2.0, 0.0),
                Vec3::new(1.0, 1.0, 0.0),
            ],
            vec![
                Vec3::new(-1.0, 0.0, 1.0),
                Vec3::new(0.0, 0.0, 1.0),
                Vec3::new(1.0, 0.0, 1.0),
            ],
        ];

        let surface = BSplineSurface {
            control_points,
            degree_u: 2,
            degree_v: 2,
            knots_u: generate_knot_vector(3, 2, true),
            knots_v: generate_knot_vector(3, 2, true),
        };

        // Test that normals are unit vectors
        for u_steps in 0..=5 {
            for v_steps in 0..=5 {
                let u = u_steps as f32 / 5.0;
                let v = v_steps as f32 / 5.0;

                let normal = surface.normal(u, v);
                let length = normal.length();

                assert!(
                    (length - 1.0).abs() < 0.1,
                    "Normal should be unit length at ({}, {}): got length {}",
                    u,
                    v,
                    length
                );
            }
        }
    }

    #[test]
    fn test_bspline_surface_flat_plane() {
        // All control points in XZ plane should produce flat surface
        let control_points = vec![
            vec![
                Vec3::new(-1.0, 0.0, -1.0),
                Vec3::new(0.0, 0.0, -1.0),
                Vec3::new(1.0, 0.0, -1.0),
            ],
            vec![
                Vec3::new(-1.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(1.0, 0.0, 0.0),
            ],
            vec![
                Vec3::new(-1.0, 0.0, 1.0),
                Vec3::new(0.0, 0.0, 1.0),
                Vec3::new(1.0, 0.0, 1.0),
            ],
        ];

        let surface = BSplineSurface {
            control_points,
            degree_u: 2,
            degree_v: 2,
            knots_u: generate_knot_vector(3, 2, true),
            knots_v: generate_knot_vector(3, 2, true),
        };

        // All points should have Y ≈ 0
        for u_steps in 0..=10 {
            for v_steps in 0..=10 {
                let u = u_steps as f32 / 10.0;
                let v = v_steps as f32 / 10.0;

                let point = surface.evaluate(u, v);
                assert!(
                    point.y.abs() < 0.01,
                    "Flat surface should have Y ≈ 0 at ({}, {}): got Y={}",
                    u,
                    v,
                    point.y
                );
            }
        }
    }

    #[test]
    fn test_bspline_surface_convex_hull() {
        // Surface should lie within convex hull of control points
        let control_points = vec![
            vec![
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(2.0, 0.0, 0.0),
            ],
            vec![
                Vec3::new(0.0, 0.0, 1.0),
                Vec3::new(1.0, 1.0, 1.0),
                Vec3::new(2.0, 0.0, 1.0),
            ],
            vec![
                Vec3::new(0.0, 0.0, 2.0),
                Vec3::new(1.0, 0.0, 2.0),
                Vec3::new(2.0, 0.0, 2.0),
            ],
        ];

        let surface = BSplineSurface {
            control_points,
            degree_u: 2,
            degree_v: 2,
            knots_u: generate_knot_vector(3, 2, true),
            knots_v: generate_knot_vector(3, 2, true),
        };

        // All surface points should satisfy bounds
        for u_steps in 0..=10 {
            for v_steps in 0..=10 {
                let u = u_steps as f32 / 10.0;
                let v = v_steps as f32 / 10.0;

                let point = surface.evaluate(u, v);

                assert!(point.x >= -0.1 && point.x <= 2.1, "X should be in bounds");
                assert!(point.y >= -0.1 && point.y <= 1.1, "Y should be in bounds");
                assert!(point.z >= -0.1 && point.z <= 2.1, "Z should be in bounds");
            }
        }
    }
}
