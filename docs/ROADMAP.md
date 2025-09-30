# Implementation Roadmap

**Status**: 🚀 In Progress - Phase 1
**Last Updated**: 2025-09-30

## Progress Summary

- ✅ **Task 1.1**: Rust Workspace initialized with 3 crates (core, components, wasm)
- ✅ **Task 1.2**: WASM Build System configured with wasm-pack, build scripts, and tests
- ✅ **Task 1.3**: SvelteKit Frontend initialized with TailwindCSS v4, TypeScript, testing setup
- ✅ **Task 1.4**: Development Workflow with unified commands and documentation
- 🔄 **Current**: Ready to begin Epic 2 (Core Math Library)

## Overview

This roadmap breaks down the Floraison project into manageable epics and tasks following agile principles. Each task is scoped to be completable in 2-8 hours, with clear acceptance criteria and dependencies.

### Timeline Estimates

- **Phase 1**: Foundation & Single Flower MVP (1 week)
- **Phase 2**: Complete Flower System (1 week)
- **Phase 3**: Inflorescence System (1 week)
- **Phase 4**: Polish & Launch (3-4 days)

**Total**: ~3.5 weeks

---

## Phase 1: Foundation & Single Flower MVP

**Goal**: Create a working flower generator that produces a simple lily-like flower with basic geometry, rendered in the browser.

### Epic 1: Project Setup & Infrastructure

**Goal**: Initialize all necessary project structure, tooling, and build configuration.

**Estimated Effort**: 4-6 hours

#### Task 1.1: Initialize Rust Workspace ✅

**Description**: Set up Cargo workspace with multiple crates for separation of concerns.

**Status**: ✅ COMPLETED

**Acceptance Criteria**:
- [x] Workspace created with `Cargo.toml` at root
- [x] Crate structure created:
  - `floraison-core` (library)
  - `floraison-components` (library)
  - `floraison-wasm` (cdylib + rlib)
- [x] All crates compile successfully
- [x] Basic `lib.rs` files with placeholder modules
- [x] Workspace dependencies configured (glam, serde, wasm-bindgen, etc.)
- [x] Release profile optimized for WASM size

**Dependencies**: None

**Technical Notes**:
- Use workspace dependencies for shared crates (glam, serde)
- Set up workspace-level profile configurations

**Effort**: 1 hour

---

#### Task 1.2: Configure WASM Build System ✅

**Description**: Set up wasm-pack configuration and build scripts.

**Status**: ✅ COMPLETED

**Acceptance Criteria**:
- [x] `wasm-pack` installed (v0.13.1)
- [x] `Cargo.toml` for `floraison-wasm` configured with:
  - `crate-type = ["cdylib", "rlib"]`
  - `wasm-bindgen` dependency
  - Proper feature flags
- [x] Build scripts created:
  - `build-wasm.sh` (Bash)
  - `build-wasm.ps1` (PowerShell)
  - npm scripts: `wasm:build`, `wasm:dev`
- [x] Generated WASM outputs to `floraison-ui/src/lib/wasm/`
- [x] WASM loader utility created (`loader.ts`)
- [x] Vite config updated for WASM support
- [x] Import tests pass (3/3)
- [x] Documentation created (README.md, TESTING.md)

**Dependencies**: Task 1.1

**Technical Notes**:
```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"
serde = { version = "1.0", features = ["derive"] }
serde-wasm-bindgen = "0.6"
```

Build command: `wasm-pack build --target web --out-dir ../frontend/src/lib/wasm`

**Effort**: 2 hours

---

#### Task 1.3: Initialize SvelteKit Frontend ✅

**Description**: Create SvelteKit project with TypeScript and TailwindCSS.

**Status**: ✅ COMPLETED

**Acceptance Criteria**:
- [x] SvelteKit project initialized in `floraison-ui/` directory
- [x] TypeScript configured
- [x] TailwindCSS v4 installed and configured
- [x] Dev server runs successfully (`npm run dev`)
- [x] Testing setup (Vitest + Playwright)
- [x] Vite config updated to handle WASM imports (completed in Task 1.2)

**Dependencies**: None

**Technical Notes**:
```bash
npm create svelte@latest frontend
cd frontend
npm install -D tailwindcss postcss autoprefixer
npx tailwindcss init -p
npm install three @types/three
```

Vite config for WASM:
```js
export default defineConfig({
  server: {
    fs: {
      allow: ['..']
    }
  }
});
```

**Effort**: 1.5 hours

---

#### Task 1.4: Set Up Development Workflow ✅

**Description**: Configure hot reload, build scripts, and development conveniences.

**Status**: ✅ COMPLETED

**Acceptance Criteria**:
- [x] Root-level `package.json` with unified commands
- [x] `npm run dev`: Watches Rust (cargo-watch), rebuilds WASM, runs SvelteKit dev server (concurrently)
- [x] `npm run build`: Release build for both Rust and frontend
- [x] `.gitignore` configured properly for both Rust and Node
- [x] README instructions for running locally
- [x] Additional commands: test, check, format, lint, clean
- [x] VSCode settings and extensions recommended
- [x] GitHub Actions CI workflow

**Dependencies**: Tasks 1.1, 1.2, 1.3

**Technical Notes**:
Consider using `cargo-watch` for Rust file monitoring:
```bash
cargo watch -i frontend/ -s "wasm-pack build ..."
```

Or create a simple bash/PowerShell script for Windows compatibility.

**Effort**: 1.5 hours

---

### Epic 2: Core Math Library

**Goal**: Implement fundamental mathematical primitives for geometry generation.

**Estimated Effort**: 6-8 hours

#### Task 2.1: Vector Math Wrapper

**Description**: Set up 3D vector math using `glam` crate with convenience wrappers.

**Acceptance Criteria**:
- [ ] `glam` crate added to `floraison-core`
- [ ] Re-export types: `Vec3`, `Vec2`, `Mat3`, `Mat4`, `Quat`
- [ ] Add custom helper functions:
  - `Vec3::cylindrical(radius, angle, height) -> Vec3`
  - `Vec3::spherical(radius, theta, phi) -> Vec3`
- [ ] Unit tests for all helper functions
- [ ] Documentation comments on all public functions

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

#### Task 2.2: Mesh Data Structures

**Description**: Define core mesh representation with vertices, indices, normals, UVs.

**Acceptance Criteria**:
- [ ] `Mesh` struct created with fields:
  - `positions: Vec<Vec3>`
  - `normals: Vec<Vec3>`
  - `uvs: Vec<Vec2>`
  - `indices: Vec<u32>`
- [ ] Methods implemented:
  - `new() -> Mesh`
  - `add_vertex(pos, normal, uv) -> usize` (returns index)
  - `add_triangle(i0, i1, i2)`
  - `merge(&mut self, other: &Mesh)` (combines two meshes)
  - `compute_normals(&mut self)` (auto-generate from faces)
  - `transform(&mut self, matrix: Mat4)` (apply transformation)
- [ ] Unit tests for all methods
- [ ] Serialization support (for debugging)

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

#### Task 2.3: Phyllotaxis Functions

**Description**: Implement Fibonacci spiral arrangement calculations.

**Acceptance Criteria**:
- [ ] Module `floraison-core/src/math/phyllotaxis.rs` created
- [ ] Function `golden_angle() -> f32` returns 137.5078° in radians
- [ ] Function `fibonacci_angle(n: usize) -> f32` calculates angle for Fibonacci sequence
- [ ] Function `vogel_spiral(index: usize, count: usize, radius: f32) -> Vec2` for disc packing
- [ ] Precomputed constants for common angles (120°, 144°, 180°)
- [ ] Unit tests verify angle calculations
- [ ] Documentation with botanical context

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

