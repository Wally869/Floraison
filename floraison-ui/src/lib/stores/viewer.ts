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
	wireframe: boolean;
	enableShadows: boolean; // NEW
}

const defaultSettings: ViewerSettings = {
	showAxes: false,
	backgroundColor: '#f0f0f0',
	ambientIntensity: 0.5,
	directionalIntensity: 0.8,
	wireframe: false,
	enableShadows: true // Shadows enabled by default
};

export const viewerSettings = writable<ViewerSettings>(defaultSettings);

/**
 * Reset viewer settings to defaults
 */
export function resetViewerSettings(): void {
	viewerSettings.set(defaultSettings);
}
