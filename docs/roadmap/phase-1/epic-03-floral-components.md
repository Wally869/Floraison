# Epic 3: Basic Floral Components

**Phase**: 1 - Foundation

**Goal**: Implement simplified geometry generators for receptacle, pistil, stamen, and basic petal.

**Estimated Effort**: 8-10 hours

---

## Task 3.1: Receptacle Component

**Description**: Create parametric receptacle generator using surface of revolution.

**Acceptance Criteria**:
- [ ] Module `floraison-components/src/receptacle.rs` created
- [ ] Parameter struct defined:
  ```rust
  pub struct ReceptacleParams {
      pub height: f32,
      pub base_radius: f32,
      pub bulge_height: f32,  // 0-1, where bulge occurs
      pub bulge_radius: f32,
      pub top_radius: f32,
      pub segments: usize,    // angular resolution
  }
  ```
- [ ] Function `generate(params: &ReceptacleParams) -> Mesh`
- [ ] Uses Bézier curve for smooth profile
- [ ] Default parameters create reasonable shape
- [ ] Unit test verifies mesh validity (no NaN, proper indices)
- [ ] Visual validation helper (output to OBJ for manual check)

**Dependencies**: Tasks 2.2, 2.4, 2.5

**Technical Notes**:
Create profile using 4-point Bézier:
- P0 = (base_radius, 0)
- P1 = (base_radius, height * 0.2)
- P2 = (bulge_radius, height * bulge_height)
- P3 = (top_radius, height)

Sample this curve to get profile points, then call `surface_of_revolution`.

**Effort**: 2 hours

---

## Task 3.2: Pistil Component

**Description**: Create simple pistil as tapered cylinder with spherical stigma.

**Acceptance Criteria**:
- [ ] Module `floraison-components/src/pistil.rs` created
- [ ] Parameter struct:
  ```rust
  pub struct PistilParams {
      pub length: f32,
      pub base_radius: f32,
      pub tip_radius: f32,
      pub stigma_radius: f32,
      pub segments: usize,
  }
  ```
- [ ] Function `generate(params: &PistilParams) -> Mesh`
- [ ] Style (main body) as straight tapered cylinder
- [ ] Stigma as UV sphere at top
- [ ] Proper mesh merging between parts
- [ ] Default parameters
- [ ] Unit test

**Dependencies**: Tasks 2.2, 2.4

**Technical Notes**:
- Style: Surface of revolution with linear taper profile
- Stigma: Standard UV sphere generation
- Merge meshes and weld vertices at connection point

Consider helper function for sphere generation:
```rust
fn generate_sphere(radius: f32, segments: usize, rings: usize) -> Mesh
```

**Effort**: 2.5 hours

---

## Task 3.3: Stamen Component ✅

**Description**: Create stamen as thin filament with ellipsoid anther.

**Status**: ✅ COMPLETED

**Acceptance Criteria**:
- [x] Module `floraison-components/src/stamen.rs` created
- [x] Parameter struct:
  ```rust
  pub struct StamenParams {
      pub filament_length: f32,
      pub filament_radius: f32,
      pub anther_length: f32,
      pub anther_width: f32,
      pub anther_height: f32,
      pub segments: usize,
  }
  ```
- [x] Function `generate(params: &StamenParams) -> Mesh`
- [x] Filament as thin cylinder
- [x] Anther as stretched sphere (ellipsoid)
- [x] Anther positioned at top of filament
- [x] Default parameters and 3 presets (short, slender, elongated_anther)
- [x] 9 unit tests + 2 doc-tests

**Dependencies**: Tasks 2.2, 2.4

**Technical Notes**:
Ellipsoid: Generate UV sphere, then scale non-uniformly:
```rust
let sphere = generate_sphere(1.0, segments, rings);
sphere.transform(Mat4::from_scale(Vec3::new(
    anther_radius,
    anther_length * 0.5,
    anther_radius
)));
```

**Effort**: 2 hours

---

## Task 3.4: Simple Petal Component (Flat Mesh) ✅

**Description**: Create basic petal as flat textured quad with elliptical outline.

**Status**: ✅ COMPLETED

**Acceptance Criteria**:
- [x] Module `floraison-components/src/petal.rs` created
- [x] Parameter struct:
  ```rust
  pub struct PetalParams {
      pub length: f32,
      pub width: f32,
      pub tip_sharpness: f32,
      pub base_width: f32,
      pub outline_samples: usize,
  }
  ```
- [x] Function `generate(params: &PetalParams) -> Mesh`
- [x] Generates flat mesh in XY plane (Z = 0)
- [x] Smooth Bézier curve outline (4 cubic curves)
- [x] Proper UV mapping (0-1 range)
- [x] Fan triangulation from center point
- [x] Default parameters and 3 presets (wide, narrow, short)
- [x] 10 unit tests + 2 doc-tests

**Dependencies**: Task 2.2

**Technical Notes**:
Create grid of points with elliptical boundary check:
```rust
for row in 0..resolution {
    let v = row as f32 / (resolution - 1) as f32;
    let width_at_v = interpolate_width(v, base_width, width, tip_width);

    for col in 0..resolution {
        let u = col as f32 / (resolution - 1) as f32;
        let x = (u - 0.5) * width_at_v;
        let y = v * length;

        // Add if within elliptical boundary
        if within_boundary(x, y) {
            mesh.add_vertex(Vec3::new(x, y, 0.0), Vec3::Z, Vec2::new(u, v));
        }
    }
}
```

**Effort**: 2.5 hours

---
