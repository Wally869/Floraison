/**
 * Random Generation Utilities
 *
 * Functions for generating random but reasonable flower parameters.
 * Uses smart constraints to ensure visually pleasing results.
 */

import type {
	DiagramParams,
	ReceptacleParams,
	PistilParams,
	StamenParams,
	PetalParams
} from '$lib/stores/parameters';
import type { InflorescenceParams, PatternType } from '$lib/stores/inflorescence';
import { hslToRgb } from './colors';

// ============================================================================
// Basic Random Utilities
// ============================================================================

/**
 * Generate random number in range [min, max]
 */
export function randomRange(min: number, max: number): number {
	return min + Math.random() * (max - min);
}

/**
 * Generate random integer in range [min, max] (inclusive)
 */
export function randomInt(min: number, max: number): number {
	return Math.floor(randomRange(min, max + 1));
}

/**
 * Pick random element from array
 */
export function randomChoice<T>(array: T[]): T {
	return array[Math.floor(Math.random() * array.length)];
}

/**
 * Random boolean with specified probability
 * @param probability - Chance of true (0-1, default 0.5)
 */
export function randomBoolean(probability: number = 0.5): boolean {
	return Math.random() < probability;
}

// ============================================================================
// Color Generation
// ============================================================================

/**
 * Generate random pastel color (pleasant, not too saturated)
 * Avoids pure black/white, tends toward pleasant hues
 */
export function randomPastelColor(): [number, number, number] {
	const hue = Math.random(); // Full hue range
	const saturation = 0.4 + Math.random() * 0.4; // 0.4-0.8 (moderate)
	const lightness = 0.6 + Math.random() * 0.3; // 0.6-0.9 (bright)
	return hslToRgb(hue, saturation, lightness);
}

/**
 * Generate random greenish color for receptacles/stems
 */
export function randomGreenColor(): [number, number, number] {
	const hue = 0.25 + (Math.random() - 0.5) * 0.15; // Green hue range (0.175-0.325)
	const saturation = 0.3 + Math.random() * 0.4; // 0.3-0.7
	const lightness = 0.4 + Math.random() * 0.4; // 0.4-0.8
	return hslToRgb(hue, saturation, lightness);
}

/**
 * Generate random yellow/golden color for pistils/stamens
 */
export function randomYellowColor(): [number, number, number] {
	const hue = 0.12 + (Math.random() - 0.5) * 0.08; // Yellow-orange (0.08-0.16)
	const saturation = 0.6 + Math.random() * 0.3; // 0.6-0.9 (vibrant)
	const lightness = 0.65 + Math.random() * 0.25; // 0.65-0.9 (bright)
	return hslToRgb(hue, saturation, lightness);
}

// ============================================================================
// Component Parameter Generation
// ============================================================================

/**
 * Generate random diagram parameters (counts and arrangement)
 */
function randomDiagramParams(): DiagramParams {
	return {
		pistilCount: randomInt(1, 3), // Usually 1, occasionally 2-3
		stamenCount: randomInt(4, 12), // Typical range
		petalCount: randomInt(4, 12), // Common petal counts
		sepalCount: randomInt(0, 6), // Often 0, sometimes present
		stamenTilt: randomInt(30, 90) // Droop to horizontal
	};
}

/**
 * Generate random receptacle parameters
 */
function randomReceptacleParams(): ReceptacleParams {
	const base_radius = randomRange(0.15, 0.4);
	const bulge_radius = randomRange(base_radius, base_radius * 1.5);
	const top_radius = randomRange(0.1, base_radius * 0.8);

	return {
		height: randomRange(0.5, 1.5),
		base_radius,
		bulge_radius,
		top_radius,
		bulge_position: randomRange(0.3, 0.7),
		segments: 16, // Fixed for performance
		profile_samples: 8, // Fixed for performance
		color: randomGreenColor()
	};
}

/**
 * Generate random pistil parameters
 */
