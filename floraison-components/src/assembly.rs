//! Flower assembly and component positioning
//!
//! This module handles the assembly of individual floral components into complete flowers.
//! It maps 2D floral diagram positions to 3D positions on the receptacle surface.

use crate::{Vec2, Vec3, Mat3, Mat4, Quat, Mesh};
use crate::receptacle::ReceptacleParams;
use crate::pistil::PistilParams;
use crate::stamen::StamenParams;
use crate::petal::PetalParams;
use crate::diagram::FloralDiagram;
use floraison_core::math::bezier::{cubic_bezier_2d, cubic_bezier_derivative_2d};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Type of floral component
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ComponentType {
    /// Receptacle (flower base)
    Receptacle,
    /// Pistil (female reproductive structure)
    Pistil,
    /// Stamen (male reproductive structure)
    Stamen,
    /// Petal
    Petal,
    /// Sepal (outer protective leaves)
    Sepal,
}

/// Placement of a component in 2D floral diagram space
///
/// This represents where a component should be positioned before
/// mapping to 3D coordinates on the receptacle surface.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ComponentPlacement {
    /// Type of component to place
    pub component_type: ComponentType,

    /// Radial distance from center (0.0 = center, 1.0 = edge)
    /// This will be mapped to a height on the receptacle
    pub radius: f32,

    /// Angular position in radians (0 = +X axis, counterclockwise)
    pub angle: f32,

    /// Height on receptacle (0.0 = bottom, 1.0 = top)
    /// Typically computed from radius, but can be overridden
    pub height: f32,

    /// Scale multiplier for this component
    pub scale: f32,

    /// Tilt angle in radians for component orientation
    /// Controls how the component tilts relative to the receptacle surface
    pub tilt_angle: f32,
}

/// 3D transformation (position, rotation, scale)
///
/// Represents the complete transformation to apply to a component mesh.
#[derive(Debug, Clone)]
pub struct Transform3D {
    /// Position in 3D space
    pub position: Vec3,

    /// Rotation as quaternion
    pub rotation: Quat,

    /// Scale factor (uniform or per-axis)
    pub scale: Vec3,
}

impl Transform3D {
    /// Create a new transform with identity rotation and uniform scale
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }

    /// Create a transform with position and rotation
    pub fn with_rotation(position: Vec3, rotation: Quat) -> Self {
        Self {
            position,
            rotation,
            scale: Vec3::ONE,
        }
    }

    /// Create a transform with position, rotation, and uniform scale
    pub fn with_scale(position: Vec3, rotation: Quat, scale: f32) -> Self {
        Self {
            position,
            rotation,
            scale: Vec3::splat(scale),
        }
    }

    /// Convert to a 4x4 transformation matrix
    ///
    /// Combines translation, rotation, and scale into a single matrix
    /// suitable for transforming mesh vertices.
    pub fn to_matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.position)
    }
}

/// Maps 2D component placements to 3D positions on receptacle surface
///
/// The mapper uses the receptacle's Bézier profile curve to compute
/// the radius at any given height, and calculates proper surface normals
/// for orienting components.
pub struct ReceptacleMapper {
    /// Bézier control points defining the receptacle profile
    /// (radius, height) pairs from bottom to top
    p0: Vec2,
    p1: Vec2,
    p2: Vec2,
    p3: Vec2,

    /// Total height of receptacle
    height: f32,
}

impl ReceptacleMapper {
    /// Create a mapper from receptacle parameters
    ///
    /// Reconstructs the Bézier control points used to generate the receptacle.
    pub fn from_params(params: &ReceptacleParams) -> Self {
        // These control points match the logic in receptacle::generate()
        let p0 = Vec2::new(params.base_radius, 0.0);

        let p1 = Vec2::new(
            params.base_radius + (params.bulge_radius - params.base_radius) * 0.3,
            params.height * 0.2,
        );

        let p2 = Vec2::new(
            params.bulge_radius,
            params.height * params.bulge_position,
        );

        let p3 = Vec2::new(params.top_radius, params.height);

        Self {
            p0,
            p1,
            p2,
            p3,
            height: params.height,
        }
    }

