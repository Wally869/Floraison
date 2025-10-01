//! Stamen (male reproductive structure) generator
//!
//! A stamen consists of a filament (thin stalk) topped with an anther (pollen sac).
//! This generator creates a cylindrical filament and an ellipsoid anther.

use crate::{Mesh, Vec2, Vec3, Mat4};
use floraison_core::geometry::surface_revolution::{surface_of_revolution, uv_sphere};
use floraison_core::geometry::sweep::sweep_along_curve;
use floraison_core::math::curves::sample_catmull_rom_curve;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Parameters for stamen generation
///
/// # Example
/// ```
/// use floraison_components::stamen::{StamenParams, generate};
///
/// let params = StamenParams {
///     filament_length: 1.5,
///     filament_radius: 0.05,
///     anther_length: 0.3,
///     anther_width: 0.08,
///     anther_height: 0.08,
///     segments: 10,
///     filament_curve: None,  // Straight filament
/// };
///
/// let mesh = generate(&params);
/// assert!(mesh.vertex_count() > 0);
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StamenParams {
    /// Length of the filament (stalk) - used only for straight filaments
    pub filament_length: f32,

    /// Radius of the filament
    pub filament_radius: f32,

    /// Length of the anther (along Y-axis)
    pub anther_length: f32,

    /// Width of the anther (along X-axis)
    pub anther_width: f32,

    /// Height of the anther (along Z-axis)
    pub anther_height: f32,

    /// Number of segments around the circumference
    pub segments: usize,

    /// Color of the stamen
    pub color: Vec3,

    /// Optional 3D curve for the filament path
    ///
    /// If None, creates a straight vertical filament of length `filament_length`.
    /// If Some, sweeps the filament along the curve (ignores `filament_length` field).
    /// The curve should be specified as Catmull-Rom control points.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub filament_curve: Option<Vec<Vec3>>,
}

impl Default for StamenParams {
    /// Create default parameters for a lily-like stamen
    fn default() -> Self {
        Self {
            filament_length: 1.5,
            filament_radius: 0.04,
            anther_length: 0.25,
            anther_width: 0.07,
            anther_height: 0.07,
            segments: 10,
            color: Vec3::ONE,
            filament_curve: None,
        }
    }
}

impl StamenParams {
    /// Create a short stamen with thick anther
    pub fn short() -> Self {
        Self {
            filament_length: 0.8,
            filament_radius: 0.05,
            anther_length: 0.2,
            anther_width: 0.1,
            anther_height: 0.1,
            segments: 10,
            color: Vec3::ONE,
            filament_curve: None,
        }
    }

    /// Create a long, slender stamen
    pub fn slender() -> Self {
        Self {
            filament_length: 2.5,
            filament_radius: 0.03,
            anther_length: 0.3,
            anther_width: 0.05,
            anther_height: 0.05,
            segments: 8,
            color: Vec3::ONE,
            filament_curve: None,
        }
    }

    /// Create a stamen with elongated anther
    pub fn elongated_anther() -> Self {
        Self {
            filament_length: 1.5,
            filament_radius: 0.04,
            anther_length: 0.4,
            anther_width: 0.06,
            anther_height: 0.06,
            segments: 10,
            color: Vec3::ONE,
            filament_curve: None,
        }
    }
}

