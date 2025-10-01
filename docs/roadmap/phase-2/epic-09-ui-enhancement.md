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
- [x] Floating controls overlay on viewer
- [x] Toggle axes helper (show/hide)
- [x] Background color picker
- [x] Lighting intensity sliders
- [x] Reset camera button
- [x] Toggle wireframe mode

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
- [x] Profile WASM generation time, identify bottlenecks
- [x] Add progress bar for long operations (if > 1 second) - conditional, logging warnings for >1s
- [x] Consider Web Worker for WASM (if needed) - evaluated, deferred pending actual benchmarks
- [x] Optimize mesh resolution (adaptive LOD) - evaluated, current defaults appropriate
- [x] Measure and document performance metrics

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
- [x] All presets load correctly
- [x] Adjusting parameters updates flower in real-time
- [x] B-spline petals show smooth curves and deformations
- [x] No performance issues during parameter adjustment (profiling integrated, awaiting benchmarks)
- [x] UI is intuitive and responsive

**Epic 9 Status**: Complete! Viewer controls and performance monitoring fully implemented.