#### Task 2.4: Surface of Revolution Generator

**Description**: Create function to generate mesh from 2D profile curve rotated around axis.

**Acceptance Criteria**:
- [ ] Function `surface_of_revolution(profile: &[Vec2], segments: usize) -> Mesh`
- [ ] Profile points represent (radius, height) pairs
- [ ] Generates proper triangulation with smooth normals
- [ ] Handles degenerate cases (radius = 0 at ends)
- [ ] Proper UV mapping (u = angle, v = height)
- [ ] Unit tests with simple shapes (cylinder, cone, sphere)
- [ ] Documentation with diagrams

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

#### Task 2.5: Bézier Curve Utilities

**Description**: Implement Bézier curve evaluation for smooth profiles.

**Acceptance Criteria**:
- [ ] Function `bezier_quadratic(p0, p1, p2: Vec2, t: f32) -> Vec2`
- [ ] Function `bezier_cubic(p0, p1, p2, p3: Vec2, t: f32) -> Vec2`
- [ ] Function `sample_bezier_cubic(p0, p1, p2, p3: Vec2, samples: usize) -> Vec<Vec2>`
- [ ] Tangent calculation functions for each type
- [ ] Unit tests verify known curve properties (endpoints, tangents)
- [ ] Documentation with visual examples

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

### Epic 3: Basic Floral Components

**Goal**: Implement simplified geometry generators for receptacle, pistil, stamen, and basic petal.

**Estimated Effort**: 8-10 hours

#### Task 3.1: Receptacle Component

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

#### Task 3.2: Pistil Component

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

#### Task 3.3: Stamen Component

**Description**: Create stamen as thin filament with ellipsoid anther.

**Acceptance Criteria**:
- [ ] Module `floraison-components/src/stamen.rs` created
- [ ] Parameter struct:
  ```rust
  pub struct StamenParams {
      pub filament_length: f32,
      pub filament_radius: f32,
      pub anther_length: f32,
      pub anther_radius: f32,
      pub segments: usize,
  }
  ```
- [ ] Function `generate(params: &StamenParams) -> Mesh`
- [ ] Filament as thin cylinder
- [ ] Anther as stretched sphere (ellipsoid)
- [ ] Anther positioned at top of filament
- [ ] Default parameters
- [ ] Unit test

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

#### Task 3.4: Simple Petal Component (Flat Mesh)

**Description**: Create basic petal as flat textured quad with elliptical outline.

**Acceptance Criteria**:
- [ ] Module `floraison-components/src/petal.rs` created
- [ ] Parameter struct:
  ```rust
  pub struct SimplePetalParams {
      pub length: f32,
      pub width: f32,
      pub tip_width: f32,    // for tapering
      pub base_width: f32,
      pub resolution: usize, // subdivisions
  }
  ```
- [ ] Function `generate_simple(params: &SimplePetalParams) -> Mesh`
- [ ] Generates flat mesh in XY plane (Z = 0)
- [ ] Smooth outline (not rectangular)
- [ ] Proper UV mapping (0-1 range)
- [ ] Back faces included (duplicate and flip normals)
- [ ] Default parameters create lily-like petal
- [ ] Unit test

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

### Epic 4: Single Flower Assembly

**Goal**: Assemble individual components into a complete flower using basic floral diagram.

**Estimated Effort**: 6-8 hours

#### Task 4.1: Floral Diagram Data Structure

**Description**: Define data structures for floral diagram (component counts and layout).

**Acceptance Criteria**:
- [ ] Module `floraison-diagram/src/layout.rs` created
- [ ] Enums for component types:
  ```rust
  #[derive(Debug, Clone, Copy)]
  pub enum ComponentType {
      Pistil,
      Stamen,
      Petal,
      Sepal,
  }
  ```
- [ ] Struct for component placement:
  ```rust
  pub struct ComponentPlacement {
      pub component_type: ComponentType,
      pub radius: f32,      // radial distance from center
      pub angle: f32,       // angular position (radians)
      pub scale: f32,       // size multiplier
      pub rotation: f32,    // rotation around attachment point
  }
  ```
- [ ] Struct for floral diagram:
  ```rust
  pub struct FloralDiagram {
      pub pistil_count: usize,
      pub stamen_count: usize,
      pub petal_count: usize,
      pub sepal_count: usize,
      pub radial_symmetry: bool,
  }
  ```
- [ ] Method `generate_placements(&self) -> Vec<ComponentPlacement>` computes 2D positions
- [ ] Unit tests verify symmetry and count

**Dependencies**: Task 2.3 (for phyllotaxis)

**Technical Notes**:
```rust
impl FloralDiagram {
    pub fn generate_placements(&self) -> Vec<ComponentPlacement> {
        let mut placements = Vec::new();

        // Pistils at center (radius = 0)
        for i in 0..self.pistil_count {
            placements.push(ComponentPlacement {
                component_type: ComponentType::Pistil,
                radius: 0.0,
                angle: if self.pistil_count > 1 {
                    2.0 * PI * i as f32 / self.pistil_count as f32
                } else { 0.0 },
                scale: 1.0,
                rotation: 0.0,
            });
        }

        // Stamens in ring
        for i in 0..self.stamen_count {
            placements.push(ComponentPlacement {
                component_type: ComponentType::Stamen,
                radius: 0.3, // relative to receptacle size
                angle: 2.0 * PI * i as f32 / self.stamen_count as f32,
                scale: 1.0,
                rotation: 0.0,
            });
        }

        // Petals in outer ring
        // ... similar

        placements
    }
}
```

**Effort**: 2 hours

---

#### Task 4.2: Component Instance Mapping

**Description**: Map 2D floral diagram positions to 3D positions on receptacle surface.

**Acceptance Criteria**:
- [ ] Module `floraison-diagram/src/mapping.rs` created
- [ ] Struct `ReceptacleMapper` that holds receptacle profile data
- [ ] Method `map_to_3d(placement: &ComponentPlacement, receptacle_height: f32) -> Transform3D`
  - Returns position and orientation (as Mat4 or separate pos/rotation)
- [ ] Radial position (2D) maps to cylindrical coordinates on receptacle
- [ ] Orientation computed so component points outward from center
- [ ] Unit tests verify mapping correctness

**Dependencies**: Task 4.1, Task 3.1

**Technical Notes**:
```rust
pub struct Transform3D {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

pub struct ReceptacleMapper {
    profile: Vec<Vec2>, // (radius, height) pairs
}

impl ReceptacleMapper {
    pub fn map_to_3d(&self, placement: &ComponentPlacement) -> Transform3D {
        // Map placement.radius (0-1) to height on receptacle
        let height = self.map_radius_to_height(placement.radius);

        // Get receptacle radius at this height
        let receptacle_radius = self.radius_at_height(height);

        // Compute 3D position in cylindrical coordinates
        let position = Vec3::cylindrical(
            receptacle_radius,
            placement.angle,
            height
        );

        // Compute rotation (normal pointing outward and up)
        let tangent = self.tangent_at_height(height);
        let radial = Vec3::new(
            placement.angle.cos(),
            0.0,
            placement.angle.sin()
        );
        let normal = radial.cross(tangent).normalize();
        let rotation = Quat::from_rotation_arc(Vec3::Y, normal);

        Transform3D {
            position,
            rotation,
            scale: Vec3::splat(placement.scale),
        }
    }
}
```

**Effort**: 3 hours