    /// Get the radius of the receptacle at a given height
    ///
    /// # Arguments
    /// * `height` - Height value (0.0 = bottom, self.height = top)
    ///
    /// # Returns
    /// Radius at that height
    pub fn radius_at_height(&self, height: f32) -> f32 {
        // Normalize height to [0, 1] for Bézier parameter
        let t = (height / self.height).clamp(0.0, 1.0);

        // Evaluate Bézier curve at t
        let point = cubic_bezier_2d(self.p0, self.p1, self.p2, self.p3, t);

        // Return x coordinate (radius)
        point.x
    }

    /// Get the tangent vector at a given height
    ///
    /// The tangent points in the direction of increasing height along the surface.
    ///
    /// # Arguments
    /// * `height` - Height value (0.0 = bottom, self.height = top)
    ///
    /// # Returns
    /// Tangent vector in 3D (not normalized)
    pub fn tangent_at_height(&self, height: f32) -> Vec3 {
        // Normalize height to [0, 1]
        let t = (height / self.height).clamp(0.0, 1.0);

        // Get 2D derivative (dx/dt, dy/dt)
        let derivative = cubic_bezier_derivative_2d(self.p0, self.p1, self.p2, self.p3, t);

        // Convert to 3D tangent
        // In cylindrical coordinates, the tangent in the (r, y) plane
        // The derivative gives us (dr/dt, dy/dt)
        Vec3::new(derivative.x, derivative.y, 0.0)
    }

    /// Map a 2D placement to a 3D transform on the receptacle surface
    ///
    /// # Arguments
    /// * `placement` - Component placement in diagram space
    ///
    /// # Returns
    /// 3D transform with position on receptacle surface and orientation
    pub fn map_to_3d(&self, placement: &ComponentPlacement) -> Transform3D {
        let height = placement.height * self.height;

        // Special case: pistils at center (radius ≈ 0) should be positioned on the central axis
        // and oriented straight up, not following the receptacle surface
        if placement.component_type == ComponentType::Pistil && placement.radius < 0.001 {
            let position = Vec3::new(0.0, height, 0.0);

            // Pistil points straight up (identity rotation)
            // Local Y-axis aligned with global Y-axis
            let rotation = Quat::IDENTITY;

            return Transform3D::with_scale(position, rotation, placement.scale);
        }

        let receptacle_radius = self.radius_at_height(height);

        // Compute position in cylindrical coordinates
        let position = Vec3::new(
            receptacle_radius * placement.angle.cos(),
            height,
            receptacle_radius * placement.angle.sin(),
        );

        // Stamens and pistils: tiltable components that start upright
        // - tilt_angle = 0 → points straight up (parallel to pistil)
        // - tilt_angle = π/2 → points radially outward (spreading)
        if placement.component_type == ComponentType::Stamen ||
           placement.component_type == ComponentType::Pistil {

            // Compute azimuthal tangent (direction around the flower)
            let azimuthal_tangent = Vec3::new(
                -placement.angle.sin(),
                0.0,
                placement.angle.cos(),
            ).normalize();

            // Start with upright orientation (local Y = global Y)
            // Then rotate around azimuthal tangent by -tilt_angle:
            // - 0° keeps it upright
            // - 90° rotates it to point radially outward
            // (negative angle ensures outward rotation, not inward)
            let base_up = Vec3::Y;
            let tilt_rotation = Quat::from_axis_angle(azimuthal_tangent, -placement.tilt_angle);
            let local_y = tilt_rotation * base_up;

            // Build orthonormal basis
            // Local Y-axis: direction stamen/pistil points (after tilt)
            // Local X-axis: azimuthal tangent (around flower)
            // Local Z-axis: perpendicular to both
            let local_x = azimuthal_tangent;
            let local_z = local_x.cross(local_y).normalize();
            let local_y = local_z.cross(local_x).normalize(); // Re-orthogonalize

            let rotation_matrix = Mat3::from_cols(local_x, local_y, local_z);
            let rotation = Quat::from_mat3(&rotation_matrix);

            return Transform3D::with_scale(position, rotation, placement.scale);
        }

        // Petals and sepals: follow receptacle surface normal
        // Get tangent to the generating curve in 2D: (dr/dt, dy/dt)
        let tangent_2d = self.tangent_at_height(height);

        // The 2D normal perpendicular to the tangent, pointing outward:
        // If tangent = (dr/dt, dy/dt), then normal = (dy/dt, -dr/dt)
        // This is the "right-hand" normal (90° clockwise from tangent)
        let normal_2d = Vec2::new(tangent_2d.y, -tangent_2d.x).normalize();

        // Convert the 2D normal to 3D by rotating around Y-axis
        // The normal has:
        // - Radial component: normal_2d.x (in XZ plane)
        // - Vertical component: normal_2d.y (along Y axis)
        let normal = Vec3::new(
            normal_2d.x * placement.angle.cos(),  // radial X component
            normal_2d.y,                          // vertical Y component
            normal_2d.x * placement.angle.sin(),  // radial Z component
        ).normalize();

        // Compute tangent to the circle at this azimuthal angle (in XZ plane)
        // This is the direction "around" the flower at this position
        let tangent = Vec3::new(
            -placement.angle.sin(),  // d/dθ of cos(θ)
            0.0,
            placement.angle.cos(),   // d/dθ of sin(θ)
        ).normalize();

        // Compute binormal (perpendicular to both normal and tangent)
        // Use right-handed coordinate system: binormal = tangent × normal
        let binormal = tangent.cross(normal).normalize();

        // Re-orthogonalize tangent to ensure perfect orthogonality
        // (in case normal and tangent weren't exactly perpendicular)
        let tangent = normal.cross(binormal).normalize();

        // Build rotation matrix from coordinate frame:
        // - Local X-axis → tangent (azimuthal direction around flower)
        // - Local Y-axis → normal (radially outward from receptacle)
        // - Local Z-axis → binormal (perpendicular to both)
        let rotation_matrix = Mat3::from_cols(tangent, normal, binormal);
        let rotation = Quat::from_mat3(&rotation_matrix);

        Transform3D::with_scale(position, rotation, placement.scale)
    }
}

