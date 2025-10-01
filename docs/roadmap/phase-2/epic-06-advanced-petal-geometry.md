# Epic 6: Advanced Petal Geometry (B-splines)

**Phase**: 2 - Single Flower Refinement

**Goal**: Replace simple flat petals with B-spline surfaces that support deformations.

**Estimated Effort**: 10-12 hours

**Status**: ✅ COMPLETED

---

## Task 6.1: B-Spline Basis Function Evaluation ✅

**Description**: Implement Cox-de Boor algorithm for B-spline basis functions.

**Status**: ✅ COMPLETED

**Acceptance Criteria**:
- [x] Module `floraison-core/src/math/bspline.rs` created
- [x] Function `basis_function(i: usize, p: usize, u: f32, knots: &[f32]) -> f32`
  - Implements Cox-de Boor recursion
  - Handles edge cases (division by zero, endpoint u=1.0)
- [x] Function `generate_knot_vector(n: usize, p: usize, uniform: bool) -> Vec<f32>`
  - Creates open uniform knot vector
- [x] Unit tests verify known properties:
  - Partition of unity: Σ Nᵢ(u) = 1
  - Local support: Nᵢ(u) = 0 outside [uᵢ, uᵢ₊ₚ₊₁]
  - Endpoint interpolation
- [x] Documentation with mathematical notation

**Dependencies**: Task 2.1

**Technical Notes**:
```rust
/// Cox-de Boor recursion for B-spline basis functions
/// i: basis function index
/// p: degree
/// u: parameter value
/// knots: knot vector
pub fn basis_function(i: usize, p: usize, u: f32, knots: &[f32]) -> f32 {
    if p == 0 {
        // Degree 0: step function
        if u >= knots[i] && u < knots[i + 1] {
            1.0
        } else {
            0.0
        }
    } else {
        // Recursive case
        let left_num = u - knots[i];
        let left_denom = knots[i + p] - knots[i];
        let left = if left_denom.abs() < 1e-6 {
            0.0
        } else {
            left_num / left_denom * basis_function(i, p - 1, u, knots)
        };

        let right_num = knots[i + p + 1] - u;
        let right_denom = knots[i + p + 1] - knots[i + 1];
        let right = if right_denom.abs() < 1e-6 {
            0.0
        } else {
            right_num / right_denom * basis_function(i + 1, p - 1, u, knots)
        };

        left + right
    }
}

/// Generate open uniform knot vector for B-spline
/// n: number of control points
/// p: degree
pub fn generate_knot_vector(n: usize, p: usize, uniform: bool) -> Vec<f32> {
    let m = n + p + 1;
    let mut knots = vec![0.0; m];

    // First p+1 knots are 0
    for i in 0..=p {
        knots[i] = 0.0;
    }

    // Middle knots
    if uniform {
        for i in (p + 1)..(n) {
            knots[i] = (i - p) as f32 / (n - p) as f32;
        }
    }

    // Last p+1 knots are 1
    for i in n..m {
        knots[i] = 1.0;
    }

    knots
}
```

**Effort**: 3 hours

---

## Task 6.2: B-Spline Surface Evaluation ✅

**Description**: Implement tensor product B-spline surface evaluation.

**Status**: ✅ COMPLETED

**Acceptance Criteria**:
- [x] Struct `BSplineSurface` in `bspline.rs`:
  ```rust
  pub struct BSplineSurface {
      pub control_points: Vec<Vec<Vec3>>,  // 2D grid
      pub degree_u: usize,
      pub degree_v: usize,
      pub knots_u: Vec<f32>,
      pub knots_v: Vec<f32>,
  }
  ```
- [x] Method `evaluate(&self, u: f32, v: f32) -> Vec3` evaluates surface at (u,v)
- [x] Method `evaluate_derivative_u(&self, u: f32, v: f32) -> Vec3` for tangent (numerical)
- [x] Method `evaluate_derivative_v(&self, u: f32, v: f32) -> Vec3` for tangent (numerical)
- [x] Method `normal(&self, u: f32, v: f32) -> Vec3` via cross product
- [x] Unit tests verify:
  - Interpolation of corner control points
  - Surface lies in convex hull of control points
- [x] Documentation

**Dependencies**: Task 6.1

