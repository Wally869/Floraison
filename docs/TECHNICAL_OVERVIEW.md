# Technical Overview

## Implementation Status

**Project Completion**: ~95% (Phases 1-4)

- ✅ **Phase 1**: Core Math, Components, Assembly, Frontend Foundation (Epics 1-5)
- ✅ **Phase 2**: Advanced Geometry, Complete Components, Floral Diagram, UI (Epics 6-9)
- ✅ **Phase 3**: Inflorescence Foundation, Patterns, Polish (Epics 10-12)
- ✅ **Epic 13**: glTF Export (Three.js GLTFExporter approach)
- ✅ **Epic 14**: UI Polish, Visual Enhancements, Mobile Support
- 🚧 **Epic 15**: Documentation (in progress)

**Key Features Implemented**:
- 8 inflorescence patterns (Raceme, Spike, Umbel, Corymb, Dichasium, Drepanium, CompoundRaceme, CompoundUmbel)
- 11 presets (5 single flowers + 6 inflorescences)
- Vertex colors (full pipeline from Rust to renderer)
- glTF 2.0 export with PBR materials
- Professional lighting (VSM shadows, ACES tone mapping, three-point lighting)
- Responsive mobile UI with drawer pattern
- User-controllable lighting and exposure

---

## Introduction

Floraison implements a procedural flower generation system based on botanical morphology principles. Unlike traditional 3D modeling tools that require manual mesh manipulation, Floraison generates complete flower models from high-level structural and parametric descriptions.

## Motivation

Flowers present unique challenges for 3D modeling:
- Complex hierarchical structures (individual flowers → inflorescences)
- Many similar components arranged in patterns (radial symmetry, phyllotaxis)
- Organic freeform shapes (petals, sepals) that are tedious to model manually
- Botanical constraints that must be respected for realism

The goal is to balance **ease of use** with **botanical accuracy** while maintaining **flexibility** in the types of flowers that can be generated.

## Source Material

This implementation is based on the SIGGRAPH 2005 paper:

**"Floral diagrams and inflorescences: Interactive flower modeling using botanical structural constraints"**
by Takashi Ijiri, Shigeru Owada, Makoto Okabe, Takeo Igarashi

### Key Concepts from Paper

**Floral Diagrams**: Compact 2D representations showing the layout of floral components (pistils, stamens, petals, sepals) on a receptacle. Used by botanists to describe flower structure independent of geometry.

**Inflorescences**: Branching patterns for multiple flowers. Categories include:
- *Indeterminate*: Lower flowers bloom first (raceme, spike, umbel, corymb)
- *Determinate*: Central/top flowers bloom first (dichasium, drepanium)
- *Compound*: Recursive combinations of the above

**Separation of Structure and Geometry**: The paper's core insight—define *what* components exist and *how* they're arranged separately from *what shape* they take.

### Adaptation: Sketch → Parameters

The original paper uses sketch-based interfaces for geometry definition:
- Draw curves to define receptacle profiles
- Draw strokes to deform petal surfaces
- Draw freehand inflorescence axes

Our implementation replaces sketching with **parametric controls**, making the system fully procedural and suitable for algorithmic generation, batch processing, and programmatic control.

## Architecture

### High-Level Data Flow

```
┌─────────────────┐
│  Svelte UI      │  User adjusts parameters
│  (Frontend)     │
└────────┬────────┘
         │ Parameter objects (JSON)
         ↓
┌─────────────────┐
│  WASM Bridge    │  wasm-bindgen bindings
└────────┬────────┘
         │ Rust structs
         ↓
┌─────────────────┐
│  Rust Core      │  Geometry generation
│  - Floral parts │
│  - Assembly     │
│  - Colors       │
└────────┬────────┘
         │ Float32Array (vertices, normals, UVs, colors, indices)
         ↓
┌─────────────────┐
│  Three.js       │  WebGL rendering with vertex colors
│  (Frontend)     │  MeshPhysicalMaterial (vertexColors: true)
└─────────────────┘
         │ User clicks Export GLB
         ↓
┌─────────────────┐
│  GLTFExporter   │  Three.js → glTF conversion
│  (Three.js)     │
└─────────────────┘
         │ Binary GLB download
         ↓
┌─────────────────┐
│  glTF File      │  Standard 3D format with PBR materials
└─────────────────┘
```

