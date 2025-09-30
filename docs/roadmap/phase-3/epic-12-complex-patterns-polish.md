# Epic 12: Complex Patterns & Polish

**Phase**: 3 - Inflorescence System

**Goal**: Implement determinate and compound patterns, add flower aging, polish visuals.

**Estimated Effort**: 10-12 hours

---

## Task 12.1: Dichasium Pattern Generator

**Description**: Implement dichasium (branching pattern with two branches per node).

**Acceptance Criteria**:
- [ ] Module `floraison-inflorescence/src/patterns/dichasium.rs` created
- [ ] Recursive branching structure
- [ ] Parameters for branching ratio (child branch length/angle relative to parent)
- [ ] Age determinate (top flowers oldest)
- [ ] Unit test

**Dependencies**: Task 10.1, 10.3

**Technical Notes**:
This is more complex as it's hierarchical. Need to define recursion depth parameter.

```rust
pub struct DichasiumParams {
    pub depth: usize,  // Number of branching levels
    pub branch_ratio: f32,  // Each child is this ratio of parent length
    pub angle_ratio: f32,
}

pub fn generate_recursive(
    origin: Vec3,
    direction: Vec3,
    length: f32,
    depth: usize,
    params: &DichasiumParams,
) -> Vec<BranchPoint> {
    // Base case
    if depth == 0 {
        return vec![BranchPoint {
            position: origin + direction * length,
            direction,
            length,
            flower_scale: 1.0,
            age: 1.0,
        }];
    }

    // Recursive case: create two child branches
    let left_angle = params.angle_ratio * 30.0_f32.to_radians();
    let right_angle = -left_angle;

    // ... compute left and right branch directions
    // ... recurse
}
```

**Effort**: 4 hours

---

## Task 12.2: Drepanium Pattern Generator

**Description**: Implement drepanium (branching with single curved branch per node).

**Acceptance Criteria**:
- [ ] Module `floraison-inflorescence/src/patterns/drepanium.rs` created
- [ ] Similar to dichasium but only one child per node
- [ ] Creates spiral/helix shape
- [ ] Age determinate
- [ ] Unit test

**Dependencies**: Task 10.1, 10.3

**Technical Notes**:
Similar structure to dichasium but simpler (only one branch per level).

**Effort**: 3 hours

---

## Task 12.3: Compound Pattern Support

**Description**: Implement compound raceme and umbel (recursive patterns).

**Acceptance Criteria**:
- [ ] Modules `compound_raceme.rs` and `compound_umbel.rs` created
- [ ] Replace each flower in simple pattern with sub-inflorescence
- [ ] Recursion depth parameter (typically 2)
- [ ] Each sub-inflorescence is scaled smaller
- [ ] Unit tests

**Dependencies**: Tasks 11.1, 11.3

**Technical Notes**:
```rust
// Compound raceme: Each branch point becomes a mini raceme
pub fn generate_compound_raceme(params: &InflorescenceParams) -> Mesh {
    let main_axis = /* ... */;
    let main_branches = raceme::generate_branch_points(params, &main_axis);

    let mut final_mesh = Mesh::default();

    for branch in main_branches {
        // Create sub-raceme at this branch point
        let sub_params = params.scaled(0.5);  // Half size
        let sub_mesh = assemble_inflorescence(&sub_params, flower_mesh);

        // Transform and merge
        // ...
    }

    final_mesh
}
```

**Effort**: 3 hours

---

## Task 12.4: Flower Aging System

**Description**: Support multiple flower models representing different developmental stages.

**Acceptance Criteria**:
- [ ] Parameter struct `FlowerAging`:
  ```rust
  pub struct FlowerAging {
      pub bud_mesh: Mesh,
      pub bloom_mesh: Mesh,
      pub wilt_mesh: Option<Mesh>,
  }
  ```
- [ ] Function `interpolate_flower(age: f32, aging: &FlowerAging) -> Mesh`
  - age = 0.0: bud
  - age = 1.0: bloom
  - Intermediate: transition (morph or discrete switch)
- [ ] Update assembly to use age-appropriate flower
- [ ] Unit test

**Dependencies**: Task 11.5

**Technical Notes**:
For MVP, use discrete switch at age thresholds:
- age < 0.3: bud
- age >= 0.3: bloom

For advanced version, implement mesh morphing (vertex interpolation) if meshes have same topology.

**Effort**: 2 hours

---

## Epic 12 Completion & Phase 3 Checkpoint

**Deliverable**: Complete inflorescence system with 8 patterns, flower aging, and full integration.

**Testing**:
- [ ] All 8 patterns generate correctly
- [ ] Compound patterns show hierarchical structure
- [ ] Flower aging visible in indeterminate patterns
- [ ] Stem geometry looks reasonable
- [ ] Parameters in UI control inflorescence appearance
