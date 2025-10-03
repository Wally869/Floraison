/**
 * WASM Mesh Data Types
 *
 * This module provides types and utilities for working with mesh data from WASM,
 * including serialization for cross-worker communication.
 */

import type { MeshData } from './floraison';

/**
 * Serialized mesh data that can be transferred across worker boundaries.
 *
 * This contains the raw TypedArrays extracted from a WASM MeshData instance.
 * These arrays can be transferred using Transferable Objects for zero-copy performance.
 */
export interface SerializedMeshData {
	positions: Float32Array;
	normals: Float32Array;
	uvs: Float32Array;
	colors: Float32Array;
	indices: Uint32Array;
}

/**
 * Union type representing either a WASM MeshData instance or serialized mesh data.
 *
 * This allows functions to work with both:
 * - MeshData: Direct WASM instance (main thread, non-worker usage)
 * - SerializedMeshData: Transferred data from worker
 */
export type MeshDataLike = MeshData | SerializedMeshData;

/**
 * Type guard to check if a MeshDataLike is a WASM MeshData instance.
 *
 * @param mesh - The mesh data to check
 * @returns true if mesh is a WASM MeshData instance
 */
export function isWasmMeshData(mesh: MeshDataLike): mesh is MeshData {
	return 'free' in mesh && typeof mesh.free === 'function';
}

/**
 * Serialize a WASM MeshData instance for transfer across worker boundary.
 *
 * This extracts all TypedArrays from the WASM instance. The arrays can then
 * be transferred using postMessage with Transferable Objects for zero-copy.
 *
 * @param meshData - The WASM MeshData instance to serialize
 * @returns Serialized mesh data with TypedArrays
 */
export function serializeMeshData(meshData: MeshData): SerializedMeshData {
	return {
		positions: meshData.positions(),
		normals: meshData.normals(),
		uvs: meshData.uvs(),
		colors: meshData.colors(),
		indices: meshData.indices()
	};
}

/**
 * Get positions array from any MeshDataLike.
 */
export function getPositions(mesh: MeshDataLike): Float32Array {
	if (isWasmMeshData(mesh)) {
		return mesh.positions();
	}
	return mesh.positions;
}

/**
 * Get normals array from any MeshDataLike.
 */
export function getNormals(mesh: MeshDataLike): Float32Array {
	if (isWasmMeshData(mesh)) {
		return mesh.normals();
	}
	return mesh.normals;
}

/**
 * Get UVs array from any MeshDataLike.
 */
export function getUVs(mesh: MeshDataLike): Float32Array {
	if (isWasmMeshData(mesh)) {
		return mesh.uvs();
	}
	return mesh.uvs;
}

/**
 * Get colors array from any MeshDataLike.
 */
export function getColors(mesh: MeshDataLike): Float32Array {
	if (isWasmMeshData(mesh)) {
		return mesh.colors();
	}
	return mesh.colors;
}

/**
 * Get indices array from any MeshDataLike.
 */
export function getIndices(mesh: MeshDataLike): Uint32Array {
	if (isWasmMeshData(mesh)) {
		return mesh.indices();
	}
	return mesh.indices;
}

/**
 * Get vertex count from any MeshDataLike.
 *
 * @param mesh - The mesh data
 * @returns Number of vertices
 */
export function getVertexCount(mesh: MeshDataLike): number {
	return getPositions(mesh).length / 3;
}

/**
 * Get triangle count from any MeshDataLike.
 *
 * @param mesh - The mesh data
 * @returns Number of triangles
 */
export function getTriangleCount(mesh: MeshDataLike): number {
	return getIndices(mesh).length / 3;
}
