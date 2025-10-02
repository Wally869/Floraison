/**
 * Flower Presets
 *
 * Pre-configured parameter sets for common flower types.
 * Each preset defines the complete set of parameters needed to generate
 * a specific flower variety.
 */

import type { FlowerParams, InflorescenceParams } from './stores/parameters';

export interface FlowerPreset {
	name: string;
	description: string;
	params: FlowerParams;
	inflorescence?: InflorescenceParams; // Optional inflorescence configuration
}

/**
 * Lily preset
 *
 * Classic lily with 6 elegant petals, subtle curl, and minimal twist.
 * Simple and symmetrical structure with central pistil.
 */
export const lilyPreset: FlowerPreset = {
	name: 'Lily',
	description: '6 elegant petals with gentle curl',
	params: {
		diagram: {
			receptacle_height: 1.0,
			receptacle_radius: 0.25,
			petal_whorls: [
				{
					count: 6,
					radius: 1.0,
					height: 0.8,
					pattern: 'EvenlySpaced',
					rotation_offset: 0.0,
					tilt_angle: 0.0
				}
			],
			stamen_whorls: [
				{
					count: 6,
					radius: 0.6,
					height: 0.6,
					pattern: 'EvenlySpaced',
					rotation_offset: 0.523599, // 30 degrees offset
					tilt_angle: 1.047 // π/3 (60°) - natural drooping spread
				}
			],
			pistil_whorls: [
				{
					count: 1,
					radius: 0.0,
					height: 0.5,
					pattern: 'EvenlySpaced',
					rotation_offset: 0.0,
					tilt_angle: 0.0
				}
			],
			sepal_whorls: []
		},
		receptacle: {
			height: 1.0,
			base_radius: 0.25,
			bulge_radius: 0.35,
			top_radius: 0.15,
			bulge_position: 0.5,
			segments: 16,
			profile_samples: 8,
			color: [0.9, 0.95, 0.9] // Light greenish
		},
		pistil: {
			length: 2.0,
			base_radius: 0.08,
			tip_radius: 0.06,
			stigma_radius: 0.12,
			segments: 12,
			color: [0.95, 0.9, 0.3], // Yellow-green
			pistil_bend: 0.0,
			pistil_droop: 0.0
		},
		stamen: {
			filament_length: 1.5,
			filament_radius: 0.04,
			anther_length: 0.25,
			anther_width: 0.07,
			anther_height: 0.07,
			segments: 10,
			color: [0.95, 0.75, 0.2], // Golden
			stamen_bend: 0.0,
			stamen_droop: 0.0
		},
		petal: {
			length: 3.0,
			width: 1.2,
			tip_sharpness: 0.4,
			base_width: 0.4,
			curl: 0.4,
			twist: 15.0,
			ruffle_freq: 0.0,
			ruffle_amp: 0.0,
			resolution: 20,
			color: [1.0, 1.0, 1.0] // White
		}
	}
};

/**
 * Rose preset
 *
 * Many layered petals with ruffled edges.
 * Compact structure with numerous stamens.
 */
export const rosePreset: FlowerPreset = {
	name: 'Rose',
	description: 'Layered petals with ruffled edges',
	params: {
		diagram: {
			receptacle_height: 0.8,
			receptacle_radius: 0.4,
			petal_whorls: [
				{
					count: 24,
					radius: 1.0,
					height: 0.6,
					pattern: 'EvenlySpaced',
					rotation_offset: 0.0,
					tilt_angle: 0.0
				}
			],
			stamen_whorls: [
				{
					count: 20,
					radius: 0.5,
					height: 0.4,
					pattern: 'EvenlySpaced',
					rotation_offset: 0.0,
					tilt_angle: 1.047 // PI/3 - moderate upward angle
				}
			],
			pistil_whorls: [],
			sepal_whorls: [
				{
					count: 5,
					radius: 1.2,
					height: 0.3,
					pattern: 'EvenlySpaced',
					rotation_offset: 0.0,
					tilt_angle: 0.0
				}
			]
		},
		receptacle: {
			height: 0.8,
			base_radius: 0.4,
			bulge_radius: 0.5,
			top_radius: 0.3,
			bulge_position: 0.6,
			segments: 16,
			profile_samples: 8,
			color: [0.3, 0.6, 0.3] // Green
		},
		pistil: {
			length: 1.0,
			base_radius: 0.1,
			tip_radius: 0.08,
			stigma_radius: 0.15,
			segments: 12,
			color: [0.95, 0.9, 0.3],
			pistil_bend: 0.0,
			pistil_droop: 0.0
		},
		stamen: {
			filament_length: 0.8,
			filament_radius: 0.03,
			anther_length: 0.15,
			anther_width: 0.06,
			anther_height: 0.06,
			segments: 10,
			color: [0.95, 0.8, 0.2],
			stamen_bend: 0.0,
			stamen_droop: 0.0
		},
		petal: {
			length: 2.5,
			width: 2.0,
			tip_sharpness: 0.2,
			base_width: 0.8,
			curl: 0.8,
			twist: 5.0,
			ruffle_freq: 3.0,
			ruffle_amp: 0.15,
			resolution: 24,
			color: [0.95, 0.2, 0.3] // Red
		}
	}
};

