# Epic 7: Complete Floral Components

**Phase**: 2 - Single Flower Refinement

**Goal**: Add sepals and enhance existing components with more parameters.

**Estimated Effort**: 4-6 hours

---

## Task 7.1: Sepal Component (Reuse Petal)

**Description**: Create sepal generator that wraps petal generator with different defaults.

**Acceptance Criteria**:
- [ ] Type alias or wrapper in `floraison-components/src/sepal.rs`
- [ ] Function `generate(params: &PetalParams) -> Mesh` (reuses petal generator)
- [ ] Preset parameters for typical sepal shapes (narrower, less curly, green)
- [ ] Documentation explaining relationship to petals

**Dependencies**: Task 6.5

**Technical Notes**:
```rust
// Sepals are structurally similar to petals, just use the petal generator
pub use crate::petal::{PetalParams, generate};

pub fn default_sepal_params() -> PetalParams {
    PetalParams {
        length: 3.0,
        width: 1.0,
        tip_width: 0.5,
        base_width: 0.4,
        curl: -0.2,  // Slight downward curl
        twist: 0.0,
        ruffle_freq: 0.0,
        ruffle_amp: 0.0,
        resolution: 16,
    }
}
```

**Effort**: 0.5 hours

---

## Task 7.2: Enhanced Pistil with Style Curve

**Description**: Add curved style (central stalk) to pistil instead of straight.

**Acceptance Criteria**:
- [ ] Add field to `PistilParams`: `style_curve: Vec<Vec3>`
- [ ] If curve provided, sweep ovary along curve instead of straight up
- [ ] Default curve is straight (for backward compatibility)
- [ ] Curve specified as control points, interpolated with Catmull-Rom spline
- [ ] Unit test with curved pistil

**Dependencies**: Task 3.2

**Technical Notes**:
Add helper function for Catmull-Rom spline interpolation:
```rust
fn catmull_rom_spline(points: &[Vec3], t: f32) -> Vec3 {
    // Standard Catmull-Rom implementation
    // ...
}
```

Update `generate()` to use curve for sweep path instead of straight line.

**Effort**: 2 hours

---

## Task 7.3: Enhanced Stamen with Curved Filament

**Description**: Support curved filament path (like pistil style curve).

**Acceptance Criteria**:
- [ ] Add field to `StamenParams`: `filament_curve: Vec<Vec3>`
- [ ] If curve provided, sweep filament along curve
- [ ] Default curve is straight
- [ ] Unit test with curved stamen

**Dependencies**: Task 3.3, Task 7.2 (for curve utilities)

**Technical Notes**:
Reuse Catmull-Rom spline utilities from pistil implementation.

**Effort**: 1.5 hours

---

## Task 7.4: Component Color Parameters

**Description**: Add color/material parameters to all components.

**Acceptance Criteria**:
- [ ] Add color field to all parameter structs (RGB or named colors)
- [ ] Store color as vertex colors in mesh or as material tag
- [ ] Default colors:
  - Pistil: greenish-yellow
  - Stamen: yellow (anther), green (filament)
  - Petal: white/pink/purple (user-specified)
  - Sepal: green
- [ ] Colors passed through to WASM and applied in Three.js

**Dependencies**: Tasks 3.1-3.4, 7.1-7.3

**Technical Notes**:
Consider adding `color: Vec3` (RGB) to each params struct. Store as vertex attribute or as separate material index. For MVP, can use single color per component.

For Three.js side, create materials with specified colors during mesh creation.

**Effort**: 1.5 hours

---
