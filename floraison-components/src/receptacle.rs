//! Receptacle (flower base) generator
//!
//! The receptacle is the base structure of a flower where all other components attach.
//! It's generated using surface of revolution with a smooth Bézier curve profile.

use crate::{Mesh, Vec2, Vec3};
use floraison_core::geometry::surface_revolution::surface_of_revolution;
use floraison_core::math::bezier::sample_cubic_2d;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Parameters for receptacle generation
///
/// The receptacle profile is defined by a cubic Bézier curve that can create
/// various natural shapes: flat, convex (bulbous), or concave.
///
/// # Example
/// ```
/// use floraison_components::receptacle::{ReceptacleParams, generate};
/// use floraison_components::Vec3;
///
/// // Create a bulbous receptacle
/// let params = ReceptacleParams {
///     height: 1.0,
///     base_radius: 0.3,
///     bulge_radius: 0.5,
///     top_radius: 0.2,
///     bulge_position: 0.6,
///     segments: 16,
///     profile_samples: 8,
///     color: Vec3::ONE,
/// };
///
/// let mesh = generate(&params);
/// assert!(mesh.vertex_count() > 0);
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ReceptacleParams {
    /// Total height of the receptacle
    pub height: f32,

    /// Radius at the bottom (base)
    pub base_radius: f32,

    /// Radius at the widest point (bulge)
    pub bulge_radius: f32,

    /// Radius at the top
    pub top_radius: f32,

    /// Height where the bulge occurs (0.0 = bottom, 1.0 = top)
    pub bulge_position: f32,

    /// Number of segments around the circumference (typically 12-32)
    pub segments: usize,

    /// Number of samples along the profile curve (affects smoothness)
    pub profile_samples: usize,

    /// Color of the receptacle
    pub color: Vec3,
}

impl Default for ReceptacleParams {
    /// Create default parameters for a lily-like receptacle
    ///
    /// Creates a slightly bulbous receptacle with smooth taper
    fn default() -> Self {
        Self {
            height: 1.0,
            base_radius: 0.25,
            bulge_radius: 0.35,
            top_radius: 0.15,
            bulge_position: 0.5,
            segments: 16,
            profile_samples: 8,
            color: Vec3::ONE,
        }
    }
}

impl ReceptacleParams {
    /// Create a flat disc-like receptacle
    pub fn flat() -> Self {
        Self {
            height: 0.2,
            base_radius: 0.5,
            bulge_radius: 0.5,
            top_radius: 0.5,
            bulge_position: 0.5,
            segments: 16,
            profile_samples: 4,
            color: Vec3::ONE,
        }
    }

    /// Create a convex (bulbous) receptacle
    pub fn convex() -> Self {
        Self {
            height: 1.2,
            base_radius: 0.2,
            bulge_radius: 0.6,
            top_radius: 0.25,
            bulge_position: 0.6,
            segments: 20,
            profile_samples: 10,
            color: Vec3::ONE,
        }
    }

    /// Create a concave (cup-like) receptacle
    pub fn concave() -> Self {
        Self {
            height: 0.8,
            base_radius: 0.4,
            bulge_radius: 0.3,
            top_radius: 0.5,
            bulge_position: 0.3,
            segments: 16,
            profile_samples: 8,
            color: Vec3::ONE,
        }
    }
}

