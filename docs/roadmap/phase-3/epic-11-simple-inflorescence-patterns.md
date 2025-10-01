# Epic 11: Simple Inflorescence Patterns

**Phase**: 3 - Inflorescence System

**Goal**: Implement 4 simple indeterminate inflorescence patterns.

**Estimated Effort**: 10-12 hours

---

## Task 11.1: Raceme Pattern Generator

**Description**: Implement raceme pattern (single axis, flowers on pedicels).

**Acceptance Criteria**:
- [x] Module `floraison-inflorescence/src/patterns/raceme.rs` created
- [x] Function `generate_branch_points(params: &InflorescenceParams, axis: &AxisCurve) -> Vec<BranchPoint>`
- [x] Flowers spaced evenly along axis
- [x] Pedicel length interpolates from top to bottom
- [x] Down angle interpolates from top to bottom
- [x] Rotation angle applies Fibonacci spiral
- [x] Age increases bottom to top (indeterminate)
- [x] Unit test verifies branch count and positioning

**Dependencies**: Task 10.1, 10.3

**Technical Notes**:
```rust
pub fn generate_branch_points(
    params: &InflorescenceParams,
    axis: &AxisCurve,
) -> Vec<BranchPoint> {
    let mut branches = Vec::new();

    for i in 0..params.branch_count {
        let t = i as f32 / (params.branch_count - 1).max(1) as f32;

        // Sample axis
        let sample = axis.sample_at_t(t);

        // Interpolate parameters
        let angle = lerp(params.angle_bottom, params.angle_top, t);
        let length = lerp(params.branch_length_bottom, params.branch_length_top, t);
        let flower_scale = lerp(params.flower_size_bottom, params.flower_size_top, t);

        // Compute rotation around axis (phyllotaxis)
        let rotation = params.rotation_angle * i as f32;

        // Compute branch direction
        let down_rotation = Quat::from_axis_angle(sample.binormal, -angle.to_radians());
        let spiral_rotation = Quat::from_axis_angle(sample.tangent, rotation.to_radians());
        let direction = spiral_rotation * down_rotation * sample.normal;

        // Branch endpoint
        let position = sample.position + direction * length;

        // Age: indeterminate (bottom = oldest)
        let age = 1.0 - t;

        branches.push(BranchPoint {
            position,
            direction,
            length,
            flower_scale,
            age,
        });
    }

    branches
}
```

**Effort**: 3 hours

---

## Task 11.2: Spike Pattern Generator

**Description**: Implement spike pattern (like raceme but flowers sessile - no pedicels).

**Acceptance Criteria**:
- [x] Module `floraison-inflorescence/src/patterns/spike.rs` created
- [x] Function `generate_branch_points(params: &InflorescenceParams, axis: &AxisCurve) -> Vec<BranchPoint>`
- [x] Flowers placed directly on axis (length = 0)
- [x] Rotation angle still applies
- [x] Flower scale interpolates
- [x] Age indeterminate
- [x] Unit test

**Dependencies**: Task 10.1, 10.3

**Technical Notes**:
Very similar to raceme, but branch length is always 0 (or very small). Flowers attach directly to axis.

**Effort**: 1.5 hours

---

## Task 11.3: Umbel Pattern Generator

**Description**: Implement umbel pattern (all pedicels from single point, umbrella-like).

**Acceptance Criteria**:
- [x] Module `floraison-inflorescence/src/patterns/umbel.rs` created
- [x] Function `generate_branch_points(params: &InflorescenceParams, axis: &AxisCurve) -> Vec<BranchPoint>`
- [x] All branches originate from top of axis
- [x] Spread out in umbrella shape
- [x] Rotation angle determines angular spacing
- [x] Down angle determines spread
- [x] All flowers same age (determinate)
- [x] Unit test

**Dependencies**: Task 10.1, 10.3

