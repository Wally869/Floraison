/**
 * WASM to Three.js Mesh Converter
 *
 * Converts MeshData from WASM to Three.js BufferGeometry.
 * Now supports both WASM MeshData instances and serialized mesh data from workers.
 */

import * as THREE from 'three';
import type { MeshDataLike } from '$lib/wasm/types';

/**
 * Convert MeshData to Three.js BufferGeometry
 *
 * Takes the flat typed arrays from mesh data and creates a BufferGeometry
 * with proper attributes for positions, normals, UVs, and indices.
 *
 * Works with both:
 * - WASM MeshData instances (direct generation on main thread)
 * - SerializedMeshData (from worker generation)
 *
 * @param meshData - MeshData instance from WASM or serialized mesh data
 * @returns BufferGeometry ready to be used with a Three.js Mesh
 */
export function wasmMeshToGeometry(meshData: MeshDataLike): THREE.BufferGeometry {
	const geometry = new THREE.BufferGeometry();

	// Get typed arrays from WASM
	const positions = meshData.positions();
	const normals = meshData.normals();
	const uvs = meshData.uvs();
	const colors = meshData.colors();
	const indices = meshData.indices();

	// Set position attribute (stride 3: x, y, z)
	geometry.setAttribute('position', new THREE.BufferAttribute(positions, 3));

	// Set normal attribute (stride 3: x, y, z)
	geometry.setAttribute('normal', new THREE.BufferAttribute(normals, 3));

	// Set UV attribute (stride 2: u, v)
	geometry.setAttribute('uv', new THREE.BufferAttribute(uvs, 2));

	// Set color attribute (stride 3: r, g, b)
	geometry.setAttribute('color', new THREE.BufferAttribute(colors, 3));

	// Set index buffer
	geometry.setIndex(new THREE.BufferAttribute(indices, 1));

	// Compute bounding sphere for camera framing
	geometry.computeBoundingSphere();

	return geometry;
}