---

#### Task 4.3: Flower Assembly Function

**Description**: Combine all components into single flower mesh.

**Acceptance Criteria**:
- [ ] Module `floraison-diagram/src/flower.rs` created
- [ ] Struct `FlowerParams` combining all component parameters:
  ```rust
  pub struct FlowerParams {
      pub diagram: FloralDiagram,
      pub receptacle: ReceptacleParams,
      pub pistil: PistilParams,
      pub stamen: StamenParams,
      pub petal: SimplePetalParams,
  }
  ```
- [ ] Function `generate_flower(params: &FlowerParams) -> Mesh`
  - Generates receptacle
  - Generates each component type once
  - For each placement: clone, transform, merge into final mesh
- [ ] Proper coordinate transformations applied
- [ ] All meshes welded together
- [ ] Single unified mesh returned
- [ ] Unit test generates valid flower

**Dependencies**: Tasks 3.1-3.4, 4.1, 4.2

**Technical Notes**:
```rust
pub fn generate_flower(params: &FlowerParams) -> Mesh {
    let mut final_mesh = Mesh::default();

    // Generate receptacle
    let receptacle = receptacle::generate(&params.receptacle);
    final_mesh.merge(&receptacle);

    // Create mapper for positioning
    let mapper = ReceptacleMapper::from_params(&params.receptacle);

    // Generate template meshes for each component type
    let pistil_template = pistil::generate(&params.pistil);
    let stamen_template = stamen::generate(&params.stamen);
    let petal_template = petal::generate_simple(&params.petal);

    // Get placements from diagram
    let placements = params.diagram.generate_placements();

    // For each placement, instantiate and transform component
    for placement in placements {
        let template = match placement.component_type {
            ComponentType::Pistil => &pistil_template,
            ComponentType::Stamen => &stamen_template,
            ComponentType::Petal => &petal_template,
            ComponentType::Sepal => &petal_template, // reuse for now
        };

        let transform = mapper.map_to_3d(&placement);
        let mut instance = template.clone();
        instance.transform(transform.to_matrix());

        final_mesh.merge(&instance);
    }

    final_mesh
}
```

**Effort**: 2.5 hours

---

### Epic 5: Frontend Foundation

**Goal**: Create minimal UI to display generated flower in Three.js viewer.

**Estimated Effort**: 8-10 hours

#### Task 5.1: WASM Bindings for Flower Generation

**Description**: Expose flower generation to JavaScript via wasm-bindgen.

**Acceptance Criteria**:
- [ ] Module `floraison-wasm/src/lib.rs` created
- [ ] Struct `WasmFlowerGenerator` with `#[wasm_bindgen]`
- [ ] Method `generate_flower(params_json: &str) -> WasmMesh`
- [ ] Struct `WasmMesh` with getters for typed arrays:
  ```rust
  #[wasm_bindgen]
  pub struct WasmMesh {
      positions: Vec<f32>,
      normals: Vec<f32>,
      uvs: Vec<f32>,
      indices: Vec<u32>,
  }

  #[wasm_bindgen]
  impl WasmMesh {
      pub fn positions(&self) -> js_sys::Float32Array { /* ... */ }
      pub fn normals(&self) -> js_sys::Float32Array { /* ... */ }
      pub fn uvs(&self) -> js_sys::Float32Array { /* ... */ }
      pub fn indices(&self) -> js_sys::Uint32Array { /* ... */ }
  }
  ```
- [ ] Proper error handling (return Result, convert to JsValue)
- [ ] Build succeeds with wasm-pack
- [ ] Can be imported in Node.js test

**Dependencies**: Task 4.3, Task 1.2

**Technical Notes**:
```rust
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

#[wasm_bindgen]
pub struct WasmFlowerGenerator;

#[wasm_bindgen]
impl WasmFlowerGenerator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        // Set panic hook for better error messages
        console_error_panic_hook::set_once();
        Self
    }

    pub fn generate_flower(&self, params_json: &str) -> Result<WasmMesh, JsValue> {
        let params: FlowerParams = serde_json::from_str(params_json)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let mesh = floraison_diagram::generate_flower(&params);

        Ok(WasmMesh::from_mesh(mesh))
    }
}
```

**Effort**: 2.5 hours

---

#### Task 5.2: Three.js Scene Setup

**Description**: Create Three.js scene with camera, lights, and orbit controls.

**Acceptance Criteria**:
- [ ] Module `frontend/src/lib/three/scene.ts` created
- [ ] Function `createScene(canvas: HTMLCanvasElement) -> SceneContext`
- [ ] Scene contains:
  - Perspective camera with appropriate FOV
  - Ambient light + directional light
  - OrbitControls attached to camera
  - WebGL renderer
- [ ] Background color set (light gray or gradient)
- [ ] Helper grid (optional, toggleable)
- [ ] Function `animate()` starts render loop
- [ ] Function `dispose()` cleans up resources
- [ ] TypeScript types exported

**Dependencies**: Task 1.3

**Technical Notes**:
```typescript
import * as THREE from 'three';
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls';

export interface SceneContext {
  scene: THREE.Scene;
  camera: THREE.PerspectiveCamera;
  renderer: THREE.WebGLRenderer;
  controls: OrbitControls;
  animate: () => void;
  dispose: () => void;
}

export function createScene(canvas: HTMLCanvasElement): SceneContext {
  const scene = new THREE.Scene();
  scene.background = new THREE.Color(0xf0f0f0);

  const camera = new THREE.PerspectiveCamera(
    50,
    canvas.clientWidth / canvas.clientHeight,
    0.1,
    1000
  );
  camera.position.set(10, 10, 10);

  const renderer = new THREE.WebGLRenderer({ canvas, antialias: true });
  renderer.setSize(canvas.clientWidth, canvas.clientHeight);
  renderer.setPixelRatio(window.devicePixelRatio);

  const ambientLight = new THREE.AmbientLight(0xffffff, 0.5);
  scene.add(ambientLight);

  const dirLight = new THREE.DirectionalLight(0xffffff, 0.8);
  dirLight.position.set(5, 10, 5);
  scene.add(dirLight);

  const controls = new OrbitControls(camera, renderer.domElement);
  controls.enableDamping = true;

  let animationId: number;

  function animate() {
    animationId = requestAnimationFrame(animate);
    controls.update();
    renderer.render(scene, camera);
  }

  function dispose() {
    cancelAnimationFrame(animationId);
    renderer.dispose();
    controls.dispose();
  }

  return { scene, camera, renderer, controls, animate, dispose };
}
```

**Effort**: 2 hours

---

#### Task 5.3: WASM Loader Module

**Description**: Create utility to load and initialize WASM module.

**Acceptance Criteria**:
- [ ] Module `frontend/src/lib/wasm/loader.ts` created
- [ ] Function `loadWasm() -> Promise<typeof import('*.wasm')>`
- [ ] Handles initialization with proper error messages
- [ ] Caches loaded module (singleton pattern)
- [ ] Loading state exported for UI feedback
- [ ] TypeScript types properly imported

**Dependencies**: Task 5.1, Task 1.3

**Technical Notes**:
```typescript
import init, * as wasm from './floraison_wasm';

let wasmModule: typeof wasm | null = null;
let initPromise: Promise<typeof wasm> | null = null;

export async function loadWasm(): Promise<typeof wasm> {
  if (wasmModule) return wasmModule;

  if (!initPromise) {
    initPromise = init().then(() => {
      wasmModule = wasm;
      return wasm;
    });
  }

  return initPromise;
}

export function isWasmLoaded(): boolean {
  return wasmModule !== null;
}
```