impl FloralDiagram {
    /// Generate all component placements from this diagram
    ///
    /// Converts the abstract whorl specifications into concrete placements
    /// that can be mapped to 3D positions.
    ///
    /// # Returns
    /// Vector of placements for all components in the flower
    pub fn generate_placements(&self) -> Vec<ComponentPlacement> {
        let mut placements = Vec::new();
        let mut component_index: u64 = 0;

        // Check if jitter is enabled (any param > 0)
        let jitter_enabled = self.position_jitter > 0.0 || self.angle_jitter > 0.0 || self.size_jitter > 0.0;

        // Add pistils
        for whorl in &self.pistil_whorls {
            let angles = whorl.calculate_angles();
            for angle in angles {
                let (radius, jitter_angle, scale) = if jitter_enabled {
                    self.apply_jitter(whorl.radius, angle, component_index)
                } else {
                    (whorl.radius, angle, 1.0)
                };

                placements.push(ComponentPlacement {
                    component_type: ComponentType::Pistil,
                    radius,
                    angle: jitter_angle,
                    height: whorl.height,
                    scale,
                    tilt_angle: whorl.tilt_angle,
                });
                component_index += 1;
            }
        }

        // Add stamens
        for whorl in &self.stamen_whorls {
            let angles = whorl.calculate_angles();
            for angle in angles {
                let (radius, jitter_angle, scale) = if jitter_enabled {
                    self.apply_jitter(whorl.radius, angle, component_index)
                } else {
                    (whorl.radius, angle, 1.0)
                };

                placements.push(ComponentPlacement {
                    component_type: ComponentType::Stamen,
                    radius,
                    angle: jitter_angle,
                    height: whorl.height,
                    scale,
                    tilt_angle: whorl.tilt_angle,
                });
                component_index += 1;
            }
        }

        // Add petals
        for whorl in &self.petal_whorls {
            let angles = whorl.calculate_angles();
            for angle in angles {
                let (radius, jitter_angle, scale) = if jitter_enabled {
                    self.apply_jitter(whorl.radius, angle, component_index)
                } else {
                    (whorl.radius, angle, 1.0)
                };

                placements.push(ComponentPlacement {
                    component_type: ComponentType::Petal,
                    radius,
                    angle: jitter_angle,
                    height: whorl.height,
                    scale,
                    tilt_angle: whorl.tilt_angle,
                });
                component_index += 1;
            }
        }

        // Add sepals
        for whorl in &self.sepal_whorls {
            let angles = whorl.calculate_angles();
            for angle in angles {
                let (radius, jitter_angle, scale) = if jitter_enabled {
                    self.apply_jitter(whorl.radius, angle, component_index)
                } else {
                    (whorl.radius, angle, 1.0)
                };

                placements.push(ComponentPlacement {
                    component_type: ComponentType::Sepal,
                    radius,
                    angle: jitter_angle,
                    height: whorl.height,
                    scale,
                    tilt_angle: whorl.tilt_angle,
                });
                component_index += 1;
            }
        }

        placements
    }

