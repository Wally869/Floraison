# Epic 10: Inflorescence Foundation

**Phase**: 3 - Inflorescence System

**Goal**: Core data structures and axis generation for inflorescences.

**Estimated Effort**: 6-8 hours

---

## Task 10.1: Inflorescence Data Structures

**Description**: Define parameter structs for inflorescence patterns.

**Acceptance Criteria**:
- [ ] Module `floraison-inflorescence/src/mod.rs` created
- [ ] Enum `PatternType` with variants:
  ```rust
  pub enum PatternType {
      Raceme,
      Spike,
      Umbel,
      Corymb,
      Dichasium,
      Drepanium,
      CompoundRaceme,
      CompoundUmbel,
  }
  ```
- [ ] Base parameters struct:
  ```rust
  pub struct InflorescenceParams {
      pub pattern: PatternType,
      pub axis_length: f32,
      pub branch_count: usize,
      pub angle_top: f32,
      pub angle_bottom: f32,
      pub branch_length_top: f32,
      pub branch_length_bottom: f32,
      pub rotation_angle: f32,  // Fibonacci angle
      pub flower_size_top: f32,
      pub flower_size_bottom: f32,
  }
  ```
- [ ] Struct `BranchPoint` for branch data:
  ```rust
  pub struct BranchPoint {
      pub position: Vec3,
      pub direction: Vec3,
      pub length: f32,
      pub flower_scale: f32,
      pub age: f32,  // 0-1, for bud/bloom interpolation
  }
  ```
- [ ] Documentation

**Dependencies**: None (new module)

**Technical Notes**:
Each pattern will have specific rules for computing branch points.

**Effort**: 1.5 hours

---

## Task 10.2: 3D Axis Curve Generation

**Description**: Implement constant-curvature curve reconstruction from paper.

**Acceptance Criteria**:
- [ ] Module `floraison-core/src/math/curves.rs` created
- [ ] Function `reconstruct_3d_curve(points_2d: &[Vec2]) -> Vec<Vec3>`
  - Implements algorithm from paper (Section 5.2)
  - Assumes Y is vertical, X is horizontal input
  - Computes Z values for constant 3D curvature
- [ ] Helper function `sample_curve_uniform(points: &[Vec3], samples: usize) -> Vec<Vec3>`
  - Resamples curve at uniform arc-length intervals
- [ ] Unit tests verify:
  - Straight line input → straight line output
  - Sine wave input → spiral output
- [ ] Documentation with algorithm description

**Dependencies**: Task 2.1

**Technical Notes**:
From paper Section 5.2:
```
(d²x/dy²)² + (d²z/dy²)² = constant
```

Implementation:
```rust
pub fn reconstruct_3d_curve(points_2d: &[Vec2]) -> Vec<Vec3> {
    let n = points_2d.len();

    // Resample so points are evenly spaced in Y
    let points_2d = resample_uniform_y(points_2d, n);

    // Compute second derivatives of X
    let dx2: Vec<f32> = compute_second_derivatives_x(&points_2d);

    // Find constant (max curvature)
    let max_dx2 = dx2.iter().map(|&v| v.abs()).fold(0.0_f32, f32::max);
    let curvature_const = max_dx2;

    // Solve for |d²z/dy²|
    let mut dz2: Vec<f32> = dx2.iter()
        .map(|&dx2_val| {
            let val = curvature_const.powi(2) - dx2_val.powi(2);
            if val > 0.0 { val.sqrt() } else { 0.0 }
        })
        .collect();

    // Determine signs: flip when dx crosses zero
    determine_signs(&points_2d, &mut dz2);

    // Integrate twice to get Z values
    let z_values = integrate_twice(&dz2);

    // Construct 3D points
    points_2d.iter().zip(z_values.iter())
        .map(|(p2d, &z)| Vec3::new(p2d.x, p2d.y, z))
        .collect()
}
```

**Effort**: 4 hours

---

## Task 10.3: Axis Parameterization

**Description**: Create utility to parameterize axis curve for positioning branches.

**Acceptance Criteria**:
- [ ] Struct `AxisCurve` wraps curve points
- [ ] Method `sample_at_t(t: f32) -> AxisSample` returns:
  - `position: Vec3`
  - `tangent: Vec3`
  - `normal: Vec3` (perpendicular to tangent)
  - `binormal: Vec3` (for branch direction)
- [ ] Method `sample_uniform(count: usize) -> Vec<AxisSample>`
  - Samples curve at evenly spaced arc-length positions
- [ ] Frenet frame computation for proper orientation
- [ ] Unit tests

**Dependencies**: Task 10.2

**Technical Notes**:
```rust
pub struct AxisSample {
    pub position: Vec3,
    pub tangent: Vec3,
    pub normal: Vec3,
    pub binormal: Vec3,
}

pub struct AxisCurve {
    points: Vec<Vec3>,
    arc_lengths: Vec<f32>,  // Cumulative arc length at each point
    total_length: f32,
}

impl AxisCurve {
    pub fn new(points: Vec<Vec3>) -> Self {
        let arc_lengths = compute_arc_lengths(&points);
        let total_length = *arc_lengths.last().unwrap();
        Self { points, arc_lengths, total_length }
    }

    pub fn sample_at_t(&self, t: f32) -> AxisSample {
        // Interpolate position along curve
        let arc_length = t * self.total_length;
        let position = self.position_at_arc_length(arc_length);

        // Compute tangent (first derivative)
        let tangent = self.tangent_at_arc_length(arc_length);

        // Compute normal (second derivative or perpendicular)
        let normal = self.normal_at_arc_length(arc_length);

        // Binormal (cross product)
        let binormal = tangent.cross(normal).normalize();

        AxisSample { position, tangent, normal, binormal }
    }
}
```

**Effort**: 2.5 hours

---
