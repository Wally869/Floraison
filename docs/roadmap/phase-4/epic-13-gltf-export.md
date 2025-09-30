# Epic 13: glTF Export

**Phase**: 4 - Polish & Launch

**Goal**: Implement proper glTF 2.0 export with materials and hierarchy.

**Estimated Effort**: 6-8 hours

---

## Task 13.1: glTF Scene Graph Builder

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

## Task 13.2: Material Support

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

## Task 13.3: Export Button in UI

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
