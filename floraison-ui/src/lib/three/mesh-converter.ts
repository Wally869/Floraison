/**
 * WASM to Three.js Mesh Converter
 *
 * Converts MeshData from WASM to Three.js BufferGeometry.
 */

import * as THREE from 'three';
import type { MeshData } from '$lib/wasm/floraison';

/**
 * Convert WASM MeshData to Three.js BufferGeometry
 *
 * Takes the flat typed arrays from WASM and creates a BufferGeometry
 * with proper attributes for positions, normals, UVs, and indices.
 *
 * @param meshData - MeshData instance from WASM
 * @returns BufferGeometry ready to be used with a Three.js Mesh
 */
export function wasmMeshToGeometry(meshData: MeshData): THREE.BufferGeometry {
	const geometry = new THREE.BufferGeometry();

	// Get typed arrays from WASM
	const positions = meshData.positions();
	const normals = meshData.normals();
	const uvs = meshData.uvs();
	const indices = meshData.indices();

	// Set position attribute (stride 3: x, y, z)
	geometry.setAttribute('position', new THREE.BufferAttribute(positions, 3));

	// Set normal attribute (stride 3: x, y, z)
	geometry.setAttribute('normal', new THREE.BufferAttribute(normals, 3));

	// Set UV attribute (stride 2: u, v)
	geometry.setAttribute('uv', new THREE.BufferAttribute(uvs, 2));

	// Set index buffer
	geometry.setIndex(new THREE.BufferAttribute(indices, 1));

	// Compute bounding sphere for camera framing
	geometry.computeBoundingSphere();

	return geometry;
}
