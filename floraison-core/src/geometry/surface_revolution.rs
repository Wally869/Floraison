//! Surface of revolution mesh generation
//!
//! This module provides functions to create 3D meshes by revolving a 2D profile
//! curve around the Y-axis. This is essential for creating rotationally symmetric
//! flower components like receptacles, stems, and ovaries.
//!
//! # Profile Definition
//!
//! A profile is defined as an array of 2D points where:
//! - `x` component = radius from the Y-axis
//! - `y` component = height along the Y-axis
//!
//! Points should be ordered from bottom to top (increasing Y).
//!
//! # Example
//!
//! ```
//! use floraison_core::geometry::surface_revolution::surface_of_revolution;
//! use floraison_core::{Vec2, Vec3};
//!
//! // Create a simple cylinder profile
//! let profile = vec![
//!     Vec2::new(1.0, 0.0),  // Bottom edge
//!     Vec2::new(1.0, 2.0),  // Top edge
//! ];
//!
//! // Revolve around Y-axis with 16 segments
//! let mesh = surface_of_revolution(&profile, 16, Vec3::ONE);
//! assert_eq!(mesh.vertex_count(), 32); // 2 rings × 16 segments
//! ```

use crate::{geometry::mesh::Mesh, Vec2, Vec3};
use std::f32::consts::PI;

