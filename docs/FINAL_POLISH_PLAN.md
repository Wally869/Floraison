# Final Polish Plan - Floraison

**Comprehensive review for final development pass before Made with Claude submission**

---

## Progress Tracker

**Last Updated**: 2025-10-02

### ✅ Completed (Phase 1 - Quick Fixes)
- **Lily Stamen Tilt**: Changed from 90° (horizontal) to 60° (natural droop)
- **Ground Plane**: Increased from 30×30 to 60×60 units
- **Shadow Camera**: Expanded frustum from 20×20 to 40×40 coverage
- **Astilbe Preset**: Enhanced with 10 branches (was 6), larger flowers (0.7-0.8 vs 0.5-0.6), taller axis (14 vs 10)
- **TypeScript Check**: 0 errors, 0 warnings

### ✅ Completed (Phase 2 - High-Priority Features)
- **Random Generation Button**: One-click randomization with smart constraints
  - Purple "🎲 Random" button next to Reset
  - Pastel color palette for pleasant aesthetics
  - Smart parameter constraints for natural proportions
  - 50% chance for inflorescence patterns
  - HSL→RGB color utilities
- **Jitter/Natural Variation**: Seeded randomness for organic component placement
  - Position jitter (0-0.5): Random radial offset
  - Angle jitter (0-15°): Random rotation variation
  - Size jitter (0-0.3): Random scale variation (±30%)
  - Deterministic seeded RNG (reproducible with same seed)
  - UI controls in "Natural Variation" subsection
  - Integrated into all 11 presets and random generation
- **Bend/Droop Parameters**: Curved pistils and stamens for organic shapes
  - Pistil bend (0-1): Arc amount for curved style
  - Pistil droop (-1 to 1): Vertical tilt control
  - Stamen bend (0-1): Arc amount for curved filament
  - Stamen droop (-1 to 1): Vertical tilt control
  - Automatic curve generation via generateBendCurve()
  - Random generation includes 20-30% chance of curves
- **Age Distribution Controls**: Control flower maturity across inflorescences
  - Age distribution slider (0-1): 0=all buds, 0.5=natural gradient, 1=all blooms
  - apply_age_distribution() function in Rust
  - Integrated into all 8 inflorescence patterns (simple + compound)
  - UI slider in inflorescence section
  - All presets default to 0.5 (natural gradient)

### ✅ Completed (Phase 3 - Optional Polish)
- **Petal Lateral Curvature**: Side-to-side bending for petals
  - Added `lateral_curve` parameter (-1=left, 0=straight, 1=right)
  - Implemented `apply_lateral_curve()` with XY plane rotation
  - Integrated into Rust petal generation
  - UI slider in petal controls
  - Updated all 11 presets (default 0.0)
  - Random generation includes 20% chance of lateral curve
- **UI Reorganization (Partial)**: Natural Variation moved to Advanced section
  - Extracted jitter parameters into collapsed "Advanced - Natural Variation" section
  - Cleaner "Flower Structure" section with just counts and tilt
  - Improved accessibility of common controls

### 🚧 In Progress
- None currently

### 📋 Remaining
- Phase 4: Performance & UX Improvements (8-11 hours)
  - Web Worker for non-blocking generation (4-6h)
  - Axis/Branch curvature controls (2-3h)
  - Optimize scene refresh to avoid full recreation (1-2h)
- Phase 5: Final Testing (2-3 hours)

**Total Time Invested**: 8.0 hours (30min + 2h + 4h + 15min + 1.25h Phase 3)
**Remaining Estimate**: 10-14 hours (8-11h improvements + 2-3h testing)

---

## Issues Identified

### 1. **Random Generation Button** ⭐ ✅ COMPLETE
**Status**: ✅ Implemented
**Priority**: HIGH
**Description**: Add a "Randomize" button that generates random parameter values within reasonable bounds.

**Implementation**: ✅ COMPLETE
- ✅ Purple "🎲 Random" button in ParameterPanel header (next to Reset button)
- ✅ Smart constraint-based randomization with natural ranges
- ✅ Pastel color generation using HSL→RGB conversion (0.4-0.8 saturation, 0.6-0.9 lightness)
- ✅ Green colors for receptacles, yellow for pistils/stamens
- ✅ Natural parameter relationships (tip_radius based on base_radius, etc.)
- ✅ Golden angle and natural rotation angles (137.5°, 120°, 144°, 180°)
- ✅ 50% chance for inflorescence patterns
- ✅ 30% chance for petal ruffle
- ✅ New utility files: `lib/utils/random.ts` and `lib/utils/colors.ts`

**Time Spent**: 2 hours

---

### 2. **Bending/Curve Parameters for Components** ⭐⭐ ✅ COMPLETE
**Status**: ✅ Fully Implemented
**Priority**: HIGH
**Description**: Add curve/bend controls for pistil and stamen

**Implementation**: ✅ COMPLETE
- ✅ UI sliders for pistil_bend (0-1) and pistil_droop (-1 to 1)
- ✅ UI sliders for stamen_bend (0-1) and stamen_droop (-1 to 1)
- ✅ TypeScript generateBendCurve() converts bend+droop to control points
- ✅ Rust applies curves during pistil/stamen generation
- ✅ All 11 presets include bend/droop defaults (mostly 0.0 for straight)
- ✅ Random generation includes 20-30% chance of curves
- ✅ Prevents mesh pinching with proper curve handling

