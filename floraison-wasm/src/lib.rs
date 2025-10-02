//! Floraison WASM
//!
//! WebAssembly bindings for the Floraison flower generator.
//! Exposes the Rust implementation to JavaScript/TypeScript.

use wasm_bindgen::prelude::*;
use floraison_components::assembly::{FlowerParams, generate_flower};
use floraison_core::geometry::mesh::Mesh;
use floraison_inflorescence::{InflorescenceParams, assembly, aging::FlowerAging};

/// Initialize the WASM module
/// Sets up panic hook for better error messages in the browser console
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

/// Flower generator exposed to JavaScript
#[wasm_bindgen]
pub struct FlowerGenerator {
    // Internal state will be added as we implement features
}

#[wasm_bindgen]
impl FlowerGenerator {
    /// Create a new flower generator
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {}
    }

    /// Generate a flower from JSON parameters
    /// Returns mesh data that can be used to create Three.js geometry
    pub fn generate_flower(&self, params_json: &str) -> Result<MeshData, JsValue> {
        // Parse JSON parameters
        let params: FlowerParams = serde_json::from_str(params_json)
            .map_err(|e| JsValue::from_str(&format!("Failed to parse parameters: {}", e)))?;

        // Generate flower mesh
        let mesh = generate_flower(&params);

        // Convert to WASM mesh data
        Ok(MeshData::from_mesh(&mesh))
    }

    /// Generate a lily flower with default parameters
    pub fn generate_lily(&self) -> Result<MeshData, JsValue> {
        let params = FlowerParams::lily();
        let mesh = generate_flower(&params);
        Ok(MeshData::from_mesh(&mesh))
    }

    /// Generate a five-petal flower with default parameters
    pub fn generate_five_petal(&self) -> Result<MeshData, JsValue> {
        let params = FlowerParams::five_petal();
        let mesh = generate_flower(&params);
        Ok(MeshData::from_mesh(&mesh))
    }

    /// Generate a daisy flower with default parameters
    pub fn generate_daisy(&self) -> Result<MeshData, JsValue> {
        let params = FlowerParams::daisy();
        let mesh = generate_flower(&params);
        Ok(MeshData::from_mesh(&mesh))
    }

    /// Generate an inflorescence (multi-flower structure) from JSON parameters
    ///
    /// # Arguments
    /// * `inflo_params_json` - JSON string containing InflorescenceParams
    /// * `flower_params_json` - JSON string containing FlowerParams for individual flowers
    ///
    /// # Returns
    /// Mesh data for the complete inflorescence structure
    pub fn generate_inflorescence(
        &self,
        inflo_params_json: &str,
        flower_params_json: &str,
    ) -> Result<MeshData, JsValue> {
        // Parse inflorescence parameters
        let inflo_params: InflorescenceParams = serde_json::from_str(inflo_params_json)
            .map_err(|e| JsValue::from_str(&format!("Failed to parse inflorescence parameters: {}", e)))?;

        // Parse flower parameters
        let flower_params: FlowerParams = serde_json::from_str(flower_params_json)
            .map_err(|e| JsValue::from_str(&format!("Failed to parse flower parameters: {}", e)))?;

        // Generate distinct meshes for each age stage
        let bloom_params = create_bloom_params(&flower_params);
        let bud_params = create_bud_params(&flower_params);
        let wilt_params = create_wilt_params(&flower_params);

        let bud_mesh = generate_flower(&bud_params);
        let bloom_mesh = generate_flower(&bloom_params);
        let wilt_mesh = generate_flower(&wilt_params);

        // Create aging struct with stage-specific meshes
        let aging = FlowerAging {
            bud_mesh,
            bloom_mesh,
            wilt_mesh: Some(wilt_mesh),
        };

        // Stem color (green)
        let stem_color = floraison_core::Vec3::new(0.3, 0.6, 0.3);

        // Generate inflorescence mesh
        let inflo_mesh = assembly::assemble_inflorescence_with_aging(
            &inflo_params,
            &aging,
            stem_color,
        );

        // Convert to WASM mesh data
        Ok(MeshData::from_mesh(&inflo_mesh))
    }
}

