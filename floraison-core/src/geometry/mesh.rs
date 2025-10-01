//! Core mesh data structures for 3D geometry
//!
//! This module provides the fundamental `Mesh` type used throughout Floraison
//! for representing triangulated 3D geometry. Meshes are built procedurally
//! from flower components and can be merged, transformed, and exported.

use crate::{Mat4, Vec2, Vec3};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A triangulated 3D mesh with positions, normals, UVs, and indices
///
/// The mesh stores vertex attributes in separate arrays (Structure of Arrays pattern)
/// and uses indexed triangle lists for efficient storage and rendering.
///
/// # Example
/// ```
/// use floraison_core::geometry::mesh::Mesh;
/// use floraison_core::{Vec3, Vec2};
///
/// let mut mesh = Mesh::new();
///
/// // Add vertices
/// let v0 = mesh.add_vertex(Vec3::new(0.0, 0.0, 0.0), Vec3::Y, Vec2::new(0.0, 0.0), Vec3::ONE);
/// let v1 = mesh.add_vertex(Vec3::new(1.0, 0.0, 0.0), Vec3::Y, Vec2::new(1.0, 0.0), Vec3::ONE);
/// let v2 = mesh.add_vertex(Vec3::new(0.0, 0.0, 1.0), Vec3::Y, Vec2::new(0.0, 1.0), Vec3::ONE);
///
/// // Add triangle
/// mesh.add_triangle(v0, v1, v2);
///
/// assert_eq!(mesh.vertex_count(), 3);
/// assert_eq!(mesh.triangle_count(), 1);
/// ```
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Mesh {
    /// Vertex positions in 3D space
    pub positions: Vec<Vec3>,

    /// Vertex normals (should be normalized)
    pub normals: Vec<Vec3>,

    /// Vertex UV texture coordinates
    pub uvs: Vec<Vec2>,

    /// Vertex colors (RGB in 0.0-1.0 range)
    pub colors: Vec<Vec3>,

    /// Triangle indices (every 3 indices form one triangle)
    pub indices: Vec<u32>,
}

impl Mesh {
    /// Create a new empty mesh
    ///
    /// # Example
    /// ```
    /// use floraison_core::geometry::mesh::Mesh;
    ///
    /// let mesh = Mesh::new();
    /// assert_eq!(mesh.vertex_count(), 0);
    /// assert_eq!(mesh.triangle_count(), 0);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new mesh with pre-allocated capacity
    ///
    /// This is more efficient when you know the approximate mesh size in advance.
    ///
    /// # Arguments
    /// * `vertex_capacity` - Expected number of vertices
    /// * `index_capacity` - Expected number of indices (triangles * 3)
    ///
    /// # Example
    /// ```
    /// use floraison_core::geometry::mesh::Mesh;
    ///
    /// let mesh = Mesh::with_capacity(100, 300); // 100 vertices, 100 triangles
    /// assert_eq!(mesh.vertex_count(), 0); // Empty but pre-allocated
    /// ```
    pub fn with_capacity(vertex_capacity: usize, index_capacity: usize) -> Self {
        Self {
            positions: Vec::with_capacity(vertex_capacity),
            normals: Vec::with_capacity(vertex_capacity),
            uvs: Vec::with_capacity(vertex_capacity),
            colors: Vec::with_capacity(vertex_capacity),
            indices: Vec::with_capacity(index_capacity),
        }
    }

    /// Add a vertex to the mesh and return its index
    ///
    /// # Arguments
    /// * `position` - 3D position of the vertex
    /// * `normal` - Surface normal at the vertex (should be normalized)
    /// * `uv` - Texture coordinates
    /// * `color` - RGB color in 0.0-1.0 range
    ///
    /// # Returns
    /// The index of the newly added vertex
    ///
    /// # Example
    /// ```
    /// use floraison_core::geometry::mesh::Mesh;
    /// use floraison_core::{Vec3, Vec2};
    ///
    /// let mut mesh = Mesh::new();
    /// let idx = mesh.add_vertex(Vec3::ZERO, Vec3::Y, Vec2::ZERO, Vec3::ONE);
    /// assert_eq!(idx, 0);
    /// ```
    pub fn add_vertex(&mut self, position: Vec3, normal: Vec3, uv: Vec2, color: Vec3) -> u32 {
        let index = self.positions.len() as u32;
        self.positions.push(position);
        self.normals.push(normal);
        self.uvs.push(uv);
        self.colors.push(color);
        index
    }