**Time Spent**: Already complete (from previous work)

---

### 3. **Lily Stamen Tilt Default** ⭐
**Status**: Incorrect default value
**Priority**: MEDIUM
**Description**: Lily preset has stamen `tilt_angle: 1.5708` (π/2 = 90°, fully horizontal). This looks unnatural.

**Current**:
```typescript
stamen_whorls: [{
    tilt_angle: 1.5708  // π/2 - horizontal spreading
}]
```

**Fix**:
Change to 0.785 (π/4 = 45°) or 1.047 (π/3 = 60°) for more natural drooping

```typescript
stamen_whorls: [{
    tilt_angle: 1.047  // π/3 - 60° moderate spread
}]
```

**Estimated Effort**: 5 minutes

---

### 4. **Ground Plane / Shadow Cutoff** ⭐
**Status**: Ground plane too small for shadow camera
**Priority**: MEDIUM
**Description**: Ground plane is 30×30 units, but shadow camera frustum is only 20×20 (-10 to 10). Shadows cut off awkwardly at edges.

**Current State** (`scene.ts`):
```typescript
const groundGeometry = new THREE.PlaneGeometry(30, 30);

dirLight.shadow.camera.left = -10;
dirLight.shadow.camera.right = 10;
dirLight.shadow.camera.top = 10;
dirLight.shadow.camera.bottom = -10;
```

**Fix Options**:
1. **Increase ground plane**: 50×50 or 80×80
2. **Increase shadow camera frustum**: -20 to 20 (40×40 coverage)
3. **Make shadow camera match ground size**: -15 to 15 for 30×30 ground

**Recommended**:
```typescript
const groundGeometry = new THREE.PlaneGeometry(60, 60);  // Larger ground

dirLight.shadow.camera.left = -20;
dirLight.shadow.camera.right = 20;
dirLight.shadow.camera.top = 20;
dirLight.shadow.camera.bottom = -20;
```

**Estimated Effort**: 5 minutes

---

### 5. **Age Controls (Flower Aging)** ⭐⭐ ✅ COMPLETE
**Status**: ✅ Fully Implemented (Option B)
**Priority**: MEDIUM-HIGH
**Description**: User control for flower maturity distribution in inflorescences

**Implementation**: ✅ COMPLETE
- ✅ TypeScript `age_distribution` parameter (0-1) in InflorescenceParams
- ✅ UI slider in inflorescence section (line 360, 368 of ParameterPanel.svelte)
- ✅ Rust `apply_age_distribution()` function in lib.rs (lines 247-260)
  - 0.0 = All buds (age ≈ 0.15)
  - 0.5 = Natural age gradient (default)
  - 1.0 = All blooms (age ≈ 0.55)
- ✅ Integrated into all 6 simple patterns (Raceme, Spike, Umbel, Corymb, Dichasium, Drepanium)
- ✅ Compound patterns inherit via delegation to simple patterns
- ✅ All 11 presets default to 0.5 (natural gradient)
- ✅ Random generation includes age_distribution
- ✅ Comprehensive unit tests

**Time Spent**: Already complete (from previous work)

---

### 6. **Astilbe Plume Preset Issue** ⭐
**Status**: Investigate
**Priority**: MEDIUM
**Description**: User reports Astilbe defaults to single flower instead of inflorescence

**Current Preset**:
```typescript
inflorescence: {
    enabled: true,  // Should show inflorescence
    pattern: 'CompoundRaceme',
    branch_count: 6,  // Only 6 flowers - might look sparse?
    recursion_depth: 2,
    // ...
}
```

**Possible Issues**:
1. Preset loading code looks correct
2. `branch_count: 6` might be too low for compound raceme (looks sparse)
3. Small individual flowers (`flower_size: 0.5-0.6`)

**Fix**:
- Increase `branch_count` from 6 to 10-12
- Increase `flower_size_top/bottom` from 0.5-0.6 to 0.7-0.9
- Increase `axis_length` from 10 to 12-15 for taller plume

**Estimated Effort**: 10 minutes

---

### 7. **Randomness/Jitter Parameters** ⭐ ✅ COMPLETE (From DEV_NOTES.md)
**Status**: ✅ Implemented
**Priority**: MEDIUM
**Description**: Add randomness sliders to reduce perfect symmetry

**Implementation**: ✅ COMPLETE
- ✅ Position jitter (0-0.5): Random radial offset from base position
- ✅ Angle jitter (0-15°): Random rotation variation in degrees
- ✅ Size jitter (0-0.3): Random scale variation (±30%)
- ✅ Jitter seed: Deterministic seeded RNG for reproducibility
- ✅ UI controls: "Natural Variation" subsection with 4 sliders + randomize seed button
- ✅ Rust implementation: `apply_jitter()` using `rand::SmallRng` with component-specific seeds
- ✅ Added `rand` dependency to `floraison-components` crate
- ✅ Updated all 11 presets with jitter defaults (disabled: 0.0)
- ✅ Random generation includes subtle jitter (40% position, 50% angle, 30% size chance)
- ✅ Performance: Early exit when jitter disabled, no overhead