/**
 * Daisy preset
 *
 * Many simple petals in spiral arrangement.
 * Flat disc-like receptacle with numerous stamens in center.
 */
export const daisyPreset: FlowerPreset = {
	name: 'Daisy',
	description: 'Simple ray petals around flat disc',
	params: {
		diagram: {
			receptacle_height: 0.5,
			receptacle_radius: 0.8,
			petal_whorls: [
				{
					count: 21,
					radius: 1.5,
					height: 0.4,
					pattern: 'GoldenSpiral',
					rotation_offset: 0.0,
					tilt_angle: 0.0
				}
			],
			stamen_whorls: [
				{
					count: 34,
					radius: 0.7,
					height: 0.3,
					pattern: 'GoldenSpiral',
					rotation_offset: 0.5,
					tilt_angle: 0.524 // PI/6 - slight upward angle
				}
			],
			pistil_whorls: [
				{
					count: 13,
					radius: 0.4,
					height: 0.2,
					pattern: 'GoldenSpiral',
					rotation_offset: 1.0,
					tilt_angle: 0.0
				}
			],
			sepal_whorls: []
		},
		receptacle: {
			height: 0.5,
			base_radius: 0.8,
			bulge_radius: 0.8,
			top_radius: 0.8,
			bulge_position: 0.5,
			segments: 16,
			profile_samples: 4,
			color: [0.4, 0.6, 0.3]
		},
		pistil: {
			length: 1.0,
			base_radius: 0.05,
			tip_radius: 0.04,
			stigma_radius: 0.08,
			segments: 8,
			color: [0.85, 0.7, 0.2],
			pistil_bend: 0.0,
			pistil_droop: 0.0
		},
		stamen: {
			filament_length: 0.6,
			filament_radius: 0.02,
			anther_length: 0.12,
			anther_width: 0.04,
			anther_height: 0.04,
			segments: 8,
			color: [0.9, 0.75, 0.2],
			stamen_bend: 0.0,
			stamen_droop: 0.0
		},
		petal: {
			length: 2.0,
			width: 0.6,
			tip_sharpness: 0.3,
			base_width: 0.3,
			curl: 0.0,
			twist: 0.0,
			ruffle_freq: 0.0,
			ruffle_amp: 0.0,
			resolution: 12,
			color: [1.0, 1.0, 1.0] // White
		}
	}
};

/**
 * Tulip preset
 *
 * 6 cup-shaped petals forming elegant chalice.
 * Strong upward curl creates characteristic tulip shape.
 */
