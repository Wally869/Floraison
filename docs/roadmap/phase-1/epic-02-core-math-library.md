# Epic 2: Core Math Library

**Phase**: 1 - Foundation

**Goal**: Implement fundamental mathematical primitives for geometry generation.

**Estimated Effort**: 6-8 hours

---

## Task 2.1: Vector Math Wrapper ✅

**Description**: Set up 3D vector math using `glam` crate with convenience wrappers.

**Status**: ✅ COMPLETED

**Acceptance Criteria**:
- [x] `glam` crate added to `floraison-core`
- [x] Re-export types: `Vec3`, `Vec2`, `Mat3`, `Mat4`, `Quat`
- [x] Add custom helper functions:
  - `Vec3Ext::from_cylindrical(radius, angle, height) -> Vec3`
  - `Vec3Ext::to_cylindrical(&self) -> (f32, f32, f32)`
  - `Vec3Ext::from_spherical(radius, theta, phi) -> Vec3`
  - `Vec3Ext::to_spherical(&self) -> (f32, f32, f32)`
  - `Vec2Ext::from_polar(radius, angle) -> Vec2`
  - `Vec2Ext::to_polar(&self) -> (f32, f32)`
  - `Vec2Ext::rotate_by_angle(&self, angle) -> Vec2`
  - Utility functions: `lerp`, `smoothstep`, `remap`
- [x] Unit tests for all helper functions (12 tests, all passing)
- [x] Documentation comments with examples on all public functions
- [x] Doc-tests pass (10 doc-tests)
- [x] WASM compatibility verified

**Dependencies**: Task 1.1

**Technical Notes**:
```rust
// floraison-core/src/math/vector.rs
pub use glam::{Vec2, Vec3, Mat3, Mat4, Quat};

pub trait Vec3Extensions {
    fn cylindrical(radius: f32, angle: f32, height: f32) -> Vec3;
}

impl Vec3Extensions for Vec3 {
    fn cylindrical(radius: f32, angle: f32, height: f32) -> Vec3 {
        Vec3::new(
            radius * angle.cos(),
            height,
            radius * angle.sin()
        )
    }
}
```

**Effort**: 1 hour

---

## Task 2.2: Mesh Data Structures ✅

**Description**: Define core mesh representation with vertices, indices, normals, UVs.

**Status**: ✅ COMPLETED

**Acceptance Criteria**:
- [x] `Mesh` struct created with fields:
  - `positions: Vec<Vec3>`
  - `normals: Vec<Vec3>`
  - `uvs: Vec<Vec2>`
  - `indices: Vec<u32>`
- [x] Methods implemented:
  - `new() -> Mesh`
  - `with_capacity(vertices, indices) -> Mesh`
  - `add_vertex(pos, normal, uv) -> u32` (returns index)
  - `add_triangle(i0, i1, i2)`
  - `add_quad(i0, i1, i2, i3)` (bonus: adds two triangles)
  - `merge(&mut self, other: &Mesh)` (combines meshes with index offsetting)
  - `compute_normals(&mut self)` (area-weighted, handles degenerate triangles)
  - `transform(&mut self, matrix: Mat4)` (inverse transpose for normals)
  - `vertex_count()`, `triangle_count()`, `is_empty()`, `clear()`
- [x] Unit tests for all methods (14 tests, all passing)
- [x] Doc-tests with examples (13 doc-tests, all passing)
- [x] Serialization support with serde feature flag
- [x] Comprehensive documentation with usage examples
- [x] WASM compatibility verified

**Dependencies**: Task 2.1