**Time Spent**: 4 hours

---

### 8. **Petal/Sepal Curvature Controls** ⭐ ✅ COMPLETE (From DEV_NOTES.md)
**Status**: ✅ Implemented
**Priority**: LOW-MEDIUM
**Description**: Add bend/curve controls for petals and sepals (already have curl/twist, but not curvature)

**Current Controls**:
- Curl: Upward/downward arc along petal length ✅
- Twist: Rotation along central vein ✅
- Ruffle: Edge waviness ✅
- **Lateral Curvature**: Side-to-side bend ✅ **NEW**

**Implementation**: ✅ COMPLETE
- ✅ Added `lateral_curve` parameter (-1 to 1) to PetalParams
- ✅ Implemented `apply_lateral_curve()` function in petal.rs
  - Progressive XY plane rotation (v² curve factor)
  - Applied before B-spline surface evaluation
- ✅ UI slider in petal controls (-1=curve left, 1=curve right)
- ✅ Updated all 11 presets with lateral_curve: 0.0 (straight)
- ✅ Random generation includes 20% chance of lateral curve (-0.4 to 0.4)
- ✅ TypeScript interface and stores updated

**Time Spent**: 1.25 hours

---

### 9. **Web Worker for Generation** ⭐⭐ (From DEV_NOTES.md)
**Status**: Not implemented
**Priority**: HIGH (User requested)
**Description**: Move WASM generation to web worker to avoid main thread hangs

**Current Issues**:
- Generation runs on main thread, freezing UI for complex inflorescences
- No visual feedback during generation
- Poor UX for compound patterns with high branch counts
- Browser "Page Unresponsive" warnings on complex flowers

**Proposed Solution**:
1. **Web Worker Setup**:
   - Create dedicated worker thread for WASM execution
   - Load floraison WASM module in worker context
   - Implement message passing protocol for parameters and mesh data

2. **Loading Spinner**:
   - Display animated spinner during generation
   - Show in center of canvas or as overlay
   - Optional: Progress indicator for multi-step generation

3. **Async Generation Flow**:
   ```typescript
   // Main thread
   worker.postMessage({ type: 'generate', params: flowerParams });
   showSpinner();

   worker.onmessage = (e) => {
     hideSpinner();
     updateMesh(e.data.vertices, e.data.indices, e.data.normals);
   };
   ```

**Benefits**:
- ✅ Non-blocking UI during generation
- ✅ Smooth loading spinner animation
- ✅ Better perceived performance
- ✅ No browser warnings on complex flowers
- ✅ Maintains UI responsiveness for parameter changes

**Implementation Steps**:
1. Create `floraison-worker.ts` with WASM module initialization
2. Implement message passing protocol (generate, result, error)
3. Add transferable objects for mesh buffers (zero-copy transfer)
4. Create spinner component (CSS animation or Three.js sprite)
5. Update scene.ts to use worker instead of direct WASM calls
6. Handle worker errors and fallback to main thread
7. Add worker preloading on app startup

**Files to Modify**:
- `floraison-ui/src/lib/workers/floraison-worker.ts` - NEW: Worker implementation
- `floraison-ui/src/lib/components/ui/LoadingSpinner.svelte` - NEW: Spinner component
- `floraison-ui/src/lib/three/scene.ts` - Update to use worker
- `floraison-ui/src/routes/+page.svelte` - Initialize worker on mount

**Technical Considerations**:
- WASM module must support worker context (check `wasm-bindgen` compatibility)
- Use `Atomics` and `SharedArrayBuffer` for progress updates (optional)
- Ensure proper worker cleanup on page unload (avoid memory leaks)
- Debug tools: Chrome DevTools supports worker debugging

**Testing**:
- Test with simple flowers (should be instant, no spinner flash)
- Test with complex inflorescences (30+ branches, recursion depth 3)
- Verify no memory leaks (check with Chrome DevTools Memory profiler)
- Test rapid parameter changes (debounce or cancel pending generation)

**Estimated Effort**: 4-6 hours
- Worker setup and WASM integration: 2-3h
- Spinner component and UI integration: 1h
- Testing and edge cases: 1-2h

**Priority Justification**: User explicitly requested this feature for better UX

---

### 10. **UI Controls Reorganization** ⭐ ✅ PARTIAL
**Status**: ✅ Partially Complete
**Priority**: MEDIUM
**Description**: Review parameter panel organization for better UX

**Completed Improvements**:
- ✅ **Natural Variation → Advanced**: Moved jitter parameters into collapsed "Advanced - Natural Variation" section
  - Position, angle, and size jitter controls
  - Random seed with randomize button
  - Descriptive section help text
- ✅ **Cleaner Flower Structure**: Core counts (pistils, stamens, petals, sepals) and tilt now stand alone
- ✅ **Progressive Disclosure**: Advanced controls hidden by default, accessible when needed

