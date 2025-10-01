# Floraison User Guide

Welcome to Floraison, a procedural 3D flower generator based on botanical principles.

---

## Quick Start

1. **Run the application**
   ```bash
   npm run dev
   ```
   Open `http://localhost:5174` in your browser

2. **Select a preset**
   - Open the parameter panel (click hamburger menu on mobile)
   - Choose from the preset dropdown (e.g., "Lily", "Rose", "Daisy")
   - The flower generates automatically

3. **Adjust parameters**
   - Expand sections: Floral Diagram, Receptacle, Pistil, Stamen, Petal
   - Move sliders to change values
   - Changes regenerate the flower after 300ms debounce

4. **Export your model**
   - Click the three-dots button (top right)
   - Click "Export GLB"
   - File downloads as `floraison_[preset]_[timestamp].glb`
   - Import into Blender, Unity, Unreal, etc.

---

## Interface Overview

### Parameter Panel (Left Side / Mobile Drawer)

**Preset Dropdown**
- Select from 11 pre-made flowers
- "Custom" appears when you modify a preset

**Inflorescence Toggle**
- Enable for multi-flower clusters (raceme, umbel, etc.)
- Disable for single flowers

**Floral Diagram Section**
- Controls the structural arrangement of flower parts
- Whorls: Rings of components (pistils, stamens, petals, sepals)
- Each whorl has:
  - **Count**: Number of components
  - **Radius**: Distance from center (0-2)
  - **Height**: Vertical position on receptacle (0-1)
  - **Pattern**: EvenlySpaced or GoldenSpiral
  - **Rotation Offset**: Angular offset in radians
  - **Tilt Angle**: Outward spread angle in radians

**Component Sections**
- **Receptacle**: Base structure parameters
- **Pistil**: Central reproductive structure
- **Stamen**: Filament + anther (male parts)
- **Petal**: Main colorful parts
- **Sepal**: Outer protective parts (usually green)

**Inflorescence Section** (when enabled)
- **Pattern**: Raceme, Spike, Umbel, Corymb, Dichasium, Drepanium, CompoundRaceme, CompoundUmbel
- **Axis Length**: Main stem height
- **Branch Count**: Number of flowers
- **Angles**: Branch drooping angles (top/bottom)
- **Branch Lengths**: Pedicel lengths (top/bottom)
- **Rotation Angle**: Angular spacing (137.5° = golden angle)
- **Flower Size**: Size variation (top/bottom)
- **Recursion Depth**: Nesting level (recursive patterns only)
- **Branch Ratio**: Sub-branch scale (recursive patterns only)

### 3D Viewer (Right Side)

**Controls**
- **Rotate**: Click and drag (or touch and drag on mobile)
- **Zoom**: Scroll wheel (or pinch on mobile)
- **Pan**: Right-click drag (or two-finger drag on mobile)

**Indicators**
- Blue "Generating..." banner appears during regeneration
- Red error banner appears if generation fails

### Viewer Controls (Top Right)

Click the three-dots button to open settings panel:

- **Show Axes**: Toggle XYZ axis visualization
- **Wireframe**: Show mesh edges
- **Enable Shadows**: Toggle realistic shadows (auto-disabled on mobile)
- **Background**: Color picker for background
- **Exposure**: Tone mapping exposure (0.5-2.0)
- **Ambient Intensity**: Overall ambient light (0-2)
- **Sky Color**: Hemisphere light sky color
- **Ground Color**: Hemisphere light ground color
- **Directional Intensity**: Main light intensity (0-3)
- **Directional Color**: Main light color
- **Reset Camera**: Return to default view
- **Export GLB**: Download 3D model

---

## Parameter Reference

### Floral Diagram Parameters

**Receptacle**
- `Height`: Overall height (0.1-2.0)
- `Radius`: Base size (0.1-1.0)

**Whorls** (Pistil, Stamen, Petal, Sepal)
- `Count`: Number of components (1-50)
- `Radius`: Distance from center (0-2)
  - 0 = center (pistil)
  - 0.5 = inner ring (stamens)
  - 1.0 = outer ring (petals)
  - 1.5 = outermost (sepals)
- `Height`: Vertical position (0-1)
  - 0 = base of receptacle
  - 0.5 = middle
  - 1.0 = top
- `Pattern`:
  - **EvenlySpaced**: Uniform angular distribution
  - **GoldenSpiral**: Natural Fibonacci spiral (137.5°)
- `Rotation Offset`: Angular offset in radians (0-6.28)
- `Tilt Angle`: Outward/upward angle in radians (0-1.57)
  - 0 = vertical (pointing up)
  - 0.785 (π/4) = 45° spread
  - 1.57 (π/2) = horizontal

### Component Parameters

**Receptacle**
- `Height`: Overall height
- `Base Radius`: Radius at bottom
- `Bulge Radius`: Maximum radius
- `Top Radius`: Radius at top
- `Bulge Position`: Where bulge occurs (0-1)
- `Segments`: Angular resolution (8-32)
- `Profile Samples`: Vertical resolution (4-16)
- `Color`: RGB (0-1 range)