**Technical Notes**:
```rust
impl BSplineSurface {
    pub fn evaluate(&self, u: f32, v: f32) -> Vec3 {
        let n = self.control_points.len();
        let m = self.control_points[0].len();

        let mut point = Vec3::ZERO;

        for i in 0..n {
            for j in 0..m {
                let basis_u = basis_function(i, self.degree_u, u, &self.knots_u);
                let basis_v = basis_function(j, self.degree_v, v, &self.knots_v);
                point += self.control_points[i][j] * basis_u * basis_v;
            }
        }

        point
    }

    pub fn normal(&self, u: f32, v: f32) -> Vec3 {
        let tangent_u = self.evaluate_derivative_u(u, v);
        let tangent_v = self.evaluate_derivative_v(u, v);
        tangent_u.cross(tangent_v).normalize()
    }
}
```

**Effort**: 3 hours

---

## Task 6.3: Petal Control Point Grid Generation ✅

**Description**: Generate initial control point grid from outline parameters.

**Status**: ✅ COMPLETED

**Acceptance Criteria**:
- [x] Function in `petal.rs`: `generate_control_grid(params: &PetalParams) -> Vec<Vec<Vec3>>`
- [x] Creates grid matching petal outline shape:
  - Base: narrow
  - Middle: widest
  - Tip: tapered
- [x] Grid dimensions 9×5 (rows × cols)
- [x] All points initially in XY plane (Z=0)
- [x] Proper spacing for good surface quality
- [x] Unit tests verify grid dimensions, symmetry, and boundary points

**Dependencies**: Task 3.4

**Technical Notes**:
```rust
pub fn generate_control_grid(params: &PetalParams) -> Vec<Vec<Vec3>> {
    let rows = 9;  // along length (v direction)
    let cols = 5;  // across width (u direction)

    let mut grid = vec![vec![Vec3::ZERO; cols]; rows];

    for row in 0..rows {
        let v = row as f32 / (rows - 1) as f32;
        let y = v * params.length;

        // Interpolate width along length
        let width_at_v = if v < 0.5 {
            // Base to middle: narrow to wide
            params.base_width + (params.width - params.base_width) * (v * 2.0)
        } else {
            // Middle to tip: wide to narrow
            params.width + (params.tip_width - params.width) * ((v - 0.5) * 2.0)
        };

        for col in 0..cols {
            let u = col as f32 / (cols - 1) as f32;
            let x = (u - 0.5) * width_at_v;

            grid[row][col] = Vec3::new(x, y, 0.0);
        }
    }

    grid
}
```

**Effort**: 2 hours

---

## Task 6.4: Petal Surface Deformations ✅

**Description**: Apply curl, twist, and ruffle deformations to petal surface.

**Status**: ✅ COMPLETED

**Acceptance Criteria**:
- [x] Function `apply_curl(control_points: &mut Vec<Vec<Vec3>>, amount: f32)`
  - Bends petal around horizontal axis
  - Amount in range [-1, 1]: negative = curl down, positive = curl up
- [x] Function `apply_twist(control_points: &mut Vec<Vec<Vec3>>, angle: f32)`
  - Twists petal around central vein
  - Angle in degrees
  - Twist increases toward tip
- [x] Function `apply_ruffle(control_points: &mut Vec<Vec<Vec3>>, freq: f32, amp: f32)`
  - Adds sinusoidal waves to edges
  - Frequency = number of waves
  - Amplitude = wave height
- [x] Each function modifies control points in place
- [x] Combined deformations work correctly
- [x] Unit tests verify deformation magnitudes and composition

**Dependencies**: Task 6.2, 6.3