/// Generate a receptacle mesh from parameters
///
/// Uses a cubic Bézier curve to define the profile, then revolves it around
/// the Y-axis to create the 3D mesh.
///
/// # Arguments
///
/// * `params` - Receptacle parameters
///
/// # Returns
///
/// A mesh with the receptacle geometry
///
/// # Example
///
/// ```
/// use floraison_components::receptacle::{ReceptacleParams, generate};
///
/// let receptacle = generate(&ReceptacleParams::default());
/// assert!(receptacle.triangle_count() > 0);
/// ```
pub fn generate(params: &ReceptacleParams) -> Mesh {
    // Define Bézier control points for the profile curve
    // p0: base, p1: lower control, p2: upper control, p3: top

    let p0 = Vec2::new(params.base_radius, 0.0);

    // First control point influences the lower portion
    // Positioned at ~20% height, slightly pulled toward bulge radius
    let p1 = Vec2::new(
        params.base_radius + (params.bulge_radius - params.base_radius) * 0.3,
        params.height * 0.2,
    );

    // Second control point influences the upper portion
    // Positioned at bulge height, at bulge radius
    let p2 = Vec2::new(params.bulge_radius, params.height * params.bulge_position);

    let p3 = Vec2::new(params.top_radius, params.height);

    // Sample the Bézier curve to get profile points
    let profile = sample_cubic_2d(p0, p1, p2, p3, params.profile_samples);

    // Revolve the profile around the Y-axis
    surface_of_revolution(&profile, params.segments, params.color)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_receptacle() {
        let mesh = generate(&ReceptacleParams::default());

        assert!(mesh.vertex_count() > 0, "Should have vertices");
        assert!(mesh.triangle_count() > 0, "Should have triangles");

        // Check for NaN or infinite values
        for pos in &mesh.positions {
            assert!(pos.is_finite(), "Position should be finite: {:?}", pos);
        }

        for normal in &mesh.normals {
            assert!(normal.is_finite(), "Normal should be finite: {:?}", normal);
            let len = normal.length();
            assert!(
                (len - 1.0).abs() < 0.01,
                "Normal should be normalized, got length {}",
                len
            );
        }
    }

    #[test]
    fn test_flat_receptacle() {
        let mesh = generate(&ReceptacleParams::flat());

        assert!(mesh.vertex_count() > 0);
        assert!(mesh.triangle_count() > 0);

        // Flat receptacle should be short
        let max_height = mesh.positions.iter().map(|p| p.y).fold(0.0f32, f32::max);
        assert!(max_height < 0.5, "Flat receptacle should be low");
    }

    #[test]
    fn test_convex_receptacle() {
        let mesh = generate(&ReceptacleParams::convex());

        assert!(mesh.vertex_count() > 0);

        // Convex should have vertices at various radii
        let radii: Vec<f32> = mesh
            .positions
            .iter()
            .map(|p| (p.x * p.x + p.z * p.z).sqrt())
            .collect();

        let max_radius = radii.iter().fold(0.0f32, |a, &b| a.max(b));
        let min_radius = radii.iter().fold(f32::MAX, |a, &b| a.min(b));

        // Should have variation in radius (bulge)
        assert!(
            max_radius - min_radius > 0.2,
            "Should have significant radius variation"
        );
    }

    #[test]
    fn test_concave_receptacle() {
        let mesh = generate(&ReceptacleParams::concave());

        assert!(mesh.vertex_count() > 0);
        assert!(mesh.triangle_count() > 0);
    }

    #[test]
    fn test_mesh_topology() {
        let params = ReceptacleParams {
            height: 1.0,
            base_radius: 0.5,
            bulge_radius: 0.6,
            top_radius: 0.3,
            bulge_position: 0.5,
            segments: 8,
            profile_samples: 4,
            color: Vec3::ONE,
        };

        let mesh = generate(&params);

        // Vertex count should be profile_samples * segments
        assert_eq!(
            mesh.vertex_count(),
            params.profile_samples * params.segments
        );

        // Triangle count should be (profile_samples - 1) * segments * 2
        let expected_triangles = (params.profile_samples - 1) * params.segments * 2;
        assert_eq!(mesh.triangle_count(), expected_triangles);
    }

    #[test]
    fn test_indices_in_bounds() {
        let mesh = generate(&ReceptacleParams::default());
        let vertex_count = mesh.vertex_count() as u32;

        for &index in &mesh.indices {
            assert!(
                index < vertex_count,
                "Index {} out of bounds (vertex count: {})",
                index,
                vertex_count
            );
        }
    }

    #[test]
    fn test_uv_mapping() {
        let mesh = generate(&ReceptacleParams::default());

        // All UVs should be in [0, 1] range
        for uv in &mesh.uvs {
            assert!(
                uv.x >= 0.0 && uv.x <= 1.0,
                "UV x coordinate should be in [0,1]: {}",
                uv.x
            );
            assert!(
                uv.y >= 0.0 && uv.y <= 1.0,
                "UV y coordinate should be in [0,1]: {}",
                uv.y
            );
        }
    }

    #[test]
    fn test_height_bounds() {
        let params = ReceptacleParams {
            height: 2.5,
            ..ReceptacleParams::default()
        };

        let mesh = generate(&params);

        // All vertices should be within height bounds
        for pos in &mesh.positions {
            assert!(
                pos.y >= 0.0 && pos.y <= params.height + 0.01,
                "Y position {} should be in [0, {}]",
                pos.y,
                params.height
            );
        }
    }
}
