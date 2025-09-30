//! Floraison WASM
//!
//! WebAssembly bindings for the Floraison flower generator.
//! Exposes the Rust implementation to JavaScript/TypeScript.

use wasm_bindgen::prelude::*;

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
    pub fn generate_flower(&self, _params_json: &str) -> Result<JsValue, JsValue> {
        // Placeholder - will be implemented in later tasks
        Err(JsValue::from_str("Not yet implemented"))
    }
}

/// Mesh data structure for passing to JavaScript
#[wasm_bindgen]
pub struct MeshData {
    positions: Vec<f32>,
    normals: Vec<f32>,
    uvs: Vec<f32>,
    indices: Vec<u32>,
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

    /// Get triangle indices as Uint32Array
    pub fn indices(&self) -> js_sys::Uint32Array {
        js_sys::Uint32Array::from(&self.indices[..])
    }
}