    /// Apply jitter to placement parameters for natural variation
    ///
    /// Uses seeded RNG for deterministic randomness
    fn apply_jitter(&self, base_radius: f32, base_angle: f32, index: u64) -> (f32, f32, f32) {
        use rand::{Rng, SeedableRng};
        use rand::rngs::SmallRng;

        // Create seeded RNG unique to this component
        let mut rng = SmallRng::seed_from_u64(self.jitter_seed.wrapping_add(index));

        // Position jitter: offset radius slightly
        let radius_offset = if self.position_jitter > 0.0 {
            rng.gen_range(-self.position_jitter..self.position_jitter)
        } else {
            0.0
        };
        let jittered_radius = (base_radius + radius_offset).max(0.0);

        // Angle jitter: rotate slightly (convert degrees to radians)
        let angle_offset = if self.angle_jitter > 0.0 {
            let max_angle_rad = self.angle_jitter.to_radians();
            rng.gen_range(-max_angle_rad..max_angle_rad)
        } else {
            0.0
        };
        let jittered_angle = base_angle + angle_offset;

        // Size jitter: scale slightly
        let scale: f32 = if self.size_jitter > 0.0 {
            1.0 + rng.gen_range(-self.size_jitter..self.size_jitter)
        } else {
            1.0
        };

        (jittered_radius, jittered_angle, scale.max(0.1)) // Clamp scale to avoid invisibility
    }
}

/// Complete parameters for flower generation
///
/// Combines diagram specification with parameters for each component type.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FlowerParams {
    /// Floral diagram (component counts and arrangement)
    pub diagram: FloralDiagram,

    /// Receptacle parameters
    pub receptacle: ReceptacleParams,

    /// Pistil parameters
    pub pistil: PistilParams,

    /// Stamen parameters
    pub stamen: StamenParams,

    /// Petal parameters
    pub petal: PetalParams,
}

impl FlowerParams {
    /// Create parameters for a lily-like flower with B-spline petal deformations
    pub fn lily() -> Self {
        Self {
            diagram: FloralDiagram::lily(),
            receptacle: ReceptacleParams::default(),
            pistil: PistilParams::default(),
            stamen: StamenParams::default(),
            petal: PetalParams {
                length: 3.0,
                width: 1.2,
                tip_sharpness: 0.4,
                base_width: 0.4,
                curl: 0.4,           // Gentle upward curl
                twist: 15.0,         // Slight twist for organic look
                ruffle_freq: 0.0,
                ruffle_amp: 0.0,
                resolution: 20,      // Higher resolution for smooth curves
                color: Vec3::ONE,    // White petals
            },
        }
    }

