/**
 * Floraison Parameter Stores
 *
 * Svelte stores for managing flower generation parameters.
 * These stores are two-way bound to UI controls and trigger flower regeneration.
 */

import { writable, derived, type Readable } from 'svelte/store';
import { generateBendCurve } from '$lib/utils/curves';

// Re-export inflorescence store for convenience
export {
	inflorescenceParams,
	resetInflorescenceParams,
	isRecursivePattern,
	getRecursiveDefaults,
	type InflorescenceParams,
	type PatternType
} from './inflorescence';

// ============================================================================
// Current Preset Tracking
// ============================================================================

/**
 * Current preset name (used for export filename)
 */
export const currentPresetName = writable<string>('lily');

// ============================================================================
// TypeScript Types (matching Rust structs)
// ============================================================================

export interface DiagramParams {
	pistilCount: number;
	stamenCount: number;
	petalCount: number;
	sepalCount: number;
	stamenTilt: number; // Tilt angle in degrees (0-90)
	// Natural variation parameters
	position_jitter: number; // 0-0.5: Random position offset
	angle_jitter: number; // 0-15: Random rotation variation in degrees
	size_jitter: number; // 0-0.3: Random scale variation (±30%)
	jitter_seed: number; // Seed for deterministic randomness
}

export interface ReceptacleParams {
	height: number;
	base_radius: number;
	bulge_radius: number;
	top_radius: number;
	bulge_position: number;
	segments: number;
	profile_samples: number;
	color: [number, number, number];
}

export interface PistilParams {
	length: number;
	base_radius: number;
	tip_radius: number;
	stigma_radius: number;
	segments: number;
	color: [number, number, number];
	pistil_bend: number; // 0-1: bend amount for curved style (0=straight, 1=dramatic arc)
	pistil_droop: number; // -1 to 1: vertical tilt (-1=droop down, 0=straight, 1=lift up)
	// Note: style_curve generated from pistil_bend and pistil_droop
}

export interface StamenParams {
	filament_length: number;
	filament_radius: number;
	anther_length: number;
	anther_width: number;
	anther_height: number;
	segments: number;
	color: [number, number, number];
	stamen_bend: number; // 0-1: bend amount for curved filament (0=straight, 1=dramatic arc)
	stamen_droop: number; // -1 to 1: vertical tilt (-1=droop down, 0=straight, 1=lift up)
	// Note: filament_curve generated from stamen_bend and stamen_droop
}

export interface PetalParams {
	length: number;
	width: number;
	tip_sharpness: number;
	base_width: number;
	curl: number;
	twist: number;
	ruffle_freq: number;
	ruffle_amp: number;
	lateral_curve: number;
	resolution: number;
	color: [number, number, number];
}

// Rust-compatible whorl structure
interface ComponentWhorl {
	count: number;
	radius: number;
	height: number;
	pattern: 'EvenlySpaced' | 'GoldenSpiral' | { CustomOffset: number };
	rotation_offset: number;
	tilt_angle: number; // Tilt angle in radians
}

// Rust-compatible FloralDiagram structure
interface FloralDiagram {
	receptacle_height: number;
	receptacle_radius: number;
	petal_whorls: ComponentWhorl[];
	stamen_whorls: ComponentWhorl[];
	pistil_whorls: ComponentWhorl[];
	sepal_whorls: ComponentWhorl[];
	position_jitter: number;
	angle_jitter: number;
	size_jitter: number;
	jitter_seed: number;
}

// Complete FlowerParams structure for WASM
export interface FlowerParams {
	diagram: FloralDiagram;
	receptacle: ReceptacleParams;
	pistil: PistilParams;
	stamen: StamenParams;
	petal: PetalParams;
}

// ============================================================================
// Default Values (matching Rust defaults)
// ============================================================================

const defaultDiagramParams: DiagramParams = {
	pistilCount: 1,
	stamenCount: 6,
	petalCount: 6,
	sepalCount: 0,
	stamenTilt: 90, // Default: 90° horizontal spreading (like lily)
	// Natural variation (disabled by default for perfect symmetry)
	position_jitter: 0.0,
	angle_jitter: 0.0,
	size_jitter: 0.0,
	jitter_seed: 42
};