**Pistil**
- `Length`: Overall length
- `Base Radius`: Radius at base
- `Tip Radius`: Radius at tip
- `Stigma Radius`: Sphere at top
- `Segments`: Angular resolution
- `Color`: RGB

**Stamen**
- `Filament Length`: Thin stem length
- `Filament Radius`: Stem thickness
- `Anther Length`: Pollen sac length
- `Anther Width/Height`: Pollen sac dimensions
- `Segments`: Resolution
- `Color`: RGB

**Petal**
- `Length`: Overall length (1-5)
- `Width`: Overall width (0.5-3)
- `Tip Sharpness`: How pointed the tip is (0-1)
- `Base Width`: Width at attachment point (0.2-1)
- `Curl`: Upward/downward curve (-1 to 1)
  - Negative = cup shape (tulip)
  - Positive = backward curl (lily)
- `Twist`: Rotation along length (0-60 degrees)
- `Ruffle Freq`: Wave frequency for edges (0-5)
- `Ruffle Amp`: Wave amplitude (0-0.5)
- `Resolution`: Mesh detail (8-32)
- `Color`: RGB

**Sepal**
- Same parameters as Petal
- Typically smaller and green

### Inflorescence Parameters

**Pattern Types**
- **Raceme**: Stalked flowers along vertical axis (e.g., lily)
- **Spike**: Sessile flowers along axis (e.g., lavender)
- **Umbel**: Flowers radiating from single point (e.g., cherry, allium)
- **Corymb**: Flat-topped cluster with varied pedicel lengths (e.g., hydrangea)
- **Dichasium**: Y-shaped branching (recursive)
- **Drepanium**: Spiral branching (recursive)
- **CompoundRaceme**: Raceme of racemes (e.g., astilbe)
- **CompoundUmbel**: Umbel of umbels

**Common Parameters**
- `Axis Length`: Main stem height (5-20)
- `Branch Count`: Number of flowers (5-30)
- `Angle Top/Bottom`: Branch drooping angle (0-90°)
- `Branch Length Top/Bottom`: Pedicel length variation (0-3)
- `Rotation Angle`: Angular spacing between branches
  - 137.5° = golden angle (natural)
  - 360°/count = even spacing
- `Flower Size Top/Bottom`: Size scaling (0.3-1.0)

**Recursive Parameters** (Dichasium, Drepanium, CompoundRaceme, CompoundUmbel)
- `Recursion Depth`: Nesting levels (1-3)
- `Branch Ratio`: Scale factor for sub-branches (0.5-0.8)
- `Angle Divergence`: Branching angle variation

---

## Preset Gallery

### Single Flowers

**Lily**
- 6 elegant petals with gentle curl
- White color
- Horizontal spreading stamens
- Classic symmetrical structure

**Rose**
- 24 layered petals
- Ruffled edges
- Red color
- Dense, compact structure
- 5 sepals

**Daisy**
- 21 ray petals in golden spiral
- White petals
- Flat disc-like receptacle
- Many small stamens in center

**Tulip**
- 6 cup-shaped petals
- Pink-red color
- Negative curl creates cup
- Simple, elegant form

**Orchid**
- 5 dramatic petals
- Heavy twist (35°)
- Purple-pink color
- Ruffled edges
- Exotic appearance

### Inflorescences

**Lily Raceme**
- 8 lily flowers along vertical stem
- Golden spiral arrangement
- Stalked flowers (pedicels)
- 12 units tall

**Lavender Spike**
- 24 small purple flowers
- Dense, compact spike
- Sessile flowers (no pedicels)
- Tight spiral pattern
- 15 units tall

**Cherry Umbel**
- 5 pale pink blossoms
- Umbrella-like cluster
- Equal pedicel lengths
- Radiates from central point

**Hydrangea Corymb**
- 15 small blue flowers
- Flat-topped appearance
- Varied pedicel lengths
- Golden spiral arrangement

**Astilbe Plume**
- Compound raceme (raceme of racemes)
- Feathery appearance
- Pink flowers
- Recursion depth = 2
- 10 units tall

**Allium Umbel**
- 30 small purple flowers
- Dense spherical cluster
- Equal radiating pedicels
- Wide branch angles (70°)

---

## Export

### Exporting Models

1. Generate your desired flower or inflorescence
2. Adjust camera angle and lighting (optional)
3. Click three-dots button (top right)
4. Click "Export GLB" button
5. File downloads automatically

### Export Format

**File Format**: glTF 2.0 Binary (.glb)

**Filename**: `floraison_[preset]_[timestamp].glb`
- Example: `floraison_lily_2025-10-01T18-30-45.glb`

**Contents**:
- Geometry: positions, normals, UVs
- Vertex colors (per-vertex RGB)
- PBR materials (MeshPhysicalMaterial)
  - Metalness: 0.0
  - Roughness: 0.6
  - Transmission: 0.0
  - IOR: 1.4
  - Sheen: 0.5
  - Clearcoat: 0.3

**Compatible With**:
- Blender (File → Import → glTF)
- Unity (drag and drop)
- Unreal Engine (import FBX after conversion)
- Windows 3D Viewer
- Any glTF 2.0 compatible software

