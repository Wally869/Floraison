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

        // Generate flower mesh for bloom stage
        let flower_mesh = generate_flower(&flower_params);

        // Create aging struct (MVP: use same mesh for all stages)
        let aging = FlowerAging {
            bud_mesh: flower_mesh.clone(),
            bloom_mesh: flower_mesh.clone(),
            wilt_mesh: Some(flower_mesh.clone()),
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