const defaultReceptacleParams: ReceptacleParams = {
	height: 1.0,
	base_radius: 0.25,
	bulge_radius: 0.35,
	top_radius: 0.15,
	bulge_position: 0.5,
	segments: 16,
	profile_samples: 8,
	color: [1.0, 1.0, 1.0]
};

const defaultPistilParams: PistilParams = {
	length: 2.0,
	base_radius: 0.08,
	tip_radius: 0.06,
	stigma_radius: 0.12,
	segments: 12,
	color: [1.0, 1.0, 1.0],
	pistil_bend: 0.0, // Straight by default
	pistil_droop: 0.0 // No droop by default
};

const defaultStamenParams: StamenParams = {
	filament_length: 1.5,
	filament_radius: 0.04,
	anther_length: 0.25,
	anther_width: 0.07,
	anther_height: 0.07,
	segments: 10,
	color: [1.0, 1.0, 1.0],
	stamen_bend: 0.0, // Straight by default
	stamen_droop: 0.0 // No droop by default
};

const defaultPetalParams: PetalParams = {
	length: 3.0,
	width: 1.2,
	tip_sharpness: 0.4,
	base_width: 0.4,
	curl: 0.0,
	twist: 0.0,
	ruffle_freq: 0.0,
	ruffle_amp: 0.0,
	lateral_curve: 0.0,
	resolution: 14,
	color: [1.0, 1.0, 1.0]
};

// ============================================================================
// Writable Stores
// ============================================================================

export const diagramParams = writable<DiagramParams>(defaultDiagramParams);
export const receptacleParams = writable<ReceptacleParams>(defaultReceptacleParams);
export const pistilParams = writable<PistilParams>(defaultPistilParams);
export const stamenParams = writable<StamenParams>(defaultStamenParams);
export const petalParams = writable<PetalParams>(defaultPetalParams);

// ============================================================================
// Helper Functions
// ============================================================================

/**
 * Create a whorl for a component type
 *
 * Generates whorl array from simple count using sensible defaults
 * for radius and height based on component type.
 */
function createWhorl(
	count: number,
	radius: number,
	height: number,
	tilt: number = 0.0
): ComponentWhorl[] {
	if (count === 0) return [];

	return [
		{
			count,
			radius,
			height,
			pattern: 'EvenlySpaced' as const,
			rotation_offset: 0.0,
			tilt_angle: tilt
		}
	];
}

/**
 * Convert diagram params to FloralDiagram structure
 *
 * Constructs whorl arrays from simple counts with appropriate
 * positioning for each component type.
 */
function buildFloralDiagram(
	diagram: DiagramParams,
	receptacle: ReceptacleParams
): FloralDiagram {
	// Convert stamen tilt from degrees to radians
	const stamenTiltRadians = (diagram.stamenTilt * Math.PI) / 180;

	return {
		receptacle_height: receptacle.height,
		receptacle_radius: receptacle.base_radius,
		// Petals: outer ring at 75% height
		petal_whorls: createWhorl(diagram.petalCount, 1.0, 0.75, 0.0),
		// Stamens: middle ring at 85% height (ABOVE petal base - botanically correct)
		stamen_whorls: createWhorl(diagram.stamenCount, 0.6, 0.85, stamenTiltRadians),
		// Pistils: center at 80% height (anthers level with/above pistil base)
		pistil_whorls: createWhorl(diagram.pistilCount, 0.0, 0.8, 0.0),
		// Sepals: outer ring at 70% height (just below petals, reduced gap)
		sepal_whorls: createWhorl(diagram.sepalCount, 1.0, 0.7, 0.0),
		// Natural variation parameters
		position_jitter: diagram.position_jitter,
		angle_jitter: diagram.angle_jitter,
		size_jitter: diagram.size_jitter,
		jitter_seed: diagram.jitter_seed
	};
}

// ============================================================================
// Derived Store: All Parameters
// ============================================================================

/**
 * Combined parameters store
 *
 * Derives complete FlowerParams from all parameter stores.
 * This is the single source of truth that triggers flower regeneration.
 *
 * Automatically converts bend amounts to curve control points:
 * - pistil_bend → style_curve
 * - stamen_bend → filament_curve
 */