export const tulipPreset: FlowerPreset = {
	name: 'Tulip',
	description: 'Cup-shaped with 6 elegant petals',
	params: {
		diagram: {
			receptacle_height: 0.8,
			receptacle_radius: 0.3,
			petal_whorls: [
				{
					count: 6,
					radius: 0.8,
					height: 0.6,
					pattern: 'EvenlySpaced',
					rotation_offset: 0.0,
					tilt_angle: 0.0
				}
			],
			stamen_whorls: [
				{
					count: 6,
					radius: 0.4,
					height: 0.5,
					pattern: 'EvenlySpaced',
					rotation_offset: 0.523599,
					tilt_angle: 0.785 // PI/4 - moderate spread
				}
			],
			pistil_whorls: [
				{
					count: 1,
					radius: 0.0,
					height: 0.4,
					pattern: 'EvenlySpaced',
					rotation_offset: 0.0,
					tilt_angle: 0.0
				}
			],
			sepal_whorls: []
		},
		receptacle: {
			height: 0.8,
			base_radius: 0.3,
			bulge_radius: 0.35,
			top_radius: 0.25,
			bulge_position: 0.4,
			segments: 16,
			profile_samples: 8,
			color: [0.4, 0.65, 0.3]
		},
		pistil: {
			length: 1.5,
			base_radius: 0.12,
			tip_radius: 0.1,
			stigma_radius: 0.15,
			segments: 12,
			color: [0.85, 0.9, 0.3],
			pistil_bend: 0.0,
			pistil_droop: 0.0
		},
		stamen: {
			filament_length: 1.2,
			filament_radius: 0.05,
			anther_length: 0.3,
			anther_width: 0.08,
			anther_height: 0.08,
			segments: 10,
			color: [0.2, 0.2, 0.3], // Dark
			stamen_bend: 0.0,
			stamen_droop: 0.0
		},
		petal: {
			length: 3.5,
			width: 1.8,
			tip_sharpness: 0.5,
			base_width: 0.7,
			curl: -0.6, // Negative curl creates cup shape
			twist: 0.0,
			ruffle_freq: 0.0,
			ruffle_amp: 0.0,
			resolution: 20,
			color: [0.95, 0.3, 0.4] // Pink-red
		}
	}
};

/**
 * Orchid preset
 *
 * Complex asymmetric petals with dramatic twists.
 * Fewer petals but highly detailed and ornate.
 */
export const orchidPreset: FlowerPreset = {
	name: 'Orchid',
	description: 'Exotic twisted petals',
	params: {
		diagram: {
			receptacle_height: 0.6,
			receptacle_radius: 0.2,
			petal_whorls: [
				{
					count: 5,
					radius: 1.2,
					height: 0.5,
					pattern: 'EvenlySpaced',
					rotation_offset: 0.0,
					tilt_angle: 0.0
				}
			],
			stamen_whorls: [
				{
					count: 3,
					radius: 0.3,
					height: 0.4,
					pattern: 'EvenlySpaced',
					rotation_offset: 0.0,
					tilt_angle: 1.047 // PI/3 - dramatic spread for exotic flower
				}
			],
			pistil_whorls: [
				{
					count: 1,
					radius: 0.0,
					height: 0.3,
					pattern: 'EvenlySpaced',
					rotation_offset: 0.0,
					tilt_angle: 0.0
				}
			],
			sepal_whorls: []
		},
		receptacle: {
			height: 0.6,
			base_radius: 0.2,
			bulge_radius: 0.25,
			top_radius: 0.15,
			bulge_position: 0.5,
			segments: 16,
			profile_samples: 8,
			color: [0.5, 0.7, 0.4]
		},
		pistil: {
			length: 1.2,
			base_radius: 0.06,
			tip_radius: 0.05,
			stigma_radius: 0.1,
			segments: 12,
			color: [0.95, 0.9, 0.4],
			pistil_bend: 0.0,
			pistil_droop: 0.0
		},
		stamen: {
			filament_length: 1.0,
			filament_radius: 0.04,
			anther_length: 0.2,
			anther_width: 0.08,
			anther_height: 0.08,
			segments: 10,
			color: [0.9, 0.85, 0.3],
			stamen_bend: 0.0,
			stamen_droop: 0.0
		},
		petal: {
			length: 4.0,
			width: 1.5,
			tip_sharpness: 0.7,
			base_width: 0.5,
			curl: 0.3,
			twist: 35.0, // Dramatic twist
			ruffle_freq: 2.0,
			ruffle_amp: 0.2,
			resolution: 24,
			color: [0.85, 0.4, 0.75] // Purple-pink
		}
	}
};

/**
 * Lily Raceme Inflorescence
 *
 * Classic raceme pattern with stalked lily flowers along vertical axis.
 */
export const lilyRacemePreset: FlowerPreset = {
	name: 'Lily Raceme',
	description: 'Lily flowers along vertical stem with golden spiral',
	params: lilyPreset.params, // Use lily flower
	inflorescence: {
		enabled: true,
		pattern: 'Raceme',
		axis_length: 12.0,
		branch_count: 8,
		angle_top: 45.0,
		angle_bottom: 60.0,
		branch_length_top: 0.8,
		branch_length_bottom: 1.2,
		rotation_angle: 137.5, // Golden angle
		flower_size_top: 0.7,
		flower_size_bottom: 0.9,
		recursion_depth: 1,
		branch_ratio: 0.7,
		angle_divergence: 0.0,
		age_distribution: 0.5
	}
};