    /// Add a triangle face to the mesh
    ///
    /// # Arguments
    /// * `i0`, `i1`, `i2` - Vertex indices forming the triangle (counter-clockwise winding)
    ///
    /// # Panics
    /// Panics in debug builds if any index is out of bounds
    ///
    /// # Example
    /// ```
    /// use floraison_core::geometry::mesh::Mesh;
    /// use floraison_core::{Vec3, Vec2};
    ///
    /// let mut mesh = Mesh::new();
    /// let v0 = mesh.add_vertex(Vec3::ZERO, Vec3::Y, Vec2::ZERO, Vec3::ONE);
    /// let v1 = mesh.add_vertex(Vec3::X, Vec3::Y, Vec2::ZERO, Vec3::ONE);
    /// let v2 = mesh.add_vertex(Vec3::Z, Vec3::Y, Vec2::ZERO, Vec3::ONE);
    /// mesh.add_triangle(v0, v1, v2);
    /// ```
    pub fn add_triangle(&mut self, i0: u32, i1: u32, i2: u32) {
        debug_assert!(
            (i0 as usize) < self.positions.len(),
            "Triangle index i0={} out of bounds (vertex count: {})",
            i0,
            self.positions.len()
        );
        debug_assert!(
            (i1 as usize) < self.positions.len(),
            "Triangle index i1={} out of bounds (vertex count: {})",
            i1,
            self.positions.len()
        );
        debug_assert!(
            (i2 as usize) < self.positions.len(),
            "Triangle index i2={} out of bounds (vertex count: {})",
            i2,
            self.positions.len()
        );

        self.indices.push(i0);
        self.indices.push(i1);
        self.indices.push(i2);
    }

    /// Add a quad (quadrilateral) as two triangles
    ///
    /// The quad is defined by four vertices in counter-clockwise order.
    /// It will be triangulated as (i0, i1, i2) and (i0, i2, i3).
    ///
    /// # Arguments
    /// * `i0`, `i1`, `i2`, `i3` - Vertex indices in counter-clockwise order
    ///
    /// # Example
    /// ```
    /// use floraison_core::geometry::mesh::Mesh;
    /// use floraison_core::{Vec3, Vec2};
    ///
    /// let mut mesh = Mesh::new();
    /// let v0 = mesh.add_vertex(Vec3::new(0.0, 0.0, 0.0), Vec3::Y, Vec2::ZERO, Vec3::ONE);
    /// let v1 = mesh.add_vertex(Vec3::new(1.0, 0.0, 0.0), Vec3::Y, Vec2::ZERO, Vec3::ONE);
    /// let v2 = mesh.add_vertex(Vec3::new(1.0, 0.0, 1.0), Vec3::Y, Vec2::ZERO, Vec3::ONE);
    /// let v3 = mesh.add_vertex(Vec3::new(0.0, 0.0, 1.0), Vec3::Y, Vec2::ZERO, Vec3::ONE);
    /// mesh.add_quad(v0, v1, v2, v3);
    /// assert_eq!(mesh.triangle_count(), 2);
    /// ```
    pub fn add_quad(&mut self, i0: u32, i1: u32, i2: u32, i3: u32) {
        self.add_triangle(i0, i1, i2);
        self.add_triangle(i0, i2, i3);
    }