**Technical Notes**:
```rust
pub fn generate_branch_points(
    params: &InflorescenceParams,
    axis: &AxisCurve,
) -> Vec<BranchPoint> {
    let mut branches = Vec::new();

    // All branches from top of axis
    let sample = axis.sample_at_t(1.0);

    for i in 0..params.branch_count {
        let rotation = params.rotation_angle * i as f32;

        // Compute direction (down and rotated around axis)
        let down_rotation = Quat::from_axis_angle(sample.binormal, -params.angle_top.to_radians());
        let spiral_rotation = Quat::from_axis_angle(sample.tangent, rotation.to_radians());
        let direction = spiral_rotation * down_rotation * sample.normal;

        let position = sample.position + direction * params.branch_length_top;

        branches.push(BranchPoint {
            position,
            direction,
            length: params.branch_length_top,
            flower_scale: params.flower_size_top,
            age: 1.0,  // All same age
        });
    }

    branches
}
```

**Effort**: 2 hours

---

## Task 11.4: Corymb Pattern Generator

**Description**: Implement corymb pattern (pedicels of varying length, flat-topped).

**Acceptance Criteria**:
- [x] Module `floraison-inflorescence/src/patterns/corymb.rs` created
- [x] Function `generate_branch_points(params: &InflorescenceParams, axis: &AxisCurve) -> Vec<BranchPoint>`
- [x] Branches along axis like raceme
- [x] Pedicel length adjusted so all flowers reach same height
- [x] Age indeterminate
- [x] Unit test

**Dependencies**: Task 10.1, 10.3

**Technical Notes**:
Similar to raceme, but calculate branch lengths such that all branch endpoints have the same Y coordinate.

```rust
// For each branch at height h along axis:
// Want: position.y + direction.y * length = target_height
// Solve for length
let target_height = axis.sample_at_t(1.0).position.y;
let length = (target_height - sample.position.y) / direction.y.max(0.01);
```

**Effort**: 2.5 hours

---

## Task 11.5: Inflorescence Assembly Function

**Description**: Combine axis, branches, and flowers into complete inflorescence mesh.

**Acceptance Criteria**:
- [x] Module `floraison-inflorescence/src/assembly.rs` created
- [x] Function `assemble_inflorescence(params: &InflorescenceParams, flower_mesh: &Mesh) -> Mesh`
  - Generates axis curve from parameters
  - Calls pattern-specific generator to get branch points
  - Instantiates flower at each branch point
  - Generates stem geometry (cylinder along axis and branches)
  - Merges all into single mesh
- [x] Proper transformations applied
- [x] Unit test

**Dependencies**: Tasks 11.1-11.4

**Technical Notes**:
```rust
pub fn assemble_inflorescence(
    params: &InflorescenceParams,
    flower_mesh: &Mesh,
) -> Mesh {
    let mut final_mesh = Mesh::default();

    // 1. Generate axis curve (for now, simple straight line)
    let axis_points = vec![
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, params.axis_length, 0.0),
    ];
    let axis = AxisCurve::new(axis_points);

    // 2. Generate branch points based on pattern
    let branches = match params.pattern {
        PatternType::Raceme => raceme::generate_branch_points(params, &axis),
        PatternType::Spike => spike::generate_branch_points(params, &axis),
        PatternType::Umbel => umbel::generate_branch_points(params, &axis),
        PatternType::Corymb => corymb::generate_branch_points(params, &axis),
        _ => todo!("Other patterns not implemented yet"),
    };

    // 3. Generate main stem mesh
    let stem_mesh = generate_stem_along_axis(&axis, 0.1);
    final_mesh.merge(&stem_mesh);

    // 4. For each branch, add pedicel and flower
    for branch in &branches {
        // Pedicel (thin stem from axis to flower)
        if branch.length > 0.01 {
            let pedicel = generate_pedicel(branch);
            final_mesh.merge(&pedicel);
        }

        // Flower
        let mut flower = flower_mesh.clone();
        flower.transform(Mat4::from_scale_rotation_translation(
            Vec3::splat(branch.flower_scale),
            Quat::from_rotation_arc(Vec3::Y, branch.direction),
            branch.position,
        ));
        final_mesh.merge(&flower);
    }

    final_mesh
}
```

**Effort**: 3 hours

---