### Rust Module Structure

```
floraison/
├── crates/
│   ├── floraison-core/        # Pure computational geometry
│   │   ├── math/
│   │   │   ├── vector.rs      # 3D vector operations (or re-export glam)
│   │   │   ├── bspline.rs     # B-spline curves and surfaces
│   │   │   ├── curves.rs      # 3D curve reconstruction
│   │   │   └── phyllotaxis.rs # Fibonacci spiral arrangements
│   │   ├── geometry/
│   │   │   ├── mesh.rs        # Vertex/index buffer structures
│   │   │   ├── surface_rev.rs # Surface of revolution generator
│   │   │   └── tessellation.rs# Mesh tessellation utilities
│   │   └── lib.rs
│   │
│   ├── floraison-components/  # Individual floral parts
│   │   ├── receptacle.rs      # Base structure (surface of revolution)
│   │   ├── pistil.rs          # Center reproductive structure
│   │   ├── stamen.rs          # Filament + anther
│   │   ├── petal.rs           # B-spline surface with deformations
│   │   └── sepal.rs           # Similar to petal
│   │
│   ├── floraison-diagram/     # Single flower assembly
│   │   ├── layout.rs          # Radial symmetry, positioning
│   │   ├── mapping.rs         # 2D floral diagram → 3D on receptacle
│   │   └── flower.rs          # Complete flower assembly
│   │
│   ├── floraison-inflorescence/ # Multi-flower structures
│   │   ├── patterns/
│   │   │   ├── raceme.rs
│   │   │   ├── umbel.rs
│   │   │   ├── dichasium.rs
│   │   │   └── ... (8 patterns total)
│   │   ├── axis.rs            # 3D stem curve generation
│   │   └── assembly.rs        # Combine flowers along branches
│   │
│   ├── floraison-export/      # glTF generation
│   │   └── gltf.rs
│   │
│   └── floraison-wasm/        # WASM bindings
│       └── lib.rs
│
└── frontend/                  # SvelteKit application
    ├── src/
    │   ├── lib/
    │   │   ├── wasm/          # WASM loader
    │   │   ├── three/         # Three.js scene setup
    │   │   └── components/    # Svelte UI components
    │   └── routes/
    └── static/
```

## Core Systems

### 1. Mathematical Foundations

#### B-Spline Surfaces

Petals and sepals are represented as B-spline surfaces, which provide smooth, controllable freeform shapes.

**Surface definition**:
```
S(u, v) = Σᵢ Σⱼ Pᵢⱼ · Nᵢ(u) · Nⱼ(v)
```

Where:
- `Pᵢⱼ` are control points in a 2D grid
- `Nᵢ(u)`, `Nⱼ(v)` are B-spline basis functions
- `u, v ∈ [0, 1]` are surface parameters

**Implementation approach**:
1. Generate control point grid from outline parameters (length, width, shape)
2. Evaluate surface at regular (u,v) samples
3. Apply deformations (curl, twist, ruffle) to evaluated points
4. Triangulate the regular grid into mesh

**Deformations**:
- *Curl*: Rotate points around axis perpendicular to petal plane
- *Twist*: Rotate around central vein with falloff
- *Ruffle*: Add noise/sinusoidal waves to surface normals

#### Phyllotaxis (Spiral Arrangements)

Natural spiral patterns use Fibonacci angles:

```
angle_n = (F_n / F_{n+2}) × 360°
```

Common values: 180°, 120°, 144°, 137.5° (golden angle)

Used for:
- Rotating successive stamens/petals around receptacle
- Arranging seeds on disc (sunflower head)
- Branch rotation angles in inflorescences

#### Surface of Revolution

Receptacles are generated by rotating a 2D profile curve around a vertical axis.

**Parameterization**:
- Profile curve defined by parameters: `(height, base_radius, bulge_factor, top_radius)`
- Generate curve points as Bézier or spline
- Revolve around Y-axis at N angular steps
- Generate triangle mesh from grid

