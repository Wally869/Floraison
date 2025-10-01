//! Pistil (female reproductive structure) generator
//!
//! A pistil consists of a style (elongated stalk) topped with a stigma (receptive surface).
//! This generator creates a simple tapered cylinder for the style and a sphere for the stigma.

use crate::{Mesh, Vec2, Vec3, Mat4};
use floraison_core::geometry::surface_revolution::{surface_of_revolution, uv_sphere};
use floraison_core::geometry::sweep::sweep_along_curve;
use floraison_core::math::curves::sample_catmull_rom_curve;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Parameters for pistil generation
///
/// # Example
/// ```
/// use floraison_components::pistil::{PistilParams, generate};
/// use floraison_components::Vec3;
///
/// let params = PistilParams {
///     length: 2.0,
///     base_radius: 0.1,
///     tip_radius: 0.08,
///     stigma_radius: 0.15,
///     segments: 12,
///     color: Vec3::ONE,
///     style_curve: None,  // Straight style
/// };
///
/// let mesh = generate(&params);
/// assert!(mesh.vertex_count() > 0);
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PistilParams {
    /// Length of the style (stalk) - used only for straight styles
    pub length: f32,

    /// Radius at the base of the style
    pub base_radius: f32,

    /// Radius at the tip of the style
    pub tip_radius: f32,

    /// Radius of the stigma (sphere at top)
    pub stigma_radius: f32,

    /// Number of segments around the circumference
    pub segments: usize,

    /// Color of the pistil
    pub color: Vec3,

    /// Optional 3D curve for the style path
    ///
    /// If None, creates a straight vertical style of length `length`.
    /// If Some, sweeps the style profile along the curve (ignores `length` field).
    /// The curve should be specified as Catmull-Rom control points.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub style_curve: Option<Vec<Vec3>>,
}

impl Default for PistilParams {
    /// Create default parameters for a lily-like pistil
    fn default() -> Self {
        Self {
            length: 2.0,
            base_radius: 0.08,
            tip_radius: 0.06,
            stigma_radius: 0.12,
            segments: 12,
            color: Vec3::ONE,
            style_curve: None,  // Straight style
        }
    }
}

impl PistilParams {
    /// Create a short, thick pistil
    pub fn short() -> Self {
        Self {
            length: 1.0,
            base_radius: 0.15,
            tip_radius: 0.12,
            stigma_radius: 0.2,
            segments: 12,
            color: Vec3::ONE,
            style_curve: None,
        }
    }

    /// Create a long, slender pistil
    pub fn slender() -> Self {
        Self {
            length: 3.0,
            base_radius: 0.05,
            tip_radius: 0.04,
            stigma_radius: 0.08,
            segments: 10,
            color: Vec3::ONE,
            style_curve: None,
        }
    }
}

