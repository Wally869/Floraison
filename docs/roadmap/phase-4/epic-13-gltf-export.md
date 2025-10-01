# Epic 13: glTF Export

**Phase**: 4 - Polish & Launch

**Goal**: Implement proper glTF 2.0 export with materials and hierarchy.

**Estimated Effort**: 6-8 hours

**Actual Effort**: ~1.5 hours

**Status**: ✅ Complete

**Implementation Notes**:
- Used Three.js GLTFExporter instead of Rust gltf-json crate
- Much simpler implementation - exports directly from Three.js scene
- Includes all PBR material properties automatically
- Exports vertex colors correctly
- Battle-tested, standards-compliant output

---

## Task 13.1: glTF Export Module

**Description**: Create export functionality using Three.js GLTFExporter.

**Acceptance Criteria**:
- [x] Module `src/lib/three/exporter.ts` created
- [x] Function `exportToGLB()` exports Three.js objects to binary glTF
- [x] Function `generateFilename()` creates timestamped filenames
- [x] Automatic download trigger with proper cleanup
- [x] Success/error callback support
- [x] Valid glTF 2.0 / GLB output

**Dependencies**: Task 2.2

**Technical Notes**:
```typescript
import { GLTFExporter } from 'three/examples/jsm/exporters/GLTFExporter.js';

export function exportToGLB(object: THREE.Object3D, options: ExportOptions): void {
    const exporter = new GLTFExporter();
    exporter.parse(object, (gltf) => {
        const blob = new Blob([gltf], { type: 'application/octet-stream' });
        // ... trigger download
    }, { binary: true });
}
```

**Effort**: 30 minutes

---

## Task 13.2: Material Support

**Description**: Export PBR materials with vertex colors.

**Acceptance Criteria**:
- [x] Exports MeshPhysicalMaterial with all PBR properties
- [x] Includes metalness, roughness, transmission, thickness, IOR
- [x] Includes sheen and clearcoat properties
- [x] Vertex colors exported correctly
- [x] Materials render correctly in external viewers

**Dependencies**: Task 13.1

**Technical Notes**:
Three.js GLTFExporter automatically handles material export:
- Converts MeshPhysicalMaterial → glTF PBR Metallic-Roughness
- Exports all supported PBR extensions
- Preserves vertex color attributes
- No manual material conversion needed

**Effort**: Automatic (included in Task 13.1)

---

## Task 13.3: Export Button in UI

**Description**: Add export functionality to frontend.

**Acceptance Criteria**:
- [x] Export button in ViewerControls UI
- [x] Click button exports current flower mesh
- [x] Downloads .glb file (binary glTF)
- [x] Filename includes preset name and timestamp
- [x] Export state feedback (shows "Exporting...")
- [x] Success/error handling

**Dependencies**: Task 13.1, 13.2

**Technical Notes**:
```typescript
// ThreeViewer.svelte
function handleExport() {
    const filename = generateFilename($currentPresetName);
    exportToGLB(flowerMesh, {
        filename,
        onSuccess: () => console.log('Export successful!'),
        onError: (error) => console.error('Export failed:', error)
    });
}
```

**Files Modified**:
- `src/lib/three/exporter.ts` - Export module
- `src/lib/stores/parameters.ts` - Added currentPresetName store
- `src/lib/components/viewer/ThreeViewer.svelte` - Added handleExport
- `src/lib/components/viewer/ViewerControls.svelte` - Added export button
- `src/lib/components/ui/ParameterPanel.svelte` - Track preset changes

**Effort**: 1 hour

---

## Summary

**Epic 13 completed in ~1.5 hours** vs estimated 6-8 hours by using Three.js GLTFExporter instead of building Rust gltf-json implementation. Exported files:
- Work correctly in Windows 3D Viewer
- Work in Blender, Unity, Unreal Engine
- Include full PBR materials with vertex colors
- Have descriptive filenames (e.g., `floraison_lily_2025-10-01T18-30-45.glb`)

---