**Effort**: 1 hour

---

#### Task 5.4: Mesh Converter (WASM → Three.js)

**Description**: Convert WASM mesh data to Three.js BufferGeometry.

**Acceptance Criteria**:
- [ ] Module `frontend/src/lib/three/mesh-converter.ts` created
- [ ] Function `wasmMeshToGeometry(wasmMesh: WasmMesh) -> THREE.BufferGeometry`
- [ ] Properly transfers typed arrays to BufferAttribute
- [ ] Sets all necessary attributes (position, normal, uv)
- [ ] Sets index buffer
- [ ] Computes bounding sphere for proper camera framing
- [ ] TypeScript types

**Dependencies**: Task 5.1, Task 5.2

**Technical Notes**:
```typescript
import * as THREE from 'three';
import type { WasmMesh } from '$lib/wasm/floraison_wasm';

export function wasmMeshToGeometry(wasmMesh: WasmMesh): THREE.BufferGeometry {
  const geometry = new THREE.BufferGeometry();

  const positions = wasmMesh.positions();
  const normals = wasmMesh.normals();
  const uvs = wasmMesh.uvs();
  const indices = wasmMesh.indices();

  geometry.setAttribute('position', new THREE.BufferAttribute(positions, 3));
  geometry.setAttribute('normal', new THREE.BufferAttribute(normals, 3));
  geometry.setAttribute('uv', new THREE.BufferAttribute(uvs, 2));
  geometry.setIndex(new THREE.BufferAttribute(indices, 1));

  geometry.computeBoundingSphere();

  return geometry;
}
```

**Effort**: 1 hour

---

#### Task 5.5: Basic ThreeViewer Component

**Description**: Create Svelte component that renders Three.js scene.

**Acceptance Criteria**:
- [ ] Component `frontend/src/lib/components/viewer/ThreeViewer.svelte` created
- [ ] Props: `mesh` (optional WasmMesh)
- [ ] Canvas element with proper sizing
- [ ] Initializes Three.js scene on mount
- [ ] Updates geometry when `mesh` prop changes
- [ ] Handles cleanup on unmount
- [ ] Responsive to container size changes
- [ ] Loading state displayed when mesh is null

**Dependencies**: Tasks 5.2, 5.3, 5.4

**Technical Notes**:
```svelte
<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { createScene } from '$lib/three/scene';
  import { wasmMeshToGeometry } from '$lib/three/mesh-converter';
  import type { WasmMesh } from '$lib/wasm/floraison_wasm';
  import * as THREE from 'three';

  export let mesh: WasmMesh | null = null;

  let canvas: HTMLCanvasElement;
  let sceneCtx: ReturnType<typeof createScene> | null = null;
  let flowerMesh: THREE.Mesh | null = null;

  onMount(() => {
    sceneCtx = createScene(canvas);
    sceneCtx.animate();

    if (mesh) {
      updateMesh(mesh);
    }
  });

  onDestroy(() => {
    if (flowerMesh) {
      flowerMesh.geometry.dispose();
      (flowerMesh.material as THREE.Material).dispose();
    }
    sceneCtx?.dispose();
  });

  $: if (mesh && sceneCtx) {
    updateMesh(mesh);
  }

  function updateMesh(newMesh: WasmMesh) {
    if (!sceneCtx) return;

    // Remove old mesh
    if (flowerMesh) {
      sceneCtx.scene.remove(flowerMesh);
      flowerMesh.geometry.dispose();
      (flowerMesh.material as THREE.Material).dispose();
    }

    // Create new mesh
    const geometry = wasmMeshToGeometry(newMesh);
    const material = new THREE.MeshStandardMaterial({
      color: 0xffcc00,
      side: THREE.DoubleSide,
    });
    flowerMesh = new THREE.Mesh(geometry, material);

    sceneCtx.scene.add(flowerMesh);

    // Frame camera
    const boundingSphere = geometry.boundingSphere!;
    const center = boundingSphere.center;
    const radius = boundingSphere.radius;
    sceneCtx.camera.position.set(
      center.x + radius * 2,
      center.y + radius * 2,
      center.z + radius * 2
    );
    sceneCtx.controls.target.copy(center);
  }
</script>

<div class="viewer-container">
  <canvas bind:this={canvas}></canvas>
  {#if !mesh}
    <div class="loading">Loading...</div>
  {/if}
</div>

<style>
  .viewer-container {
    width: 100%;
    height: 100%;
    position: relative;
  }

  canvas {
    width: 100%;
    height: 100%;
    display: block;
  }

  .loading {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
  }
</style>
```

**Effort**: 2.5 hours

---

#### Task 5.6: Main App Page with Default Flower

**Description**: Create main page that loads WASM and displays a default flower.

**Acceptance Criteria**:
- [ ] Page `frontend/src/routes/+page.svelte` created
- [ ] Loads WASM on mount
- [ ] Generates default flower with hardcoded params
- [ ] Displays ThreeViewer with generated mesh
- [ ] Shows loading state during WASM initialization
- [ ] Error handling with user-friendly messages
- [ ] Basic layout with TailwindCSS

**Dependencies**: Tasks 5.3, 5.5

**Technical Notes**:
```svelte
<script lang="ts">
  import { onMount } from 'svelte';
  import ThreeViewer from '$lib/components/viewer/ThreeViewer.svelte';
  import { loadWasm } from '$lib/wasm/loader';
  import type { WasmMesh } from '$lib/wasm/floraison_wasm';

  let mesh: WasmMesh | null = null;
  let loading = true;
  let error = '';

  onMount(async () => {
    try {
      const wasm = await loadWasm();
      const generator = new wasm.WasmFlowerGenerator();

      const defaultParams = {
        diagram: {
          pistil_count: 1,
          stamen_count: 6,
          petal_count: 6,
          sepal_count: 0,
          radial_symmetry: true,
        },
        receptacle: {
          height: 1.0,
          base_radius: 0.5,
          bulge_height: 0.3,
          bulge_radius: 0.6,
          top_radius: 0.3,
          segments: 16,
        },
        pistil: {
          length: 2.0,
          base_radius: 0.1,
          tip_radius: 0.08,
          stigma_radius: 0.15,
          segments: 8,
        },
        stamen: {
          filament_length: 1.5,
          filament_radius: 0.03,
          anther_length: 0.3,
          anther_radius: 0.06,
          segments: 6,
        },
        petal: {
          length: 4.0,
          width: 1.5,
          tip_width: 0.8,
          base_width: 0.6,
          resolution: 16,
        },
      };

      mesh = generator.generate_flower(JSON.stringify(defaultParams));
      loading = false;
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to load';
      loading = false;
    }
  });
</script>

<main class="w-screen h-screen">
  {#if loading}
    <div class="flex items-center justify-center h-full">
      <p class="text-xl">Loading Floraison...</p>
    </div>
  {:else if error}
    <div class="flex items-center justify-center h-full">
      <p class="text-xl text-red-600">Error: {error}</p>
    </div>
  {:else}
    <ThreeViewer {mesh} />
  {/if}
</main>
```

**Effort**: 1.5 hours

---

### Epic 5 Completion Checkpoint

**Deliverable**: A working web application that displays a single procedurally-generated lily-like flower in a 3D viewer.

**Testing**:
- [ ] Run `npm run dev` in frontend directory
- [ ] Navigate to http://localhost:5173
- [ ] Flower renders correctly
- [ ] Can orbit camera around flower
- [ ] No console errors
- [ ] Mesh looks reasonable (no holes, proper shading)