/// Generate a pistil mesh from parameters
///
/// Creates a tapered style with a spherical stigma at the top.
/// The style can be straight (using `length`) or curved (using `style_curve`).
///
/// # Arguments
///
/// * `params` - Pistil parameters
///
/// # Returns
///
/// A mesh with the complete pistil geometry
///
/// # Example
///
/// ```
/// use floraison_components::pistil::{PistilParams, generate};
///
/// let pistil = generate(&PistilParams::default());
/// assert!(pistil.triangle_count() > 0);
/// ```
pub fn generate(params: &PistilParams) -> Mesh {
    // Generate style based on whether curve is provided
    let (mut style, tip_position) = if let Some(ref curve_points) = params.style_curve {
        // Curved style: sweep profile along curve
        assert!(
            curve_points.len() >= 4,
            "Style curve requires at least 4 control points"
        );

        // Sample curve using Catmull-Rom spline
        let sampled_curve = sample_catmull_rom_curve(curve_points, 20);

        // Create tapered profile (radius, offset_along_curve)
        let profile = vec![
            Vec2::new(params.base_radius, 0.0),
            Vec2::new(params.tip_radius, 1.0),
        ];

        let style_mesh = sweep_along_curve(&profile, &sampled_curve, params.segments, params.color);

        // Tip position is at the end of the curve
        let tip_pos = *sampled_curve.last().unwrap();

        (style_mesh, tip_pos)
    } else {
        // Straight style: surface of revolution
        let style_profile = vec![
            Vec2::new(params.base_radius, 0.0),
            Vec2::new(params.tip_radius, params.length),
        ];

        let style_mesh = surface_of_revolution(&style_profile, params.segments, params.color);
        let tip_pos = Vec3::new(0.0, params.length, 0.0);

        (style_mesh, tip_pos)
    };

    // Create the stigma (sphere)
    let mut stigma = uv_sphere(params.stigma_radius, 6, params.segments, params.color);

    // Position the stigma at the tip
    let stigma_position = Mat4::from_translation(tip_position);
    stigma.transform(&stigma_position);

    // Merge style and stigma
    style.merge(&stigma);

    style
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_pistil() {
        let mesh = generate(&PistilParams::default());

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
    fn test_short_pistil() {
        let mesh = generate(&PistilParams::short());

        assert!(mesh.vertex_count() > 0);

        // Check height is as expected
        let max_y = mesh.positions.iter().map(|p| p.y).fold(0.0f32, f32::max);
        let params = PistilParams::short();

        // Max height should be approximately length + stigma_radius
        assert!(
            (max_y - (params.length + params.stigma_radius)).abs() < 0.5,
            "Max height should be around {}, got {}",
            params.length + params.stigma_radius,
            max_y
        );
    }

    #[test]
    fn test_slender_pistil() {
        let mesh = generate(&PistilParams::slender());

        assert!(mesh.vertex_count() > 0);
        assert!(mesh.triangle_count() > 0);
    }

    #[test]
    fn test_pistil_has_two_components() {
        let params = PistilParams::default();
        let mesh = generate(&params);

        // Should have vertices from both style and stigma
        // Style: 2 rings * segments vertices
        // Stigma: (rings+1) * segments vertices (sphere has 7 rings for 6 subdivisions)

        let style_vertices = 2 * params.segments;
        let stigma_vertices = 7 * params.segments; // uv_sphere with 6 rings
        let expected_vertices = style_vertices + stigma_vertices;

        assert_eq!(
            mesh.vertex_count(),
            expected_vertices,
            "Should have vertices from both components"
        );
    }

    #[test]
    fn test_indices_in_bounds() {
        let mesh = generate(&PistilParams::default());
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
    fn test_pistil_height_range() {
        let params = PistilParams {
            length: 2.5,
            base_radius: 0.1,
            tip_radius: 0.08,
            stigma_radius: 0.15,
            segments: 12,
            style_curve: None,
            color: Vec3::ONE,
        };

        let mesh = generate(&params);

        // Minimum Y should be at or near 0 (base of style)
        let min_y = mesh.positions.iter().map(|p| p.y).fold(f32::MAX, f32::min);
        assert!(
            min_y.abs() < 0.1,
            "Min Y should be near 0, got {}",
            min_y
        );

        // Maximum Y should be around length + stigma_radius
        let max_y = mesh.positions.iter().map(|p| p.y).fold(0.0f32, f32::max);
        assert!(
            (max_y - (params.length + params.stigma_radius)).abs() < 0.5,
            "Max Y should be around {}, got {}",
            params.length + params.stigma_radius,
            max_y
        );
    }

    #[test]
    fn test_style_taper() {
        let params = PistilParams {
            length: 2.0,
            base_radius: 0.2,
            tip_radius: 0.1,
            stigma_radius: 0.15,
            segments: 12,
            style_curve: None,
            color: Vec3::ONE,
        };

        let mesh = generate(&params);

        // Check that base vertices have larger radius than tip vertices
        // Base vertices are at y ≈ 0
        let base_radii: Vec<f32> = mesh
            .positions
            .iter()
            .filter(|p| p.y < 0.1)
            .map(|p| (p.x * p.x + p.z * p.z).sqrt())
            .collect();

        // Tip vertices are at y ≈ length (before stigma)
        let tip_radii: Vec<f32> = mesh
            .positions
            .iter()
            .filter(|p| (p.y - params.length).abs() < 0.1)
            .filter(|p| {
                // Exclude stigma vertices (far from center)
                let r = (p.x * p.x + p.z * p.z).sqrt();
                r < params.tip_radius + 0.05
            })
            .map(|p| (p.x * p.x + p.z * p.z).sqrt())
            .collect();

        if !base_radii.is_empty() && !tip_radii.is_empty() {
            let avg_base = base_radii.iter().sum::<f32>() / base_radii.len() as f32;
            let avg_tip = tip_radii.iter().sum::<f32>() / tip_radii.len() as f32;

            assert!(
                avg_base > avg_tip,
                "Base radius {} should be larger than tip radius {}",
                avg_base,
                avg_tip
            );
        }
    }

    #[test]
    fn test_curved_pistil() {
        // Create a curved style using Catmull-Rom control points
        let curve = vec![
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.5, 0.0),
            Vec3::new(0.2, 1.0, 0.1),
            Vec3::new(0.3, 1.5, 0.3),
        ];

        let params = PistilParams {
            length: 2.0, // Ignored when curve is provided
            base_radius: 0.1,
            tip_radius: 0.08,
            stigma_radius: 0.15,
            segments: 12,
            style_curve: Some(curve),
            color: Vec3::ONE,
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
    fn test_curved_pistil_too_few_points() {
        let curve = vec![
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        ];

        let params = PistilParams {
            length: 2.0,
            base_radius: 0.1,
            tip_radius: 0.08,
            stigma_radius: 0.15,
            segments: 12,
            style_curve: Some(curve),
            color: Vec3::ONE,
        };

        generate(&params); // Should panic
    }
}