**Remaining Options** (deferred to post-launch):
- Full multi-tier reorganization (Quick/Basic/Advanced hierarchy)
- Grouped color pickers section
- Tab-based interface
- Parameter search/filter

**Implemented**: Option A (partial) - Advanced parameters collapsed

**Time Spent**: 15 minutes

---

### 11. **Inflorescence Axis/Branch Curvature** ⭐⭐ (New Feature Request)
**Status**: Not implemented
**Priority**: HIGH (User requested)
**Description**: Add curvature controls for inflorescence main axis and branches to create more natural, organic forms

**Current Limitation**:
- Inflorescence axes are perfectly straight vertical lines
- Branches are perfectly straight radial lines
- Results in geometric, artificial appearance
- No way to create drooping, arching, or spiraling inflorescences

**Proposed Solution**:
Add curvature parameters to inflorescence generation:

1. **Main Axis Curvature**:
   - `axis_curve_amount` (0-1): Degree of curve along main axis
   - `axis_curve_direction`: Direction of curve (auto: gravity droop, or user-specified)
   - Apply progressive curve using bezier or arc interpolation
   - Example: Wisteria (drooping clusters), Delphinium (slight backward lean)

2. **Branch Curvature**:
   - `branch_curve_amount` (0-1): Degree of curve for individual branches
   - `branch_curve_type`: Uniform (all same) vs. Gradient (varies by position)
   - Apply curve to each branch from attachment point to flower
   - Example: Lilac (upward curving branches), Laburnum (drooping branches)