#### 3D Curve Reconstruction

For stem/axis generation, convert 2D stroke-like curves to natural 3D curves.

**Constant curvature approach** (from paper):
```
(d²x/dy²)² + (d²z/dy²)² = constant
```

Given 2D input with (x, y) values:
1. Compute second derivatives of x
2. Determine constant from max curvature
3. Solve for second derivatives of z
4. Integrate twice to get z values

Result: Input sine wave → 3D spiral with consistent curvature

### 2. Floral Diagram System

#### Coordinate System

**2D Diagram Space**: Polar coordinates (r, θ) in floral diagram editor
- r: Radial distance from center (which ring: pistil < stamen < petal < sepal)
- θ: Angular position around center

**3D Receptacle Space**: Cylindrical coordinates (R, φ, h) on receptacle surface
- R: Radius at height h (determined by receptacle profile)
- φ: Same as θ from 2D
- h: Height on receptacle (mapped from r)

**Mapping algorithm**:
1. Component at (r, θ) in diagram
2. Map r → h using radial zone boundaries
3. θ → φ directly (preserving angular position)
4. Lookup R from receptacle profile at height h
5. Convert (R, φ, h) to Cartesian (x, y, z)

#### Layout Algorithms

**Radial Symmetry**:
- For n components in ring: `θᵢ = i × (360°/n) + offset`
- Offset can be 0 or Fibonacci angle for natural look

**Indefinite Filling** (Ranunculus-style):
- Fill region with maximum possible components
- Use Vogel's method for disc packing:
  ```
  r = √i × scale
  θ = i × 137.5°
  ```

#### Assembly Process

1. Instantiate receptacle mesh
2. For each component in diagram:
   - Instantiate component mesh
   - Map diagram position to 3D position
   - Apply rotation to orient component (normal = radial direction)
   - Apply scale if specified
   - Merge into final mesh
3. Weld vertices at attachment points (optional)

### 3. Component Geometry

#### Receptacle

**Parameters**:
```rust
struct ReceptacleParams {
    height: f32,
    base_radius: f32,
    bulge_height: f32,  // where maximum radius occurs (0-1)
    bulge_radius: f32,  // maximum radius
    top_radius: f32,
}
```

**Generation**:
1. Create profile points using cubic Bézier through control points
2. Sample N points along curve
3. Revolve around Y-axis with M angular steps
4. Generate N×M vertex grid, triangulate

#### Pistil

**Parameters**:
```rust
struct PistilParams {
    axis_length: f32,
    axis_curve: Vec<Vec3>,  // or parameterized curve
    base_radius: f32,
    tip_radius: f32,
    style_length: f32,      // thin part at top
    stigma_radius: f32,     // bulb at very top
}
```

**Generation**:
1. Generate or sample axis curve
2. At each point, get radius (interpolated base→tip)
3. Sweep circle perpendicular to curve tangent
4. Cap ends
5. Add stigma sphere at top

#### Stamen

**Parameters**:
```rust
struct StamenParams {
    filament_length: f32,
    filament_curve: Vec<Vec3>,
    filament_radius: f32,
    anther_length: f32,
    anther_width: f32,
    anther_offset: f32,  // how far from tip
}
```

**Generation**:
1. Filament: Sweep circle along axis (like pistil)
2. Anther: Start with ellipsoid
3. Deform ellipsoid along anther axis curve
4. Attach to filament at specified offset

#### Petal/Sepal

**Parameters**:
```rust
struct PetalParams {
    length: f32,
    width: f32,
    tip_width: f32,       // taper at tip
    base_width: f32,      // narrow at base
    curl_amount: f32,     // -1 to 1
    twist_angle: f32,     // degrees
    ruffle_freq: f32,     // wave frequency
    ruffle_amp: f32,      // wave amplitude
    thickness: f32,       // if creating volume
}
```

**Generation** (most complex):
1. Generate control point grid (typically 4×8 or similar)
   - Outer points define outline
   - Interior points control curvature
