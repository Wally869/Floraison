/**
 * Viewer Settings Store
 *
 * Manages 3D viewer display settings (axes, lighting, background, wireframe).
 */

import { writable } from 'svelte/store';

export interface ViewerSettings {
	showAxes: boolean;
	backgroundColor: string; // hex color
	ambientIntensity: number; // 0-1
	directionalIntensity: number; // 0-1
	ambientColor: string; // hex color (hemisphere sky)
	directionalColor: string; // hex color
	hemisphereSkyColor: string; // hex color
	hemisphereGroundColor: string; // hex color
	exposure: number; // 0.5-2.0
	wireframe: boolean;
	enableShadows: boolean;
}

const defaultSettings: ViewerSettings = {
	showAxes: false,
	backgroundColor: '#f0f0f0',
	ambientIntensity: 0.6,
	directionalIntensity: 1.2,
	ambientColor: '#87ceeb', // Light blue (sky color)
	directionalColor: '#ffffff', // White
	hemisphereSkyColor: '#87ceeb', // Light blue
	hemisphereGroundColor: '#8b7355', // Brownish earth
	exposure: 1.0, // Neutral exposure
	wireframe: false,
	enableShadows: true
};

export const viewerSettings = writable<ViewerSettings>(defaultSettings);

/**
 * Reset viewer settings to defaults
 */
export function resetViewerSettings(): void {
	viewerSettings.set(defaultSettings);
}