---

## Phase 2: Complete Flower System

**Goal**: Implement full B-spline petal geometry, complete all floral components, and add comprehensive parameter UI.

### Epic 6: Advanced Petal Geometry (B-splines)

**Goal**: Replace simple flat petals with B-spline surfaces that support deformations.

**Estimated Effort**: 10-12 hours

#### Task 6.1: B-Spline Basis Function Evaluation

**Description**: Implement Cox-de Boor algorithm for B-spline basis functions.

**Acceptance Criteria**:
- [ ] Module `floraison-core/src/math/bspline.rs` created
- [ ] Function `basis_function(i: usize, p: usize, u: f32, knots: &[f32]) -> f32`
  - Implements Cox-de Boor recursion
  - Handles edge cases (division by zero)
- [ ] Function `generate_knot_vector(n: usize, p: usize, uniform: bool) -> Vec<f32>`
  - Creates open uniform knot vector
- [ ] Unit tests verify known properties:
  - Partition of unity: Σ Nᵢ(u) = 1
  - Local support: Nᵢ(u) = 0 outside [uᵢ, uᵢ₊ₚ₊₁]
  - Endpoint interpolation
- [ ] Documentation with mathematical notation

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

#### Task 6.2: B-Spline Surface Evaluation

**Description**: Implement tensor product B-spline surface evaluation.

**Acceptance Criteria**:
- [ ] Struct `BSplineSurface` in `bspline.rs`:
  ```rust
  pub struct BSplineSurface {
      pub control_points: Vec<Vec<Vec3>>,  // 2D grid
      pub degree_u: usize,
      pub degree_v: usize,
      pub knots_u: Vec<f32>,
      pub knots_v: Vec<f32>,
  }
  ```
- [ ] Method `evaluate(&self, u: f32, v: f32) -> Vec3` evaluates surface at (u,v)
- [ ] Method `evaluate_derivative_u(&self, u: f32, v: f32) -> Vec3` for tangent
- [ ] Method `evaluate_derivative_v(&self, u: f32, v: f32) -> Vec3` for tangent
- [ ] Method `normal(&self, u: f32, v: f32) -> Vec3` via cross product
- [ ] Unit tests verify:
  - Interpolation of corner control points
  - Surface lies in convex hull of control points
- [ ] Documentation

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

#### Task 6.3: Petal Control Point Grid Generation

**Description**: Generate initial control point grid from outline parameters.

**Acceptance Criteria**:
- [ ] Function in `petal.rs`: `generate_control_grid(params: &PetalParams) -> Vec<Vec<Vec3>>`
- [ ] Creates grid matching petal outline shape:
  - Base: narrow
  - Middle: widest
  - Tip: tapered
- [ ] Grid dimensions configurable (e.g., 5×9 for smooth curves)
- [ ] All points initially in XY plane (Z=0)
- [ ] Proper spacing for good surface quality
- [ ] Unit test verifies grid dimensions and boundary points

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

#### Task 6.4: Petal Surface Deformations

**Description**: Apply curl, twist, and ruffle deformations to petal surface.

**Acceptance Criteria**:
- [ ] Function `apply_curl(surface: &mut BSplineSurface, amount: f32)`
  - Bends petal around horizontal axis
  - Amount in range [-1, 1]: negative = curl down, positive = curl up
- [ ] Function `apply_twist(surface: &mut BSplineSurface, angle: f32)`
  - Twists petal around central vein
  - Angle in degrees
  - Twist increases toward tip
- [ ] Function `apply_ruffle(surface: &mut BSplineSurface, freq: f32, amp: f32)`
  - Adds sinusoidal waves to edges
  - Frequency = number of waves
  - Amplitude = wave height
- [ ] Each function modifies control points in place
- [ ] Combined deformations work correctly
- [ ] Unit tests verify deformation magnitudes

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

#### Task 6.5: Complete B-Spline Petal Generator

**Description**: Integrate B-spline surface evaluation with mesh generation and deformations.

**Acceptance Criteria**:
- [ ] Enhanced `PetalParams` struct:
  ```rust
  pub struct PetalParams {
      pub length: f32,
      pub width: f32,
      pub tip_width: f32,
      pub base_width: f32,
      pub curl: f32,        // -1 to 1
      pub twist: f32,       // degrees
      pub ruffle_freq: f32,
      pub ruffle_amp: f32,
      pub resolution: usize, // tessellation density
  }
  ```
- [ ] Function `generate(params: &PetalParams) -> Mesh`
  - Generates control grid
  - Creates B-spline surface
  - Applies deformations
  - Tessellates surface at specified resolution
  - Returns mesh with front and back faces
- [ ] Proper UV coordinates
- [ ] Smooth normals
- [ ] Unit test generates valid mesh
- [ ] Visual test shows deformations work

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

### Epic 7: Complete Floral Components

**Goal**: Add sepals and enhance existing components with more parameters.

**Estimated Effort**: 4-6 hours

#### Task 7.1: Sepal Component (Reuse Petal)

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

#### Task 7.2: Enhanced Pistil with Style Curve

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

#### Task 7.3: Enhanced Stamen with Curved Filament

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

#### Task 7.4: Component Color Parameters

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

### Epic 8: Floral Diagram System

**Goal**: Create interactive UI for defining floral diagrams (not just hardcoded).

**Estimated Effort**: 8-10 hours

#### Task 8.1: Diagram Parameter Store

**Description**: Create Svelte store for floral diagram parameters with reactivity.

**Acceptance Criteria**:
- [ ] Store module `frontend/src/lib/stores/parameters.ts` created
- [ ] Writable store for `FloralDiagram`
- [ ] Writable stores for each component type parameters
- [ ] Derived store that combines all parameters into JSON for WASM
- [ ] Default values loaded on init
- [ ] TypeScript types match Rust structs

**Dependencies**: Task 4.1

**Technical Notes**:
```typescript
import { writable, derived } from 'svelte/store';

export interface FloralDiagramParams {
  pistilCount: number;
  stamenCount: number;
  petalCount: number;
  sepalCount: number;
  radialSymmetry: boolean;
}

export const diagramParams = writable<FloralDiagramParams>({
  pistilCount: 1,
  stamenCount: 6,
  petalCount: 6,
  sepalCount: 0,
  radialSymmetry: true,
});

export const receptacleParams = writable<ReceptacleParams>({ /* ... */ });
export const pistilParams = writable<PistilParams>({ /* ... */ });
export const stamenParams = writable<StamenParams>({ /* ... */ });
export const petalParams = writable<PetalParams>({ /* ... */ });

// Combine all parameters for WASM
export const allParams = derived(
  [diagramParams, receptacleParams, pistilParams, stamenParams, petalParams],
  ([$diagram, $receptacle, $pistil, $stamen, $petal]) => ({
    diagram: $diagram,
    receptacle: $receptacle,
    pistil: $pistil,
    stamen: $stamen,
    petal: $petal,
  })
);
```

**Effort**: 1.5 hours

---

#### Task 8.2: Parameter Panel Component

**Description**: Create UI panel with sliders/inputs for all flower parameters.

**Acceptance Criteria**:
- [ ] Component `frontend/src/lib/components/ui/ParameterPanel.svelte` created
- [ ] Organized into sections (Diagram, Receptacle, Pistil, Stamen, Petal)
- [ ] Each parameter has:
  - Label
  - Input (slider for 0-1 values, number input for counts)
  - Current value display
- [ ] Binds to parameter stores
- [ ] Styled with TailwindCSS
- [ ] Collapsible sections (optional)
- [ ] Responsive layout

