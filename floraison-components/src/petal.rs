//! Petal generator
//!
//! Creates simple flat petals with Bézier curve outlines.
//! This is a basic implementation - curvature and advanced deformation will be added in Epic 5.

use crate::{Mesh, Vec2, Vec3};
use floraison_core::math::bezier::sample_cubic_2d;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Parameters for petal generation
///
/// # Example
/// ```
/// use floraison_components::petal::{PetalParams, generate};
///
/// let params = PetalParams {
///     length: 3.0,
///     width: 1.5,
///     tip_sharpness: 0.3,
///     base_width: 0.5,
///     outline_samples: 16,
/// };
///
/// let mesh = generate(&params);
/// assert!(mesh.vertex_count() > 0);
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PetalParams {
    /// Length of the petal (from base to tip)
    pub length: f32,

    /// Maximum width of the petal
    pub width: f32,

    /// Tip sharpness (0.0 = rounded, 1.0 = very pointed)
    /// Controls how much the tip curves inward
    pub tip_sharpness: f32,

    /// Width at the base of the petal
    pub base_width: f32,

    /// Number of samples along the outline (per side)
    pub outline_samples: usize,
}

impl Default for PetalParams {
    /// Create default parameters for a lily-like petal
    fn default() -> Self {
        Self {
            length: 3.0,
            width: 1.2,
            tip_sharpness: 0.4,
            base_width: 0.4,
            outline_samples: 16,
        }
    }
}

impl PetalParams {
    /// Create a wide, rounded petal
    pub fn wide() -> Self {
        Self {
            length: 2.5,
            width: 2.0,
            tip_sharpness: 0.2,
            base_width: 0.8,
            outline_samples: 20,
        }
    }

    /// Create a narrow, pointed petal
    pub fn narrow() -> Self {
        Self {
            length: 4.0,
            width: 1.0,
            tip_sharpness: 0.7,
            base_width: 0.3,
            outline_samples: 16,
        }
    }

    /// Create a short, rounded petal
    pub fn short() -> Self {
        Self {
            length: 1.5,
            width: 1.2,
            tip_sharpness: 0.1,
            base_width: 0.6,
            outline_samples: 12,
        }
    }
}

/// Generate a petal mesh from parameters
///
/// Creates a flat petal with a Bézier curve outline. The petal lies in the XY plane
/// (Z = 0) with the base at the origin and tip extending along the Y axis.
///
/// # Arguments
///
/// * `params` - Petal parameters
///
/// # Returns
///
/// A mesh with the petal geometry
///
/// # Example
///
/// ```
/// use floraison_components::petal::{PetalParams, generate};
///
/// let petal = generate(&PetalParams::default());
/// assert!(petal.triangle_count() > 0);
/// ```
pub fn generate(params: &PetalParams) -> Mesh {
    // Define key points for the petal outline
    let base_left = Vec2::new(-params.base_width / 2.0, 0.0);
    let base_right = Vec2::new(params.base_width / 2.0, 0.0);
    let tip = Vec2::new(0.0, params.length);

    // Point of maximum width (typically around 60% of length)
    let max_width_height = params.length * 0.6;
    let max_left = Vec2::new(-params.width / 2.0, max_width_height);
    let max_right = Vec2::new(params.width / 2.0, max_width_height);

    // Control point for tip sharpness
    // The closer this is to the tip, the sharper the point
    let tip_control_height = params.length * (1.0 - params.tip_sharpness * 0.5);
    let tip_control_width = params.width * (0.5 - params.tip_sharpness * 0.4);

    // Left side: base -> max width -> tip
    // Using cubic Bézier: base_left -> control1 -> control2 -> max_left
    let left_lower = sample_cubic_2d(
        base_left,
        Vec2::new(-params.base_width / 2.0, params.length * 0.2),
        Vec2::new(-params.width * 0.4, max_width_height * 0.6),
        max_left,
        params.outline_samples / 2,
    );

    // Max width to tip: max_left -> control -> tip
    let left_upper = sample_cubic_2d(
        max_left,
        Vec2::new(-params.width * 0.45, max_width_height * 1.2),
        Vec2::new(-tip_control_width, tip_control_height),
        tip,
        params.outline_samples / 2,
    );

    // Right side: tip -> max width -> base
    let right_upper = sample_cubic_2d(
        tip,
        Vec2::new(tip_control_width, tip_control_height),
        Vec2::new(params.width * 0.45, max_width_height * 1.2),
        max_right,
        params.outline_samples / 2,
    );

    let right_lower = sample_cubic_2d(
        max_right,
        Vec2::new(params.width * 0.4, max_width_height * 0.6),
        Vec2::new(params.base_width / 2.0, params.length * 0.2),
        base_right,
        params.outline_samples / 2,
    );

    // Combine all outline points
    let mut outline = Vec::new();
    outline.extend(left_lower);
    outline.extend(left_upper);
    outline.extend(right_upper);
    outline.extend(right_lower);

    // Remove duplicate points at connections
    outline.dedup_by(|a, b| (a.x - b.x).abs() < 0.0001 && (a.y - b.y).abs() < 0.0001);

    // Create mesh from outline using fan triangulation
    create_petal_mesh(&outline)
}