2. Evaluate B-spline surface at high resolution (e.g., 32×64)
3. Apply deformations:
   - Curl: `z = curl_amount × (1 - u) × sin(v × π)`
   - Twist: Rotate around central vein, intensity proportional to u
   - Ruffle: Add `amp × sin(v × freq × 2π)` to edge points
4. Compute normals (cross product of tangent vectors)
5. Generate UVs (use u,v parameters directly)
6. Triangulate grid
7. Optionally create back faces or add thickness

#### Vertex Colors

All components generate per-vertex colors that flow through the entire pipeline:

**Rust → WASM Pipeline**:
```rust
// In floraison-components
pub struct Mesh {
    pub positions: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub uvs: Vec<Vec2>,
    pub colors: Vec<Vec3>,  // Per-vertex RGB colors
    pub indices: Vec<u32>,
}

// In floraison-wasm
#[wasm_bindgen]
pub struct MeshData {
    positions: Vec<f32>,
    normals: Vec<f32>,
    uvs: Vec<f32>,
    colors: Vec<f32>,  // Flattened: [r,g,b, r,g,b, ...]
    indices: Vec<u32>,
}

impl MeshData {
    pub fn from_mesh(mesh: &Mesh) -> Self {
        // Flatten Vec<Vec3> → Vec<f32> with stride 3
        let colors: Vec<f32> = mesh.colors
            .iter()
            .flat_map(|v| [v.x, v.y, v.z])
            .collect();

        Self { positions, normals, uvs, colors, indices }
    }
}

#[wasm_bindgen]
impl MeshData {
    pub fn colors(&self) -> js_sys::Float32Array {
        js_sys::Float32Array::from(&self.colors[..])
    }
}
```

**WASM → Three.js Pipeline**:
```typescript
// mesh-converter.ts
export function wasmMeshToGeometry(meshData: MeshData): THREE.BufferGeometry {
    const geometry = new THREE.BufferGeometry();

    const colors = meshData.colors();  // Float32Array
    geometry.setAttribute('color', new THREE.BufferAttribute(colors, 3));

    // ... positions, normals, uvs, indices
    return geometry;
}

// ThreeViewer.svelte
const material = new THREE.MeshPhysicalMaterial({
    vertexColors: true,  // Enable per-vertex colors
    metalness: 0.0,
    roughness: 0.6,
    // ... other PBR properties
});
```

**Color Assignment**:
- Each component has a `color: [f32; 3]` parameter (RGB, 0-1 range)
- Colors are assigned to all vertices of that component
- Renderer interpolates colors across triangles
- Preserved through glTF export

### 4. Inflorescence System

#### Pattern Types (8 implemented)

**Indeterminate**:
- **Raceme**: Single axis, flowers on pedicels, blooming bottom-up
- **Spike**: Like raceme but flowers sessile (no pedicels)
- **Umbel**: All pedicels from single point (like umbrella)
- **Corymb**: Pedicels of different lengths, flat-topped appearance

**Determinate**:
- **Dichasium**: Two branches from each node, forming Y-shapes
- **Drepanium**: Single branch per node, spiraling one direction

**Compound**:
- **Compound Raceme**: Raceme where each flower is replaced by sub-raceme
- **Compound Umbel**: Umbel where each ray ends in sub-umbel

#### Pattern Data Structures

```rust
struct InflorescencePattern {
    pattern_type: PatternType,
    axis_params: AxisParams,
    branch_params: BranchParams,
    flower_params: FlowerPlacementParams,
}

struct AxisParams {
    length: f32,
    curve: Curve3D,
    segments: usize,
}

struct BranchParams {
    angle_top: f32,       // down angle at top
    angle_bottom: f32,    // down angle at bottom
    length_top: f32,
    length_bottom: f32,
    rotation_angle: f32,  // Fibonacci angle (137.5° typically)
}

struct FlowerPlacementParams {
    count: usize,
    size_top: f32,
    size_bottom: f32,
    age_interpolation: AgeMode,  // Indeterminate/Determinate
}
```

#### Assembly Algorithm

