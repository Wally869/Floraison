# Epic 5: Frontend Foundation

**Phase**: 1 - Foundation

**Goal**: Create minimal UI to display generated flower in Three.js viewer.

**Estimated Effort**: 8-10 hours

---

## Task 5.1: WASM Bindings for Flower Generation

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

## Task 5.2: Three.js Scene Setup

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

## Task 5.3: WASM Loader Module

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

## Task 5.4: Mesh Converter (WASM → Three.js)

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

## Task 5.5: Basic ThreeViewer Component

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

## Task 5.6: Main App Page with Default Flower

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

## Epic 5 Completion Checkpoint

**Deliverable**: A working web application that displays a single procedurally-generated lily-like flower in a 3D viewer.

**Testing**:
- [ ] Run `npm run dev` in frontend directory
- [ ] Navigate to http://localhost:5173
- [ ] Flower renders correctly
- [ ] Can orbit camera around flower
- [ ] No console errors
- [ ] Mesh looks reasonable (no holes, proper shading)
