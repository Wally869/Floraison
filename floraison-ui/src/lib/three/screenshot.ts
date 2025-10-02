/**
 * Screenshot Utilities
 *
 * Provides functionality to capture Three.js renderer output as PNG images.
 * Useful for generating demo content and allowing users to save their creations.
 */

import type * as THREE from 'three';

export interface ScreenshotOptions {
	filename?: string;
	onSuccess?: () => void;
	onError?: (error: Error) => void;
}

/**
 * Capture the current renderer canvas as a PNG image and download it
 *
 * @param renderer - Three.js WebGLRenderer to capture
 * @param options - Screenshot configuration
 */
export function captureScreenshot(
	renderer: THREE.WebGLRenderer,
	options: ScreenshotOptions = {}
): void {
	const filename = options.filename || generateScreenshotFilename();

	try {
		// Capture canvas as blob
		renderer.domElement.toBlob(
			(blob) => {
				if (!blob) {
					const error = new Error('Failed to capture canvas as blob');
					console.error('Screenshot failed:', error);
					options.onError?.(error);
					return;
				}

				// Create download link
				const url = URL.createObjectURL(blob);
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
			'image/png', // PNG format for lossless quality
			1.0 // Maximum quality
		);
	} catch (error) {
		console.error('Screenshot failed:', error);
		const errorObj = error instanceof Error ? error : new Error(String(error));
		options.onError?.(errorObj);
	}
}

/**
 * Generate filename for screenshot with timestamp
 *
 * @param presetName - Optional preset name to include in filename
 * @returns Filename with timestamp
 */
export function generateScreenshotFilename(presetName?: string): string {
	const timestamp = new Date().toISOString().slice(0, 19).replace(/:/g, '-');
	const name =
		presetName && presetName !== 'custom'
			? `floraison_${presetName}_${timestamp}`
			: `floraison_custom_${timestamp}`;
	return `${name}.png`;
}
