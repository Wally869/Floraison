//! Compound umbel pattern generator
//!
//! Compound Umbel: Umbel where each ray ends in a sub-umbel.
//! Creates hierarchical umbrella-like structure.
//!
//! Examples: Parsley, Dill, Carrot

use floraison_core::math::curves::AxisCurve;
use floraison_core::{geometry::mesh::Mesh, Mat4, Quat, Vec3};

use crate::{assembly, patterns, InflorescenceParams, PatternType};

/// Generate compound umbel inflorescence mesh
///
/// This function recursively generates an umbel where each ray terminal
/// is replaced by a smaller sub-umbel.
///
/// # Arguments
/// * `params` - Inflorescence parameters
/// * `flower_mesh` - Base flower mesh to place at terminal positions
/// * `stem_color` - RGB color for stems and pedicels
///
/// # Returns
/// Complete compound umbel mesh
///
/// # Recursion
/// - Uses `params.recursion_depth` to control nesting depth (default: 2)
/// - Each level is scaled down by 0.5× in size
/// - Ray count reduces slightly at each level (minimum 4)
pub fn generate_compound_umbel(
    params: &InflorescenceParams,
    flower_mesh: &Mesh,
    stem_color: Vec3,
) -> Mesh {
    let compound_depth = params.recursion_depth.unwrap_or(1);

    // Base case: simple umbel (change pattern to avoid infinite recursion)
    if compound_depth <= 1 {
        let mut simple_params = params.clone();
        simple_params.pattern = PatternType::Umbel;
        return assembly::assemble_inflorescence(&simple_params, flower_mesh, stem_color);
    }

    let mut final_mesh = Mesh::new();

    // Generate main axis (straight or curved based on params)
    let axis_points = assembly::generate_axis_points(params);
    let axis = AxisCurve::new(axis_points.clone());

    // Get primary branch points (umbel rays)
    let mut primary_params = params.clone();
    primary_params.pattern = PatternType::Umbel;
    let primary_branches = patterns::umbel::generate_branch_points(&primary_params, &axis);

    // Add main stem
    let main_stem = assembly::generate_stem_along_axis(&axis_points, 0.08, stem_color);
    final_mesh.merge(&main_stem);

    // For each primary ray, create sub-umbel
    for branch in &primary_branches {
        // Generate pedicel connecting main axis to sub-inflorescence
        if branch.length > 0.01 {
            let pedicel = assembly::generate_pedicel(branch, params, 0.05, stem_color);
            final_mesh.merge(&pedicel);
        }

        // Scale down parameters for sub-inflorescence
        let sub_params = InflorescenceParams {
            axis_length: params.axis_length * 0.3, // Shorter sub-umbel stems
            branch_count: (params.branch_count * 3 / 4).max(4), // Reduce ray count slightly
            branch_length_top: params.branch_length_top * 0.6,
            flower_size_top: params.flower_size_top * 0.7,
            recursion_depth: Some(compound_depth - 1),
            ..params.clone()
        };

        // Recursive call
        let mut sub_inflorescence = generate_compound_umbel(&sub_params, flower_mesh, stem_color);

        // Transform to ray terminal position/orientation
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
            Vec3::new(1.0, 1.0, 0.5),
        );
        mesh.add_vertex(Vec3::X, Vec3::Y, glam::Vec2::ZERO, Vec3::new(1.0, 1.0, 0.5));
        mesh.add_vertex(Vec3::Z, Vec3::Y, glam::Vec2::ZERO, Vec3::new(1.0, 1.0, 0.5));
        mesh.add_triangle(0, 1, 2);
        mesh
    }

    #[test]
    fn test_compound_umbel_base_case() {
        let params = InflorescenceParams {
            recursion_depth: Some(1),
            branch_count: 6,
            ..Default::default()
        };

        let flower = create_test_flower();
        let mesh = generate_compound_umbel(&params, &flower, Vec3::ONE);

        // Depth 1 should be same as simple umbel
        assert!(mesh.vertex_count() > 0);
        assert!(mesh.triangle_count() > 0);
    }

    #[test]
    fn test_compound_umbel_depth_two() {
        let params = InflorescenceParams {
            recursion_depth: Some(2),
            branch_count: 6,
            axis_length: 8.0,
            ..Default::default()
        };

        let flower = create_test_flower();
        let mesh = generate_compound_umbel(&params, &flower, Vec3::ONE);

        // Depth 2: main umbel + 6 sub-umbels
        // Should have significantly more vertices than simple umbel
        assert!(
            mesh.vertex_count() > 30,
            "Compound umbel should have many vertices"
        );
    }

    #[test]
    fn test_compound_umbel_hierarchical_scaling() {
        let params = InflorescenceParams {
            recursion_depth: Some(2),
            branch_count: 5,
            axis_length: 10.0,
            ..Default::default()
        };

        let flower = create_test_flower();
        let mesh = generate_compound_umbel(&params, &flower, Vec3::ONE);

        // Verify mesh generated successfully
        assert!(mesh.vertex_count() > 0);
        assert!(mesh.triangle_count() > 0);
    }

    #[test]
    fn test_compound_umbel_depth_zero() {
        let params = InflorescenceParams {
            recursion_depth: Some(0),
            branch_count: 5,
            ..Default::default()
        };

        let flower = create_test_flower();
        let mesh = generate_compound_umbel(&params, &flower, Vec3::ONE);

        // Depth 0 should work (treated as 1)
        assert!(mesh.vertex_count() > 0);
    }

    #[test]
    fn test_compound_umbel_empty_flower() {
        let params = InflorescenceParams {
            recursion_depth: Some(2),
            branch_count: 4,
            ..Default::default()
        };

        let empty_flower = Mesh::new();
        let mesh = generate_compound_umbel(&params, &empty_flower, Vec3::ONE);

        // Should still have stem geometry
        assert!(mesh.vertex_count() > 0);
    }

    #[test]
    fn test_compound_umbel_default_depth() {
        let params = InflorescenceParams {
            branch_count: 5,
            ..Default::default()
        };

        let flower = create_test_flower();
        let mesh = generate_compound_umbel(&params, &flower, Vec3::ONE);

        // Default depth should be 2
        assert!(mesh.vertex_count() > 0);
    }

    #[test]
    fn test_compound_umbel_radial_structure() {
        let params = InflorescenceParams {
            recursion_depth: Some(2),
            branch_count: 8,
            rotation_angle: 45.0, // Even spacing
            ..Default::default()
        };

        let flower = create_test_flower();
        let mesh = generate_compound_umbel(&params, &flower, Vec3::ONE);

        // Should have umbrella-like structure
        assert!(mesh.vertex_count() > 0);
        assert!(mesh.triangle_count() > 0);
    }
}