export const allParams: Readable<FlowerParams> = derived(
	[diagramParams, receptacleParams, pistilParams, stamenParams, petalParams],
	([$diagram, $receptacle, $pistil, $stamen, $petal]) => {
		// Generate pistil curve if bend or droop > 0
		const pistilCurve = generateBendCurve(
			$pistil.length,
			$pistil.pistil_bend,
			$pistil.pistil_droop,
			1
		);

		// Generate stamen curve if bend or droop > 0
		const stamenCurve = generateBendCurve(
			$stamen.filament_length,
			$stamen.stamen_bend,
			$stamen.stamen_droop,
			1
		);

		// Build pistil params with optional curve
		// Remove UI-only pistil_bend and pistil_droop fields before sending to Rust
		const { pistil_bend: _pistilBend, pistil_droop: _pistilDroop, ...pistilBase } = $pistil;
		const pistilRust = {
			...pistilBase,
			...(pistilCurve && { style_curve: pistilCurve })
		};

		// Build stamen params with optional curve
		// Remove UI-only stamen_bend and stamen_droop fields before sending to Rust
		const { stamen_bend: _stamenBend, stamen_droop: _stamenDroop, ...stamenBase } = $stamen;
		const stamenRust = {
			...stamenBase,
			...(stamenCurve && { filament_curve: stamenCurve })
		};

		return {
			diagram: buildFloralDiagram($diagram, $receptacle),
			receptacle: $receptacle,
			pistil: pistilRust as any, // Cast needed since we add optional style_curve
			stamen: stamenRust as any, // Cast needed since we add optional filament_curve
			petal: $petal
		};
	}
);

// ============================================================================
// Utility Functions
// ============================================================================

/**
 * Reset all parameters to defaults
 */
export function resetToDefaults(): void {
	diagramParams.set(defaultDiagramParams);
	receptacleParams.set(defaultReceptacleParams);
	pistilParams.set(defaultPistilParams);
	stamenParams.set(defaultStamenParams);
	petalParams.set(defaultPetalParams);
}

/**
 * Load parameters from FlowerParams object
 *
 * Used for loading presets. Extracts simple counts from diagram whorls.
 */
export function loadParams(params: FlowerParams): void {
	// Extract tilt angle from first stamen whorl (convert radians to degrees)
	const firstStamenWhorl = params.diagram.stamen_whorls[0];
	const stamenTiltDegrees = firstStamenWhorl
		? Math.round((firstStamenWhorl.tilt_angle * 180) / Math.PI)
		: 90;

	// Extract counts from whorls, with default jitter values for backward compatibility
	diagramParams.set({
		pistilCount: params.diagram.pistil_whorls.reduce((sum, w) => sum + w.count, 0),
		stamenCount: params.diagram.stamen_whorls.reduce((sum, w) => sum + w.count, 0),
		petalCount: params.diagram.petal_whorls.reduce((sum, w) => sum + w.count, 0),
		sepalCount: params.diagram.sepal_whorls.reduce((sum, w) => sum + w.count, 0),
		stamenTilt: stamenTiltDegrees,
		// Extract jitter params if present, otherwise use defaults (0 = no jitter)
		position_jitter: (params.diagram as any).position_jitter ?? 0.0,
		angle_jitter: (params.diagram as any).angle_jitter ?? 0.0,
		size_jitter: (params.diagram as any).size_jitter ?? 0.0,
		jitter_seed: (params.diagram as any).jitter_seed ?? 42
	});

	receptacleParams.set(params.receptacle);
	pistilParams.set(params.pistil);
	stamenParams.set(params.stamen);
	petalParams.set(params.petal);
}

/**
 * Convert RGB color [r, g, b] to hex string #RRGGBB
 */
export function rgbToHex(rgb: [number, number, number]): string {
	const toHex = (n: number) =>
		Math.round(n * 255)
			.toString(16)
			.padStart(2, '0');
	return `#${toHex(rgb[0])}${toHex(rgb[1])}${toHex(rgb[2])}`;
}

/**
 * Convert hex color #RRGGBB to RGB [r, g, b] in 0-1 range
 */
export function hexToRgb(hex: string): [number, number, number] {
	const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
	if (!result) return [1.0, 1.0, 1.0];

	return [
		parseInt(result[1], 16) / 255,
		parseInt(result[2], 16) / 255,
		parseInt(result[3], 16) / 255
	];
}