---

## Tips & Techniques

### Creating Realistic Flowers

1. **Start with a preset**
   - Find the closest match to your target
   - Modify from there rather than starting from scratch

2. **Use natural patterns**
   - GoldenSpiral pattern looks more organic than EvenlySpaced
   - Golden angle (137.5°) for inflorescence rotation

3. **Add organic variation**
   - Slight curl (0.3-0.5) adds realism
   - Small twist (5-15°) prevents artificial symmetry
   - Ruffle for delicate flowers (roses, orchids)

4. **Adjust tilt angles**
   - 0 = vertical (iris, tulip)
   - π/4 (0.785) = moderate spread (lily)
   - π/2 (1.57) = horizontal (daisy)

5. **Layer whorls**
   - Multiple petal whorls at different radii create depth
   - Vary colors slightly between layers

### Performance Optimization

**For Faster Generation**:
- Lower `Resolution` parameter (petals: 12-16 instead of 20-24)
- Reduce `Segments` on components (8-12 instead of 16)
- Fewer `Branch Count` in inflorescences (8-12 instead of 20-30)
- Lower `Recursion Depth` (1 instead of 2)

**For Mobile Devices**:
- Disable shadows (auto-disabled by default)
- Use simpler presets (Lily, Tulip)
- Avoid compound patterns (CompoundRaceme, CompoundUmbel)
- Reduce branch count (<15)

**Generation Time Targets**:
- Single flower: <300ms
- Simple inflorescence: <500ms
- Complex inflorescence: <1000ms
- Warning appears if >1000ms

### Color Customization

**RGB Format**: Values from 0.0 to 1.0

**Common Colors**:
- Pure white: `[1.0, 1.0, 1.0]`
- Pure red: `[1.0, 0.0, 0.0]`
- Pure green: `[0.0, 1.0, 0.0]`
- Pure blue: `[0.0, 0.0, 1.0]`
- Yellow: `[1.0, 1.0, 0.0]`
- Purple: `[0.7, 0.4, 0.85]`
- Pink: `[1.0, 0.8, 0.85]`
- Orange: `[1.0, 0.6, 0.2]`

**Natural Flower Colors**:
- Lily white: `[1.0, 1.0, 1.0]`
- Rose red: `[0.95, 0.2, 0.3]`
- Tulip pink: `[0.95, 0.3, 0.4]`
- Orchid purple: `[0.85, 0.4, 0.75]`
- Daisy white: `[1.0, 1.0, 1.0]`

**Greenery**:
- Light green receptacle: `[0.9, 0.95, 0.9]`
- Medium green: `[0.4, 0.65, 0.3]`
- Dark green sepal: `[0.3, 0.6, 0.3]`

**Stamens/Pistils**:
- Yellow-green: `[0.95, 0.9, 0.3]`
- Golden: `[0.95, 0.75, 0.2]`
- Dark brown: `[0.3, 0.2, 0.15]`

### Common Issues & Solutions

**Flower looks too small**
- ✅ Increase petal `Length` and `Width`
- ✅ Increase component `Count` in whorls

**Petals overlap or intersect**
- ✅ Increase petal whorl `Radius` (move outward)
- ✅ Decrease petal `Width`
- ✅ Increase `Rotation Offset` to stagger whorls

**Inflorescence looks flat/compressed**
- ✅ Increase `Axis Length`
- ✅ Increase `Branch Length`
- ✅ Adjust `Angle` parameters (try 45-70°)

**Generation is slow (>1s)**
- ⚠️ Reduce petal `Resolution`
- ⚠️ Reduce `Branch Count`
- ⚠️ Lower `Recursion Depth`
- ⚠️ Disable `Enable Shadows`

**Flower looks artificial/symmetric**
- ✅ Use `GoldenSpiral` pattern instead of `EvenlySpaced`
- ✅ Add slight `Curl` (0.2-0.5)
- ✅ Add slight `Twist` (5-15°)
- ✅ Use `Rotation Offset` to break symmetry

**Colors don't appear**
- ⚠️ Check if WASM loaded correctly (console)
- ⚠️ Rebuild WASM: `npm run build:wasm`
- ⚠️ Clear browser cache and reload

**Export fails**
- ⚠️ Wait for "Generating..." to finish
- ⚠️ Check browser console for errors
- ⚠️ Try exporting a preset first
- ⚠️ Ensure mesh was generated successfully

---

## Keyboard Shortcuts

- **Escape**: Close mobile panel (when open)

---

## Browser Compatibility

**Fully Supported**:
- Chrome 90+
- Firefox 88+
- Edge 90+
- Safari 14+

**Mobile**:
- iOS Safari 14+
- Android Chrome 90+

**Requirements**:
- WebAssembly support
- WebGL 2.0
- ES2020+ JavaScript

---

## Need Help?

- Check the [Technical Overview](TECHNICAL_OVERVIEW.md) for architecture details
- Review the [Roadmap](roadmap.md) for implementation progress
- Open an issue on GitHub for bugs or feature requests