function randomPistilParams(): PistilParams {
	const base_radius = randomRange(0.05, 0.12);
	const tip_radius = randomRange(base_radius * 0.5, base_radius * 0.9);
	const stigma_radius = randomRange(base_radius * 0.8, base_radius * 1.5);

	return {
		length: randomRange(1.0, 3.0),
		base_radius,
		tip_radius,
		stigma_radius,
		segments: 12, // Fixed
		color: randomYellowColor()
	};
}

/**
 * Generate random stamen parameters
 */
function randomStamenParams(): StamenParams {
	return {
		filament_length: randomRange(0.8, 2.5),
		filament_radius: randomRange(0.02, 0.06),
		anther_length: randomRange(0.15, 0.35),
		anther_width: randomRange(0.04, 0.1),
		anther_height: randomRange(0.04, 0.1),
		segments: 10, // Fixed
		color: randomYellowColor()
	};
}

/**
 * Generate random petal parameters
 */
function randomPetalParams(): PetalParams {
	const length = randomRange(1.5, 4.0);
	const width = randomRange(0.8, 2.5);
	const ruffle_freq = randomBoolean(0.3) ? randomRange(0.5, 2.5) : 0; // 30% chance of ruffle

	return {
		length,
		width,
		tip_sharpness: randomRange(0.2, 0.7),
		base_width: randomRange(0.3, width * 0.6),
		curl: randomRange(-0.6, 0.6),
		twist: randomRange(0, 30),
		ruffle_freq,
		ruffle_amp: ruffle_freq > 0 ? randomRange(0.1, 0.3) : 0,
		resolution: 20, // Fixed for quality
		color: randomPastelColor()
	};
}

/**
 * Generate random inflorescence parameters
 */
function randomInflorescenceParams(): InflorescenceParams {
	const patterns: PatternType[] = [
		'Raceme',
		'Spike',
		'Umbel',
		'Corymb',
		'Dichasium',
		'Drepanium',
		'CompoundRaceme',
		'CompoundUmbel'
	];

	const pattern = randomChoice(patterns);
	const isRecursive =
		pattern === 'Dichasium' ||
		pattern === 'Drepanium' ||
		pattern === 'CompoundRaceme' ||
		pattern === 'CompoundUmbel';

	// Natural rotation angles (golden angle, 120°, 144°, 180°)
	const rotation_angle = randomChoice([137.5, 120, 144, 180]);

	return {
		enabled: randomBoolean(0.5), // 50% chance
		pattern,
		axis_length: randomRange(8.0, 16.0),
		branch_count: randomInt(6, 20),
		angle_top: randomRange(20, 60),
		angle_bottom: randomRange(40, 70),
		branch_length_top: randomRange(0.4, 1.2),
		branch_length_bottom: randomRange(0.8, 1.8),
		rotation_angle,
		flower_size_top: randomRange(0.5, 0.9),
		flower_size_bottom: randomRange(0.6, 1.0),
		recursion_depth: isRecursive ? randomInt(1, 2) : 1,
		branch_ratio: randomRange(0.5, 0.8),
		angle_divergence: 0, // Fixed for now
		age_distribution: 0.5 // Natural gradient (default)
	};
}

// ============================================================================
// Main Generation Function
// ============================================================================

/**
 * Generate complete random flower parameters
 *
 * Creates a full set of randomized parameters with smart constraints
 * to ensure visually pleasing results.
 *
 * @returns Object with all parameter types randomized
 */
export function generateRandomFlowerParams(): {
	diagram: DiagramParams;
	receptacle: ReceptacleParams;
	pistil: PistilParams;
	stamen: StamenParams;
	petal: PetalParams;
	inflorescence: InflorescenceParams;
} {
	return {
		diagram: randomDiagramParams(),
		receptacle: randomReceptacleParams(),
		pistil: randomPistilParams(),
		stamen: randomStamenParams(),
		petal: randomPetalParams(),
		inflorescence: randomInflorescenceParams()
	};
}
