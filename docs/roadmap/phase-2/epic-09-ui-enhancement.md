# Epic 9: UI Enhancement

**Phase**: 2 - Single Flower Refinement

**Goal**: Improve user experience with better layout, controls, and visual feedback.

**Estimated Effort**: 4-6 hours

---

## Task 9.1: Split View Layout

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

## Task 9.2: Viewer Controls UI

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

## Task 9.3: Loading States and Error Handling

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

## Task 9.4: Performance Optimization

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

## Phase 2 Completion Checkpoint

**Deliverable**: Full-featured single flower generator with advanced B-spline petals, comprehensive parameter UI, and presets.

**Testing**:
- [ ] All presets load correctly
- [ ] Adjusting parameters updates flower in real-time
- [ ] B-spline petals show smooth curves and deformations
- [ ] No performance issues during parameter adjustment
- [ ] UI is intuitive and responsive