    /// Merge another mesh into this one
    ///
    /// All vertices and triangles from `other` are appended to this mesh.
    /// Triangle indices are automatically adjusted to account for the offset.
    ///
    /// # Arguments
    /// * `other` - The mesh to merge into this one
    ///
    /// # Example
    /// ```
    /// use floraison_core::geometry::mesh::Mesh;
    /// use floraison_core::{Vec3, Vec2};
    ///
    /// let mut mesh1 = Mesh::new();
    /// let v0 = mesh1.add_vertex(Vec3::ZERO, Vec3::Y, Vec2::ZERO, Vec3::ONE);
    /// let v1 = mesh1.add_vertex(Vec3::X, Vec3::Y, Vec2::ZERO, Vec3::ONE);
    /// let v2 = mesh1.add_vertex(Vec3::Z, Vec3::Y, Vec2::ZERO, Vec3::ONE);
    /// mesh1.add_triangle(v0, v1, v2);
    ///
    /// let mut mesh2 = Mesh::new();
    /// let v0 = mesh2.add_vertex(Vec3::new(2.0, 0.0, 0.0), Vec3::Y, Vec2::ZERO, Vec3::ONE);
    /// let v1 = mesh2.add_vertex(Vec3::new(3.0, 0.0, 0.0), Vec3::Y, Vec2::ZERO, Vec3::ONE);
    /// let v2 = mesh2.add_vertex(Vec3::new(2.0, 0.0, 1.0), Vec3::Y, Vec2::ZERO, Vec3::ONE);
    /// mesh2.add_triangle(v0, v1, v2);
    ///
    /// mesh1.merge(&mesh2);
    /// assert_eq!(mesh1.vertex_count(), 6);
    /// assert_eq!(mesh1.triangle_count(), 2);
    /// ```
    pub fn merge(&mut self, other: &Mesh) {
        let index_offset = self.positions.len() as u32;

        // Append vertex data
        self.positions.extend_from_slice(&other.positions);
        self.normals.extend_from_slice(&other.normals);
        self.uvs.extend_from_slice(&other.uvs);
        self.colors.extend_from_slice(&other.colors);

        // Append indices with offset
        self.indices
            .extend(other.indices.iter().map(|&idx| idx + index_offset));
    }

    /// Compute vertex normals from face geometry
    ///
    /// This replaces all existing normals with normals computed from the mesh triangles.
    /// Each vertex normal is the normalized sum of all adjacent face normals (weighted by area).
    ///
    /// Degenerate triangles (zero area) are skipped and don't contribute to normals.
    ///
    /// # Example
    /// ```
    /// use floraison_core::geometry::mesh::Mesh;
    /// use floraison_core::{Vec3, Vec2};
    ///
    /// let mut mesh = Mesh::new();
    /// // Add vertices with dummy normals (counter-clockwise winding when viewed from above)
    /// let v0 = mesh.add_vertex(Vec3::new(0.0, 0.0, 0.0), Vec3::ZERO, Vec2::ZERO, Vec3::ONE);
    /// let v1 = mesh.add_vertex(Vec3::new(0.0, 0.0, 1.0), Vec3::ZERO, Vec2::ZERO, Vec3::ONE);
    /// let v2 = mesh.add_vertex(Vec3::new(1.0, 0.0, 0.0), Vec3::ZERO, Vec2::ZERO, Vec3::ONE);
    /// mesh.add_triangle(v0, v1, v2);
    ///
    /// // Compute proper normals from geometry
    /// mesh.compute_normals();
    ///
    /// // All normals should point up (Y-axis) for this horizontal triangle
    /// assert!((mesh.normals[0].y - 1.0).abs() < 0.001);
    /// ```
    pub fn compute_normals(&mut self) {
        // Reset all normals to zero
        self.normals.clear();
        self.normals.resize(self.positions.len(), Vec3::ZERO);

        // Accumulate face normals (weighted by area)
        for i in (0..self.indices.len()).step_by(3) {
            let i0 = self.indices[i] as usize;
            let i1 = self.indices[i + 1] as usize;
            let i2 = self.indices[i + 2] as usize;

            let v0 = self.positions[i0];
            let v1 = self.positions[i1];
            let v2 = self.positions[i2];

            // Compute face normal (cross product gives area-weighted normal)
            let edge1 = v1 - v0;
            let edge2 = v2 - v0;
            let face_normal = edge1.cross(edge2);

            // Skip degenerate triangles (zero or near-zero area)
            if face_normal.length_squared() < 1e-10 {
                continue;
            }

            // Accumulate to vertex normals (already area-weighted)
            self.normals[i0] += face_normal;
            self.normals[i1] += face_normal;
            self.normals[i2] += face_normal;
        }

        // Normalize all vertex normals
        for normal in &mut self.normals {
            let len = normal.length();
            if len > 1e-6 {
                *normal /= len;
            } else {
                // Fallback for vertices with no valid adjacent faces
                *normal = Vec3::Y;
            }
        }
    }