1. **Generate main axis** as 3D curve
2. **Determine branch points** along axis (evenly spaced or parameterized)
3. **For each branch point**:
   - Calculate branch direction (down angle + rotation angle)
   - Interpolate length, flower size based on position
   - For compound patterns: recursively generate sub-pattern
   - Place flower at branch terminal
4. **Generate stem geometry**:
   - Main axis as swept cylinder with varying radius
   - Branch stems similarly
5. **Merge all meshes**: Flowers + stems into single structure

#### Flower Aging

Represent developmental stages with multiple flower models:
- Bud (closed)
- Partial bloom
- Full bloom
- Wilting (optional)

Interpolation schemes:
- *Indeterminate*: Bottom = oldest, top = youngest
- *Determinate*: Top/center = oldest, bottom/outer = youngest

Linear interpolate age parameter, select between models or blend parameters.

### 5. Export System

#### glTF Structure

```
Scene
└── Node (root)
    ├── Node (flower_1)
    │   └── Mesh (flower_geometry)
    ├── Node (flower_2)
    │   └── Mesh (flower_geometry)
    └── Node (stem)
        └── Mesh (stem_geometry)
```

Each Mesh contains:
- **Primitives**: Array of geometry chunks
  - **Attributes**: POSITION, NORMAL, TEXCOORD_0
  - **Indices**: Triangle list
  - **Material**: Color, roughness, metallic

#### Buffer Layout

glTF uses binary buffers with typed accessors:

```
Buffer (binary blob)
├── BufferView (vertices: bytes 0-N)
├── BufferView (normals: bytes N-M)
├── BufferView (UVs: bytes M-P)
└── BufferView (indices: bytes P-Q)

Accessors define interpretation:
├── Accessor (POSITION: VEC3, FLOAT)
├── Accessor (NORMAL: VEC3, FLOAT)
├── Accessor (TEXCOORD_0: VEC2, FLOAT)
└── Accessor (indices: SCALAR, UNSIGNED_SHORT)
```

#### Export Pipeline

**Implementation**: Three.js GLTFExporter (not Rust gltf-json)

**Decision Rationale**:
- Original roadmap planned Rust gltf-json implementation (6-8 hours)
- Switched to Three.js GLTFExporter (~1.5 hours actual)
- Benefits: Battle-tested, standards-compliant, automatic material conversion
- Exports exactly what's rendered (WYSIWYG)

**Implementation**:
```typescript
// src/lib/three/exporter.ts
import { GLTFExporter } from 'three/examples/jsm/exporters/GLTFExporter.js';

export function exportToGLB(object: THREE.Object3D, options: ExportOptions = {}): void {
    const exporter = new GLTFExporter();
    const filename = options.filename || generateFilename();

    exporter.parse(
        object,
        (gltf) => {
            // gltf is ArrayBuffer for binary mode
            const blob = new Blob([gltf as ArrayBuffer], {
                type: 'application/octet-stream'
            });
            const url = URL.createObjectURL(blob);

            // Trigger download
            const link = document.createElement('a');
            link.href = url;
            link.download = filename;
            document.body.appendChild(link);
            link.click();
            document.body.removeChild(link);
            URL.revokeObjectURL(url);

            options.onSuccess?.();
        },
        (error) => {
            const errorObj = error instanceof Error
                ? error
                : new Error(String(error));
            options.onError?.(errorObj);
        },
        {
            binary: true,           // Export as .glb (binary)
            embedImages: true,      // Embed textures
            truncateDrawRange: true // Optimize buffers
        }
    );
}

export function generateFilename(presetName?: string): string {
    const timestamp = new Date().toISOString().slice(0, 19).replace(/:/g, '-');
    const name = presetName && presetName !== 'custom'
        ? `floraison_${presetName}_${timestamp}`
        : `floraison_custom_${timestamp}`;
    return `${name}.glb`;
}
```

**Export Process**:
1. User clicks "Export GLB" in ViewerControls
2. ThreeViewer calls `exportToGLB(flowerMesh, { filename })`
3. GLTFExporter converts Three.js scene → glTF 2.0
4. Automatic material conversion:
   - MeshPhysicalMaterial → PBR Metallic-Roughness
   - Vertex colors preserved in COLOR_0 attribute
   - All PBR properties included