/// Create bud-stage flower parameters (closed, small, immature)
///
/// Modifies base parameters to create a flower in bud stage:
/// - Petals are smaller (50% length/width) and less curled
/// - Reproductive parts are shorter and smaller
/// - No ruffle or twist
fn create_bud_params(base: &FlowerParams) -> FlowerParams {
    let mut bud = base.clone();

    // Smaller, closed petals
    bud.petal.length *= 0.5;
    bud.petal.width *= 0.5;
    bud.petal.base_width *= 0.6;
    bud.petal.curl *= 0.2;  // Minimal curl (more closed)
    bud.petal.twist = 0.0;  // No twist in buds
    bud.petal.ruffle_freq = 0.0;  // No ruffle in buds
    bud.petal.ruffle_amp = 0.0;

    // Shorter reproductive parts
    bud.pistil.length *= 0.6;
    bud.pistil.stigma_radius *= 0.7;
    bud.stamen.filament_length *= 0.5;
    bud.stamen.anther_length *= 0.7;
    bud.stamen.anther_width *= 0.7;
    bud.stamen.anther_height *= 0.7;

    bud
}

/// Create bloom-stage flower parameters (full size, open, mature)
///
/// Returns the base parameters unchanged - this is the reference stage.
fn create_bloom_params(base: &FlowerParams) -> FlowerParams {
    base.clone()  // Bloom uses base params unchanged
}

/// Create wilt-stage flower parameters (drooping, faded, aging)
///
/// Modifies base parameters to create a flower in wilt stage:
/// - Petals droop more (increased curl)
/// - Slightly smaller and more twisted
/// - Colors darkened to simulate aging
fn create_wilt_params(base: &FlowerParams) -> FlowerParams {
    use floraison_core::Vec3;

    let mut wilt = base.clone();

    // Drooping petals (more downward curl)
    wilt.petal.length *= 0.9;  // Slightly smaller
    wilt.petal.curl += 0.3;    // More downward curl
    wilt.petal.twist *= 1.2;   // Slightly more twisted

    // Darkened color (aging/browning effect)
    wilt.petal.color = Vec3::new(
        wilt.petal.color.x * 0.8,
        wilt.petal.color.y * 0.8,
        wilt.petal.color.z * 0.8,
    );

    // Darken reproductive parts too
    wilt.pistil.color = Vec3::new(
        wilt.pistil.color.x * 0.8,
        wilt.pistil.color.y * 0.8,
        wilt.pistil.color.z * 0.8,
    );
    wilt.stamen.color = Vec3::new(
        wilt.stamen.color.x * 0.8,
        wilt.stamen.color.y * 0.8,
        wilt.stamen.color.z * 0.8,
    );

    wilt
}

/// Mesh data structure for passing to JavaScript
#[wasm_bindgen]
pub struct MeshData {
    positions: Vec<f32>,
    normals: Vec<f32>,
    uvs: Vec<f32>,
    colors: Vec<f32>,
    indices: Vec<u32>,
}

impl MeshData {
    /// Convert a Mesh to flat arrays for JavaScript
    pub fn from_mesh(mesh: &Mesh) -> Self {
        // Flatten Vec<Vec3> positions to Vec<f32> with stride 3
        let positions: Vec<f32> = mesh.positions
            .iter()
            .flat_map(|v| [v.x, v.y, v.z])
            .collect();

        // Flatten Vec<Vec3> normals to Vec<f32> with stride 3
        let normals: Vec<f32> = mesh.normals
            .iter()
            .flat_map(|v| [v.x, v.y, v.z])
            .collect();

        // Flatten Vec<Vec2> uvs to Vec<f32> with stride 2
        let uvs: Vec<f32> = mesh.uvs
            .iter()
            .flat_map(|v| [v.x, v.y])
            .collect();

        // Flatten Vec<Vec3> colors to Vec<f32> with stride 3
        let colors: Vec<f32> = mesh.colors
            .iter()
            .flat_map(|v| [v.x, v.y, v.z])
            .collect();

        // Copy indices directly
        let indices = mesh.indices.clone();

        Self {
            positions,
            normals,
            uvs,
            colors,
            indices,
        }
    }
}

#[wasm_bindgen]
impl MeshData {
    /// Get vertex positions as Float32Array
    pub fn positions(&self) -> js_sys::Float32Array {
        js_sys::Float32Array::from(&self.positions[..])
    }

    /// Get vertex normals as Float32Array
    pub fn normals(&self) -> js_sys::Float32Array {
        js_sys::Float32Array::from(&self.normals[..])
    }

    /// Get UV coordinates as Float32Array
    pub fn uvs(&self) -> js_sys::Float32Array {
        js_sys::Float32Array::from(&self.uvs[..])
    }

    /// Get vertex colors as Float32Array
    pub fn colors(&self) -> js_sys::Float32Array {
        js_sys::Float32Array::from(&self.colors[..])
    }

    /// Get triangle indices as Uint32Array
    pub fn indices(&self) -> js_sys::Uint32Array {
        js_sys::Uint32Array::from(&self.indices[..])
    }
}
