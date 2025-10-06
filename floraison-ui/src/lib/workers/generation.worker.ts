/**
 * Flower Generation Web Worker
 *
 * This worker handles computationally expensive flower generation off the main thread,
 * preventing UI blocking during mesh generation.
 *
 * Architecture:
 * 1. Load WASM module in worker context
 * 2. Receive generation requests via postMessage
 * 3. Generate flower using WASM
 * 4. Extract TypedArrays from WASM memory
 * 5. Transfer arrays back to main thread (zero-copy)
 * 6. Clean up WASM memory
 */

import type { WorkerRequest, WorkerResponse } from './types';
import { serializeMeshData } from '$lib/wasm/types';
import wasmUrl from '$lib/wasm/floraison_bg.wasm?url';

// WASM module imports (will be loaded asynchronously)
let wasmModule: typeof import('$lib/wasm/floraison') | null = null;
let FlowerGenerator: any = null;

/**
 * Initialize WASM module in worker context.
 *
 * This is called once when the worker starts. WASM loading is async,
 * so we handle it during initialization.
 */
async function initializeWasm() {
	try {
		// Dynamic import of WASM module
		const wasm = await import('$lib/wasm/floraison');

		// CRITICAL: Initialize the WASM module with explicit URL
		// This ensures correct path resolution with base path on GitHub Pages
		await wasm.default(wasmUrl);

		wasmModule = wasm;
		FlowerGenerator = wasm.FlowerGenerator;
		console.log('[Worker] WASM module initialized successfully');
	} catch (error) {
		console.error('[Worker] Failed to load WASM module:', error);
		throw error;
	}
}

/**
 * Handle generation request from main thread.
 *
 * @param request - Generation request with parameters
 */
async function handleGenerateRequest(request: WorkerRequest) {
	if (request.type !== 'generate') return;

	const { id, flowerParams, inflorescenceParams } = request;

	try {
		// Ensure WASM is loaded
		if (!wasmModule || !FlowerGenerator) {
			throw new Error('WASM module not initialized');
		}

		// Create generator instance
		const generator = new FlowerGenerator();

		// Generate flower or inflorescence based on parameters
		let meshData;
		if (inflorescenceParams.enabled) {
			// Generate inflorescence
			const infloJson = JSON.stringify(inflorescenceParams);
			const flowerJson = JSON.stringify(flowerParams);
			meshData = generator.generate_inflorescence(infloJson, flowerJson);
		} else {
			// Generate single flower
			const paramsJson = JSON.stringify(flowerParams);
			meshData = generator.generate_flower(paramsJson);
		}

		// Extract TypedArrays from WASM memory
		const serialized = serializeMeshData(meshData);

		// Clean up WASM memory
		meshData.free();
		generator.free();

		// Prepare transferable arrays for zero-copy transfer
		const transferables: Transferable[] = [
			serialized.positions.buffer,
			serialized.normals.buffer,
			serialized.uvs.buffer,
			serialized.colors.buffer,
			serialized.indices.buffer
		];

		// Send success response with transferred arrays
		const response: WorkerResponse = {
			type: 'generate-success',
			id,
			mesh: serialized
		};

		self.postMessage(response, { transfer: transferables });
	} catch (error) {
		// Send error response
		const response: WorkerResponse = {
			type: 'generate-error',
			id,
			error: error instanceof Error ? error.message : String(error)
		};

		self.postMessage(response);
	}
}

/**
 * Worker message handler.
 *
 * Receives requests from main thread and dispatches to appropriate handler.
 */
self.onmessage = async (event: MessageEvent<WorkerRequest>) => {
	const request = event.data;

	switch (request.type) {
		case 'generate':
			await handleGenerateRequest(request);
			break;
		default:
			console.warn('[Worker] Unknown request type:', request);
	}
};

// Initialize WASM on worker startup
initializeWasm()
	.then(() => {
		// Send ready message to main thread
		const response: WorkerResponse = {
			type: 'init-ready'
		};
		self.postMessage(response);
	})
	.catch((error) => {
		console.error('[Worker] WASM initialization failed:', error);
		// Send error message to main thread
		const response: WorkerResponse = {
			type: 'init-error',
			error: error instanceof Error ? error.message : String(error)
		};
		self.postMessage(response);
	});
