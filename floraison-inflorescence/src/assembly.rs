//! Inflorescence assembly - combining axis, branches, and flowers into complete structure

use floraison_core::math::curves::AxisCurve;
use floraison_core::{geometry::mesh::Mesh, geometry::sweep::sweep_along_curve, Mat4, Quat, Vec2, Vec3};

use crate::{patterns, InflorescenceParams, PatternType};

/// Assemble a complete inflorescence from parameters and a flower mesh
///
/// This function:
/// 1. Generates the main axis curve
/// 2. Calls the appropriate pattern generator to get branch points
/// 3. Generates stem geometry along the axis
/// 4. For each branch point:
///    - Generates pedicel mesh (if length > 0)
///    - Transforms and places flower mesh
/// 5. Merges all geometry into a single mesh
///
/// # Arguments
/// * `params` - Inflorescence parameters (pattern type, dimensions, angles, etc.)
/// * `flower_mesh` - The flower mesh to replicate at each branch point
/// * `stem_color` - RGB color for stem and pedicel geometry
///
/// # Returns
/// Complete inflorescence mesh with all components merged
///
/// # Example
/// ```no_run
/// use floraison_inflorescence::{InflorescenceParams, PatternType, assembly::assemble_inflorescence};
/// use floraison_core::{geometry::mesh::Mesh, Vec3};
///
/// let params = InflorescenceParams {
///     pattern: PatternType::Raceme,
///     branch_count: 10,
///     ..Default::default()
/// };
///
/// let flower = Mesh::new(); // Your flower mesh
/// let stem_color = Vec3::new(0.2, 0.6, 0.2); // Green
///
/// let inflorescence = assemble_inflorescence(&params, &flower, stem_color);
/// ```
pub fn assemble_inflorescence(
    params: &InflorescenceParams,
    flower_mesh: &Mesh,
    stem_color: Vec3,
) -> Mesh {
    let mut final_mesh = Mesh::new();

    // 1. Generate axis curve (for now, simple straight vertical line)
    let axis_points = vec![
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, params.axis_length, 0.0),
    ];
    let axis = AxisCurve::new(axis_points.clone());

    // 2. Generate branch points based on pattern type
    let branches = match params.pattern {
        PatternType::Raceme => patterns::raceme::generate_branch_points(params, &axis),
        PatternType::Spike => patterns::spike::generate_branch_points(params, &axis),
        PatternType::Umbel => patterns::umbel::generate_branch_points(params, &axis),
        PatternType::Corymb => patterns::corymb::generate_branch_points(params, &axis),
        _ => {
            // Other patterns not yet implemented
            vec![]
        }
    };

    // 3. Generate main stem mesh (cylinder along axis)
    let stem_radius = 0.05; // Fixed radius for now
    let stem_mesh = generate_stem_along_axis(&axis_points, stem_radius, stem_color);
    final_mesh.merge(&stem_mesh);

    // 4. For each branch, add pedicel and flower
    for branch in &branches {
        // 4a. Generate pedicel mesh if branch has length
        if branch.length > 0.01 {
            let pedicel = generate_pedicel(branch, stem_radius * 0.6, stem_color);
            final_mesh.merge(&pedicel);
        }

        // 4b. Clone and transform flower mesh
        let mut flower = flower_mesh.clone();

        // Compute transformation matrix
        // 1. Scale by flower_scale
        // 2. Rotate to align with branch direction
        // 3. Translate to branch position
        let scale = Vec3::splat(branch.flower_scale);
        let rotation = Quat::from_rotation_arc(Vec3::Y, branch.direction);
        let translation = branch.position;

        let transform = Mat4::from_scale_rotation_translation(scale, rotation, translation);
        flower.transform(&transform);

        final_mesh.merge(&flower);
    }

    final_mesh
}

/// Generate a cylindrical stem mesh along an axis curve
///
/// # Arguments
/// * `axis_points` - Points defining the axis curve
/// * `radius` - Radius of the stem
/// * `color` - RGB color for the stem
///
/// # Returns
/// Mesh of the stem geometry
fn generate_stem_along_axis(axis_points: &[Vec3], radius: f32, color: Vec3) -> Mesh {
    // Create cylindrical profile
    let profile = vec![Vec2::new(radius, 0.0), Vec2::new(radius, 1.0)];

    // Sweep profile along axis
    sweep_along_curve(&profile, axis_points, 8, color)
}