5. Binary GLB file downloads
6. Compatible with Blender, Unity, Unreal, Windows 3D Viewer

### 6. WASM Integration

#### Rust → JavaScript Bridge

```rust
#[wasm_bindgen]
pub struct FlowerGenerator {
    // Internal state
}

#[wasm_bindgen]
impl FlowerGenerator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> FlowerGenerator { ... }

    pub fn generate_flower(&mut self, params: JsValue) -> FlowerMesh { ... }

    pub fn generate_inflorescence(&mut self, params: JsValue) -> InfMesh { ... }

    pub fn export_gltf(&self) -> Vec<u8> { ... }
}

#[wasm_bindgen]
pub struct FlowerMesh {
    positions: Vec<f32>,
    normals: Vec<f32>,
    uvs: Vec<f32>,
    indices: Vec<u16>,
}

#[wasm_bindgen]
impl FlowerMesh {
    pub fn positions(&self) -> js_sys::Float32Array { ... }
    pub fn normals(&self) -> js_sys::Float32Array { ... }
    pub fn uvs(&self) -> js_sys::Float32Array { ... }
    pub fn indices(&self) -> js_sys::Uint16Array { ... }
}
```

#### JavaScript Side

```typescript
import init, { FlowerGenerator } from './floraison_wasm';

await init();
const generator = new FlowerGenerator();

const params = {
  diagram: { pistils: 1, stamens: 6, petals: 6, sepals: 0 },
  petal: { length: 5, width: 2, curl: 0.3 },
  // ...
};

const mesh = generator.generate_flower(params);

const geometry = new THREE.BufferGeometry();
geometry.setAttribute('position', new THREE.BufferAttribute(mesh.positions(), 3));
geometry.setAttribute('normal', new THREE.BufferAttribute(mesh.normals(), 3));
geometry.setAttribute('uv', new THREE.BufferAttribute(mesh.uvs(), 2));
geometry.setIndex(new THREE.BufferAttribute(mesh.indices(), 1));

const material = new THREE.MeshStandardMaterial({ color: 0xffcc00 });
const flower = new THREE.Mesh(geometry, material);
scene.add(flower);
```

## Frontend Architecture

### SvelteKit Structure

```
src/
├── lib/
│   ├── wasm/
│   │   └── loader.ts          # Initialize WASM module
│   ├── three/
│   │   ├── scene.ts           # Three.js scene setup
│   │   ├── materials.ts       # Material presets
│   │   └── controls.ts        # Camera orbit controls
│   ├── stores/
│   │   ├── parameters.ts      # Svelte stores for flower params
│   │   └── scene.ts           # Three.js scene state
│   └── components/
│       ├── ui/
│       │   ├── ParameterPanel.svelte
│       │   ├── DiagramEditor.svelte
│       │   ├── InfEditor.svelte
│       │   └── ExportDialog.svelte
│       └── viewer/
│           └── ThreeViewer.svelte
└── routes/
    └── +page.svelte           # Main app
```

### Parameter Flow

```
User adjusts slider
    ↓
Svelte store updates
    ↓
Store subscriber triggers regeneration
    ↓
Call WASM generator with new params
    ↓
Receive new mesh buffers
    ↓
Update Three.js BufferGeometry
    ↓
Scene re-renders
```

### Performance Considerations

**Debouncing**: Don't regenerate on every slider tick
- Use `setTimeout` to wait for user to stop adjusting
- Or only regenerate on mouse up

**Web Workers** (future optimization):
- Run WASM in worker to avoid blocking UI
- Post message with params, receive mesh buffers

**Progressive Loading**:
- Show low-poly preview while high-poly generates
- Or start with single flower, then add inflorescence

### Lighting & Rendering

**Implementation**: Professional PBR rendering with film-quality lighting