/// Generate a stamen mesh from parameters
///
/// Creates a filament with an ellipsoid anther at the top.
/// The filament can be straight (using `filament_length`) or curved (using `filament_curve`).
///
/// # Arguments
///
/// * `params` - Stamen parameters
///
/// # Returns
///
/// A mesh with the complete stamen geometry
///
/// # Example
///
/// ```
/// use floraison_components::stamen::{StamenParams, generate};
///
/// let stamen = generate(&StamenParams::default());
/// assert!(stamen.triangle_count() > 0);
/// ```
pub fn generate(params: &StamenParams) -> Mesh {
    // Generate filament based on whether curve is provided
    let (mut filament, tip_position) = if let Some(ref curve_points) = params.filament_curve {
        // Curved filament: sweep profile along curve
        assert!(
            curve_points.len() >= 4,
            "Filament curve requires at least 4 control points"
        );

        // Sample curve using Catmull-Rom spline
        let sampled_curve = sample_catmull_rom_curve(curve_points, 20);

        // Create cylindrical profile (constant radius)
        let profile = vec![
            Vec2::new(params.filament_radius, 0.0),
            Vec2::new(params.filament_radius, 1.0),
        ];

        let filament_mesh = sweep_along_curve(&profile, &sampled_curve, params.segments, params.color);

        // Tip position is at the end of the curve
        let tip_pos = *sampled_curve.last().unwrap();

        (filament_mesh, tip_pos)
    } else {
        // Straight filament: surface of revolution
        let filament_profile = vec![
            Vec2::new(params.filament_radius, 0.0),
            Vec2::new(params.filament_radius, params.filament_length),
        ];

        let filament_mesh = surface_of_revolution(&filament_profile, params.segments, params.color);
        let tip_pos = Vec3::new(0.0, params.filament_length, 0.0);

        (filament_mesh, tip_pos)
    };

    // Create the anther as a sphere that will be scaled to an ellipsoid
    let anther_base_radius = params.anther_width.max(params.anther_height);
    let mut anther = uv_sphere(anther_base_radius, 6, params.segments, params.color);

    // Scale the sphere to create an ellipsoid
    let scale_x = params.anther_width / anther_base_radius;
    let scale_y = params.anther_length / anther_base_radius;
    let scale_z = params.anther_height / anther_base_radius;

    let anther_scale = Mat4::from_scale(Vec3::new(scale_x, scale_y, scale_z));
    anther.transform(&anther_scale);

    // Position the anther at the tip
    let anther_position = Mat4::from_translation(tip_position);
    anther.transform(&anther_position);

    // Merge filament and anther
    filament.merge(&anther);

    filament
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_stamen() {
        let mesh = generate(&StamenParams::default());

        assert!(mesh.vertex_count() > 0, "Should have vertices");
        assert!(mesh.triangle_count() > 0, "Should have triangles");

        // Check for valid geometry
        for pos in &mesh.positions {
            assert!(pos.is_finite(), "Position should be finite");
        }

        for normal in &mesh.normals {
            assert!(normal.is_finite(), "Normal should be finite");
            let len = normal.length();
            assert!(
                (len - 1.0).abs() < 0.01,
                "Normal should be normalized, got length {}",
                len
            );
        }
    }

    #[test]
    fn test_short_stamen() {
        let mesh = generate(&StamenParams::short());

        assert!(mesh.vertex_count() > 0);

        // Check height is as expected
        let max_y = mesh.positions.iter().map(|p| p.y).fold(0.0f32, f32::max);
        let params = StamenParams::short();

        // Max height should be approximately filament_length + anther_length/2
        assert!(
            (max_y - (params.filament_length + params.anther_length / 2.0)).abs() < 0.3,
            "Max height should be around {}, got {}",
            params.filament_length + params.anther_length / 2.0,
            max_y
        );
    }

    #[test]
    fn test_slender_stamen() {
        let mesh = generate(&StamenParams::slender());

        assert!(mesh.vertex_count() > 0);
        assert!(mesh.triangle_count() > 0);
    }

    #[test]
    fn test_elongated_anther_stamen() {
        let mesh = generate(&StamenParams::elongated_anther());

        assert!(mesh.vertex_count() > 0);
        assert!(mesh.triangle_count() > 0);
    }

    #[test]
    fn test_stamen_has_two_components() {
        let params = StamenParams::default();
        let mesh = generate(&params);

        // Should have vertices from both filament and anther
        // Filament: 2 rings * segments vertices
        // Anther: 7 rings * segments vertices (sphere has 7 rings for 6 subdivisions)

        let filament_vertices = 2 * params.segments;
        let anther_vertices = 7 * params.segments;
        let expected_vertices = filament_vertices + anther_vertices;

        assert_eq!(
            mesh.vertex_count(),
            expected_vertices,
            "Should have vertices from both components"
        );
    }

    #[test]
    fn test_indices_in_bounds() {
        let mesh = generate(&StamenParams::default());
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
    fn test_stamen_height_range() {
        let params = StamenParams {
            filament_length: 2.0,
            filament_radius: 0.05,
            anther_length: 0.3,
            anther_width: 0.08,
            anther_height: 0.08,
            segments: 10,
            color: Vec3::ONE,
            filament_curve: None,
        };

        let mesh = generate(&params);

        // Minimum Y should be at or near 0 (base of filament)
        let min_y = mesh.positions.iter().map(|p| p.y).fold(f32::MAX, f32::min);
        assert!(
            min_y.abs() < 0.1,
            "Min Y should be near 0, got {}",
            min_y
        );

        // Maximum Y should be around filament_length + anther_length/2
        let max_y = mesh.positions.iter().map(|p| p.y).fold(0.0f32, f32::max);
        let expected_max = params.filament_length + params.anther_length / 2.0;
        assert!(
            (max_y - expected_max).abs() < 0.3,
            "Max Y should be around {}, got {}",
            expected_max,
            max_y
        );
    }

    #[test]
    fn test_anther_ellipsoid_shape() {
        let params = StamenParams {
            filament_length: 1.5,
            filament_radius: 0.04,
            anther_length: 0.4,    // Elongated in Y
            anther_width: 0.1,     // Wide in X
            anther_height: 0.06,   // Narrow in Z
            segments: 12,
            color: Vec3::ONE,
            filament_curve: None,
        };

        let mesh = generate(&params);

        // Find anther vertices (near filament_length in Y)
        let anther_vertices: Vec<&Vec3> = mesh
            .positions
            .iter()
            .filter(|p| (p.y - params.filament_length).abs() < params.anther_length)
            .collect();

        assert!(!anther_vertices.is_empty(), "Should have anther vertices");

        // Check that the anther has the expected dimensions
        let max_x = anther_vertices.iter().map(|p| p.x.abs()).fold(0.0f32, f32::max);
        let max_z = anther_vertices.iter().map(|p| p.z.abs()).fold(0.0f32, f32::max);

        // X dimension should be close to anther_width
        assert!(
            (max_x - params.anther_width).abs() < 0.1,
            "Max X {} should be close to anther_width {}",
            max_x,
            params.anther_width
        );

        // Z dimension should be close to anther_height
        assert!(
            (max_z - params.anther_height).abs() < 0.1,
            "Max Z {} should be close to anther_height {}",
            max_z,
            params.anther_height
        );
    }

    #[test]
    fn test_filament_cylindrical() {
        let params = StamenParams::default();
        let mesh = generate(&params);

        // Find filament vertices (below anther)
        let filament_vertices: Vec<&Vec3> = mesh
            .positions
            .iter()
            .filter(|p| p.y < params.filament_length * 0.5)
            .collect();

        assert!(!filament_vertices.is_empty(), "Should have filament vertices");

        // Check that all filament vertices have similar radius
        let radii: Vec<f32> = filament_vertices
            .iter()
            .map(|p| (p.x * p.x + p.z * p.z).sqrt())
            .collect();

        let avg_radius = radii.iter().sum::<f32>() / radii.len() as f32;

        assert!(
            (avg_radius - params.filament_radius).abs() < 0.02,
            "Average filament radius {} should be close to {}",
            avg_radius,
            params.filament_radius
        );
    }

    #[test]
    fn test_curved_stamen() {
        // Create a curved filament using Catmull-Rom control points
        let curve = vec![
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.5, 0.0),
            Vec3::new(0.2, 1.0, 0.1),
            Vec3::new(0.3, 1.5, 0.3),
        ];

        let params = StamenParams {
            filament_length: 2.0, // Ignored when curve is provided
            filament_radius: 0.05,
            anther_length: 0.3,
            anther_width: 0.08,
            anther_height: 0.08,
            segments: 10,
            color: Vec3::ONE,
            filament_curve: Some(curve),
        };

        let mesh = generate(&params);

        assert!(mesh.vertex_count() > 0, "Should have vertices");
        assert!(mesh.triangle_count() > 0, "Should have triangles");

        // Check geometry validity
        for pos in &mesh.positions {
            assert!(pos.is_finite(), "Position should be finite");
        }

        for normal in &mesh.normals {
            assert!(normal.is_finite(), "Normal should be finite");
        }
    }

    #[test]
    #[should_panic(expected = "at least 4 control points")]
    fn test_curved_stamen_too_few_points() {
        let curve = vec![
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        ];

        let params = StamenParams {
            filament_length: 2.0,
            filament_radius: 0.05,
            anther_length: 0.3,
            anther_width: 0.08,
            anther_height: 0.08,
            segments: 10,
            color: Vec3::ONE,
            filament_curve: Some(curve),
        };

        generate(&params); // Should panic
    }
}