    /// Create parameters for a 5-petal flower (rose-like) with ruffled edges
    pub fn five_petal() -> Self {
        Self {
            diagram: FloralDiagram::five_petal(),
            receptacle: ReceptacleParams::default(),
            pistil: PistilParams::default(),
            stamen: StamenParams::slender(),
            petal: PetalParams {
                length: 2.5,
                width: 2.0,
                tip_sharpness: 0.2,
                base_width: 0.8,
                curl: 0.2,           // Slight curl
                twist: 5.0,          // Minimal twist
                ruffle_freq: 3.0,    // 3 waves along edges
                ruffle_amp: 0.15,    // Visible ruffle
                resolution: 24,      // High resolution for ruffle detail
                color: Vec3::ONE,    // White petals
            },
        }
    }

    /// Create parameters for a daisy-like flower
    pub fn daisy() -> Self {
        Self {
            diagram: FloralDiagram::daisy(),
            receptacle: ReceptacleParams::flat(),
            pistil: PistilParams::short(),
            stamen: StamenParams::short(),
            petal: PetalParams::narrow(),
        }
    }
}

/// Generate a complete flower mesh from parameters
///
/// This function assembles all components into a single unified mesh:
/// 1. Generates the receptacle
/// 2. Creates template meshes for each component type
/// 3. For each placement in the diagram:
///    - Clones the appropriate template
///    - Transforms it to the correct position and orientation
///    - Merges it into the final mesh
///
/// # Arguments
/// * `params` - Complete flower parameters
///
/// # Returns
/// A single mesh containing all components
///
/// # Example
/// ```
/// use floraison_components::assembly::{FlowerParams, generate_flower};
///
/// let flower = generate_flower(&FlowerParams::lily());
/// assert!(flower.vertex_count() > 0);
/// ```
pub fn generate_flower(params: &FlowerParams) -> Mesh {
    let mut final_mesh = Mesh::default();

    // Generate receptacle
    let receptacle = crate::receptacle::generate(&params.receptacle);
    final_mesh.merge(&receptacle);

    // Create mapper for positioning components on receptacle surface
    let mapper = ReceptacleMapper::from_params(&params.receptacle);

    // Generate template meshes for each component type
    let pistil_template = crate::pistil::generate(&params.pistil);
    let stamen_template = crate::stamen::generate(&params.stamen);
    let petal_template = crate::petal::generate(&params.petal);

    // Get all component placements from diagram
    let placements = params.diagram.generate_placements();

    // For each placement, instantiate and position the component
    for placement in placements {
        // Select appropriate template
        let template = match placement.component_type {
            ComponentType::Receptacle => continue, // Already added
            ComponentType::Pistil => &pistil_template,
            ComponentType::Stamen => &stamen_template,
            ComponentType::Petal | ComponentType::Sepal => &petal_template,
        };

        // Clone template and transform to position
        let mut instance = template.clone();
        let transform = mapper.map_to_3d(&placement);
        instance.transform(&transform.to_matrix());

        // Merge into final mesh
        final_mesh.merge(&instance);
    }

    final_mesh
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn test_transform3d_identity() {
        let transform = Transform3D::new(Vec3::ZERO);
        assert_eq!(transform.position, Vec3::ZERO);
        assert_eq!(transform.rotation, Quat::IDENTITY);
        assert_eq!(transform.scale, Vec3::ONE);
    }

    #[test]
    fn test_transform3d_to_matrix() {
        let transform = Transform3D::new(Vec3::new(1.0, 2.0, 3.0));
        let matrix = transform.to_matrix();

        // Test that identity rotation + translation works
        let point = Vec3::ZERO;
        let transformed = matrix.transform_point3(point);
        assert!((transformed - Vec3::new(1.0, 2.0, 3.0)).length() < 0.001);
    }

    #[test]
    fn test_receptacle_mapper_radius_at_height() {
        let params = ReceptacleParams {
            height: 1.0,
            base_radius: 0.5,
            bulge_radius: 0.8,
            top_radius: 0.3,
            bulge_position: 0.5,
            segments: 16,
            profile_samples: 8,
            color: Vec3::ONE,
        };

        let mapper = ReceptacleMapper::from_params(&params);

        // Check base radius
        let r_bottom = mapper.radius_at_height(0.0);
        assert!((r_bottom - 0.5).abs() < 0.01, "Base radius should be ~0.5");

        // Check top radius
        let r_top = mapper.radius_at_height(1.0);
        assert!((r_top - 0.3).abs() < 0.01, "Top radius should be ~0.3");

        // Middle should be larger (bulge)
        let r_middle = mapper.radius_at_height(0.5);
        assert!(r_middle > r_bottom && r_middle > r_top, "Middle should bulge");
    }

    #[test]
    fn test_receptacle_mapper_3d_position() {
        let params = ReceptacleParams {
            height: 2.0,
            base_radius: 1.0,
            bulge_radius: 1.0,
            top_radius: 1.0,
            bulge_position: 0.5,
            segments: 16,
            profile_samples: 8,
            color: Vec3::ONE,
        };

        let mapper = ReceptacleMapper::from_params(&params);

        // Test placement at angle 0, mid-height
        let placement = ComponentPlacement {
            component_type: ComponentType::Petal,
            radius: 0.5,
            angle: 0.0,
            height: 0.5,
            scale: 1.0,
            tilt_angle: 0.0,
        };

        let transform = mapper.map_to_3d(&placement);

        // Should be at (1.0, 1.0, 0.0) for a cylinder
        assert!((transform.position.x - 1.0).abs() < 0.1);
        assert!((transform.position.y - 1.0).abs() < 0.1);
        assert!((transform.position.z).abs() < 0.1);
    }

    #[test]
    fn test_floral_diagram_generate_placements() {
        let diagram = FloralDiagram::lily();
        let placements = diagram.generate_placements();

        // Lily has 6 petals + 6 stamens + 1 pistil = 13 components
        assert_eq!(placements.len(), 13);

        // Count by type
        let pistils = placements.iter().filter(|p| p.component_type == ComponentType::Pistil).count();
        let stamens = placements.iter().filter(|p| p.component_type == ComponentType::Stamen).count();
        let petals = placements.iter().filter(|p| p.component_type == ComponentType::Petal).count();

        assert_eq!(pistils, 1);
        assert_eq!(stamens, 6);
        assert_eq!(petals, 6);
    }

    #[test]
    fn test_placement_angles_evenly_spaced() {
        let diagram = FloralDiagram::lily();
        let placements = diagram.generate_placements();

        // Get petal angles
        let petal_angles: Vec<f32> = placements
            .iter()
            .filter(|p| p.component_type == ComponentType::Petal)
            .map(|p| p.angle)
            .collect();

        // Should be evenly spaced
        let expected_step = 2.0 * PI / 6.0;
        for i in 0..petal_angles.len() {
            let expected = i as f32 * expected_step;
            assert!((petal_angles[i] - expected).abs() < 0.01);
        }
    }

    #[test]
    fn test_mapper_normals_point_outward_radially() {
        // Test that normals have correct radial component
        let params = ReceptacleParams {
            height: 1.0,
            base_radius: 1.0,
            bulge_radius: 1.2,
            top_radius: 1.0,
            bulge_position: 0.5,
            segments: 16,
            profile_samples: 8,
            color: Vec3::ONE,
        };

        let mapper = ReceptacleMapper::from_params(&params);

        // Test several placements around the receptacle
        for i in 0..8 {
            let angle = i as f32 * PI / 4.0;
            let placement = ComponentPlacement {
                component_type: ComponentType::Petal,
                radius: 0.5,
                angle,
                height: 0.5,
                scale: 1.0,
                tilt_angle: 0.0,
            };

            let transform = mapper.map_to_3d(&placement);

            // Transform Y-axis by rotation to get normal direction
            let normal = transform.rotation * Vec3::Y;

            // Normal should have outward radial component
            let radial_component = normal.x * angle.cos() + normal.z * angle.sin();
            assert!(radial_component > 0.5, "Normal should point outward radially");

            // Verify normal is normalized
            let length = normal.length();
            assert!((length - 1.0).abs() < 0.01, "Normal should be unit length");
        }
    }

    #[test]
    fn test_mapper_cylinder_normals() {
        // For a perfect cylinder, normals should be horizontal (radial only)
        let params = ReceptacleParams {
            height: 1.0,
            base_radius: 1.0,
            bulge_radius: 1.0,
            top_radius: 1.0,
            bulge_position: 0.5,
            segments: 16,
            profile_samples: 8,
            color: Vec3::ONE,
        };

        let mapper = ReceptacleMapper::from_params(&params);

        let placement = ComponentPlacement {
            component_type: ComponentType::Petal,
            radius: 0.5,
            angle: 0.0,
            height: 0.5,
            scale: 1.0,
            tilt_angle: 0.0,
        };

        let transform = mapper.map_to_3d(&placement);
        let normal = transform.rotation * Vec3::Y;

        // For a cylinder, normal should be nearly horizontal (small Y component due to Bézier curve)
        assert!(normal.y.abs() < 0.2, "Cylinder normal should be mostly horizontal");

        // Should point in +X direction (angle = 0)
        assert!(normal.x > 0.8, "Should point in radial direction");
    }

    // Integration tests for complete flower assembly

    #[test]
    fn test_generate_flower_lily() {
        let params = FlowerParams::lily();
        let flower = generate_flower(&params);

        // Lily has: 1 receptacle + 1 pistil + 6 stamens + 6 petals = 14 components
        assert!(flower.vertex_count() > 0, "Should have vertices");
        assert!(flower.triangle_count() > 0, "Should have triangles");

        // Check for valid geometry
        for pos in &flower.positions {
            assert!(pos.is_finite(), "Position should be finite");
        }

        for normal in &flower.normals {
            assert!(normal.is_finite(), "Normal should be finite");
        }

        // All indices should be in bounds
        let vertex_count = flower.vertex_count() as u32;
        for &index in &flower.indices {
            assert!(
                index < vertex_count,
                "Index {} out of bounds (vertex count: {})",
                index,
                vertex_count
            );
        }
    }

    #[test]
    fn test_generate_flower_five_petal() {
        let params = FlowerParams::five_petal();
        let flower = generate_flower(&params);

        // Should have vertices from all components
        assert!(flower.vertex_count() > 500, "Should have substantial geometry");
        assert!(flower.triangle_count() > 100, "Should have many triangles");

        // Verify mesh integrity
        assert_eq!(
            flower.positions.len(),
            flower.normals.len(),
            "Should have matching positions and normals"
        );
        assert_eq!(
            flower.positions.len(),
            flower.uvs.len(),
            "Should have matching positions and UVs"
        );
    }

    #[test]
    fn test_generate_flower_daisy() {
        let params = FlowerParams::daisy();
        let flower = generate_flower(&params);

        // Daisy has many components (21 petals, 34 stamens, 13 pistils)
        assert!(flower.vertex_count() > 1000, "Daisy should have many vertices");

        // Check all geometry is valid
        for pos in &flower.positions {
            assert!(pos.is_finite());
        }
    }

    #[test]
    fn test_flower_params_presets() {
        // Test that all presets can be created
        let _lily = FlowerParams::lily();
        let _five = FlowerParams::five_petal();
        let _daisy = FlowerParams::daisy();

        // Each should have appropriate component counts
        let lily_diagram = FloralDiagram::lily();
        assert_eq!(lily_diagram.total_petal_count(), 6);
        assert_eq!(lily_diagram.total_stamen_count(), 6);
        assert_eq!(lily_diagram.total_pistil_count(), 1);
    }

    #[test]
    fn test_flower_components_positioned() {
        let params = FlowerParams::lily();
        let flower = generate_flower(&params);

        // Find the approximate extents of the flower
        let min_y = flower.positions.iter().map(|p| p.y).fold(f32::MAX, f32::min);
        let max_y = flower.positions.iter().map(|p| p.y).fold(f32::MIN, f32::max);
        let max_radius = flower
            .positions
            .iter()
            .map(|p| (p.x * p.x + p.z * p.z).sqrt())
            .fold(0.0f32, f32::max);

        // Flower should have reasonable dimensions
        assert!(min_y >= -0.1, "Flower base should be near origin");
        assert!(max_y > 1.0, "Flower should have height");
        assert!(max_radius > 1.0, "Flower should have width");
    }
}
