# Epic 4: Single Flower Assembly

**Phase**: 1 - Foundation

**Goal**: Assemble individual components into a complete flower using basic floral diagram.

**Estimated Effort**: 6-8 hours

---

## Task 4.1: Floral Diagram Data Structure ✅

**Description**: Define data structures for floral diagram (component counts and layout).

**Status**: ✅ COMPLETED

**Acceptance Criteria**:
- [x] Module `floraison-components/src/diagram.rs` created
- [x] Enum for arrangement patterns:
  ```rust
  pub enum ArrangementPattern {
      EvenlySpaced,
      GoldenSpiral,
      CustomOffset(f32),
  }
  ```
- [x] Struct for component whorl:
  ```rust
  pub struct ComponentWhorl {
      pub count: usize,
      pub radius: f32,
      pub height: f32,
      pub pattern: ArrangementPattern,
      pub rotation_offset: f32,
  }
  ```
- [x] Struct for floral diagram:
  ```rust
  pub struct FloralDiagram {
      pub receptacle_height: f32,
      pub receptacle_radius: f32,
      pub petal_whorls: Vec<ComponentWhorl>,
      pub stamen_whorls: Vec<ComponentWhorl>,
      pub pistil_whorls: Vec<ComponentWhorl>,
      pub sepal_whorls: Vec<ComponentWhorl>,
  }
  ```
- [x] Method `calculate_angles()` on ComponentWhorl computes angular positions
- [x] Four preset diagrams: lily(), five_petal(), daisy(), four_petal()
- [x] 9 unit tests + 2 doc-tests

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

## Task 4.2: Component Instance Mapping

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

## Task 4.3: Flower Assembly Function

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