**Technical Notes**:
```rust
#[derive(Debug, Clone, Default)]
pub struct Mesh {
    pub positions: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub uvs: Vec<Vec2>,
    pub indices: Vec<u32>,
}

impl Mesh {
    pub fn compute_normals(&mut self) {
        // Reset normals
        self.normals = vec![Vec3::ZERO; self.positions.len()];

        // Accumulate face normals
        for i in (0..self.indices.len()).step_by(3) {
            let i0 = self.indices[i] as usize;
            let i1 = self.indices[i+1] as usize;
            let i2 = self.indices[i+2] as usize;

            let v0 = self.positions[i0];
            let v1 = self.positions[i1];
            let v2 = self.positions[i2];

            let normal = (v1 - v0).cross(v2 - v0).normalize();

            self.normals[i0] += normal;
            self.normals[i1] += normal;
            self.normals[i2] += normal;
        }

        // Normalize
        for n in &mut self.normals {
            *n = n.normalize();
        }
    }
}
```

**Effort**: 2 hours

---

## Task 2.3: Phyllotaxis Functions ✅

**Description**: Implement Fibonacci spiral arrangement calculations.

**Status**: ✅ COMPLETED

**Acceptance Criteria**:
- [x] Module `floraison-core/src/math/phyllotaxis.rs` created
- [x] Constant `GOLDEN_ANGLE` = 137.5078° in radians (2.39996322972865332)
- [x] Function `fibonacci_angle(n: usize) -> f32` calculates angle for Fibonacci sequence
- [x] Function `vogel_spiral(index, count, radius) -> Vec2` for optimal disc packing
- [x] Function `radial_positions(count, radius, offset) -> Vec<Vec2>` for evenly spaced arrangements
- [x] Function `whorled_positions(count, radius, height, offset) -> Vec<Vec3>` for 3D whorls
- [x] Function `fibonacci_spiral_3d(count, radius, height, radius_fn) -> Vec<Vec3>` for cylindrical spirals
- [x] Radius variation functions: `radius_constant`, `radius_linear`, `radius_quadratic`, `radius_bulge`
- [x] Precomputed constants for common angles (90°, 120°, 144°, 180°)
- [x] Unit tests verify all arrangements (21 tests, all passing)
- [x] Doc-tests with usage examples (11 doc-tests, all passing)
- [x] Comprehensive documentation with botanical context and examples
- [x] WASM compatibility verified

**Dependencies**: Task 2.1

**Technical Notes**:
```rust
use std::f32::consts::PI;

/// Golden angle in radians (≈ 2.399963 rad ≈ 137.5078°)
pub const GOLDEN_ANGLE: f32 = PI * (3.0 - 5.0_f32.sqrt());

/// Common phyllotactic angles used in nature
pub const ANGLE_180: f32 = PI;
pub const ANGLE_120: f32 = 2.0 * PI / 3.0;
pub const ANGLE_144: f32 = 2.0 * PI * 2.0 / 5.0;

/// Calculate position using Vogel's method for optimal disc packing
/// Used for indefinite numbers of components (e.g., Ranunculus stamens)
pub fn vogel_spiral(index: usize, count: usize, radius: f32) -> Vec2 {
    let angle = index as f32 * GOLDEN_ANGLE;
    let r = radius * (index as f32 / count as f32).sqrt();
    Vec2::new(r * angle.cos(), r * angle.sin())
}
```

**Effort**: 1.5 hours

---

## Task 2.4: Surface of Revolution Generator ✅

**Description**: Create function to generate mesh from 2D profile curve rotated around axis.

**Status**: ✅ COMPLETED

**Acceptance Criteria**:
- [x] Function `surface_of_revolution(profile: &[Vec2], segments: usize) -> Mesh`
- [x] Profile points represent (radius, height) pairs (x=radius, y=height)
- [x] Generates proper triangulation with smooth normals and correct winding order
- [x] Handles degenerate cases (radius = 0 at poles with triangle fans)
- [x] Proper UV mapping (u = angle/2π, v = normalized height)
- [x] Convenience functions: `cylinder()`, `cone()`, `uv_sphere()`
- [x] Unit tests with simple shapes (11 tests, all passing):
  - Cylinder geometry and normals
  - Cone with pole handling
  - UV sphere
  - Double cone (two poles)
  - Bulbous receptacle
  - UV mapping verification
  - Edge case handling
