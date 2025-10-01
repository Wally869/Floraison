# Known Issues

This document tracks known issues and limitations that are not immediately blocking but should be addressed in future iterations.

---

## Petal/Sepal Base Separation

**Status:** Known visual artifact
**Severity:** Low (aesthetic)
**Issue:** Visible gap between petal/sepal bases and the receptacle surface.

### Description

When petals and sepals are rendered, there may be a small visual gap between their bases and the receptacle surface. This is particularly noticeable when components have different height values (e.g., sepals at 70% height, petals at 75% height).

### Root Cause

The gap occurs due to how components are positioned on the receptacle surface:

1. Components are positioned at specified heights on the receptacle's Bézier curve
2. The base of each component mesh starts at its local origin (y=0)
3. When components are tilted or positioned at different heights, the local y=0 may not perfectly align with the receptacle surface

### Current Workaround

Heights have been adjusted to minimize visible gaps:
- Petals: 75% height (down from 80%)
- Sepals: 70% height (up from 40%)
- Stamens: 85% height (up from 60%)

### Potential Solutions

1. **Component base anchoring**: Modify component generators to have negative y-values at base, allowing them to "sink into" the receptacle slightly
2. **Receptacle attachment points**: Add explicit attachment geometry that blends component bases into receptacle surface
3. **Procedural blending**: Generate transitional geometry at component-receptacle intersections

**Fix Priority:** Low (requires careful architectural changes to component generators)

---

## Tip Sharpness Parameter Inverted (Epic 6 Bug)

**Status:** Confirmed bug in Rust implementation
**Severity:** Medium (confusing UX)
**File:** `floraison-components/src/petal.rs:197`

### Description

The `tip_sharpness` parameter behaves opposite to its documentation:
- `tip_sharpness = 0.0` produces **sharp/pointed** tips (should be rounded)
- `tip_sharpness = 1.0` produces **blunt/flat** tips (should be pointed)

### Root Cause

The interpolation formula in `generate_control_grid()` is inverted:

**Current formula (line 197):**
```rust
let width_at_v = if v < 0.6 {
    let t = v / 0.6;
    params.base_width + (params.width - params.base_width) * t
} else {
    let t = (v - 0.6) / 0.4;
    // This line is inverted:
    params.width + (params.width * params.tip_sharpness - params.width) * t
};
```

**Correct formula should be:**
```rust
params.width * (1.0 - (1.0 - params.tip_sharpness) * t)
```

### Current Workaround

Users should treat the slider as inverted:
- Set slider to **low values (0.0-0.3)** for sharp tips
- Set slider to **high values (0.7-1.0)** for rounded tips

### Impact

- UI presets work correctly despite inversion (empirically tuned)
- Documentation is misleading
- User expectations don't match behavior

**Fix Priority:** Medium (requires careful testing across all petal presets to ensure visual consistency)

---

## Vertex Colors Not Rendered

**Status:** Deferred to future epic
**Severity:** Low (feature incomplete)
**Issue:** Colors set in UI parameters are not visible in 3D viewer.

### Description

While all components support color parameters (RGB values), these colors are not currently rendered in the Three.js viewer. The viewer shows a uniform golden color for all components.

### Root Cause

Three missing pieces:

1. **WASM export**: `MeshData` struct in `floraison-wasm/src/lib.rs` doesn't export `colors` field
   ```rust
   pub struct MeshData {
       positions: Vec<f32>,
       normals: Vec<f32>,
       uvs: Vec<f32>,
       colors: Vec<f32>,  // EXISTS in Mesh but NOT exported here
       indices: Vec<u32>,
   }
   ```

2. **Geometry attribute**: `mesh-converter.ts` doesn't add color attribute to `BufferGeometry`
   ```typescript
   geometry.setAttribute('color', new THREE.BufferAttribute(colors, 3));
   ```

3. **Material mode**: `ThreeViewer.svelte` uses hardcoded material color instead of vertex colors
   ```typescript
   const material = new THREE.MeshStandardMaterial({
       color: 0xffcc00,  // Hardcoded golden
       vertexColors: false  // Should be true
   });
   ```

### Current Behavior

- All components render with uniform golden color (`0xffcc00`)
- UI color pickers update stores but have no visual effect
- Color parameters are preserved in JSON export/import

### Future Implementation

Will be implemented in a future UI polish epic. Requires:
1. Export colors array from WASM (1 line change)
2. Add color attribute in converter (1 line change)
3. Enable vertex colors in material (2 line change)

**Fix Priority:** Low (not blocking core functionality, deferred to future epic)

---

## Curved Component UI Not Exposed

**Status:** Intentional scope limitation for Epic 8
**Severity:** Low (advanced feature)
**Issue:** Epic 7's curved pistil/stamen features not accessible in UI.

### Description

Epic 7 implemented curved components using Catmull-Rom splines:
- `PistilParams.style_curve: Option<Vec<Vec3>>`
- `StamenParams.filament_curve: Option<Vec<Vec3>>`

These fields are not exposed in the Epic 8 parameter UI.

### Reason

Requires complex curve editor UI:
- Multiple 3D control point inputs
- Curve visualization
- Interactive manipulation
- Potentially confusing for non-technical users

### Current Behavior

- Curved components can be generated via direct JSON parameter input
- Preset flowers use straight components (`None` for curve fields)
- Future UI epic will add curve editor interface

### Workaround

Advanced users can:
1. Export flower parameters to JSON
2. Manually edit curve control points
3. Import modified JSON

**Fix Priority:** Low (deferred to "Advanced Controls" epic)

---

## Fixed Issues

### Stamen Orientation (Fixed 2025-10-01)

**Status:** FIXED in Epic 8
**Issue:** Stamens were following receptacle surface normal instead of being tiltable from upright to spreading.

**Root Cause:** Orientation logic started with surface normal and applied tilt, rather than starting upright.

**Fix:** Rewrote stamen/pistil orientation in `assembly.rs`:
- Start with upright orientation (pointing along +Y axis)
- Rotate around azimuthal tangent by `tilt_angle`
- 0° = upright (parallel to pistil)
- 90° = spreading horizontally outward

**File:** `floraison-components/src/assembly.rs:231-270`

---

### Pistil Orientation (Fixed 2025-10-01)

**Status:** FIXED in Epic 8
**Issue:** Central pistil oriented perpendicular to receptacle surface instead of pointing straight up.

**Fix:** Added special case for pistils at center (radius ≈ 0) to use identity rotation.

**File:** `floraison-components/src/assembly.rs:212-220`

---

## Summary

| Issue | Severity | Status | Fix Timeline |
|-------|----------|--------|--------------|
| Petal/sepal base gap | Low | Known artifact | Future architectural revision |
| Tip sharpness inverted | Medium | Confirmed bug | Review in petal refinement epic |
| Colors not rendered | Low | Deferred feature | Future UI polish epic |
| Curved UI not exposed | Low | Scope limitation | Future advanced controls epic |
| ~~Stamen orientation~~ | ~~Medium~~ | **FIXED** | ~~Epic 8~~ |
| ~~Pistil orientation~~ | ~~Medium~~ | **FIXED** | ~~Epic 8~~ |

---

**Last Updated:** 2025-10-01
**Reviewer:** See individual issue sections for details
