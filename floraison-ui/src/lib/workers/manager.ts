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
 * Queued request waiting for worker to be ready.
 */
interface QueuedRequest {
	id: number;
	request: WorkerRequest;
	pending: PendingRequest;
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
	private isReady = false;
	private requestQueue: QueuedRequest[] = [];

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

			const pending: PendingRequest = {
				resolve,
				reject,
				onProgress
			};

			// Create request message
			const request: WorkerRequest = {
				type: 'generate',
				id,
				flowerParams,
				inflorescenceParams
			};

			// If worker not ready, queue the request
			if (!this.isReady) {
				this.requestQueue.push({ id, request, pending });
				return;
			}

			// Worker is ready, send immediately
			this.pendingRequests.set(id, pending);
			this.worker.postMessage(request);
		});
	}

	/**
	 * Handle messages from the worker.
	 */
	private handleWorkerMessage(event: MessageEvent<WorkerResponse>) {
		const response = event.data;

		switch (response.type) {
			case 'init-ready': {
				console.log('[WorkerManager] Worker ready, processing queued requests');
				this.isReady = true;

				// Process all queued requests
				for (const queued of this.requestQueue) {
					this.pendingRequests.set(queued.id, queued.pending);
					this.worker.postMessage(queued.request);
				}

				// Clear queue
				this.requestQueue = [];
				break;
			}

			case 'init-error': {
				console.error('[WorkerManager] Worker initialization failed:', response.error);

				// Reject all queued requests
				for (const queued of this.requestQueue) {
					queued.pending.reject(new Error(`Worker initialization failed: ${response.error}`));
				}

				// Clear queue
				this.requestQueue = [];
				break;
			}

			case 'generate-success': {
				const pending = this.pendingRequests.get(response.id);
				if (!pending) {
					console.warn('[WorkerManager] Received response for unknown request:', response.id);
					return;
				}

				// response.mesh is already SerializedMeshData (part of MeshDataLike union)
				pending.resolve(response.mesh);
				this.pendingRequests.delete(response.id);
				break;
			}

			case 'generate-error': {
				const pending = this.pendingRequests.get(response.id);
				if (!pending) {
					console.warn('[WorkerManager] Received response for unknown request:', response.id);
					return;
				}

				pending.reject(new Error(response.error));
				this.pendingRequests.delete(response.id);
				break;
			}

			case 'generate-progress': {
				const pending = this.pendingRequests.get(response.id);
				if (!pending) {
					console.warn('[WorkerManager] Received response for unknown request:', response.id);
					return;
				}

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
