//! Sweep surface generation
//!
//! This module provides functions to create 3D meshes by sweeping a 2D profile
//! along a 3D curve. This is used for creating curved stems, styles, and filaments.

use crate::{geometry::mesh::Mesh, Vec2, Vec3};
use std::f32::consts::PI;

/// Sweep a 2D profile along a 3D curve to create a mesh
///
/// The profile is defined in 2D as (radius, offset_along_curve) pairs.
/// At each point along the curve, the profile is:
/// 1. Positioned at the curve point
/// 2. Oriented perpendicular to the curve tangent
/// 3. Revolved around the curve to create a ring of vertices
///
/// # Arguments
///
/// * `profile` - 2D profile points where x=radius, y=offset along curve
/// * `curve` - 3D curve path (should be smoothly sampled)
/// * `segments` - Number of angular divisions around the curve (8-32 typical)
///
/// # Returns
///
/// A mesh with the swept surface geometry
///
/// # Panics
///
/// Panics if:
/// - `profile` is empty
/// - `curve` has fewer than 2 points
/// - `segments` < 3
///
/// # Example
///
/// ```
/// use floraison_core::geometry::sweep::sweep_along_curve;
/// use floraison_core::{Vec2, Vec3};
///
/// // Create a cylindrical profile
/// let profile = vec![
///     Vec2::new(0.1, 0.0),
///     Vec2::new(0.1, 1.0),
/// ];
///
/// // Create a curved path
/// let curve = vec![
///     Vec3::new(0.0, 0.0, 0.0),
///     Vec3::new(0.0, 0.5, 0.1),
///     Vec3::new(0.0, 1.0, 0.3),
/// ];
///
/// let mesh = sweep_along_curve(&profile, &curve, 16, Vec3::ONE);
/// assert!(mesh.vertex_count() > 0);
/// ```
pub fn sweep_along_curve(profile: &[Vec2], curve: &[Vec3], segments: usize, color: Vec3) -> Mesh {
    assert!(!profile.is_empty(), "Profile cannot be empty");
    assert!(curve.len() >= 2, "Curve must have at least 2 points");
    assert!(segments >= 3, "Need at least 3 segments");

    let num_profile_points = profile.len();
    let num_curve_points = curve.len();

    // Pre-allocate mesh capacity
    let vertex_capacity = num_profile_points * num_curve_points * segments;
    let triangle_capacity = (num_profile_points - 1) * num_curve_points * segments * 2 * 3;
    let mut mesh = Mesh::with_capacity(vertex_capacity, triangle_capacity);

    // Compute tangents at each curve point
    let tangents = compute_curve_tangents(&curve);

    // For each point along the curve
    for (curve_idx, (&curve_point, &tangent)) in curve.iter().zip(tangents.iter()).enumerate() {
        // Compute coordinate frame at this curve point
        let (right, up) = compute_orthonormal_frame(tangent);

        // Interpolate profile radius at this curve position
        // Map curve_idx to profile y-coordinate
        let curve_t = curve_idx as f32 / (num_curve_points - 1) as f32;

        // For each profile point
        for profile_point in profile {
            let radius = profile_point.x;

            // Create a ring of vertices around the curve
            for seg_idx in 0..segments {
                let angle = seg_idx as f32 * 2.0 * PI / segments as f32;
                let cos_a = angle.cos();
                let sin_a = angle.sin();

                // Position vertex in circular cross-section
                let local_pos = right * (radius * cos_a) + up * (radius * sin_a);
                let position = curve_point + local_pos;

                // Normal points radially outward from curve
                let normal = (right * cos_a + up * sin_a).normalize();

                // UV coordinates
                let u = seg_idx as f32 / segments as f32;
                let v = curve_t;
                let uv = Vec2::new(u, v);

                mesh.add_vertex(position, normal, uv, color);
            }
        }
    }

    // Generate triangles connecting the rings
    for curve_idx in 0..(num_curve_points - 1) {
        for profile_idx in 0..(num_profile_points - 1) {
            for seg_idx in 0..segments {
                let next_seg = (seg_idx + 1) % segments;

                // Vertex indices for current quad
                let base_idx = (curve_idx * num_profile_points + profile_idx) * segments;
                let next_curve_base = ((curve_idx + 1) * num_profile_points + profile_idx) * segments;

                let i0 = (base_idx + seg_idx) as u32;
                let i1 = (base_idx + next_seg) as u32;
                let i2 = (next_curve_base + seg_idx) as u32;
                let i3 = (next_curve_base + next_seg) as u32;

                // Two triangles per quad
                mesh.add_triangle(i0, i2, i1);
                mesh.add_triangle(i1, i2, i3);
            }
        }
    }

    mesh
}

/// Compute tangent vectors at each point along a curve
///
/// Uses central differences for interior points and forward/backward
/// differences for endpoints.
fn compute_curve_tangents(curve: &[Vec3]) -> Vec<Vec3> {
    let n = curve.len();
    let mut tangents = Vec::with_capacity(n);

    for i in 0..n {
        let tangent = if i == 0 {
            // Forward difference at start
            (curve[1] - curve[0]).normalize()
        } else if i == n - 1 {
            // Backward difference at end
            (curve[n - 1] - curve[n - 2]).normalize()
        } else {
            // Central difference for interior points
            (curve[i + 1] - curve[i - 1]).normalize()
        };

        tangents.push(tangent);
    }

    tangents
}

