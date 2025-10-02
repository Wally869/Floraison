/**
 * Generation Worker Manager
 *
 * Provides a clean, Promise-based API for interacting with the generation worker.
 * Handles worker lifecycle, request/response matching, and error handling.
 */

import type { WorkerRequest, WorkerResponse } from './types';
import type { FlowerParams } from '$lib/stores/parameters';
import type { MeshDataLike } from '$lib/wasm/types';
import GenerationWorker from './generation.worker?worker';

/**
 * Callback for generation progress updates.
 */
type ProgressCallback = (progress: number, message?: string) => void;

/**
 * Pending request waiting for a response from the worker.
 */
interface PendingRequest {
	resolve: (mesh: MeshDataLike) => void;
	reject: (error: Error) => void;
	onProgress?: ProgressCallback;
}

/**
 * Manager for the flower generation worker.
 *
 * This class provides a singleton worker instance and manages the request/response cycle.
 *
 * Usage:
 * ```typescript
 * const manager = new GenerationWorkerManager();
 * const mesh = await manager.generate(params);
 * ```
 */
export class GenerationWorkerManager {
	private worker: Worker;
	private nextRequestId = 1;
	private pendingRequests = new Map<number, PendingRequest>();

	constructor() {
		this.worker = new GenerationWorker();
		this.worker.onmessage = this.handleWorkerMessage.bind(this);
		this.worker.onerror = this.handleWorkerError.bind(this);
	}

	/**
	 * Generate a flower mesh using the worker.
	 *
	 * @param flowerParams - Flower generation parameters
	 * @param inflorescenceParams - Inflorescence generation parameters
	 * @param onProgress - Optional progress callback
	 * @returns Promise resolving to generated mesh data
	 */
	generate(
		flowerParams: FlowerParams,
		inflorescenceParams: import('$lib/stores/parameters').InflorescenceParams,
		onProgress?: ProgressCallback
	): Promise<MeshDataLike> {
		return new Promise((resolve, reject) => {
			const id = this.nextRequestId++;

			// Store pending request
			this.pendingRequests.set(id, {
				resolve,
				reject,
				onProgress
			});

			// Send request to worker
			const request: WorkerRequest = {
				type: 'generate',
				id,
				flowerParams,
				inflorescenceParams
			};

			this.worker.postMessage(request);
		});
	}

	/**
	 * Handle messages from the worker.
	 */
	private handleWorkerMessage(event: MessageEvent<WorkerResponse>) {
		const response = event.data;
		const pending = this.pendingRequests.get(response.id);

		if (!pending) {
			console.warn('[WorkerManager] Received response for unknown request:', response.id);
			return;
		}

		switch (response.type) {
			case 'generate-success': {
				// response.mesh is already SerializedMeshData (part of MeshDataLike union)
				pending.resolve(response.mesh);
				this.pendingRequests.delete(response.id);
				break;
			}

			case 'generate-error': {
				pending.reject(new Error(response.error));
				this.pendingRequests.delete(response.id);
				break;
			}

			case 'generate-progress': {
				// Call progress callback if provided
				pending.onProgress?.(response.progress, response.message);
				break;
			}

			default: {
				console.warn('[WorkerManager] Unknown response type:', response);
			}
		}
	}

	/**
	 * Handle worker errors.
	 */
	private handleWorkerError(error: ErrorEvent) {
		console.error('[WorkerManager] Worker error:', error);

		// Reject all pending requests
		for (const [id, pending] of this.pendingRequests) {
			pending.reject(new Error(`Worker error: ${error.message}`));
			this.pendingRequests.delete(id);
		}
	}

	/**
	 * Terminate the worker and clean up resources.
	 */
	dispose() {
		// Reject all pending requests
		for (const [id, pending] of this.pendingRequests) {
			pending.reject(new Error('Worker terminated'));
			this.pendingRequests.delete(id);
		}

		this.worker.terminate();
	}
}

/**
 * Singleton instance of the worker manager.
 *
 * This ensures only one worker is created for the entire application.
 */
let instance: GenerationWorkerManager | null = null;

/**
 * Get the singleton worker manager instance.
 *
 * @returns The worker manager instance
 */
export function getWorkerManager(): GenerationWorkerManager {
	if (!instance) {
		instance = new GenerationWorkerManager();
	}
	return instance;
}

/**
 * Dispose the singleton worker manager.
 *
 * Call this when the application is shutting down.
 */
export function disposeWorkerManager() {
	if (instance) {
		instance.dispose();
		instance = null;
	}
}