/**
 * Lavender Spike Inflorescence
 *
 * Dense spike of small purple tubular flowers.
 */
export const lavenderSpikePreset: FlowerPreset = {
	name: 'Lavender Spike',
	description: 'Dense spike with small purple flowers',
	params: {
		diagram: {
			receptacle_height: 0.4,
			receptacle_radius: 0.15,
			petal_whorls: [
				{
					count: 4,
					radius: 0.5,
					height: 0.6,
					pattern: 'EvenlySpaced',
					rotation_offset: 0.0,
					tilt_angle: 0.0
				}
			],
			stamen_whorls: [
				{
					count: 2,
					radius: 0.2,
					height: 0.5,
					pattern: 'EvenlySpaced',
					rotation_offset: 0.0,
					tilt_angle: 0.785
				}
			],
			pistil_whorls: [
				{
					count: 1,
					radius: 0.0,
					height: 0.4,
					pattern: 'EvenlySpaced',
					rotation_offset: 0.0,
					tilt_angle: 0.0
				}
			],
			sepal_whorls: []
		},
		receptacle: {
			height: 0.4,
			base_radius: 0.15,
			bulge_radius: 0.15,
			top_radius: 0.1,
			bulge_position: 0.5,
			segments: 8,
			profile_samples: 4,
			color: [0.5, 0.7, 0.5]
		},
		pistil: {
			length: 0.6,
			base_radius: 0.03,
			tip_radius: 0.02,
			stigma_radius: 0.04,
			segments: 8,
			color: [0.9, 0.85, 0.4],
			pistil_bend: 0.0,
			pistil_droop: 0.0
		},
		stamen: {
			filament_length: 0.5,
			filament_radius: 0.02,
			anther_length: 0.08,
			anther_width: 0.03,
			anther_height: 0.03,
			segments: 6,
			color: [0.95, 0.9, 0.4],
			stamen_bend: 0.0,
			stamen_droop: 0.0
		},
		petal: {
			length: 1.0,
			width: 0.4,
			tip_sharpness: 0.5,
			base_width: 0.3,
			curl: 0.0,
			twist: 0.0,
			ruffle_freq: 0.0,
			ruffle_amp: 0.0,
			resolution: 12,
			color: [0.65, 0.4, 0.75] // Purple
		}
	},
	inflorescence: {
		enabled: true,
		pattern: 'Spike',
		axis_length: 15.0,
		branch_count: 24,
		angle_top: 15.0,
		angle_bottom: 15.0,
		branch_length_top: 0.0, // Sessile (no pedicels)
		branch_length_bottom: 0.0,
		rotation_angle: 144.0, // Tight spiral
		flower_size_top: 0.5,
		flower_size_bottom: 0.6,
		recursion_depth: 1,
		branch_ratio: 0.7,
		angle_divergence: 0.0,
		age_distribution: 0.5
	}
};

/**
 * Cherry Blossom Umbel
 *
 * Umbrella-like cluster of pink blossoms.
 */
export const cherryUmbelPreset: FlowerPreset = {
	name: 'Cherry Umbel',
	description: 'Umbrella cluster of pink blossoms',
	params: {
		diagram: tulipPreset.params.diagram, // Simple structure
		receptacle: tulipPreset.params.receptacle,
		pistil: tulipPreset.params.pistil,
		stamen: tulipPreset.params.stamen,
		petal: {
			length: 1.5,
			width: 1.0,
			tip_sharpness: 0.2,
			base_width: 0.3,
			curl: 0.0,
			twist: 0.0,
			ruffle_freq: 0.0,
			ruffle_amp: 0.0,
			resolution: 16,
			color: [1.0, 0.8, 0.85] // Pale pink
		}
	},
	inflorescence: {
		enabled: true,
		pattern: 'Umbel',
		axis_length: 2.0, // Short central stem
		branch_count: 5,
		angle_top: 65.0,
		angle_bottom: 65.0,
		branch_length_top: 2.5,
		branch_length_bottom: 2.5, // Equal lengths for umbrella effect
		rotation_angle: 72.0, // Even spacing (360/5)
		flower_size_top: 0.7,
		flower_size_bottom: 0.7,
		recursion_depth: 1,
		branch_ratio: 0.7,
		angle_divergence: 0.0,
		age_distribution: 0.5
	}
};

/**
 * Hydrangea Corymb
 *
 * Flat-topped cluster with varied pedicel lengths.
 */