- [x] Doc-tests with examples (7 doc-tests, all passing)
- [x] Comprehensive documentation with botanical context
- [x] WASM compatibility verified

**Dependencies**: Task 2.2

**Technical Notes**:
```rust
/// Generate mesh by revolving a 2D profile around the Y axis
/// profile: Array of (radius, height) points, ordered bottom to top
/// segments: Number of angular divisions (typically 16-32)
pub fn surface_of_revolution(profile: &[Vec2], segments: usize) -> Mesh {
    let mut mesh = Mesh::default();

    let angle_step = 2.0 * PI / segments as f32;

    // Generate vertices
    for &point in profile {
        for seg in 0..=segments {
            let angle = seg as f32 * angle_step;
            let x = point.x * angle.cos();
            let z = point.x * angle.sin();
            let y = point.y;

            mesh.positions.push(Vec3::new(x, y, z));
            mesh.uvs.push(Vec2::new(
                seg as f32 / segments as f32,
                // Find v by position in profile array
                profile.iter().position(|&p| p == point).unwrap() as f32 / (profile.len() - 1) as f32
            ));
        }
    }

    // Generate triangles
    for i in 0..profile.len() - 1 {
        for j in 0..segments {
            let i0 = i * (segments + 1) + j;
            let i1 = i0 + 1;
            let i2 = i0 + segments + 1;
            let i3 = i2 + 1;

            mesh.add_triangle(i0 as u32, i2 as u32, i1 as u32);
            mesh.add_triangle(i1 as u32, i2 as u32, i3 as u32);
        }
    }

    mesh.compute_normals();
    mesh
}
```

**Effort**: 2.5 hours

---

## Task 2.5: Bézier Curve Utilities ✅

**Description**: Implement Bézier curve evaluation for smooth profiles.

**Status**: ✅ COMPLETED

**Acceptance Criteria**:
- [x] Function `quadratic_bezier_2d(p0, p1, p2, t) -> Vec2`
- [x] Function `cubic_bezier_2d(p0, p1, p2, p3, t) -> Vec2`
- [x] 3D variants: `quadratic_bezier_3d()` and `cubic_bezier_3d()`
- [x] Derivative functions for tangent calculation:
  - `quadratic_bezier_derivative_2d()`
  - `cubic_bezier_derivative_2d()`
- [x] Sampling functions:
  - `sample_quadratic_2d()`
  - `sample_cubic_2d()`
  - `sample_cubic_3d()`
- [x] Unit tests verify curve properties (16 tests, all passing):
  - Endpoint validation
  - Midpoint calculations
  - Derivative/tangent correctness
  - Degenerate cases (linear curves)
  - Curve continuity
  - Edge case handling
- [x] Doc-tests with practical examples (10 doc-tests, all passing)
- [x] Comprehensive documentation with usage examples
- [x] WASM compatibility verified

**Dependencies**: Task 2.1

**Technical Notes**:
```rust
/// Evaluate cubic Bézier curve at parameter t ∈ [0, 1]
pub fn bezier_cubic(p0: Vec2, p1: Vec2, p2: Vec2, p3: Vec2, t: f32) -> Vec2 {
    let t2 = t * t;
    let t3 = t2 * t;
    let mt = 1.0 - t;
    let mt2 = mt * mt;
    let mt3 = mt2 * mt;

    p0 * mt3 + p1 * (3.0 * mt2 * t) + p2 * (3.0 * mt * t2) + p3 * t3
}

/// Sample cubic Bézier curve at regular intervals
pub fn sample_bezier_cubic(p0: Vec2, p1: Vec2, p2: Vec2, p3: Vec2, samples: usize) -> Vec<Vec2> {
    (0..=samples)
        .map(|i| {
            let t = i as f32 / samples as f32;
            bezier_cubic(p0, p1, p2, p3, t)
        })
        .collect()
}
```

**Effort**: 1.5 hours

---