/// Generate a mesh by revolving a 2D profile around the Y-axis
///
/// The profile curve is revolved around the vertical (Y) axis to create a
/// rotationally symmetric 3D mesh. This is commonly used for creating receptacles,
/// stems, and other cylindrical or bulbous flower components.
///
/// # Arguments
///
/// * `profile` - Array of 2D points defining the profile curve (x=radius, y=height)
///   - Points should be ordered bottom to top (increasing y)
///   - Use x=0 for closed ends (poles)
/// * `segments` - Number of angular divisions around the axis (typically 8-32)
///   - More segments = smoother appearance but more vertices
///   - Minimum 3 segments required
///
/// # Returns
///
/// A `Mesh` with positions, normals, UVs, and triangulated faces.
/// - Normals are computed from geometry (smooth shading across seams)
/// - UVs: u = angle/(2π), v = normalized height
///
/// # Panics
///
/// Panics if:
/// - Profile is empty
/// - Segments < 3
///
/// # Examples
///
/// ## Cylinder
///
/// ```
/// use floraison_core::geometry::surface_revolution::surface_of_revolution;
/// use floraison_core::{Vec2, Vec3};
///
/// let profile = vec![
///     Vec2::new(1.0, 0.0),
///     Vec2::new(1.0, 5.0),
/// ];
/// let cylinder = surface_of_revolution(&profile, 16, Vec3::ONE);
/// ```
///
/// ## Cone
///
/// ```
/// use floraison_core::geometry::surface_revolution::surface_of_revolution;
/// use floraison_core::{Vec2, Vec3};
///
/// let profile = vec![
///     Vec2::new(1.0, 0.0),  // Wide base
///     Vec2::new(0.0, 2.0),  // Point at top
/// ];
/// let cone = surface_of_revolution(&profile, 16, Vec3::ONE);
/// ```
///
/// ## Sphere (approximation)
///
/// ```
/// use floraison_core::geometry::surface_revolution::surface_of_revolution;
/// use floraison_core::{Vec2, Vec3};
/// use std::f32::consts::PI;
///
/// // Half-circle profile
/// let profile: Vec<_> = (0..=10)
///     .map(|i| {
///         let angle = (i as f32 / 10.0) * PI;
///         Vec2::new(angle.sin(), angle.cos())
///     })
///     .collect();
///
/// let sphere = surface_of_revolution(&profile, 16, Vec3::ONE);
/// ```
pub fn surface_of_revolution(profile: &[Vec2], segments: usize, color: Vec3) -> Mesh {
    assert!(!profile.is_empty(), "Profile cannot be empty");
    assert!(segments >= 3, "Need at least 3 segments for revolution");

    // Pre-allocate mesh capacity for efficiency
    let vertex_count = profile.len() * segments;
    let quad_count = (profile.len() - 1) * segments;
    let index_count = quad_count * 6; // 2 triangles per quad, 3 indices per triangle

    let mut mesh = Mesh::with_capacity(vertex_count, index_count);

    let angle_step = 2.0 * PI / segments as f32;

    // Generate vertices
    // For each point in the profile, create a ring of vertices
    for (ring_idx, &point) in profile.iter().enumerate() {
        let radius = point.x;
        let height = point.y;

        // V coordinate: normalized position along profile (0 at bottom, 1 at top)
        let v = if profile.len() > 1 {
            ring_idx as f32 / (profile.len() - 1) as f32
        } else {
            0.0
        };

        // Create vertices around the ring
        for seg in 0..segments {
            let angle = seg as f32 * angle_step;

            // U coordinate: normalized angle (0 to 1 wrapping around)
            let u = seg as f32 / segments as f32;

            // Position in 3D space
            let x = radius * angle.cos();
            let z = radius * angle.sin();
            let pos = Vec3::new(x, height, z);

            // Normal will be computed later (placeholder for now)
            let normal = Vec3::Y;
            let uv = Vec2::new(u, v);

            mesh.add_vertex(pos, normal, uv, color);
        }
    }

    // Generate faces (triangulation)
    // Connect each ring to the next ring with quads
    for ring_idx in 0..(profile.len() - 1) {
        let r0 = profile[ring_idx].x; // Radius of current ring
        let r1 = profile[ring_idx + 1].x; // Radius of next ring

        let is_bottom_pole = r0.abs() < 1e-6;
        let is_top_pole = r1.abs() < 1e-6;

        for seg in 0..segments {
            let next_seg = (seg + 1) % segments;

            // Vertex indices for the quad
            let i0 = (ring_idx * segments + seg) as u32; // Current ring, current segment
            let i1 = (ring_idx * segments + next_seg) as u32; // Current ring, next segment
            let i2 = ((ring_idx + 1) * segments + next_seg) as u32; // Next ring, next segment
            let i3 = ((ring_idx + 1) * segments + seg) as u32; // Next ring, current segment

            // Handle degenerate cases (poles where radius = 0)
            // Note: winding order is reversed (i0, i3, i2, i1) for outward-facing normals
            if is_bottom_pole && is_top_pole {
                // Both rings are poles - skip (zero-area quad)
                continue;
            } else if is_bottom_pole {
                // Bottom is a pole - create triangle fan from pole to upper ring
                // Triangle: (i0, i3, i2) - reversed for outward normal
                mesh.add_triangle(i0, i3, i2);
            } else if is_top_pole {
                // Top is a pole - create triangle fan from lower ring to pole
                // Triangle: (i0, i2, i1) - reversed for outward normal
                mesh.add_triangle(i0, i2, i1);
            } else {
                // Normal quad - add two triangles with reversed winding
                mesh.add_quad(i0, i3, i2, i1);
            }
        }
    }

    // Compute smooth normals from geometry
    mesh.compute_normals();

    mesh
}

/// Create a cylinder mesh
///
/// A convenience function for creating a simple cylinder.
///
/// # Arguments
///
/// * `radius` - Cylinder radius
/// * `height` - Cylinder height
/// * `segments` - Number of angular divisions
///
/// # Example
///
/// ```
/// use floraison_core::geometry::surface_revolution::cylinder;
/// use floraison_core::Vec3;
///
/// let cyl = cylinder(1.0, 5.0, 16, Vec3::ONE);
/// assert!(cyl.vertex_count() > 0);
/// ```
pub fn cylinder(radius: f32, height: f32, segments: usize, color: Vec3) -> Mesh {
    let profile = vec![Vec2::new(radius, 0.0), Vec2::new(radius, height)];
    surface_of_revolution(&profile, segments, color)
}

/// Create a cone mesh
///
/// A convenience function for creating a cone with a circular base.
///
/// # Arguments
///
/// * `radius` - Base radius
/// * `height` - Cone height
/// * `segments` - Number of angular divisions
///
/// # Example
///
/// ```
/// use floraison_core::geometry::surface_revolution::cone;
/// use floraison_core::Vec3;
///
/// let cone = cone(1.0, 2.0, 16, Vec3::ONE);
/// assert!(cone.vertex_count() > 0);
/// ```
pub fn cone(radius: f32, height: f32, segments: usize, color: Vec3) -> Mesh {
    let profile = vec![Vec2::new(radius, 0.0), Vec2::new(0.0, height)];
    surface_of_revolution(&profile, segments, color)
}

