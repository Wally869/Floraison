/**
 * Worker Message Protocol
 *
 * This module defines the message types for communication between the main thread
 * and the generation worker.
 */

import type { SerializedMeshData } from '$lib/wasm/types';
import type { FlowerParams, InflorescenceParams } from '$lib/stores/parameters';

/**
 * Request message sent from main thread to worker to generate a flower.
 */
export interface GenerateRequest {
	type: 'generate';
	/** Unique ID for this generation request (for matching responses) */
	id: number;
	/** Flower generation parameters */
	flowerParams: FlowerParams;
	/** Inflorescence generation parameters */
	inflorescenceParams: InflorescenceParams;
}

/**
 * All possible request types sent to worker.
 */
export type WorkerRequest = GenerateRequest;

/**
 * Success response from worker with generated mesh data.
 */
export interface GenerateSuccessResponse {
	type: 'generate-success';
	/** Request ID this response corresponds to */
	id: number;
	/** Serialized mesh data (TypedArrays can be transferred) */
	mesh: SerializedMeshData;
}

/**
 * Error response from worker when generation fails.
 */
export interface GenerateErrorResponse {
	type: 'generate-error';
	/** Request ID this response corresponds to */
	id: number;
	/** Error message */
	error: string;
}

/**
 * Progress update from worker during generation.
 * (For future use - could show progress bar)
 */
export interface GenerateProgressResponse {
	type: 'generate-progress';
	/** Request ID this response corresponds to */
	id: number;
	/** Progress percentage (0-100) */
	progress: number;
	/** Optional status message */
	message?: string;
}

/**
 * All possible response types from worker.
 */
export type WorkerResponse =
	| GenerateSuccessResponse
	| GenerateErrorResponse
	| GenerateProgressResponse;
