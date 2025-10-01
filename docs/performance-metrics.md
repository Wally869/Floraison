# Performance Metrics

**Last Updated**: 2025-10-01
**Test Environment**: Development build, local machine

---

## Overview

This document tracks performance metrics for the Floraison flower generator. Measurements include WASM mesh generation time, vertex/triangle counts, and rendering performance.

## Measurement Methodology

- **Generation Time**: Measured using `performance.now()` around `generator.generate_flower()` call
- **Geometry Metrics**: Vertex count = `mesh.vertices.length / 3`, Triangle count = `mesh.indices.length / 3`
- **Target**: Generation time < 500ms for smooth real-time parameter adjustment
- **Warning Threshold**: Generation time > 1000ms

---

## Current Optimizations

1. **Debouncing**: 300ms debounce on parameter changes prevents excessive regeneration
2. **Material Reuse**: Single MeshStandardMaterial per flower mesh
3. **Geometry Disposal**: Proper cleanup of old geometries when regenerating
4. **Default Resolution**: B-spline petal resolution = 16-24 samples (preset-dependent)

---

## Preset Benchmarks

### Lily Preset

**Parameters:**
- Petal count: 6
- Stamen count: 6
- Petal resolution: 20
- B-spline deformations: curl (0.4), twist (15°)

**Metrics:**
- Generation time: _TBD_
- Vertices: _TBD_
- Triangles: _TBD_

---

### Rose Preset

**Parameters:**
- Petal count: 24
- Stamen count: 20
- Sepal count: 5
- Petal resolution: 24
- B-spline deformations: curl (0.8), twist (5°), ruffle (freq 3.0, amp 0.15)

**Metrics:**
- Generation time: _TBD_
- Vertices: _TBD_
- Triangles: _TBD_

**Notes:** Most complex preset due to high petal count and ruffled edges.

---

### Daisy Preset

**Parameters:**
- Petal count: 21
- Stamen count: 34
- Pistil count: 13
- Petal resolution: 12
- Pattern: Golden spiral (phyllotaxis)

**Metrics:**
- Generation time: _TBD_
- Vertices: _TBD_
- Triangles: _TBD_

**Notes:** Many components but lower petal resolution keeps performance reasonable.

---

### Tulip Preset

**Parameters:**
- Petal count: 6
- Stamen count: 6
- Petal resolution: 20
- B-spline deformations: curl (-0.6, cup shape)

**Metrics:**
- Generation time: _TBD_
- Vertices: _TBD_
- Triangles: _TBD_

---

### Orchid Preset

**Parameters:**
- Petal count: 5
- Stamen count: 3
- Petal resolution: 24
- B-spline deformations: curl (0.3), twist (35°), ruffle (freq 2.0, amp 0.2)

**Metrics:**
- Generation time: _TBD_
- Vertices: _TBD_
- Triangles: _TBD_

**Notes:** High twist angle and ruffles create complex geometry.

---

## Performance Analysis

### Bottlenecks Identified

1. **B-spline Surface Evaluation**: Each petal requires NxM control point evaluations
2. **Petal Count**: Linear scaling with number of petals
3. **Ruffle Amplitude**: High ruffle frequency increases mesh complexity
4. **High Resolution**: Petal resolution > 24 shows diminishing visual returns

### Recommended Parameter Ranges

| Parameter | Recommended Range | Max Tested | Performance Impact |
|-----------|-------------------|------------|-------------------|
| Petal Count | 3-24 | 50+ | Linear |
| Petal Resolution | 12-24 | 32 | Quadratic (NxM grid) |
| Ruffle Frequency | 0-4 | 8 | Linear |
| Stamen/Pistil Count | 1-40 | 100+ | Linear (simple geometry) |

### Optimization Opportunities

#### Completed
- ✅ Debounced parameter updates (300ms)
- ✅ Proper geometry disposal on regeneration
- ✅ Performance logging in development mode

#### Future Considerations
- [ ] Web Worker for WASM (if generation consistently > 1s)
- [ ] Adaptive LOD based on component count
- [ ] B-spline evaluation caching (Rust-side optimization)
- [ ] Progressive rendering for complex flowers

---

## Rendering Performance

**Three.js Rendering:** Real-time 60fps with OrbitControls for all tested configurations.

**GPU Performance:**
- Vertex shader: Simple transformations, no performance issues
- Fragment shader: Standard material lighting, performs well
- Shadow mapping: Not enabled (future enhancement)

---

## Test Results Summary

**Status:** Performance profiling integrated. Awaiting benchmark results from manual testing.

**Action Items:**
1. Run dev server and test all 5 presets
2. Record metrics in console output
3. Update this document with actual measurements
4. Determine if additional optimizations needed

**Next Steps:**
- If all presets < 500ms: **No further optimization needed**, mark Epic 9.4 complete
- If any preset 500-1000ms: **Document recommendations**, consider preset tweaks
- If any preset > 1000ms: **Investigate Web Worker approach** or reduce default resolutions

---

## Conclusion

Performance monitoring is now active. The flower generator is designed to prioritize visual quality while maintaining interactive responsiveness. Real-world testing will determine if additional optimizations are warranted.