export const hydrangeaCorymbPreset: FlowerPreset = {
	name: 'Hydrangea Corymb',
	description: 'Flat-topped cluster of small blue flowers',
	params: {
		diagram: {
			receptacle_height: 0.3,
			receptacle_radius: 0.15,
			petal_whorls: [
				{
					count: 4,
					radius: 0.6,
					height: 0.6,
					pattern: 'EvenlySpaced',
					rotation_offset: 0.0,
					tilt_angle: 0.0
				}
			],
			stamen_whorls: [],
			pistil_whorls: [],
			sepal_whorls: []
		},
		receptacle: {
			height: 0.3,
			base_radius: 0.15,
			bulge_radius: 0.15,
			top_radius: 0.12,
			bulge_position: 0.5,
			segments: 8,
			profile_samples: 4,
			color: [0.5, 0.7, 0.5]
		},
		pistil: {
			length: 0.5,
			base_radius: 0.03,
			tip_radius: 0.02,
			stigma_radius: 0.04,
			segments: 6,
			color: [0.9, 0.9, 0.5],
			pistil_bend: 0.0,
			pistil_droop: 0.0
		},
		stamen: {
			filament_length: 0.4,
			filament_radius: 0.02,
			anther_length: 0.06,
			anther_width: 0.02,
			anther_height: 0.02,
			segments: 6,
			color: [0.95, 0.9, 0.4],
			stamen_bend: 0.0,
			stamen_droop: 0.0
		},
		petal: {
			length: 1.2,
			width: 1.0,
			tip_sharpness: 0.1,
			base_width: 0.4,
			curl: 0.0,
			twist: 0.0,
			ruffle_freq: 0.0,
			ruffle_amp: 0.0,
			resolution: 12,
			color: [0.6, 0.7, 0.9] // Light blue
		}
	},
	inflorescence: {
		enabled: true,
		pattern: 'Corymb',
		axis_length: 8.0,
		branch_count: 15,
		angle_top: 70.0,
		angle_bottom: 45.0,
		branch_length_top: 2.0,
		branch_length_bottom: 0.8, // Varied lengths create flat top
		rotation_angle: 137.5,
		flower_size_top: 0.6,
		flower_size_bottom: 0.6,
		recursion_depth: 1,
		branch_ratio: 0.7,
		angle_divergence: 0.0,
		age_distribution: 0.5
	}
};

/**
 * Astilbe Compound Raceme
 *
 * Hierarchical raceme-of-racemes creating plume effect.
 */
export const astilbeCompoundPreset: FlowerPreset = {
	name: 'Astilbe Plume',
	description: 'Feathery compound raceme',
	params: {
		diagram: {
			receptacle_height: 0.2,
			receptacle_radius: 0.1,
			petal_whorls: [
				{
					count: 4,
					radius: 0.3,
					height: 0.5,
					pattern: 'EvenlySpaced',
					rotation_offset: 0.0,
					tilt_angle: 0.0
				}
			],
			stamen_whorls: [
				{
					count: 4,
					radius: 0.15,
					height: 0.4,
					pattern: 'EvenlySpaced',
					rotation_offset: 0.785,
					tilt_angle: 0.785
				}
			],
			pistil_whorls: [],
			sepal_whorls: []
		},
		receptacle: {
			height: 0.2,
			base_radius: 0.1,
			bulge_radius: 0.1,
			top_radius: 0.08,
			bulge_position: 0.5,
			segments: 6,
			profile_samples: 4,
			color: [0.5, 0.7, 0.5]
		},
		pistil: {
			length: 0.3,
			base_radius: 0.02,
			tip_radius: 0.015,
			stigma_radius: 0.025,
			segments: 6,
			color: [0.9, 0.85, 0.4],
			pistil_bend: 0.0,
			pistil_droop: 0.0
		},
		stamen: {
			filament_length: 0.4,
			filament_radius: 0.015,
			anther_length: 0.05,
			anther_width: 0.02,
			anther_height: 0.02,
			segments: 6,
			color: [0.95, 0.9, 0.4],
			stamen_bend: 0.0,
			stamen_droop: 0.0
		},
		petal: {
			length: 0.6,
			width: 0.3,
			tip_sharpness: 0.3,
			base_width: 0.2,
			curl: 0.0,
			twist: 0.0,
			ruffle_freq: 0.0,
			ruffle_amp: 0.0,
			resolution: 10,
			color: [0.95, 0.6, 0.7] // Pink
		}
	},
	inflorescence: {
		enabled: true,
		pattern: 'CompoundRaceme',
		axis_length: 14.0,
		branch_count: 10,
		angle_top: 30.0,
		angle_bottom: 45.0,
		branch_length_top: 0.6,
		branch_length_bottom: 1.0,
		rotation_angle: 137.5,
		flower_size_top: 0.7,
		flower_size_bottom: 0.8,
		recursion_depth: 2,
		branch_ratio: 0.5,
		angle_divergence: 0.0, // Not used for compound raceme
		age_distribution: 0.5
	}
};