**Dependencies**: Task 8.1

**Technical Notes**:
```svelte
<script lang="ts">
  import { diagramParams, receptacleParams, /* ... */ } from '$lib/stores/parameters';
</script>

<div class="parameter-panel p-4 bg-white shadow-lg overflow-y-auto">
  <h2 class="text-2xl font-bold mb-4">Flower Parameters</h2>

  <!-- Diagram Section -->
  <section class="mb-6">
    <h3 class="text-xl font-semibold mb-2">Floral Diagram</h3>

    <div class="mb-3">
      <label class="block text-sm font-medium mb-1">
        Pistils: {$diagramParams.pistilCount}
      </label>
      <input
        type="number"
        min="0"
        max="10"
        bind:value={$diagramParams.pistilCount}
        class="w-full"
      />
    </div>

    <div class="mb-3">
      <label class="block text-sm font-medium mb-1">
        Stamens: {$diagramParams.stamenCount}
      </label>
      <input
        type="range"
        min="0"
        max="30"
        bind:value={$diagramParams.stamenCount}
        class="w-full"
      />
    </div>

    <!-- More parameters... -->
  </section>

  <!-- Receptacle Section -->
  <section class="mb-6">
    <h3 class="text-xl font-semibold mb-2">Receptacle</h3>
    <!-- Parameters... -->
  </section>

  <!-- More sections... -->
</div>

<style>
  .parameter-panel {
    width: 320px;
    max-height: 100vh;
  }
</style>
```

**Effort**: 3 hours

---

#### Task 8.3: Regenerate on Parameter Change

**Description**: Connect parameter store to flower regeneration with debouncing.

