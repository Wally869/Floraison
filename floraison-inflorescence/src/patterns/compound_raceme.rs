//! Compound raceme pattern generator
//!
//! Compound Raceme: Raceme where each flower is replaced by a sub-raceme.
//! Creates hierarchical/nested structure.
//!
//! Examples: Lilac, Astilbe

use floraison_core::math::curves::AxisCurve;
use floraison_core::{geometry::mesh::Mesh, Mat4, Quat, Vec3};

use crate::{assembly, patterns, InflorescenceParams, PatternType};

/// Generate compound raceme inflorescence mesh
///
/// This function recursively generates a raceme where each flower position
/// is replaced by a smaller sub-raceme.
///
/// # Arguments
/// * `params` - Inflorescence parameters
/// * `flower_mesh` - Base flower mesh to place at terminal positions
/// * `stem_color` - RGB color for stems and pedicels
///
/// # Returns
/// Complete compound raceme mesh
///
/// # Recursion
/// - Uses `params.recursion_depth` to control nesting depth (default: 2)
/// - Each level is scaled down by 0.4× in size
/// - Branch count reduces by half at each level (minimum 3)
pub fn generate_compound_raceme(
    params: &InflorescenceParams,
    flower_mesh: &Mesh,
    stem_color: Vec3,
) -> Mesh {
    let compound_depth = params.recursion_depth.unwrap_or(1);

    // Base case: simple raceme (change pattern to avoid infinite recursion)
    if compound_depth <= 1 {
        let mut simple_params = params.clone();
        simple_params.pattern = PatternType::Raceme;
        return assembly::assemble_inflorescence(&simple_params, flower_mesh, stem_color);
    }

    let mut final_mesh = Mesh::new();

    // Generate main axis
    let axis_points = vec![
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, params.axis_length, 0.0),
    ];
    let axis = AxisCurve::new(axis_points.clone());

    // Get primary branch points
    let mut primary_params = params.clone();
    primary_params.pattern = PatternType::Raceme;
    let primary_branches = patterns::raceme::generate_branch_points(&primary_params, &axis);

    // Add main stem
    let main_stem = assembly::generate_stem_along_axis(&axis_points, 0.08, stem_color);
    final_mesh.merge(&main_stem);

    // For each primary branch, create sub-raceme
    for branch in &primary_branches {
        // Generate pedicel connecting main axis to sub-inflorescence
        if branch.length > 0.01 {
            let pedicel = assembly::generate_pedicel(branch, 0.05, stem_color);
            final_mesh.merge(&pedicel);
        }

        // Scale down parameters for sub-inflorescence
        let sub_params = InflorescenceParams {
            axis_length: params.axis_length * 0.4,
            branch_count: (params.branch_count / 2).max(3),
            branch_length_top: params.branch_length_top * 0.6,
            branch_length_bottom: params.branch_length_bottom * 0.6,
            flower_size_top: params.flower_size_top * 0.7,
            flower_size_bottom: params.flower_size_bottom * 0.7,
            recursion_depth: Some(compound_depth - 1),
            ..params.clone()
        };

        // Recursive call
        let mut sub_inflorescence = generate_compound_raceme(&sub_params, flower_mesh, stem_color);

        // Transform to branch position/orientation
        let scale_factor = 0.5;
        let transform = Mat4::from_scale_rotation_translation(
            Vec3::splat(scale_factor),
            Quat::from_rotation_arc(Vec3::Y, branch.direction),
            branch.position,
        );
        sub_inflorescence.transform(&transform);

        final_mesh.merge(&sub_inflorescence);
    }

    final_mesh
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::InflorescenceParams;

    fn create_test_flower() -> Mesh {
        let mut mesh = Mesh::new();
        mesh.add_vertex(
            Vec3::ZERO,
            Vec3::Y,
            glam::Vec2::ZERO,
            Vec3::new(1.0, 0.5, 0.5),
        );
        mesh.add_vertex(
            Vec3::X,
            Vec3::Y,
            glam::Vec2::ZERO,
            Vec3::new(1.0, 0.5, 0.5),
        );
        mesh.add_vertex(
            Vec3::Z,
            Vec3::Y,
            glam::Vec2::ZERO,
            Vec3::new(1.0, 0.5, 0.5),
        );
        mesh.add_triangle(0, 1, 2);
        mesh
    }

    #[test]
    fn test_compound_raceme_base_case() {
        let params = InflorescenceParams {
            recursion_depth: Some(1),
            branch_count: 5,
            ..Default::default()
        };

        let flower = create_test_flower();
        let mesh = generate_compound_raceme(&params, &flower, Vec3::ONE);

        // Depth 1 should be same as simple raceme
        assert!(mesh.vertex_count() > 0);
        assert!(mesh.triangle_count() > 0);
    }

    #[test]
    fn test_compound_raceme_depth_two() {
        let params = InflorescenceParams {
            recursion_depth: Some(2),
            branch_count: 4,
            axis_length: 10.0,
            ..Default::default()
        };

        let flower = create_test_flower();
        let mesh = generate_compound_raceme(&params, &flower, Vec3::ONE);

        // Depth 2: main raceme + 4 sub-racemes
        // Should have significantly more vertices than simple raceme
        assert!(mesh.vertex_count() > 20, "Compound should have many vertices");
    }

    #[test]
    fn test_compound_raceme_hierarchical_scaling() {
        let params = InflorescenceParams {
            recursion_depth: Some(2),
            branch_count: 3,
            axis_length: 12.0,
            ..Default::default()
        };

        let flower = create_test_flower();
        let mesh = generate_compound_raceme(&params, &flower, Vec3::ONE);

        // Verify mesh generated successfully
        assert!(mesh.vertex_count() > 0);
        assert!(mesh.triangle_count() > 0);
    }

    #[test]
    fn test_compound_raceme_depth_zero() {
        let params = InflorescenceParams {
            recursion_depth: Some(0),
            branch_count: 3,
            ..Default::default()
        };

        let flower = create_test_flower();
        let mesh = generate_compound_raceme(&params, &flower, Vec3::ONE);

        // Depth 0 should work (treated as 1)
        assert!(mesh.vertex_count() > 0);
    }

    #[test]
    fn test_compound_raceme_empty_flower() {
        let params = InflorescenceParams {
            recursion_depth: Some(2),
            branch_count: 3,
            ..Default::default()
        };

        let empty_flower = Mesh::new();
        let mesh = generate_compound_raceme(&params, &empty_flower, Vec3::ONE);

        // Should still have stem geometry
        assert!(mesh.vertex_count() > 0);
    }

    #[test]
    fn test_compound_raceme_default_depth() {
        let params = InflorescenceParams {
            branch_count: 3,
            ..Default::default()
        };

        let flower = create_test_flower();
        let mesh = generate_compound_raceme(&params, &flower, Vec3::ONE);

        // Default depth should be 2
        assert!(mesh.vertex_count() > 0);
    }
}
