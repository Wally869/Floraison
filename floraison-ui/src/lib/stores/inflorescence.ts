/**
 * Inflorescence Parameter Store
 *
 * Manages parameters for multi-flower structures (inflorescences).
 */

import { writable } from 'svelte/store';

export type PatternType =
	| 'Raceme'
	| 'Spike'
	| 'Umbel'
	| 'Corymb'
	| 'Dichasium'
	| 'Drepanium'
	| 'CompoundRaceme'
	| 'CompoundUmbel';

export interface InflorescenceParams {
	enabled: boolean; // Toggle between flower/inflorescence mode
	pattern: PatternType;
	axis_length: number;
	branch_count: number;
	angle_top: number;
	angle_bottom: number;
	branch_length_top: number;
	branch_length_bottom: number;
	rotation_angle: number;
	flower_size_top: number;
	flower_size_bottom: number;
	// Recursive pattern parameters (always initialized)
	recursion_depth: number;
	branch_ratio: number;
	angle_divergence: number;
}

const defaultParams: InflorescenceParams = {
	enabled: false, // Start in single flower mode
	pattern: 'Raceme',
	axis_length: 10.0,
	branch_count: 12,
	angle_top: 45.0,
	angle_bottom: 60.0,
	branch_length_top: 0.5,
	branch_length_bottom: 1.5,
	rotation_angle: 137.5, // Golden angle
	flower_size_top: 0.8,
	flower_size_bottom: 1.0,
	recursion_depth: 1, // Always initialize to 1 (never null to avoid slider midpoint issue)
	branch_ratio: 0.7,
	angle_divergence: 0.0
};

export const inflorescenceParams = writable<InflorescenceParams>(defaultParams);

/**
 * Reset inflorescence parameters to defaults
 */
export function resetInflorescenceParams(): void {
	inflorescenceParams.set(defaultParams);
}

/**
 * Check if pattern requires recursive parameters
 */
export function isRecursivePattern(pattern: PatternType): boolean {
	return (
		pattern === 'Dichasium' ||
		pattern === 'Drepanium' ||
		pattern === 'CompoundRaceme' ||
		pattern === 'CompoundUmbel'
	);
}

/**
 * Get recommended defaults for recursive parameters based on pattern
 */
export function getRecursiveDefaults(pattern: PatternType): {
	recursion_depth: number;
	branch_ratio: number;
	angle_divergence: number;
} {
	switch (pattern) {
		case 'Dichasium':
			return {
				recursion_depth: 1,
				branch_ratio: 0.7,
				angle_divergence: 30.0
			};
		case 'Drepanium':
			return {
				recursion_depth: 1,
				branch_ratio: 0.8,
				angle_divergence: 137.5 // Uses rotation_angle by default, but can override
			};
		case 'CompoundRaceme':
			return {
				recursion_depth: 1,
				branch_ratio: 0.5,
				angle_divergence: 0.0 // Not used
			};
		case 'CompoundUmbel':
			return {
				recursion_depth: 1,
				branch_ratio: 0.5,
				angle_divergence: 0.0 // Not used
			};
		default:
			return {
				recursion_depth: 1,
				branch_ratio: 0.7,
				angle_divergence: 0.0
			};
	}
}