/// Create a mesh from a 2D outline using fan triangulation
fn create_petal_mesh(outline: &[Vec2]) -> Mesh {
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let mut indices = Vec::new();

    // Calculate bounding box for UV mapping
    let min_x = outline.iter().map(|p| p.x).fold(f32::MAX, f32::min);
    let max_x = outline.iter().map(|p| p.x).fold(f32::MIN, f32::max);
    let min_y = outline.iter().map(|p| p.y).fold(f32::MAX, f32::min);
    let max_y = outline.iter().map(|p| p.y).fold(f32::MIN, f32::max);
    let width = max_x - min_x;
    let height = max_y - min_y;

    // Center point for fan triangulation
    let center_x = (min_x + max_x) / 2.0;
    let center_y = (min_y + max_y) / 2.0;
    let center = Vec2::new(center_x, center_y);

    // Add center vertex
    positions.push(Vec3::new(center.x, center.y, 0.0));
    normals.push(Vec3::new(0.0, 0.0, 1.0)); // Normal pointing up (Z+)
    uvs.push(Vec2::new(
        (center.x - min_x) / width,
        (center.y - min_y) / height,
    ));

    // Add outline vertices
    for point in outline {
        positions.push(Vec3::new(point.x, point.y, 0.0));
        normals.push(Vec3::new(0.0, 0.0, 1.0));
        uvs.push(Vec2::new(
            (point.x - min_x) / width,
            (point.y - min_y) / height,
        ));
    }

    // Create fan triangles from center
    let outline_count = outline.len();
    for i in 0..outline_count {
        let next = (i + 1) % outline_count;
        indices.push(0); // Center
        indices.push((i + 1) as u32);
        indices.push((next + 1) as u32);
    }

    Mesh {
        positions,
        normals,
        uvs,
        indices,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_petal() {
        let mesh = generate(&PetalParams::default());

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
    fn test_wide_petal() {
        let mesh = generate(&PetalParams::wide());

        assert!(mesh.vertex_count() > 0);
        assert!(mesh.triangle_count() > 0);
    }

    #[test]
    fn test_narrow_petal() {
        let mesh = generate(&PetalParams::narrow());

        assert!(mesh.vertex_count() > 0);
        assert!(mesh.triangle_count() > 0);
    }

    #[test]
    fn test_short_petal() {
        let mesh = generate(&PetalParams::short());

        assert!(mesh.vertex_count() > 0);
        assert!(mesh.triangle_count() > 0);
    }

    #[test]
    fn test_petal_is_flat() {
        let mesh = generate(&PetalParams::default());

        // All Z coordinates should be 0 (flat petal)
        for pos in &mesh.positions {
            assert!(
                pos.z.abs() < 0.0001,
                "Petal should be flat (Z=0), got Z={}",
                pos.z
            );
        }
    }

    #[test]
    fn test_petal_dimensions() {
        let params = PetalParams {
            length: 5.0,
            width: 2.0,
            tip_sharpness: 0.5,
            base_width: 0.5,
            outline_samples: 16,
        };

        let mesh = generate(&params);

        // Check Y range (length)
        let min_y = mesh.positions.iter().map(|p| p.y).fold(f32::MAX, f32::min);
        let max_y = mesh.positions.iter().map(|p| p.y).fold(f32::MIN, f32::max);

        assert!(
            min_y < 0.5,
            "Base should be near 0, got min_y={}",
            min_y
        );
        assert!(
            (max_y - params.length).abs() < 0.1,
            "Tip should be at length {}, got max_y={}",
            params.length,
            max_y
        );

        // Check X range (width)
        let min_x = mesh.positions.iter().map(|p| p.x).fold(f32::MAX, f32::min);
        let max_x = mesh.positions.iter().map(|p| p.x).fold(f32::MIN, f32::max);
        let actual_width = max_x - min_x;

        assert!(
            (actual_width - params.width).abs() < 0.3,
            "Width should be around {}, got {}",
            params.width,
            actual_width
        );
    }

    #[test]
    fn test_indices_in_bounds() {
        let mesh = generate(&PetalParams::default());
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
    fn test_normals_point_up() {
        let mesh = generate(&PetalParams::default());

        // All normals should point in +Z direction
        for normal in &mesh.normals {
            assert!(
                (normal.x).abs() < 0.01,
                "Normal X should be ~0, got {}",
                normal.x
            );
            assert!(
                (normal.y).abs() < 0.01,
                "Normal Y should be ~0, got {}",
                normal.y
            );
            assert!(
                (normal.z - 1.0).abs() < 0.01,
                "Normal Z should be ~1, got {}",
                normal.z
            );
        }
    }

    #[test]
    fn test_uv_mapping() {
        let mesh = generate(&PetalParams::default());

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
    fn test_tip_sharpness() {
        let sharp_params = PetalParams {
            length: 3.0,
            width: 1.5,
            tip_sharpness: 0.9,
            base_width: 0.5,
            outline_samples: 16,
        };

        let rounded_params = PetalParams {
            length: 3.0,
            width: 1.5,
            tip_sharpness: 0.1,
            base_width: 0.5,
            outline_samples: 16,
        };

        let sharp_mesh = generate(&sharp_params);
        let rounded_mesh = generate(&rounded_params);

        // Both should generate valid meshes
        assert!(sharp_mesh.vertex_count() > 0);
        assert!(rounded_mesh.vertex_count() > 0);

        // Verify all geometry is valid for both meshes
        for pos in &sharp_mesh.positions {
            assert!(pos.is_finite());
        }
        for pos in &rounded_mesh.positions {
            assert!(pos.is_finite());
        }
    }
}