    /// Apply a transformation matrix to all vertices
    ///
    /// Positions are transformed by the matrix, while normals are transformed
    /// by the inverse transpose to handle non-uniform scaling correctly.
    ///
    /// # Arguments
    /// * `matrix` - 4x4 transformation matrix
    ///
    /// # Example
    /// ```
    /// use floraison_core::geometry::mesh::Mesh;
    /// use floraison_core::{Vec3, Vec2, Mat4};
    ///
    /// let mut mesh = Mesh::new();
    /// mesh.add_vertex(Vec3::new(1.0, 0.0, 0.0), Vec3::Y, Vec2::ZERO, Vec3::ONE);
    ///
    /// // Translate by (1, 2, 3)
    /// let transform = Mat4::from_translation(Vec3::new(1.0, 2.0, 3.0));
    /// mesh.transform(&transform);
    ///
    /// assert_eq!(mesh.positions[0], Vec3::new(2.0, 2.0, 3.0));
    /// ```
    pub fn transform(&mut self, matrix: &Mat4) {
        // Transform positions
        for pos in &mut self.positions {
            *pos = matrix.transform_point3(*pos);
        }

        // Transform normals using inverse transpose
        // This ensures normals remain perpendicular to surfaces under non-uniform scaling
        let normal_matrix = matrix.inverse().transpose();
        for normal in &mut self.normals {
            *normal = normal_matrix.transform_vector3(*normal);
            // Re-normalize after transformation
            let len = normal.length();
            if len > 1e-6 {
                *normal /= len;
            }
        }
    }

    /// Get the number of vertices in the mesh
    ///
    /// # Example
    /// ```
    /// use floraison_core::geometry::mesh::Mesh;
    /// use floraison_core::{Vec3, Vec2};
    ///
    /// let mut mesh = Mesh::new();
    /// mesh.add_vertex(Vec3::ZERO, Vec3::Y, Vec2::ZERO, Vec3::ONE);
    /// mesh.add_vertex(Vec3::X, Vec3::Y, Vec2::ZERO, Vec3::ONE);
    /// assert_eq!(mesh.vertex_count(), 2);
    /// ```
    pub fn vertex_count(&self) -> usize {
        self.positions.len()
    }

    /// Get the number of triangles in the mesh
    ///
    /// # Example
    /// ```
    /// use floraison_core::geometry::mesh::Mesh;
    /// use floraison_core::{Vec3, Vec2};
    ///
    /// let mut mesh = Mesh::new();
    /// let v0 = mesh.add_vertex(Vec3::ZERO, Vec3::Y, Vec2::ZERO, Vec3::ONE);
    /// let v1 = mesh.add_vertex(Vec3::X, Vec3::Y, Vec2::ZERO, Vec3::ONE);
    /// let v2 = mesh.add_vertex(Vec3::Z, Vec3::Y, Vec2::ZERO, Vec3::ONE);
    /// mesh.add_triangle(v0, v1, v2);
    /// mesh.add_triangle(v0, v2, v1);
    /// assert_eq!(mesh.triangle_count(), 2);
    /// ```
    pub fn triangle_count(&self) -> usize {
        self.indices.len() / 3
    }