**Shadow System**:
```typescript
// VSM (Variance Shadow Maps) - softest, most realistic
renderer.shadowMap.enabled = true;
renderer.shadowMap.type = THREE.VSMShadowMap;

// Directional light with ultra-high resolution shadows
directionalLight.shadow.mapSize.width = 4096;
directionalLight.shadow.mapSize.height = 4096;
directionalLight.shadow.radius = 3;         // Wider penumbra
directionalLight.shadow.blurSamples = 25;   // Smoothness
```

**Tone Mapping**:
```typescript
// ACES Filmic (industry standard for film/games)
renderer.toneMapping = THREE.ACESFilmicToneMapping;
renderer.toneMappingExposure = 1.0;  // User-controllable
renderer.outputColorSpace = THREE.SRGBColorSpace;
```

**Lighting Setup** (Three-Point Lighting):
```typescript
// 1. Hemisphere Light (natural ambient)
const hemisphereLight = new THREE.HemisphereLight(
    0x87ceeb,  // Sky color (light blue)
    0x8b7355,  // Ground color (brownish earth)
    0.6        // Intensity
);

// 2. Directional Light (key light)
const directionalLight = new THREE.DirectionalLight(0xffffff, 1.2);
directionalLight.position.set(5, 10, 5);
directionalLight.castShadow = true;

// 3. Fill Light (softens shadows)
const fillLight = new THREE.DirectionalLight(0xffffff, 0.4);
fillLight.position.set(-5, 5, -5);
```

**Material (PBR)**:
```typescript
const material = new THREE.MeshPhysicalMaterial({
    vertexColors: true,    // Use per-vertex colors
    metalness: 0.0,        // Non-metallic (organic)
    roughness: 0.6,        // Slightly rough surface
    transmission: 0.0,     // Opaque (not glass)
    thickness: 0.5,        // Sub-surface scattering depth
    ior: 1.4,              // Index of refraction (plant material)
    sheen: 0.5,            // Soft fabric-like sheen
    clearcoat: 0.3         // Subtle glossy layer
});
```

**User Controls**:
- Exposure (0.5-2.0)
- Ambient intensity (0-2)
- Directional intensity (0-3)
- Light colors (hex pickers for sky, ground, directional)
- Shadow toggle (auto-disabled on mobile)

**Ground Plane**:
- Dynamic positioning based on mesh bounding box
- Always contacts lowest vertex: `ground.position.y = minY - 0.1`

### Mobile Support

**Responsive Design**:
```typescript
// Tailwind breakpoint: 768px (md:)
<aside class="parameter-panel md:static fixed">
  <!-- Desktop: static sidebar -->
  <!-- Mobile: fixed drawer -->
</aside>
```

**Drawer Pattern**:
- Hamburger menu button (48×48px touch target, fixed top-left)
- Slide-in animation (transform: translateX(-100%) → 0)
- Backdrop overlay (rgba(0,0,0,0.5), tap to close)
- Escape key support
- Smooth 0.3s transitions

**Performance Optimization**:
```typescript
// Automatic shadow optimization
function isMobileDevice(): boolean {
    return /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i
        .test(navigator.userAgent);
}

const defaultSettings: ViewerSettings = {
    enableShadows: !isMobileDevice()  // Auto-disable on mobile
};
```

**Touch Controls**:
- OrbitControls built-in touch support:
  - One finger drag: Rotate
  - Two finger pinch: Zoom
  - Two finger drag: Pan

**Mobile Layout**:
```
┌──────────────────────┐
│ [☰]    Viewer   [⋮]  │ ← Hamburger + Controls
│                      │
│                      │
│   3D Viewer          │
│   (Full width)       │
│                      │
│                      │
└──────────────────────┘

When hamburger clicked:
┌──────────────────────┐
│ Parameters │ Viewer  │ ← Drawer slides in
│ [Presets]  │ (behind │
│ [Diagram]  │ dark    │
│ [Petals]   │ overlay)│
│ ...        │         │
└──────────────────────┘
```

**Tested On**:
- Android: Xiaomi Redmi
- Touch gestures verified
- Performance acceptable with shadows disabled

## Testing Strategy

### Unit Tests (Rust)

- Math functions (B-spline evaluation, phyllotaxis)
- Curve reconstruction algorithm
- Mesh utilities (normal calculation, welding)
- Each component generator in isolation