/// Generate a pedicel (branch stem) mesh
///
/// Creates a thin cylindrical stem from the axis attachment point to the flower position.
///
/// # Arguments
/// * `branch` - Branch point containing position, direction, and length
/// * `radius` - Radius of the pedicel
/// * `color` - RGB color for the pedicel
///
/// # Returns
/// Mesh of the pedicel geometry
fn generate_pedicel(
    branch: &crate::BranchPoint,
    radius: f32,
    color: Vec3,
) -> Mesh {
    // Create a simple two-point curve from base to flower position
    // Base position: work backwards from flower position using direction and length
    let base = branch.position - branch.direction * branch.length;
    let tip = branch.position;

    // Create cylindrical profile
    let profile = vec![Vec2::new(radius, 0.0), Vec2::new(radius, 1.0)];

    // Create curve (base to tip)
    let curve = vec![base, tip];

    // Sweep profile along curve
    sweep_along_curve(&profile, &curve, 6, color)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::InflorescenceParams;

    fn create_simple_flower() -> Mesh {
        // Create a simple triangle mesh for testing
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Vec3::ZERO, Vec3::Y, Vec2::ZERO, Vec3::new(1.0, 0.5, 0.5));
        let v1 = mesh.add_vertex(Vec3::X, Vec3::Y, Vec2::ZERO, Vec3::new(1.0, 0.5, 0.5));
        let v2 = mesh.add_vertex(Vec3::Z, Vec3::Y, Vec2::ZERO, Vec3::new(1.0, 0.5, 0.5));
        mesh.add_triangle(v0, v1, v2);
        mesh
    }

    #[test]
    fn test_assemble_raceme() {
        let params = InflorescenceParams {
            pattern: PatternType::Raceme,
            branch_count: 5,
            axis_length: 10.0,
            ..Default::default()
        };

        let flower = create_simple_flower();
        let stem_color = Vec3::new(0.2, 0.6, 0.2);

        let inflorescence = assemble_inflorescence(&params, &flower, stem_color);

        // Should have vertices from:
        // - Main stem
        // - 5 flowers (3 vertices each = 15)
        // - 5 pedicels
        assert!(
            inflorescence.vertex_count() > 15,
            "Should have at least flower vertices"
        );
        assert!(
            inflorescence.triangle_count() > 5,
            "Should have triangles from flowers and stems"
        );
    }

    #[test]
    fn test_assemble_spike() {
        let params = InflorescenceParams {
            pattern: PatternType::Spike,
            branch_count: 4,
            axis_length: 8.0,
            ..Default::default()
        };

        let flower = create_simple_flower();
        let stem_color = Vec3::new(0.2, 0.6, 0.2);

        let inflorescence = assemble_inflorescence(&params, &flower, stem_color);

        // Spike has sessile flowers (no pedicels), so fewer vertices than raceme
        assert!(inflorescence.vertex_count() > 0);
        assert!(inflorescence.triangle_count() > 0);
    }

    #[test]
    fn test_assemble_umbel() {
        let params = InflorescenceParams {
            pattern: PatternType::Umbel,
            branch_count: 6,
            axis_length: 5.0,
            branch_length_top: 2.0,
            ..Default::default()
        };

        let flower = create_simple_flower();
        let stem_color = Vec3::new(0.2, 0.6, 0.2);

        let inflorescence = assemble_inflorescence(&params, &flower, stem_color);

        assert!(inflorescence.vertex_count() > 0);
        assert!(inflorescence.triangle_count() > 0);
    }

    #[test]
    fn test_assemble_corymb() {
        let params = InflorescenceParams {
            pattern: PatternType::Corymb,
            branch_count: 5,
            axis_length: 10.0,
            ..Default::default()
        };

        let flower = create_simple_flower();
        let stem_color = Vec3::new(0.2, 0.6, 0.2);

        let inflorescence = assemble_inflorescence(&params, &flower, stem_color);

        assert!(inflorescence.vertex_count() > 0);
        assert!(inflorescence.triangle_count() > 0);
    }

    #[test]
    fn test_generate_stem() {
        let axis = vec![Vec3::ZERO, Vec3::new(0.0, 5.0, 0.0)];
        let radius = 0.1;
        let color = Vec3::new(0.2, 0.6, 0.2);

        let stem = generate_stem_along_axis(&axis, radius, color);

        assert!(stem.vertex_count() > 0, "Stem should have vertices");
        assert!(stem.triangle_count() > 0, "Stem should have triangles");

        // Check that all vertices have the correct color
        for c in &stem.colors {
            assert_eq!(*c, color, "Stem vertices should have correct color");
        }
    }

    #[test]
    fn test_generate_pedicel() {
        let branch = crate::BranchPoint {
            position: Vec3::new(1.0, 5.0, 0.5),
            direction: Vec3::new(0.6, 0.8, 0.0).normalize(),
            length: 2.0,
            flower_scale: 1.0,
            age: 0.5,
        };

        let radius = 0.05;
        let color = Vec3::new(0.2, 0.6, 0.2);

        let pedicel = generate_pedicel(&branch, radius, color);

        assert!(pedicel.vertex_count() > 0, "Pedicel should have vertices");
        assert!(
            pedicel.triangle_count() > 0,
            "Pedicel should have triangles"
        );
    }

    #[test]
    fn test_assemble_empty_flower() {
        let params = InflorescenceParams {
            pattern: PatternType::Raceme,
            branch_count: 3,
            ..Default::default()
        };

        let empty_flower = Mesh::new(); // Empty mesh
        let stem_color = Vec3::new(0.2, 0.6, 0.2);

        let inflorescence = assemble_inflorescence(&params, &empty_flower, stem_color);

        // Should still have stem geometry even with empty flowers
        assert!(
            inflorescence.vertex_count() > 0,
            "Should have stem vertices even with empty flower"
        );
    }

    #[test]
    fn test_flower_transformation() {
        let params = InflorescenceParams {
            pattern: PatternType::Raceme,
            branch_count: 1,
            flower_size_top: 2.0, // Scale of 2.0
            ..Default::default()
        };

        let flower = create_simple_flower();
        let original_vertex_count = flower.vertex_count();

        let inflorescence = assemble_inflorescence(&params, &flower, Vec3::ONE);

        // The inflorescence should contain the transformed flower
        // Check that it has more vertices than just the flower (includes stem)
        assert!(inflorescence.vertex_count() >= original_vertex_count);
    }
}