**Acceptance Criteria**:
- [ ] Update main page to subscribe to `allParams` store
- [ ] Debounce regeneration (300-500ms delay after last change)
- [ ] Show "Generating..." indicator during regeneration
- [ ] Handle errors gracefully
- [ ] Camera maintains position during updates (doesn't reset)

**Dependencies**: Task 8.1, 5.6

**Technical Notes**:
```typescript
import { debounce } from 'lodash-es';

let regenerating = false;

const regenerateFlower = debounce(async (params: AllParams) => {
  regenerating = true;
  try {
    const wasm = await loadWasm();
    const generator = new wasm.WasmFlowerGenerator();
    mesh = generator.generate_flower(JSON.stringify(params));
  } catch (e) {
    console.error('Failed to generate flower:', e);
  } finally {
    regenerating = false;
  }
}, 300);

$: regenerateFlower($allParams);
```

**Effort**: 2 hours

---

#### Task 8.4: Preset Flowers

**Description**: Create preset parameter sets for common flower types.

**Acceptance Criteria**:
- [ ] Module `frontend/src/lib/presets.ts` with exported presets
- [ ] At least 5 presets:
  - Lily (default)
  - Rose (many petals, layered)
  - Daisy (simple, flat petals)
  - Tulip (6 petals, cup shape)
  - Orchid (complex petal shapes)
- [ ] Dropdown in UI to select preset
- [ ] Selecting preset updates all parameter stores
- [ ] "Custom" option when user modifies preset

**Dependencies**: Task 8.1, 8.2

**Technical Notes**:
```typescript
export interface FlowerPreset {
  name: string;
  params: AllParams;
}

export const presets: FlowerPreset[] = [
  {
    name: 'Lily',
    params: {
      diagram: { pistilCount: 1, stamenCount: 6, petalCount: 6, sepalCount: 0 },
      petal: { length: 4.0, width: 1.5, curl: 0.3, twist: 10, /* ... */ },
      // ...
    },
  },
  {
    name: 'Rose',
    params: {
      diagram: { pistilCount: 0, stamenCount: 20, petalCount: 24, sepalCount: 5 },
      petal: { length: 2.5, width: 2.0, curl: 0.8, ruffle_freq: 3, /* ... */ },
      // ...
    },
  },
  // ... more presets
];
```

Add dropdown to ParameterPanel:
```svelte
<select bind:value={selectedPreset} on:change={loadPreset}>
  {#each presets as preset}
    <option value={preset.name}>{preset.name}</option>
  {/each}
  <option value="custom">Custom</option>
</select>
```

**Effort**: 2 hours

---

### Epic 9: UI Enhancement

**Goal**: Improve user experience with better layout, controls, and visual feedback.

**Estimated Effort**: 4-6 hours

#### Task 9.1: Split View Layout

**Description**: Create split-pane layout with parameter panel on left, viewer on right.

**Acceptance Criteria**:
- [ ] Update main page layout to use flex/grid
- [ ] Parameter panel on left (fixed width or resizable)
- [ ] 3D viewer on right (fills remaining space)
- [ ] Responsive: on mobile, panel is collapsible drawer
- [ ] TailwindCSS styling

**Dependencies**: Task 8.2

**Technical Notes**:
```svelte
<main class="flex h-screen">
  <aside class="w-80 border-r border-gray-300 overflow-y-auto">
    <ParameterPanel />
  </aside>

  <div class="flex-1">
    <ThreeViewer {mesh} />
  </div>
</main>
```

**Effort**: 1.5 hours

---

#### Task 9.2: Viewer Controls UI

**Description**: Add UI controls for viewer (background color, lighting, axes helper).

**Acceptance Criteria**:
- [ ] Floating controls overlay on viewer
- [ ] Toggle axes helper (show/hide)
- [ ] Background color picker
- [ ] Lighting intensity sliders
- [ ] Reset camera button
- [ ] Toggle wireframe mode

**Dependencies**: Task 5.5

**Technical Notes**:
Add props to ThreeViewer for these options, and small UI overlay component:

```svelte
<!-- ViewerControls.svelte -->
<div class="absolute top-4 right-4 bg-white p-2 rounded shadow">
  <button on:click={toggleAxes}>Axes</button>
  <button on:click={resetCamera}>Reset Camera</button>
  <input type="color" bind:value={bgColor} />
</div>
```

**Effort**: 2 hours

---

#### Task 9.3: Loading States and Error Handling

**Description**: Improve feedback for loading and error states.

**Acceptance Criteria**:
- [ ] Loading spinner during WASM initialization
- [ ] Loading indicator during flower regeneration
- [ ] Error messages displayed in UI (not just console)
- [ ] Retry button on errors
- [ ] Toast notifications for successes/errors (optional)

**Dependencies**: Task 8.3

**Technical Notes**:
Use simple toast library or create custom notification component.

```svelte
{#if loading}
  <div class="loading-overlay">
    <div class="spinner"></div>
    <p>Generating flower...</p>
  </div>
{/if}

{#if error}
  <div class="error-banner">
    <p>Error: {error}</p>
    <button on:click={retry}>Retry</button>
  </div>
{/if}
```

**Effort**: 1.5 hours

---

#### Task 9.4: Performance Optimization

**Description**: Profile and optimize for smooth interaction.

**Acceptance Criteria**:
- [ ] Profile WASM generation time, identify bottlenecks
- [ ] Add progress bar for long operations (if > 1 second)
- [ ] Consider Web Worker for WASM (if needed)
- [ ] Optimize mesh resolution (adaptive LOD)
- [ ] Measure and document performance metrics

**Dependencies**: All previous Epic 6-9 tasks

**Technical Notes**:
Use browser DevTools Performance profiler. Look for:
- Excessive B-spline evaluations (cache results)
- Large mesh vertex counts (reduce resolution parameter)
- Redundant regenerations (ensure debouncing works)

**Effort**: 2 hours

---

### Phase 2 Completion Checkpoint

**Deliverable**: Full-featured single flower generator with advanced B-spline petals, comprehensive parameter UI, and presets.

**Testing**:
- [ ] All presets load correctly
- [ ] Adjusting parameters updates flower in real-time
- [ ] B-spline petals show smooth curves and deformations
- [ ] No performance issues during parameter adjustment
- [ ] UI is intuitive and responsive

---

## Phase 3: Inflorescence System

**Goal**: Implement multi-flower branching structures with 8 inflorescence patterns.

### Epic 10: Inflorescence Foundation

**Goal**: Core data structures and axis generation for inflorescences.

**Estimated Effort**: 6-8 hours

#### Task 10.1: Inflorescence Data Structures

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

#### Task 10.2: 3D Axis Curve Generation

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

#### Task 10.3: Axis Parameterization

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

### Epic 11: Simple Inflorescence Patterns

**Goal**: Implement 4 simple indeterminate inflorescence patterns.

**Estimated Effort**: 10-12 hours

#### Task 11.1: Raceme Pattern Generator

**Description**: Implement raceme pattern (single axis, flowers on pedicels).

**Acceptance Criteria**:
- [ ] Module `floraison-inflorescence/src/patterns/raceme.rs` created
- [ ] Function `generate_branch_points(params: &InflorescenceParams, axis: &AxisCurve) -> Vec<BranchPoint>`
- [ ] Flowers spaced evenly along axis
- [ ] Pedicel length interpolates from top to bottom
- [ ] Down angle interpolates from top to bottom
- [ ] Rotation angle applies Fibonacci spiral
- [ ] Age increases bottom to top (indeterminate)
- [ ] Unit test verifies branch count and positioning

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

#### Task 11.2: Spike Pattern Generator

**Description**: Implement spike pattern (like raceme but flowers sessile - no pedicels).

**Acceptance Criteria**:
- [ ] Module `floraison-inflorescence/src/patterns/spike.rs` created
- [ ] Function `generate_branch_points(params: &InflorescenceParams, axis: &AxisCurve) -> Vec<BranchPoint>`
- [ ] Flowers placed directly on axis (length = 0)
- [ ] Rotation angle still applies
- [ ] Flower scale interpolates
- [ ] Age indeterminate
- [ ] Unit test

**Dependencies**: Task 10.1, 10.3

**Technical Notes**:
Very similar to raceme, but branch length is always 0 (or very small). Flowers attach directly to axis.

**Effort**: 1.5 hours

---

#### Task 11.3: Umbel Pattern Generator

**Description**: Implement umbel pattern (all pedicels from single point, umbrella-like).

**Acceptance Criteria**:
- [ ] Module `floraison-inflorescence/src/patterns/umbel.rs` created
- [ ] Function `generate_branch_points(params: &InflorescenceParams, axis: &AxisCurve) -> Vec<BranchPoint>`
- [ ] All branches originate from top of axis
- [ ] Spread out in umbrella shape
- [ ] Rotation angle determines angular spacing
- [ ] Down angle determines spread
- [ ] All flowers same age (determinate)
- [ ] Unit test

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

#### Task 11.4: Corymb Pattern Generator

**Description**: Implement corymb pattern (pedicels of varying length, flat-topped).

**Acceptance Criteria**:
- [ ] Module `floraison-inflorescence/src/patterns/corymb.rs` created
- [ ] Function `generate_branch_points(params: &InflorescenceParams, axis: &AxisCurve) -> Vec<BranchPoint>`
- [ ] Branches along axis like raceme
- [ ] Pedicel length adjusted so all flowers reach same height
- [ ] Age indeterminate
- [ ] Unit test

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

#### Task 11.5: Inflorescence Assembly Function

**Description**: Combine axis, branches, and flowers into complete inflorescence mesh.

**Acceptance Criteria**:
- [ ] Module `floraison-inflorescence/src/assembly.rs` created
- [ ] Function `assemble_inflorescence(params: &InflorescenceParams, flower_mesh: &Mesh) -> Mesh`
  - Generates axis curve from parameters
  - Calls pattern-specific generator to get branch points
  - Instantiates flower at each branch point
  - Generates stem geometry (cylinder along axis and branches)
  - Merges all into single mesh
- [ ] Proper transformations applied
- [ ] Unit test

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

### Epic 12: Complex Patterns & Polish

**Goal**: Implement determinate and compound patterns, add flower aging, polish visuals.

**Estimated Effort**: 10-12 hours

#### Task 12.1: Dichasium Pattern Generator

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

#### Task 12.2: Drepanium Pattern Generator

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

#### Task 12.3: Compound Pattern Support

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

#### Task 12.4: Flower Aging System

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

### Epic 12 Completion & Phase 3 Checkpoint

**Deliverable**: Complete inflorescence system with 8 patterns, flower aging, and full integration.

**Testing**:
- [ ] All 8 patterns generate correctly
- [ ] Compound patterns show hierarchical structure
- [ ] Flower aging visible in indeterminate patterns
- [ ] Stem geometry looks reasonable
- [ ] Parameters in UI control inflorescence appearance

---

## Phase 4: Polish & Launch

**Goal**: glTF export, visual polish, documentation, demo content.

### Epic 13: glTF Export

**Goal**: Implement proper glTF 2.0 export with materials and hierarchy.

**Estimated Effort**: 6-8 hours

#### Task 13.1: glTF Scene Graph Builder

**Description**: Create glTF scene structure with nodes and meshes.

**Acceptance Criteria**:
- [ ] Module `floraison-export/src/gltf.rs` created
- [ ] Struct `GltfBuilder` to construct scene
- [ ] Methods:
  - `add_mesh(name: &str, mesh: &Mesh, material: Material) -> usize` returns mesh ID
  - `add_node(name: &str, mesh_id: Option<usize>, transform: Mat4) -> usize`
  - `set_root(node_id: usize)`
  - `build() -> Vec<u8>` returns glTF binary
- [ ] Proper hierarchy support (parent/child nodes)
- [ ] Unit test generates valid glTF

**Dependencies**: Task 2.2

**Technical Notes**:
Use `gltf-json` crate:
```rust
use gltf_json as json;

pub struct GltfBuilder {
    buffers: Vec<json::Buffer>,
    buffer_views: Vec<json::BufferView>,
    accessors: Vec<json::Accessor>,
    meshes: Vec<json::Mesh>,
    nodes: Vec<json::Node>,
    materials: Vec<json::Material>,
    buffer_data: Vec<u8>,
}
```

**Effort**: 4 hours

---

#### Task 13.2: Material Support

**Description**: Add material parameters (color, roughness, metallic) to glTF export.

**Acceptance Criteria**:
- [ ] Struct `Material` with PBR parameters:
  ```rust
  pub struct Material {
      pub base_color: Vec3,
      pub roughness: f32,
      pub metallic: f32,
  }
  ```
- [ ] GltfBuilder creates glTF materials with these properties
- [ ] Each component can have its own material
- [ ] Materials exported correctly in glTF JSON

**Dependencies**: Task 13.1

**Technical Notes**:
```rust
let material = json::Material {
    pbr_metallic_roughness: json::material::PbrMetallicRoughness {
        base_color_factor: json::material::PbrBaseColorFactor([
            mat.base_color.x,
            mat.base_color.y,
            mat.base_color.z,
            1.0,
        ]),
        roughness_factor: json::material::StrengthFactor(mat.roughness),
        metallic_factor: json::material::StrengthFactor(mat.metallic),
        ..Default::default()
    },
    ..Default::default()
};
```

**Effort**: 2 hours

---

#### Task 13.3: Export Button in UI

**Description**: Add export functionality to frontend.

**Acceptance Criteria**:
- [ ] Export button in UI
- [ ] Click button calls WASM export function
- [ ] Downloads .glb file (binary glTF)
- [ ] Filename includes timestamp or flower type
- [ ] Success notification

**Dependencies**: Task 13.1, 13.2

**Technical Notes**:
```rust
// WASM binding
#[wasm_bindgen]
impl WasmFlowerGenerator {
    pub fn export_gltf(&self) -> Vec<u8> {
        let builder = GltfBuilder::new();
        // ... build scene
        builder.build()
    }
}
```

```svelte
<!-- Frontend -->
<script>
  async function exportGltf() {
    const wasm = await loadWasm();
    const generator = new wasm.WasmFlowerGenerator();
    const gltfBytes = generator.export_gltf();

    const blob = new Blob([gltfBytes], { type: 'model/gltf-binary' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `flower_${Date.now()}.glb`;
    a.click();
  }
</script>

<button on:click={exportGltf}>Export glTF</button>
```

**Effort**: 2 hours

---

### Epic 14: UI Polish & Presets

**Goal**: Final UI improvements, more presets, gallery view.

**Estimated Effort**: 6-8 hours

#### Task 14.1: Inflorescence UI Panel

**Description**: Add parameter panel for inflorescence editing.

**Acceptance Criteria**:
- [ ] New section in ParameterPanel for inflorescence
- [ ] Pattern type dropdown
- [ ] Parameters relevant to selected pattern
- [ ] Toggle between single flower / inflorescence mode
- [ ] UI disables irrelevant parameters

**Dependencies**: Task 8.2, 11.5

**Effort**: 2.5 hours

---

#### Task 14.2: Inflorescence Presets

**Description**: Add presets for common inflorescence types.

**Acceptance Criteria**:
- [ ] At least 5 inflorescence presets:
  - Lily (raceme)
  - Lavender (spike)
  - Cherry blossom (umbel)
  - Hydrangea (corymb)
  - Cimicifuga (compound raceme)
- [ ] Preset dropdown includes inflorescence presets
- [ ] Loading preset updates all params including inflorescence

**Dependencies**: Task 8.4, 11.5

**Effort**: 2 hours

---

#### Task 14.3: Visual Enhancements

**Description**: Improve visual quality of rendering.

**Acceptance Criteria**:
- [ ] Better materials (subsurface scattering approximation, translucency)
- [ ] Shadows enabled (directional light shadow map)
- [ ] Environment map for reflections (optional)
- [ ] Anti-aliasing (FXAA or MSAA)
- [ ] Depth of field (optional, post-processing)

**Dependencies**: Task 5.5

**Technical Notes**:
```typescript
// Enable shadows
renderer.shadowMap.enabled = true;
dirLight.castShadow = true;

// Better material
const material = new THREE.MeshStandardMaterial({
  color: 0xffcc00,
  roughness: 0.5,
  metalness: 0.0,
  side: THREE.DoubleSide,
  transparent: true,
  opacity: 0.95,
});

// Subsurface scattering approximation
material.onBeforeCompile = (shader) => {
  // Custom shader code for translucency
};
```

**Effort**: 3 hours

---

#### Task 14.4: Responsive Design & Mobile Support

**Description**: Ensure UI works on mobile devices.

**Acceptance Criteria**:
- [ ] Parameter panel is collapsible drawer on mobile
- [ ] Touch controls work for orbit camera
- [ ] UI scales properly on small screens
- [ ] Tested on iOS and Android

**Dependencies**: Task 9.1

**Technical Notes**:
Use Tailwind responsive classes:
```svelte
<aside class="hidden md:block md:w-80">
  <ParameterPanel />
</aside>

<button class="md:hidden" on:click={togglePanel}>
  Menu
</button>
```

**Effort**: 2.5 hours

---

### Epic 15: Documentation & Demo

**Goal**: Create documentation, demo videos, polish for Made with Claude submission.

**Estimated Effort**: 4-6 hours

#### Task 15.1: User Documentation

**Description**: Write user guide for the application.

**Acceptance Criteria**:
- [ ] `docs/USER_GUIDE.md` created
- [ ] Sections:
  - Quick start
  - UI overview
  - Parameter explanations
  - Preset gallery
  - Export workflow
- [ ] Screenshots/GIFs of UI
- [ ] Tips for creating specific flower types

**Dependencies**: All previous tasks

**Effort**: 2 hours

---

#### Task 15.2: Developer Documentation

**Description**: Document architecture and code for future contributors.

**Acceptance Criteria**:
- [ ] Update `docs/TECHNICAL_OVERVIEW.md` with implementation details
- [ ] Add inline Rust documentation (rustdoc)
- [ ] TypeScript documentation for frontend
- [ ] `CONTRIBUTING.md` with setup instructions
- [ ] Architecture diagrams (optional, can be ASCII art)

**Dependencies**: All previous tasks

**Effort**: 2 hours

---

#### Task 15.3: Demo Content Creation

**Description**: Create showcase content for Made with Claude submission.

**Acceptance Criteria**:
- [ ] Generate 10-15 diverse flower models
- [ ] Export high-quality renders (screenshots)
- [ ] Create short demo video (1-2 minutes):
  - Show UI interaction
  - Parameter adjustments
  - Different presets
  - Export workflow
- [ ] Write compelling project description
- [ ] Highlight procedural nature and Claude's role

**Dependencies**: All previous tasks

**Effort**: 2 hours

---

#### Task 15.4: Final Testing & Bug Fixes

**Description**: Comprehensive testing and bug fixing before launch.

**Acceptance Criteria**:
- [ ] Test all presets load correctly
- [ ] Test all inflorescence patterns
- [ ] Test edge cases (extreme parameter values)
- [ ] Fix any rendering glitches
- [ ] Fix any UI issues
- [ ] Performance testing on low-end devices
- [ ] Cross-browser testing (Chrome, Firefox, Safari)

**Dependencies**: All previous tasks

**Effort**: 3 hours

---

## Summary

### Total Estimated Effort

- **Phase 1**: 28-34 hours (1 week)
- **Phase 2**: 26-32 hours (1 week)
- **Phase 3**: 26-32 hours (1 week)
- **Phase 4**: 16-22 hours (3-4 days)

**Total**: 96-120 hours (~3.5 weeks of full-time work)

### Task Statistics

- **Total Epics**: 15
- **Total Tasks**: 78
- **Average Task Effort**: 1.5-2.5 hours

### Critical Path

1. Project setup → Core math → Basic components → Simple flower (Phase 1)
2. B-spline implementation → Advanced components → Parameter UI (Phase 2)
3. Inflorescence foundation → Pattern implementation → Assembly (Phase 3)
4. glTF export → Polish → Launch (Phase 4)

### Risk Areas

- **B-spline implementation** (Epic 6): Most mathematically complex, may take longer
- **3D curve reconstruction** (Task 10.2): Algorithm from paper may need debugging
- **Performance** (Task 9.4): May need optimization if meshes are too detailed
- **Mobile support** (Task 14.4): Touch controls and performance may be challenging

### Next Steps

1. Begin with Phase 1, Epic 1 (Project Setup)
2. Complete tasks in order, checking off acceptance criteria
3. Commit frequently with descriptive messages
4. Test each epic deliverable before moving to next phase
5. Document any deviations from plan or discoveries during implementation

---

**Ready to begin implementation!** 🚀
