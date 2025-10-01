# Epic 8: Floral Diagram System

**Phase**: 2 - Single Flower Refinement

**Goal**: Create interactive UI for defining floral diagrams (not just hardcoded).

**Estimated Effort**: 8-10 hours

---

## Task 8.1: Diagram Parameter Store

**Description**: Create Svelte store for floral diagram parameters with reactivity.

**Acceptance Criteria**:
- [x] Store module `frontend/src/lib/stores/parameters.ts` created
- [x] Writable store for `FloralDiagram`
- [x] Writable stores for each component type parameters
- [x] Derived store that combines all parameters into JSON for WASM
- [x] Default values loaded on init
- [x] TypeScript types match Rust structs

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

## Task 8.2: Parameter Panel Component

**Description**: Create UI panel with sliders/inputs for all flower parameters.

**Acceptance Criteria**:
- [x] Component `frontend/src/lib/components/ui/ParameterPanel.svelte` created
- [x] Organized into sections (Diagram, Receptacle, Pistil, Stamen, Petal)
- [x] Each parameter has:
  - Label
  - Input (slider for 0-1 values, number input for counts)
  - Current value display
- [x] Binds to parameter stores
- [x] Styled with TailwindCSS
- [x] Collapsible sections (optional)
- [x] Responsive layout

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

## Task 8.3: Regenerate on Parameter Change

**Description**: Connect parameter store to flower regeneration with debouncing.

**Acceptance Criteria**:
- [x] Update main page to subscribe to `allParams` store
- [x] Debounce regeneration (300-500ms delay after last change)
- [x] Show "Generating..." indicator during regeneration
- [x] Handle errors gracefully
- [x] Camera maintains position during updates (doesn't reset)

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

## Task 8.4: Preset Flowers

**Description**: Create preset parameter sets for common flower types.

**Acceptance Criteria**:
- [x] Module `frontend/src/lib/presets.ts` with exported presets
- [x] At least 5 presets:
  - Lily (default)
  - Rose (many petals, layered)
  - Daisy (simple, flat petals)
  - Tulip (6 petals, cup shape)
  - Orchid (complex petal shapes)
- [x] Dropdown in UI to select preset
- [x] Selecting preset updates all parameter stores
- [x] "Custom" option when user modifies preset

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