**Implementation Approach**:
Similar to pistil/stamen bend system (Issue #2), but applied to inflorescence structure:

```rust
// In InflorescenceParams
pub struct InflorescenceParams {
    // ... existing fields
    pub axis_curve_amount: f32,      // 0-1
    pub axis_curve_direction: Vec3,  // Direction vector (default: -Y for droop)
    pub branch_curve_amount: f32,    // 0-1
    pub branch_curve_mode: CurveMode, // Uniform, GradientUp, GradientDown
}

pub enum CurveMode {
    Uniform,       // All branches curve equally
    GradientUp,    // Top branches curve more
    GradientDown,  // Bottom branches curve more
}

// In pattern generation (e.g., raceme.rs)
fn apply_axis_curve(points: &mut Vec<Vec3>, params: &InflorescenceParams) {
    let n = points.len();
    for (i, point) in points.iter_mut().enumerate() {
        let t = i as f32 / (n - 1) as f32;  // 0 to 1 along axis
        let curve_factor = t * t;            // Quadratic for natural droop
        let offset = params.axis_curve_direction *
                     params.axis_curve_amount *
                     curve_factor *
                     params.axis_length;
        *point += offset;
    }
}

fn apply_branch_curve(
    branch_start: Vec3,
    branch_end: Vec3,
    position_on_axis: f32,  // 0-1
    params: &InflorescenceParams
) -> Vec3 {
    let curve_amount = match params.branch_curve_mode {
        CurveMode::Uniform => params.branch_curve_amount,
        CurveMode::GradientUp => params.branch_curve_amount * position_on_axis,
        CurveMode::GradientDown => params.branch_curve_amount * (1.0 - position_on_axis),
    };

    // Apply perpendicular curve (arc in horizontal plane)
    let branch_dir = (branch_end - branch_start).normalize();
    let up = Vec3::Y;
    let curve_dir = branch_dir.cross(up).normalize();

    // Quadratic bezier curve
    let control_point = branch_start +
                        (branch_end - branch_start) * 0.5 +
                        curve_dir * curve_amount;

    // Return curved endpoint (or full curve points if needed)
    control_point
}
```

**UI Controls**:
Add to Inflorescence section in ParameterPanel.svelte:

```svelte
<!-- Axis Curvature Subsection -->
<div class="subsection-header">Curvature</div>

<div class="param-group">
    <label for="axis-curve">
        <span class="param-label">Axis Curve</span>
        <span class="param-value">{$inflorescenceParams.axis_curve_amount.toFixed(2)}</span>
    </label>
    <input
        id="axis-curve"
        type="range"
        min="0"
        max="1"
        step="0.1"
        bind:value={$inflorescenceParams.axis_curve_amount}
        class="param-slider"
    />
    <p class="param-help">Main axis droop/curve (0=straight, 1=dramatic arc)</p>
</div>

<div class="param-group">
    <label for="branch-curve">
        <span class="param-label">Branch Curve</span>
        <span class="param-value">{$inflorescenceParams.branch_curve_amount.toFixed(2)}</span>
    </label>
    <input
        id="branch-curve"
        type="range"
        min="0"
        max="1"
        step="0.1"
        bind:value={$inflorescenceParams.branch_curve_amount}
        class="param-slider"
    />
    <p class="param-help">Branch curvature (0=straight, 1=arching)</p>
</div>

<div class="param-group">
    <label for="branch-curve-mode" class="param-label">Curve Distribution</label>
    <select id="branch-curve-mode" bind:value={$inflorescenceParams.branch_curve_mode} class="param-select">
        <option value="Uniform">Uniform (all equal)</option>
        <option value="GradientUp">Gradient Up (top curves more)</option>
        <option value="GradientDown">Gradient Down (bottom curves more)</option>
    </select>
</div>
```

**Files to Modify**:
- `floraison-ui/src/lib/stores/inflorescence.ts` - Add curve parameters
- `floraison-ui/src/lib/components/ui/ParameterPanel.svelte` - Add UI controls
- `floraison-inflorescence/src/lib.rs` - Add CurveMode enum and parameters
- `floraison-inflorescence/src/patterns/raceme.rs` - Implement axis curve
- `floraison-inflorescence/src/patterns/spike.rs` - Implement axis curve
- `floraison-inflorescence/src/patterns/umbel.rs` - Implement branch curve
- `floraison-inflorescence/src/patterns/corymb.rs` - Implement branch curve
- `floraison-inflorescence/src/patterns/dichasium.rs` - Implement recursive curve
- `floraison-inflorescence/src/patterns/drepanium.rs` - Implement spiral curve
- `floraison-inflorescence/src/patterns/compound_raceme.rs` - Inherit from raceme
- `floraison-inflorescence/src/patterns/compound_umbel.rs` - Inherit from umbel
- `floraison-ui/src/lib/utils/random.ts` - Add to random generation (30% chance)
- `floraison-ui/src/lib/presets.ts` - Update all inflorescence presets

**Benefits**:
- ✅ More realistic, natural-looking inflorescences
- ✅ Matches real botanical forms (drooping wisteria, arching lilac)
- ✅ Greater artistic control for users
- ✅ Complements existing bend/droop system for components

**Examples of Natural Curvature**:
- **Wisteria**: Heavy axis droop (0.7-0.9), slight branch droop (0.2-0.3)
- **Lilac**: Subtle axis lean (0.1-0.2), upward branch curve (-0.3)
- **Laburnum**: Strong axis droop (0.6), strong branch droop (0.5)
- **Delphinium**: Minimal axis curve (0.1), straight branches (0.0)

**Edge Cases to Handle**:
- Ensure curve doesn't flip axis upside down (clamp curve amount)
- Handle interaction with rotation_angle (curves should rotate with branches)
- Compound patterns: Apply curve recursively to sub-branches
- Performance: Curve calculation shouldn't significantly slow generation

**Testing**:
- Test each pattern type with various curve amounts (0, 0.5, 1.0)
- Verify natural appearance at moderate values (0.3-0.7)
- Test compound patterns with recursion
- Ensure random generation produces reasonable values

**Estimated Effort**: 2-3 hours
- Rust implementation (8 patterns): 1.5-2h
- TypeScript parameters and UI: 30min
- Testing and refinement: 30min

**Priority Justification**: User explicitly requested for more natural inflorescence forms

---

### 12. **Optimize Scene Refresh (Mesh-Only Update)** ⭐⭐ (Performance Issue)
**Status**: Not implemented
**Priority**: HIGH (User reported issue)
**Description**: Optimize mesh regeneration to only replace flower mesh instead of recreating entire scene

**Current Issue**:
- Every mesh regeneration recreates the entire Three.js scene
- Camera position resets or flickers
- Ground plane, lights, and environment are unnecessarily recreated
- Poor UX with visual flash/flicker
- Potential memory leaks from improper disposal

**Current Behavior** (probable):
```typescript
// In scene.ts or similar
function regenerateFlower(params: FlowerParams) {
    // ❌ PROBLEM: Recreating everything
    scene.clear();  // Removes everything
    createGroundPlane();
    createLights();
    createFlowerMesh(params);  // Only this needs to update
    createCamera();
}
```

**Proposed Solution**:
Maintain persistent scene objects and only update flower mesh:

```typescript
// In scene.ts
class FlowerScene {
    private scene: THREE.Scene;
    private camera: THREE.PerspectiveCamera;
    private renderer: THREE.WebGLRenderer;
    private controls: OrbitControls;
    private groundPlane: THREE.Mesh;
    private lights: THREE.Light[];
    private flowerMesh: THREE.Group | null = null;  // Track separately

    constructor(canvas: HTMLCanvasElement) {
        // Initialize scene once
        this.scene = new THREE.Scene();
        this.camera = new THREE.PerspectiveCamera(/* ... */);
        this.renderer = new THREE.WebGLRenderer({ canvas });
        this.controls = new OrbitControls(this.camera, canvas);

        // Create persistent objects
        this.createGroundPlane();
        this.createLights();
    }

    private createGroundPlane() {
        const geometry = new THREE.PlaneGeometry(60, 60);
        const material = new THREE.ShadowMaterial({ opacity: 0.2 });
        this.groundPlane = new THREE.Mesh(geometry, material);
        this.groundPlane.rotation.x = -Math.PI / 2;
        this.groundPlane.receiveShadow = true;
        this.scene.add(this.groundPlane);
    }

    private createLights() {
        const ambient = new THREE.AmbientLight(0xffffff, 0.6);
        const directional = new THREE.DirectionalLight(0xffffff, 0.8);
        // ... configure shadows, etc.

        this.lights = [ambient, directional];
        this.lights.forEach(light => this.scene.add(light));
    }

    // ✅ SOLUTION: Only update flower mesh
    updateFlowerMesh(vertices: Float32Array, indices: Uint32Array, normals: Float32Array) {
        // Remove old mesh
        if (this.flowerMesh) {
            this.scene.remove(this.flowerMesh);

            // Properly dispose geometry and materials to prevent memory leaks
            this.flowerMesh.traverse((child) => {
                if (child instanceof THREE.Mesh) {
                    child.geometry.dispose();
                    if (Array.isArray(child.material)) {
                        child.material.forEach(m => m.dispose());
                    } else {
                        child.material.dispose();
                    }
                }
            });

            this.flowerMesh = null;
        }

        // Create new mesh
        const geometry = new THREE.BufferGeometry();
        geometry.setAttribute('position', new THREE.BufferAttribute(vertices, 3));
        geometry.setAttribute('normal', new THREE.BufferAttribute(normals, 3));
        geometry.setIndex(new THREE.BufferAttribute(indices, 1));

        const material = new THREE.MeshPhongMaterial({ /* ... */ });
        const mesh = new THREE.Mesh(geometry, material);
        mesh.castShadow = true;
        mesh.receiveShadow = true;

        this.flowerMesh = new THREE.Group();
        this.flowerMesh.add(mesh);
        this.scene.add(this.flowerMesh);
    }

    // Camera and controls remain untouched
    render() {
        this.renderer.render(this.scene, this.camera);
    }

    dispose() {
        // Full cleanup when component unmounts
        this.scene.clear();
        this.renderer.dispose();
        this.controls.dispose();
        // ... dispose all resources
    }
}
```

**Benefits**:
- ✅ No visual flash/flicker on regeneration
- ✅ Camera position and zoom level maintained
- ✅ Orbit controls state preserved (rotation, pan)
- ✅ Better performance (less object creation)
- ✅ Proper memory management (explicit disposal)
- ✅ Smoother UX for parameter adjustments

**Implementation Steps**:
1. Refactor scene.ts into class-based structure (or similar pattern)
2. Separate initialization (once) from update (per regeneration)
3. Track flower mesh reference separately from scene
4. Implement proper disposal for old meshes (geometry + materials)
5. Ensure shadow maps update correctly with new mesh
6. Test camera/controls persistence across updates

**Files to Modify**:
- `floraison-ui/src/lib/three/scene.ts` - Major refactor to separate init/update
- `floraison-ui/src/routes/+page.svelte` - Update to use new scene API

**Potential Architecture**:
```typescript
// +page.svelte
let flowerScene: FlowerScene;

onMount(() => {
    flowerScene = new FlowerScene(canvas);
    flowerScene.initialize();  // Once: ground, lights, camera
});

$effect(() => {
    // Watch for parameter changes
    const meshData = generateFlower($allParams);  // WASM call
    flowerScene.updateFlowerMesh(meshData);  // Only update mesh
});

onDestroy(() => {
    flowerScene.dispose();  // Clean up everything
});
```

**Edge Cases to Handle**:
- Initial load (no previous mesh to dispose)
- Rapid parameter changes (debounce or cancel pending updates)
- WebGL context loss (recovery mechanism)
- Memory leaks (verify with Chrome DevTools heap snapshots)

**Testing**:
- Change parameters rapidly and verify no flicker
- Check camera position stays constant after regeneration
- Use Chrome DevTools Memory profiler to verify no leaks
- Test with complex meshes (30+ branches)
- Verify shadows update correctly

**Estimated Effort**: 1-2 hours
- Refactor scene.ts structure: 45min
- Implement mesh-only update: 30min
- Testing and memory leak verification: 15-30min

**Priority Justification**: User reported as annoying UX issue, quick win for perceived quality

---

## Summary Table

| Issue | Priority | Effort | Status |
|-------|----------|--------|--------|
| 1. Random Generation Button | HIGH | 2-3h | ✅ **COMPLETE** (Smart constraints) |
| 2. Bend/Curve Parameters | HIGH | 3-4h | ✅ **COMPLETE** (Bend+Droop) |
| 3. Lily Stamen Tilt | MEDIUM | 5min | ✅ **FIXED** (90°→60°) |
| 4. Ground/Shadow Fix | MEDIUM | 5min | ✅ **FIXED** (60×60, 40×40) |
| 5. Age Controls | MEDIUM-HIGH | 1-2h | ✅ **COMPLETE** (Option B slider) |
| 6. Astilbe Preset | MEDIUM | 10min | ✅ **FIXED** (10 branches, larger) |
| 7. Randomness/Jitter | MEDIUM | 3-4h | ✅ **COMPLETE** (Seeded RNG) |
| 8. Petal Curvature | LOW-MEDIUM | 1.25h | ✅ **COMPLETE** (Lateral curve) |
| 9. Web Worker + Spinner | **HIGH** ⭐⭐ | 4-6h | **PRIORITIZED** (User requested) |
| 10. UI Reorganization | MEDIUM | 15min | ✅ **PARTIAL** (Jitter→Advanced) |
| 11. Axis/Branch Curvature | **HIGH** ⭐⭐ | 2-3h | **NEW** (User requested) |
| 12. Optimize Scene Refresh | **HIGH** ⭐⭐ | 1-2h | **NEW** (User reported issue) |

**Total Estimated Effort**: ~~14-20 hours~~ → **18-22 hours** (with new features)
**Completed**: 8.0 hours (Issues #1-8, #10 partial)
**Remaining**:
- Phase 4: Issues #9, #11, #12 (7-11 hours)
- Phase 5: Final testing (2-3 hours)
- **Total Remaining**: 9-14 hours

---

## Recommended Implementation Order

### Phase 1: Quick Fixes ✅ COMPLETE (30 minutes)
1. ✅ Fix lily stamen tilt default (5min) - Changed from 90° to 60°
2. ✅ Fix ground plane / shadow camera (5min) - Ground 30×30→60×60, Shadow 20×20→40×40
3. ✅ Adjust Astilbe preset values (10min) - 10 branches, larger flowers, taller axis
4. ✅ TypeScript check passed (0 errors, 0 warnings)

### Phase 2: High-Priority Features ✅ COMPLETE
1. ✅ Random generation button (2-3h) - COMPLETE with smart constraints
2. ✅ Randomness/jitter parameters (3-4h) - COMPLETE with seeded RNG
3. ✅ Bend/curve parameters (3-4h) - COMPLETE with bend+droop sliders
4. ✅ Age distribution slider (1-2h) - COMPLETE with all 8 patterns

### Phase 3: Optional Polish & UX ✅ COMPLETE
1. ✅ Petal lateral curvature (1.25h) - COMPLETE with slider and random generation
2. ✅ UI reorganization (15min) - PARTIAL: Natural Variation moved to Advanced section

### Phase 4: Performance & UX Improvements (7-11 hours) - USER REQUESTED
1. **Web Worker for Generation** (4-6h) - Issue #9
   - Move WASM to dedicated worker thread
   - Implement loading spinner component
   - Non-blocking UI during generation
   - Proper error handling and fallback

2. **Inflorescence Axis/Branch Curvature** (2-3h) - Issue #11
   - Add axis_curve_amount and branch_curve_amount parameters
   - Implement curve application in all 8 patterns
   - UI controls in inflorescence section
   - Update presets and random generation

3. **Optimize Scene Refresh** (1-2h) - Issue #12
   - Refactor scene.ts to class-based structure
   - Separate scene initialization from mesh updates
   - Only replace flower mesh on regeneration
   - Proper disposal to prevent memory leaks
   - Preserve camera position and controls

### Phase 5: Final Testing (2-3 hours)
1. Test all 11 presets (with new features)
2. Test all 8 inflorescence patterns (with curvature)
3. Test edge cases (extreme parameters, rapid changes)
4. Verify web worker performance (no freezing)
5. Verify scene refresh optimization (no flicker)
6. Cross-browser testing (Chrome, Firefox, Safari)
7. Mobile testing (Xiaomi Redmi)
8. Performance profiling (memory leaks, frame rate)

---

## Post-Launch Enhancements (Future Work)
- Full age system with bud/wilt meshes (4-5h)
- More randomness options (color variation, per-component jitter, etc.)
- Leaf geometry and stem structure
- Animation (blooming sequence, wind sway, growth)
- Procedural texture generation (petal veins, spots, gradients)
- Export to other formats (OBJ, STL for 3D printing)
- Preset sharing (URL encoding, community gallery)
- Advanced lighting controls (HDR environment maps)

---

## Technical Notes

### Random Number Generation
Need wasm-compatible RNG:
```toml
# Cargo.toml
[dependencies]
rand = { version = "0.8", features = ["wasm-bindgen"] }
```

```rust
use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;

let mut rng = SmallRng::seed_from_u64(seed);
let jitter = rng.gen_range(-0.5..0.5);
```

### Curve/Bend Implementation
For pistil/stamen bending:
```rust
pub struct PistilParams {
    // ... existing fields
    bend_angle: f32,  // 0-90 degrees
    bend_direction: Vec3,  // Direction vector for bend
}

// In generation:
let bend_offset = bend_angle.to_radians() * (t * t) * bend_direction;
curve_point = base_point + bend_offset;
```

### Age Distribution Slider
```typescript
// UI parameter
ageDistribution: number;  // 0-1

// In Rust:
let adjusted_age = base_age * age_distribution;
let mesh = aging.select_mesh(adjusted_age);
```

### Web Worker Architecture
```typescript
// floraison-worker.ts
import init, { generate_flower } from '$lib/wasm/floraison';

let wasmInitialized = false;

self.onmessage = async (e) => {
    if (e.data.type === 'init') {
        await init();
        wasmInitialized = true;
        self.postMessage({ type: 'ready' });
    } else if (e.data.type === 'generate') {
        if (!wasmInitialized) {
            self.postMessage({ type: 'error', error: 'WASM not initialized' });
            return;
        }

        try {
            const result = generate_flower(e.data.params);
            // Transfer buffers for zero-copy
            self.postMessage(
                { type: 'result', data: result },
                [result.vertices.buffer, result.indices.buffer, result.normals.buffer]
            );
        } catch (error) {
            self.postMessage({ type: 'error', error: error.message });
        }
    }
};
```

### Axis Curvature Implementation
```rust
// In inflorescence patterns
pub fn apply_axis_curve(
    axis_points: &mut Vec<Vec3>,
    curve_amount: f32,
    curve_direction: Vec3,
    axis_length: f32
) {
    let n = axis_points.len();
    for (i, point) in axis_points.iter_mut().enumerate() {
        let t = i as f32 / (n - 1) as f32;  // Normalized position (0-1)
        let curve_factor = t * t;            // Quadratic progression
        let offset = curve_direction * curve_amount * curve_factor * axis_length;
        *point += offset;
    }
}

// For branches: apply perpendicular curve
pub fn get_curved_branch_endpoint(
    start: Vec3,
    end: Vec3,
    curve_amount: f32,
    up_direction: Vec3
) -> Vec3 {
    let branch_dir = (end - start).normalize();
    let curve_dir = branch_dir.cross(up_direction).normalize();

    // Midpoint control point for quadratic bezier
    let midpoint = start + (end - start) * 0.5;
    let control = midpoint + curve_dir * curve_amount;

    // Return curved endpoint (simplified - full implementation would generate curve points)
    end + curve_dir * curve_amount * 0.5
}
```

### Scene Mesh-Only Update Pattern
```typescript
// Persistent scene objects
class FlowerScene {
    private flowerMesh: THREE.Group | null = null;

    updateFlowerMesh(meshData: MeshData) {
        // Dispose old mesh
        if (this.flowerMesh) {
            this.scene.remove(this.flowerMesh);
            this.disposeMesh(this.flowerMesh);
        }

        // Create new mesh
        this.flowerMesh = this.createMeshFromData(meshData);
        this.scene.add(this.flowerMesh);
    }

    private disposeMesh(mesh: THREE.Group) {
        mesh.traverse((child) => {
            if (child instanceof THREE.Mesh) {
                child.geometry.dispose();
                if (Array.isArray(child.material)) {
                    child.material.forEach(m => m.dispose());
                } else {
                    child.material.dispose();
                }
            }
        });
    }
}
```

---

## Open Questions

1. **Randomize Button Behavior**:
   - Randomize all parameters, or just current component?
   - Keep inflorescence setting or randomize that too?
   - Add "Lock" toggles for individual parameters?

2. **Bend Parameters**:
   - Simple angle + direction, or full curve control?
   - Auto-calculate direction (upward bend) or let user control?

3. **UI Reorganization**:
   - Collapsible sections (Option A) or tabs (Option B)?
   - Keep color pickers with components or group in "Colors" section?

4. **Age Controls**:
   - Simple distribution slider (Option B) or full system (Option A)?
   - Add to inflorescence only or all flowers?

---

## Files to Modify

### Quick Fixes ✅ COMPLETE
- ✅ `floraison-ui/src/lib/presets.ts` - Lily tilt (line 48), Astilbe values (lines 788-796)
- ✅ `floraison-ui/src/lib/three/scene.ts` - Ground plane (line 105), shadow camera (lines 89-92)

### Random Generation ✅ COMPLETE
- ✅ `floraison-ui/src/lib/components/ui/ParameterPanel.svelte` - Added button and randomizeParameters()
- ✅ `floraison-ui/src/lib/utils/random.ts` - NEW: Random generation logic with smart constraints
- ✅ `floraison-ui/src/lib/utils/colors.ts` - NEW: HSL→RGB conversion utilities

### Bend/Curve Parameters
- `floraison-core/src/components/pistil.rs` - Add bend logic
- `floraison-core/src/components/stamen.rs` - Add bend logic
- `floraison-ui/src/lib/stores/parameters.ts` - Add bend params
- `floraison-ui/src/lib/components/ui/ParameterPanel.svelte` - Add sliders
- `floraison-wasm/Cargo.toml` - Add rand dependency

### Age Distribution
- `floraison-ui/src/lib/stores/inflorescence.ts` - Add age_distribution param
- `floraison-ui/src/lib/components/ui/ParameterPanel.svelte` - Add slider
- `floraison-inflorescence/src/patterns/*.rs` - Use distribution in age calc

### UI Reorganization
- `floraison-ui/src/lib/components/ui/ParameterPanel.svelte` - Restructure sections

---

## Success Criteria

✅ All 11 presets load correctly and look good
✅ No visual glitches (shadow cutoff, floating ground, etc.)
✅ Random generation produces interesting, valid flowers
✅ Bend/curve parameters add realistic organic variation
✅ UI is intuitive and organized
✅ Performance acceptable on mobile (no stuttering)
✅ Cross-browser compatibility (Chrome, Firefox, Safari)
✅ Ready for Made with Claude submission

---

## Next Steps

1. **Discuss with user**: Confirm priorities and approach for open questions
2. **Create implementation todos**: Break down into granular tasks
3. **Start with Phase 1**: Quick fixes (30min)
4. **Iterate on Phase 2**: High-priority features
5. **Polish and test**: Phase 3 & 4
6. **Prepare demo content**: Task 15.3 (screenshots, video)
7. **Final submission**: Task 15.4 (testing, bug fixes)