/// Compute an orthonormal frame (right, up) perpendicular to a tangent vector
///
/// This creates a coordinate system where:
/// - `tangent` is the forward direction
/// - `right` and `up` are perpendicular to tangent and each other
///
/// Uses a stable method that avoids degenerate cases.
fn compute_orthonormal_frame(tangent: Vec3) -> (Vec3, Vec3) {
    // Choose an arbitrary vector not parallel to tangent
    let arbitrary = if tangent.y.abs() < 0.9 {
        Vec3::Y // Use Y if tangent is mostly horizontal
    } else {
        Vec3::X // Use X if tangent is mostly vertical
    };

    // Compute right vector perpendicular to both tangent and arbitrary
    let right = tangent.cross(arbitrary).normalize();

    // Compute up vector to complete the orthonormal frame
    let up = tangent.cross(right).normalize();

    (right, up)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sweep_straight_cylinder() {
        // Cylindrical profile
        let profile = vec![Vec2::new(1.0, 0.0), Vec2::new(1.0, 1.0)];

        // Straight vertical curve
        let curve = vec![
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 2.0, 0.0),
        ];

        let mesh = sweep_along_curve(&profile, &curve, 8, Vec3::ONE);

        assert!(mesh.vertex_count() > 0, "Should have vertices");
        assert!(mesh.triangle_count() > 0, "Should have triangles");

        // Check all vertices are valid
        for pos in &mesh.positions {
            assert!(pos.is_finite(), "Position should be finite");
        }
    }

    #[test]
    fn test_sweep_tapered() {
        // Tapered profile (cone-like)
        let profile = vec![Vec2::new(1.0, 0.0), Vec2::new(0.5, 1.0)];

        // Straight vertical curve
        let curve = vec![Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 2.0, 0.0)];

        let mesh = sweep_along_curve(&profile, &curve, 8, Vec3::ONE);

        assert!(mesh.vertex_count() > 0);
        assert!(mesh.triangle_count() > 0);
    }

    #[test]
    fn test_sweep_curved_path() {
        // Cylindrical profile
        let profile = vec![Vec2::new(0.1, 0.0), Vec2::new(0.1, 1.0)];

        // Curved path
        let curve = vec![
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.1, 0.5, 0.0),
            Vec3::new(0.2, 1.0, 0.1),
            Vec3::new(0.3, 1.5, 0.3),
        ];

        let mesh = sweep_along_curve(&profile, &curve, 12, Vec3::ONE);

        assert!(mesh.vertex_count() > 0);
        assert!(mesh.triangle_count() > 0);

        // Normals should be normalized
        for normal in &mesh.normals {
            let len = normal.length();
            assert!(
                (len - 1.0).abs() < 0.01,
                "Normal should be normalized, got {}",
                len
            );
        }
    }

    #[test]
    fn test_compute_tangents() {
        let curve = vec![
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 2.0, 0.0),
        ];

        let tangents = compute_curve_tangents(&curve);

        assert_eq!(tangents.len(), curve.len());

        // For a straight line along Y, all tangents should point in +Y
        for tangent in &tangents {
            assert!(tangent.y > 0.9, "Tangent should point primarily in +Y");
            let len = tangent.length();
            assert!((len - 1.0).abs() < 1e-5, "Tangent should be normalized");
        }
    }

    #[test]
    fn test_orthonormal_frame() {
        let tangent = Vec3::Y;
        let (right, up) = compute_orthonormal_frame(tangent);

        // Verify orthogonality
        assert!(
            right.dot(tangent).abs() < 1e-5,
            "Right should be perpendicular to tangent"
        );
        assert!(
            up.dot(tangent).abs() < 1e-5,
            "Up should be perpendicular to tangent"
        );
        assert!(
            right.dot(up).abs() < 1e-5,
            "Right and up should be perpendicular"
        );

        // Verify normalization
        assert!((right.length() - 1.0).abs() < 1e-5, "Right should be normalized");
        assert!((up.length() - 1.0).abs() < 1e-5, "Up should be normalized");
    }

    #[test]
    fn test_sweep_normals_point_outward() {
        let profile = vec![Vec2::new(1.0, 0.0), Vec2::new(1.0, 1.0)];
        let curve = vec![Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)];

        let mesh = sweep_along_curve(&profile, &curve, 8, Vec3::ONE);

        // For a cylinder, normals should point radially outward
        // Check a few normals
        for (i, normal) in mesh.normals.iter().enumerate().take(8) {
            // Normals should have significant X or Z component (radial)
            let radial = (normal.x * normal.x + normal.z * normal.z).sqrt();
            assert!(
                radial > 0.9,
                "Normal {} should point radially outward, got {:?}",
                i,
                normal
            );
        }
    }

    #[test]
    #[should_panic(expected = "Profile cannot be empty")]
    fn test_sweep_empty_profile() {
        let profile: Vec<Vec2> = vec![];
        let curve = vec![Vec3::ZERO, Vec3::Y];
        sweep_along_curve(&profile, &curve, 8, Vec3::ONE);
    }

    #[test]
    #[should_panic(expected = "at least 2 points")]
    fn test_sweep_too_few_curve_points() {
        let profile = vec![Vec2::new(1.0, 0.0)];
        let curve = vec![Vec3::ZERO];
        sweep_along_curve(&profile, &curve, 8, Vec3::ONE);
    }
}