**Technical Notes**:
```rust
pub fn apply_curl(control_points: &mut Vec<Vec<Vec3>>, amount: f32) {
    let rows = control_points.len();

    for (row_idx, row) in control_points.iter_mut().enumerate() {
        let v = row_idx as f32 / (rows - 1) as f32;

        // Curl increases along length
        let curl_angle = amount * v * std::f32::consts::PI * 0.5;

        for point in row.iter_mut() {
            let y = point.y;
            let z = point.z;

            // Rotate in YZ plane
            point.y = y * curl_angle.cos() - z * curl_angle.sin();
            point.z = y * curl_angle.sin() + z * curl_angle.cos();
        }
    }
}

pub fn apply_twist(control_points: &mut Vec<Vec<Vec3>>, angle_deg: f32) {
    let angle_rad = angle_deg.to_radians();
    let rows = control_points.len();

    for (row_idx, row) in control_points.iter_mut().enumerate() {
        let v = row_idx as f32 / (rows - 1) as f32;

        // Twist increases toward tip
        let twist_angle = angle_rad * v;

        for point in row.iter_mut() {
            let x = point.x;
            let z = point.z;

            // Rotate in XZ plane
            point.x = x * twist_angle.cos() - z * twist_angle.sin();
            point.z = x * twist_angle.sin() + z * twist_angle.cos();
        }
    }
}

pub fn apply_ruffle(control_points: &mut Vec<Vec<Vec3>>, freq: f32, amp: f32) {
    use std::f32::consts::PI;

    let rows = control_points.len();
    let cols = control_points[0].len();

    for row in control_points.iter_mut() {
        for (col_idx, point) in row.iter_mut().enumerate() {
            let u = col_idx as f32 / (cols - 1) as f32;

            // Only affect edges (u near 0 or 1)
            let edge_weight = if u < 0.5 {
                1.0 - u * 2.0  // Left edge
            } else {
                (u - 0.5) * 2.0  // Right edge
            };

            if edge_weight > 0.5 {
                let wave = (u * freq * PI * 2.0).sin();
                point.z += wave * amp * edge_weight;
            }
        }
    }
}
```

**Effort**: 3 hours

---

## Task 6.5: Complete B-Spline Petal Generator ✅

**Description**: Integrate B-spline surface evaluation with mesh generation and deformations.

**Status**: ✅ COMPLETED

**Acceptance Criteria**:
- [x] Enhanced `PetalParams` struct:
  ```rust
  pub struct PetalParams {
      pub length: f32,
      pub width: f32,
      pub tip_sharpness: f32,  // 0-1
      pub base_width: f32,
      pub curl: f32,        // -1 to 1
      pub twist: f32,       // degrees
      pub ruffle_freq: f32,
      pub ruffle_amp: f32,
      pub resolution: usize, // tessellation density
  }
  ```
- [x] Function `generate(params: &PetalParams) -> Mesh`
  - Generates control grid
  - Applies deformations to control points
  - Creates B-spline surface (with proper transpose)
  - Tessellates surface at specified resolution
  - Returns mesh with front and back faces (double-sided)
- [x] Proper UV coordinates
- [x] Smooth normals from B-spline surface
- [x] Unit tests generate valid mesh
- [x] All 25 petal tests passing

**Dependencies**: Tasks 6.2, 6.3, 6.4

**Technical Notes**:
```rust
pub fn generate(params: &PetalParams) -> Mesh {
    // 1. Generate control grid
    let mut control_points = generate_control_grid(params);

    // 2. Apply deformations
    apply_curl(&mut control_points, params.curl);
    apply_twist(&mut control_points, params.twist);
    apply_ruffle(&mut control_points, params.ruffle_freq, params.ruffle_amp);

    // 3. Create B-spline surface
    let surface = BSplineSurface {
        control_points,
        degree_u: 3,  // cubic
        degree_v: 3,
        knots_u: generate_knot_vector(5, 3, true),
        knots_v: generate_knot_vector(9, 3, true),
    };

    // 4. Tessellate surface
    let res = params.resolution;
    let mut mesh = Mesh::default();

    for i in 0..=res {
        let u = i as f32 / res as f32;
        for j in 0..=res {
            let v = j as f32 / res as f32;

            let pos = surface.evaluate(u, v);
            let normal = surface.normal(u, v);
            let uv = Vec2::new(u, v);

            mesh.add_vertex(pos, normal, uv);
        }
    }

    // Generate triangles
    for i in 0..res {
        for j in 0..res {
            let i0 = i * (res + 1) + j;
            let i1 = i0 + 1;
            let i2 = i0 + res + 1;
            let i3 = i2 + 1;

            mesh.add_triangle(i0 as u32, i2 as u32, i1 as u32);
            mesh.add_triangle(i1 as u32, i2 as u32, i3 as u32);
        }
    }

    // 5. Add back faces
    let back_mesh = mesh.clone();
    // Flip normals and winding order for back faces
    // ... (implementation details)

    mesh.merge(&back_mesh);
    mesh
}
```

**Effort**: 2.5 hours

---