/// Create a UV sphere mesh
///
/// Creates a sphere using a semicircular profile revolved around the Y-axis.
///
/// # Arguments
///
/// * `radius` - Sphere radius
/// * `rings` - Number of horizontal divisions (latitude)
/// * `segments` - Number of vertical divisions (longitude)
///
/// # Example
///
/// ```
/// use floraison_core::geometry::surface_revolution::uv_sphere;
/// use floraison_core::Vec3;
///
/// let sphere = uv_sphere(1.0, 8, 16, Vec3::ONE);
/// assert!(sphere.vertex_count() > 0);
/// ```
pub fn uv_sphere(radius: f32, rings: usize, segments: usize, color: Vec3) -> Mesh {
    assert!(rings >= 2, "Need at least 2 rings for sphere");

    // Create semicircle profile from south pole to north pole
    let profile: Vec<_> = (0..=rings)
        .map(|i| {
            // Angle from south pole (0) to north pole (π)
            let theta = (i as f32 / rings as f32) * PI;
            let r = radius * theta.sin();
            let y = radius * theta.cos();
            Vec2::new(r, y)
        })
        .collect();

    surface_of_revolution(&profile, segments, color)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f32 = 1e-5;

    #[test]
    fn test_cylinder() {
        let mesh = cylinder(1.0, 2.0, 8, Vec3::ONE);

        // Should have 2 rings of 8 vertices each
        assert_eq!(mesh.vertex_count(), 16);

        // Should have 8 quads = 16 triangles
        assert_eq!(mesh.triangle_count(), 16);

        // All vertices should be at radius 1.0 from Y axis
        for pos in &mesh.positions {
            let r = (pos.x * pos.x + pos.z * pos.z).sqrt();
            assert!((r - 1.0).abs() < EPSILON);
        }

        // Heights should be either 0 or 2
        for pos in &mesh.positions {
            assert!((pos.y - 0.0).abs() < EPSILON || (pos.y - 2.0).abs() < EPSILON);
        }
    }

    #[test]
    fn test_cone() {
        let mesh = cone(2.0, 3.0, 8, Vec3::ONE);

        // Should have 2 rings of 8 vertices each
        assert_eq!(mesh.vertex_count(), 16);

        // Should have 8 triangles (degenerate quads at top)
        assert_eq!(mesh.triangle_count(), 8);

        // Bottom ring should be at radius 2.0
        for i in 0..8 {
            let pos = mesh.positions[i];
            let r = (pos.x * pos.x + pos.z * pos.z).sqrt();
            assert!((r - 2.0).abs() < EPSILON);
            assert!((pos.y - 0.0).abs() < EPSILON);
        }

        // Top ring should be at origin (radius 0, height 3)
        for i in 8..16 {
            let pos = mesh.positions[i];
            let r = (pos.x * pos.x + pos.z * pos.z).sqrt();
            assert!(r < EPSILON);
            assert!((pos.y - 3.0).abs() < EPSILON);
        }
    }

    #[test]
    fn test_uv_sphere() {
        let mesh = uv_sphere(1.0, 4, 8, Vec3::ONE);

        // Should have 5 rings (including poles) of 8 vertices each
        assert_eq!(mesh.vertex_count(), 40);

        // All vertices should be approximately at radius 1.0 from origin
        for pos in &mesh.positions {
            let dist = pos.length();
            assert!(
                (dist - 1.0).abs() < 0.01,
                "Vertex at distance {} from origin",
                dist
            );
        }
    }

    #[test]
    fn test_surface_of_revolution_simple() {
        let profile = vec![
            Vec2::new(1.0, 0.0),
            Vec2::new(1.5, 1.0),
            Vec2::new(1.0, 2.0),
        ];

        let mesh = surface_of_revolution(&profile, 6, Vec3::ONE);

        // 3 rings × 6 segments = 18 vertices
        assert_eq!(mesh.vertex_count(), 18);

        // 2 strips × 6 quads = 12 quads = 24 triangles
        assert_eq!(mesh.triangle_count(), 24);
    }

    #[test]
    fn test_uv_mapping() {
        let profile = vec![Vec2::new(1.0, 0.0), Vec2::new(1.0, 1.0)];

        let mesh = surface_of_revolution(&profile, 4, Vec3::ONE);

        // Check UV coordinates
        // Bottom ring should have v=0, top ring should have v=1
        for i in 0..4 {
            assert!((mesh.uvs[i].y - 0.0).abs() < EPSILON);
        }
        for i in 4..8 {
            assert!((mesh.uvs[i].y - 1.0).abs() < EPSILON);
        }

        // U coordinates should range from 0 to 0.75 in steps of 0.25
        for i in 0..4 {
            let expected_u = i as f32 / 4.0;
            assert!((mesh.uvs[i].x - expected_u).abs() < EPSILON);
        }
    }

    #[test]
    fn test_normals_computed() {
        let mesh = cylinder(1.0, 2.0, 8, Vec3::ONE);

        // All normals should be normalized
        for normal in &mesh.normals {
            let len = normal.length();
            assert!(
                (len - 1.0).abs() < EPSILON,
                "Normal length should be 1, got {}",
                len
            );
        }

        // For a cylinder, normals should point radially outward (perpendicular to Y)
        for (pos, normal) in mesh.positions.iter().zip(mesh.normals.iter()) {
            let radial = Vec3::new(pos.x, 0.0, pos.z).normalize();
            // Normal should be close to radial direction
            let dot = normal.dot(radial);
            assert!(
                dot > 0.9,
                "Cylinder normal should point radially, dot product: {}",
                dot
            );
        }
    }

    #[test]
    #[should_panic(expected = "Profile cannot be empty")]
    fn test_empty_profile_panics() {
        surface_of_revolution(&[], 8, Vec3::ONE);
    }

    #[test]
    #[should_panic(expected = "Need at least 3 segments")]
    fn test_too_few_segments_panics() {
        let profile = vec![Vec2::new(1.0, 0.0), Vec2::new(1.0, 1.0)];
        surface_of_revolution(&profile, 2, Vec3::ONE);
    }

    #[test]
    fn test_single_point_profile() {
        // A single point should create a degenerate mesh
        let profile = vec![Vec2::new(1.0, 0.0)];
        let mesh = surface_of_revolution(&profile, 8, Vec3::ONE);

        // Should have 8 vertices (one ring)
        assert_eq!(mesh.vertex_count(), 8);

        // Should have 0 triangles (no second ring to connect to)
        assert_eq!(mesh.triangle_count(), 0);
    }

    #[test]
    fn test_double_cone() {
        // Profile: point -> wide -> point (hourglass/double cone)
        let profile = vec![
            Vec2::new(0.0, 0.0), // Bottom point
            Vec2::new(1.0, 1.0), // Middle wide
            Vec2::new(0.0, 2.0), // Top point
        ];

        let mesh = surface_of_revolution(&profile, 8, Vec3::ONE);

        // 3 rings × 8 segments = 24 vertices
        assert_eq!(mesh.vertex_count(), 24);

        // First strip: 8 triangles (pole to ring)
        // Second strip: 8 triangles (ring to pole)
        // Total: 16 triangles
        assert_eq!(mesh.triangle_count(), 16);
    }

    #[test]
    fn test_bulbous_receptacle() {
        // Simulate a flower receptacle shape (bulge in middle)
        let profile = vec![
            Vec2::new(0.2, 0.0),
            Vec2::new(0.8, 0.3),
            Vec2::new(1.0, 0.6),
            Vec2::new(0.8, 0.9),
            Vec2::new(0.3, 1.2),
        ];

        let mesh = surface_of_revolution(&profile, 12, Vec3::ONE);

        // 5 rings × 12 segments = 60 vertices
        assert_eq!(mesh.vertex_count(), 60);

        // 4 strips × 12 quads = 48 quads = 96 triangles
        assert_eq!(mesh.triangle_count(), 96);

        // All normals should be normalized
        for normal in &mesh.normals {
            assert!((normal.length() - 1.0).abs() < EPSILON);
        }
    }
}
