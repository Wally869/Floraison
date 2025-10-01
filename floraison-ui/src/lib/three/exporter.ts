/**
 * glTF/GLB Export Utilities
 *
 * Provides functionality to export Three.js objects to glTF 2.0 / GLB format.
 * Uses Three.js GLTFExporter for battle-tested, standards-compliant export.
 */

import { GLTFExporter } from 'three/examples/jsm/exporters/GLTFExporter.js';
import type * as THREE from 'three';

export interface ExportOptions {
	filename?: string;
	onSuccess?: () => void;
	onError?: (error: Error) => void;
}

/**
 * Export a Three.js mesh or object to GLB (binary glTF) format
 *
 * @param object - Three.js Object3D to export (typically a Mesh)
 * @param options - Export configuration
 */
export function exportToGLB(object: THREE.Object3D, options: ExportOptions = {}): void {
	const exporter = new GLTFExporter();

	const filename = options.filename || generateFilename();

	exporter.parse(
		object,
		(gltf) => {
			// gltf is ArrayBuffer in binary mode
			const blob = new Blob([gltf as ArrayBuffer], {
				type: 'application/octet-stream'
			});
			const url = URL.createObjectURL(blob);

			// Trigger download
			const link = document.createElement('a');
			link.href = url;
			link.download = filename;
			document.body.appendChild(link);
			link.click();
			document.body.removeChild(link);

			// Cleanup
			URL.revokeObjectURL(url);

			options.onSuccess?.();
		},
		(error) => {
			console.error('Export failed:', error);
			const errorObj = error instanceof Error ? error : new Error(String(error));
			options.onError?.(errorObj);
		},
		{
			binary: true, // GLB format (single binary file)
			embedImages: true, // Include textures if any
			truncateDrawRange: true // Only export used geometry
		}
	);
}

/**
 * Generate filename for export with timestamp
 *
 * @param presetName - Optional preset name to include in filename
 * @returns Filename with timestamp
 */
export function generateFilename(presetName?: string): string {
	const timestamp = new Date().toISOString().slice(0, 19).replace(/:/g, '-');
	const name =
		presetName && presetName !== 'custom'
			? `floraison_${presetName}_${timestamp}`
			: `floraison_custom_${timestamp}`;
	return `${name}.glb`;
}