/**
 * Allium Umbel
 *
 * Dense spherical umbel with many small flowers (30 branches).
 * Inspired by ornamental onion flowers.
 */
export const alliumUmbelPreset: FlowerPreset = {
	name: 'Allium Umbel',
	description: 'Dense spherical cluster (30 flowers)',
	params: {
		diagram: {
			receptacle_height: 0.15,
			receptacle_radius: 0.08,
			petal_whorls: [
				{
					count: 6,
					radius: 0.25,
					height: 0.3,
					pattern: 'EvenlySpaced',
					rotation_offset: 0.0,
					tilt_angle: 0.0
				}
			],
			stamen_whorls: [
				{
					count: 6,
					radius: 0.12,
					height: 0.25,
					pattern: 'EvenlySpaced',
					rotation_offset: 0.524,
					tilt_angle: 0.785
				}
			],
			pistil_whorls: [
				{
					count: 1,
					radius: 0.0,
					height: 0.2,
					pattern: 'EvenlySpaced',
					rotation_offset: 0.0,
					tilt_angle: 0.0
				}
			],
			sepal_whorls: []
		},
		receptacle: {
			height: 0.15,
			base_radius: 0.08,
			bulge_radius: 0.08,
			top_radius: 0.06,
			bulge_position: 0.5,
			segments: 8,
			profile_samples: 4,
			color: [0.5, 0.7, 0.5]
		},
		pistil: {
			length: 0.6,
			base_radius: 0.02,
			tip_radius: 0.015,
			stigma_radius: 0.025,
			segments: 6,
			color: [0.9, 0.85, 0.4],
			pistil_bend: 0.0,
			pistil_droop: 0.0
		},
		stamen: {
			filament_length: 0.5,
			filament_radius: 0.015,
			anther_length: 0.04,
			anther_width: 0.015,
			anther_height: 0.015,
			segments: 6,
			color: [0.95, 0.9, 0.4],
			stamen_bend: 0.0,
			stamen_droop: 0.0
		},
		petal: {
			length: 0.5,
			width: 0.2,
			tip_sharpness: 0.3,
			base_width: 0.15,
			curl: 0.0,
			twist: 0.0,
			ruffle_freq: 0.0,
			ruffle_amp: 0.0,
			resolution: 12,
			color: [0.7, 0.4, 0.85] // Purple
		}
	},
	inflorescence: {
		enabled: true,
		pattern: 'Umbel',
		axis_length: 1.5,
		branch_count: 30,
		angle_top: 70.0, // Wide angle for spherical appearance
		angle_bottom: 70.0,
		branch_length_top: 2.0,
		branch_length_bottom: 2.0,
		rotation_angle: 12.0, // Even spacing (360/30)
		flower_size_top: 0.3, // Small flowers for density
		flower_size_bottom: 0.3,
		recursion_depth: 1,
		branch_ratio: 0.7,
		angle_divergence: 0.0,
		age_distribution: 0.5
	}
};

/**
 * All available presets
 */
export const presets: Record<string, FlowerPreset> = {
	lily: lilyPreset,
	rose: rosePreset,
	daisy: daisyPreset,
	tulip: tulipPreset,
	orchid: orchidPreset,
	'lily-raceme': lilyRacemePreset,
	'lavender-spike': lavenderSpikePreset,
	'cherry-umbel': cherryUmbelPreset,
	'hydrangea-corymb': hydrangeaCorymbPreset,
	'astilbe-plume': astilbeCompoundPreset,
	'allium-umbel': alliumUmbelPreset
};

/**
 * Preset names in display order
 */
export const presetNames = [
	'lily',
	'rose',
	'daisy',
	'tulip',
	'orchid',
	'lily-raceme',
	'lavender-spike',
	'cherry-umbel',
	'hydrangea-corymb',
	'astilbe-plume',
	'allium-umbel',
	'custom'
] as const;

export type PresetName = (typeof presetNames)[number];