    /// Check if the mesh is empty (has no vertices)
    ///
    /// # Example
    /// ```
    /// use floraison_core::geometry::mesh::Mesh;
    ///
    /// let mesh = Mesh::new();
    /// assert!(mesh.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.positions.is_empty()
    }

    /// Clear all mesh data
    ///
    /// # Example
    /// ```
    /// use floraison_core::geometry::mesh::Mesh;
    /// use floraison_core::{Vec3, Vec2};
    ///
    /// let mut mesh = Mesh::new();
    /// mesh.add_vertex(Vec3::ZERO, Vec3::Y, Vec2::ZERO, Vec3::ONE);
    /// mesh.clear();
    /// assert!(mesh.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.positions.clear();
        self.normals.clear();
        self.uvs.clear();
        self.indices.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f32 = 1e-5;

    #[test]
    fn test_new_mesh_is_empty() {
        let mesh = Mesh::new();
        assert!(mesh.is_empty());
        assert_eq!(mesh.vertex_count(), 0);
        assert_eq!(mesh.triangle_count(), 0);
    }

    #[test]
    fn test_add_vertex() {
        let mut mesh = Mesh::new();
        let idx0 = mesh.add_vertex(Vec3::ZERO, Vec3::Y, Vec2::ZERO, Vec3::ONE);
        let idx1 = mesh.add_vertex(Vec3::X, Vec3::Y, Vec2::new(1.0, 0.0), Vec3::ONE);

        assert_eq!(idx0, 0);
        assert_eq!(idx1, 1);
        assert_eq!(mesh.vertex_count(), 2);

        assert_eq!(mesh.positions[0], Vec3::ZERO);
        assert_eq!(mesh.positions[1], Vec3::X);
        assert_eq!(mesh.uvs[1], Vec2::new(1.0, 0.0));
    }

    #[test]
    fn test_add_triangle() {
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Vec3::ZERO, Vec3::Y, Vec2::ZERO, Vec3::ONE);
        let v1 = mesh.add_vertex(Vec3::X, Vec3::Y, Vec2::ZERO, Vec3::ONE);
        let v2 = mesh.add_vertex(Vec3::Z, Vec3::Y, Vec2::ZERO, Vec3::ONE);

        mesh.add_triangle(v0, v1, v2);

        assert_eq!(mesh.triangle_count(), 1);
        assert_eq!(mesh.indices.len(), 3);
        assert_eq!(mesh.indices, vec![0, 1, 2]);
    }

    #[test]
    fn test_add_quad() {
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Vec3::new(0.0, 0.0, 0.0), Vec3::Y, Vec2::ZERO, Vec3::ONE);
        let v1 = mesh.add_vertex(Vec3::new(1.0, 0.0, 0.0), Vec3::Y, Vec2::ZERO, Vec3::ONE);
        let v2 = mesh.add_vertex(Vec3::new(1.0, 0.0, 1.0), Vec3::Y, Vec2::ZERO, Vec3::ONE);
        let v3 = mesh.add_vertex(Vec3::new(0.0, 0.0, 1.0), Vec3::Y, Vec2::ZERO, Vec3::ONE);

        mesh.add_quad(v0, v1, v2, v3);

        assert_eq!(mesh.triangle_count(), 2);
        assert_eq!(mesh.indices, vec![0, 1, 2, 0, 2, 3]);
    }

    #[test]
    fn test_merge_meshes() {
        let mut mesh1 = Mesh::new();
        let v0 = mesh1.add_vertex(Vec3::ZERO, Vec3::Y, Vec2::ZERO, Vec3::ONE);
        let v1 = mesh1.add_vertex(Vec3::X, Vec3::Y, Vec2::ZERO, Vec3::ONE);
        let v2 = mesh1.add_vertex(Vec3::Z, Vec3::Y, Vec2::ZERO, Vec3::ONE);
        mesh1.add_triangle(v0, v1, v2);

        let mut mesh2 = Mesh::new();
        let v0 = mesh2.add_vertex(Vec3::new(2.0, 0.0, 0.0), Vec3::Y, Vec2::ZERO, Vec3::ONE);
        let v1 = mesh2.add_vertex(Vec3::new(3.0, 0.0, 0.0), Vec3::Y, Vec2::ZERO, Vec3::ONE);
        let v2 = mesh2.add_vertex(Vec3::new(2.0, 0.0, 1.0), Vec3::Y, Vec2::ZERO, Vec3::ONE);
        mesh2.add_triangle(v0, v1, v2);

        let original_vertex_count = mesh1.vertex_count();
        mesh1.merge(&mesh2);

        assert_eq!(mesh1.vertex_count(), 6);
        assert_eq!(mesh1.triangle_count(), 2);

        // Check that indices are properly offset
        assert_eq!(mesh1.indices[0], 0);
        assert_eq!(mesh1.indices[1], 1);
        assert_eq!(mesh1.indices[2], 2);
        assert_eq!(mesh1.indices[3], 3); // 0 + offset
        assert_eq!(mesh1.indices[4], 4); // 1 + offset
        assert_eq!(mesh1.indices[5], 5); // 2 + offset

        // Check that second mesh's positions were copied
        assert_eq!(
            mesh1.positions[original_vertex_count],
            Vec3::new(2.0, 0.0, 0.0)
        );
    }

    #[test]
    fn test_compute_normals_horizontal_triangle() {
        let mut mesh = Mesh::new();
        // Create a horizontal triangle in the XZ plane (should have Y-up normal)
        // Counter-clockwise winding when viewed from above
        let v0 = mesh.add_vertex(Vec3::new(0.0, 0.0, 0.0), Vec3::ZERO, Vec2::ZERO, Vec3::ONE);
        let v1 = mesh.add_vertex(Vec3::new(0.0, 0.0, 1.0), Vec3::ZERO, Vec2::ZERO, Vec3::ONE);
        let v2 = mesh.add_vertex(Vec3::new(1.0, 0.0, 0.0), Vec3::ZERO, Vec2::ZERO, Vec3::ONE);
        mesh.add_triangle(v0, v1, v2);

        mesh.compute_normals();

        // All normals should point up (Y-axis)
        for normal in &mesh.normals {
            assert!(
                (normal.x.abs()) < EPSILON,
                "Normal X component should be near 0, got {}",
                normal.x
            );
            assert!(
                (normal.y - 1.0).abs() < EPSILON,
                "Normal Y component should be 1, got {}",
                normal.y
            );
            assert!(
                (normal.z.abs()) < EPSILON,
                "Normal Z component should be near 0, got {}",
                normal.z
            );
        }
    }

    #[test]
    fn test_compute_normals_shared_vertex() {
        let mut mesh = Mesh::new();
        // Create two triangles sharing a vertex (forming a tent)
        let v0 = mesh.add_vertex(Vec3::new(0.0, 0.0, 0.0), Vec3::ZERO, Vec2::ZERO, Vec3::ONE);
        let v1 = mesh.add_vertex(Vec3::new(1.0, 0.0, 0.0), Vec3::ZERO, Vec2::ZERO, Vec3::ONE);
        let v2 = mesh.add_vertex(Vec3::new(0.5, 1.0, 0.0), Vec3::ZERO, Vec2::ZERO, Vec3::ONE);
        let v3 = mesh.add_vertex(Vec3::new(0.5, 0.0, 1.0), Vec3::ZERO, Vec2::ZERO, Vec3::ONE);

        mesh.add_triangle(v0, v1, v2); // Front triangle
        mesh.add_triangle(v0, v3, v1); // Bottom triangle

        mesh.compute_normals();

        // Shared vertices should have averaged normals
        // Just check they're normalized
        for normal in &mesh.normals {
            let length = normal.length();
            assert!(
                (length - 1.0).abs() < EPSILON,
                "Normal should be normalized, length = {}",
                length
            );
        }
    }

    #[test]
    fn test_compute_normals_degenerate_triangle() {
        let mut mesh = Mesh::new();
        // Create a degenerate triangle (collinear points)
        let v0 = mesh.add_vertex(Vec3::new(0.0, 0.0, 0.0), Vec3::ZERO, Vec2::ZERO, Vec3::ONE);
        let v1 = mesh.add_vertex(Vec3::new(1.0, 0.0, 0.0), Vec3::ZERO, Vec2::ZERO, Vec3::ONE);
        let v2 = mesh.add_vertex(Vec3::new(2.0, 0.0, 0.0), Vec3::ZERO, Vec2::ZERO, Vec3::ONE);
        mesh.add_triangle(v0, v1, v2);

        // Should not panic
        mesh.compute_normals();

        // Should have fallback normals (Y-up)
        for normal in &mesh.normals {
            assert_eq!(*normal, Vec3::Y);
        }
    }

    #[test]
    fn test_transform_translation() {
        let mut mesh = Mesh::new();
        mesh.add_vertex(Vec3::new(1.0, 0.0, 0.0), Vec3::Y, Vec2::ZERO, Vec3::ONE);

        let transform = Mat4::from_translation(Vec3::new(1.0, 2.0, 3.0));
        mesh.transform(&transform);

        assert_eq!(mesh.positions[0], Vec3::new(2.0, 2.0, 3.0));
        // Normals should be unchanged by translation
        assert!((mesh.normals[0] - Vec3::Y).length() < EPSILON);
    }

    #[test]
    fn test_transform_rotation() {
        let mut mesh = Mesh::new();
        mesh.add_vertex(Vec3::new(1.0, 0.0, 0.0), Vec3::Y, Vec2::ZERO, Vec3::ONE);

        // Rotate 90 degrees around Z-axis
        let transform = Mat4::from_rotation_z(std::f32::consts::PI / 2.0);
        mesh.transform(&transform);

        // X-axis point should rotate to Y-axis
        assert!((mesh.positions[0].x - 0.0).abs() < EPSILON);
        assert!((mesh.positions[0].y - 1.0).abs() < EPSILON);
        assert!((mesh.positions[0].z - 0.0).abs() < EPSILON);

        // Y-up normal should rotate to -X
        assert!((mesh.normals[0].x - (-1.0)).abs() < EPSILON);
        assert!((mesh.normals[0].y - 0.0).abs() < EPSILON);
    }

    #[test]
    fn test_transform_scale_uniform() {
        let mut mesh = Mesh::new();
        mesh.add_vertex(Vec3::new(1.0, 2.0, 3.0), Vec3::Y, Vec2::ZERO, Vec3::ONE);

        let transform = Mat4::from_scale(Vec3::splat(2.0));
        mesh.transform(&transform);

        assert_eq!(mesh.positions[0], Vec3::new(2.0, 4.0, 6.0));
        // Normals should remain normalized with uniform scaling
        assert!((mesh.normals[0].length() - 1.0).abs() < EPSILON);
    }

    #[test]
    fn test_transform_scale_nonuniform() {
        let mut mesh = Mesh::new();
        mesh.add_vertex(Vec3::X, Vec3::Y, Vec2::ZERO, Vec3::ONE);

        // Non-uniform scale
        let transform = Mat4::from_scale(Vec3::new(2.0, 0.5, 1.0));
        mesh.transform(&transform);

        assert_eq!(mesh.positions[0], Vec3::new(2.0, 0.0, 0.0));
        // Normal should still be normalized after inverse transpose
        assert!((mesh.normals[0].length() - 1.0).abs() < EPSILON);
    }

    #[test]
    fn test_clear() {
        let mut mesh = Mesh::new();
        mesh.add_vertex(Vec3::ZERO, Vec3::Y, Vec2::ZERO, Vec3::ONE);
        mesh.add_vertex(Vec3::X, Vec3::Y, Vec2::ZERO, Vec3::ONE);

        mesh.clear();

        assert!(mesh.is_empty());
        assert_eq!(mesh.vertex_count(), 0);
        assert_eq!(mesh.triangle_count(), 0);
    }

    #[test]
    fn test_with_capacity() {
        let mesh = Mesh::with_capacity(100, 300);
        assert!(mesh.is_empty());
        assert!(mesh.positions.capacity() >= 100);
        assert!(mesh.indices.capacity() >= 300);
    }
}