### Integration Tests (Rust)

- Full flower generation from parameters
- Inflorescence assembly
- glTF export (validate structure)

### Visual Tests (Frontend)

- Snapshot testing with known good parameters
- Compare rendered output to reference images
- Manual QA with various parameter combinations

### Performance Tests

- Benchmark mesh generation time
- Profile WASM memory usage
- Measure frame rate with complex inflorescences

## Challenges & Solutions

### Challenge 1: B-Spline Implementation Complexity

**Problem**: B-spline surfaces require Cox-de Boor recursion, knot vectors, basis function evaluation—mathematically complex.

**Solution**:
- Start with uniform cubic B-splines (degree 3, evenly spaced knots)
- Use Bézier patches as subset of B-splines (simpler)
- Precompute basis functions for regular grids (cache)

### Challenge 2: Parametrizing Organic Shapes

**Problem**: Original paper uses sketch input, which is intuitive but hard to parametrize.

**Solution**:
- Study real flowers to identify key shape parameters
- Use hierarchical parameters (outline → deformation)
- Provide presets for common flower types
- Allow advanced users to export/share parameter sets

### Challenge 3: Natural Randomness

**Problem**: Perfectly symmetrical flowers look artificial.

**Solution**:
- Add small random variation to positions, angles, sizes
- Use Perlin/simplex noise for organic deformations
- Provide "randomness amount" parameter
- Use seeded random for reproducibility

### Challenge 4: WASM Binary Size

**Problem**: WASM bundle can get large, affecting load time.

**Solution**:
- Compile with `--release` and `opt-level = 'z'` (size optimization)
- Use `wasm-opt` tool for further size reduction
- Tree-shake unused code
- Lazy-load WASM module (only when needed)

### Challenge 5: Mesh Complexity vs. Performance

**Problem**: High tessellation creates beautiful meshes but may be slow.

**Solution**:
- Adaptive tessellation (more detail where curvature is high)
- Level-of-detail parameter
- Preview mode (low-poly) vs. export mode (high-poly)
- Profile and optimize hottest code paths

## Future Enhancements

### Near-term
- Texture generation (procedural, not just solid colors)
- More inflorescence patterns (22 exist in literature)
- Leaf geometry (not covered in paper)
- Animation (blooming, growth, wind sway)

### Long-term
- Entire plant structures (stem, leaves, multiple inflorescences)
- Seasonal variations (spring buds → fall wilting)
- Environmental responses (phototropism, gravity)
- Genetic algorithms for flower breeding
- ML-assisted parameter suggestion from photos

## Performance Targets

**Actual Implementation Performance**:

- **Generation time** (achieved):
  - Single flower: <300ms (includes debounce)
  - Simple inflorescence: <500ms
  - Complex inflorescence: <1000ms
  - Warning displayed if generation >1000ms
- **WASM binary size**: Optimized with wasm-pack `--release`
- **Preview frame rate**:
  - Desktop: 60 FPS with shadows enabled
  - Mobile: 60 FPS with shadows disabled (auto-detected)
- **Mesh complexity**:
  - Single flower: 5K-20K triangles (resolution parameter: 12-24)
  - Inflorescence: 50K-300K triangles (branch count: 5-30)
  - Allium Umbel (30 flowers): Highest complexity preset

**Platform-Specific Optimizations**:
- Mobile devices: Shadows auto-disabled on page load
- High-DPI displays: No performance degradation
- Cross-browser: Tested on Chrome, Firefox, Edge, Safari

## Conclusion

Floraison demonstrates the power of domain-specific procedural generation. By encoding botanical knowledge into the system architecture, we achieve both **ease of use** (high-level parameters) and **flexibility** (wide variety of flowers) while maintaining **botanical accuracy**.

The separation of structure (floral diagrams, inflorescence patterns) from geometry (B-spline surfaces, procedural shapes) is the key architectural principle enabling this flexibility.

The Rust → WASM → Three.js pipeline provides high performance and broad accessibility, making sophisticated computational geometry available in the browser without plugins or installations.
