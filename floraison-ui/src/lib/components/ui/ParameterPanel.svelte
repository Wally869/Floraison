<script lang="ts">
	import {
		diagramParams,
		receptacleParams,
		pistilParams,
		stamenParams,
		petalParams,
		resetToDefaults,
		loadParams,
		rgbToHex,
		hexToRgb,
		allParams,
		inflorescenceParams,
		isRecursivePattern,
		getRecursiveDefaults,
		currentPresetName
	} from '$lib/stores/parameters';
	import { presets, presetNames, type PresetName } from '$lib/presets';
	import { generateRandomFlowerParams } from '$lib/utils/random';

	// Props
	interface Props {
		open?: boolean;
	}

	let { open = true }: Props = $props();

	// Component-level reactive variables for color pickers (hex format)
	let receptacleColor = $state(rgbToHex($receptacleParams.color));
	let pistilColor = $state(rgbToHex($pistilParams.color));
	let stamenColor = $state(rgbToHex($stamenParams.color));
	let petalColor = $state(rgbToHex($petalParams.color));

	// Preset selection
	let selectedPreset = $state<PresetName>('lily');
	let lastLoadedParams: string | null = null;
	let isLoadingPreset = false;

	// Update store when color picker changes
	function updateReceptacleColor(hex: string) {
		receptacleColor = hex;
		$receptacleParams.color = hexToRgb(hex);
	}

	function updatePistilColor(hex: string) {
		pistilColor = hex;
		$pistilParams.color = hexToRgb(hex);
	}

	function updateStamenColor(hex: string) {
		stamenColor = hex;
		$stamenParams.color = hexToRgb(hex);
	}

	function updatePetalColor(hex: string) {
		petalColor = hex;
		$petalParams.color = hexToRgb(hex);
	}

	// Load selected preset
	function loadPreset() {
		if (selectedPreset === 'custom') return;

		const preset = presets[selectedPreset];
		if (preset) {
			isLoadingPreset = true;
			loadParams(preset.params);

			// Update color pickers to match loaded preset
			receptacleColor = rgbToHex(preset.params.receptacle.color);
			pistilColor = rgbToHex(preset.params.pistil.color);
			stamenColor = rgbToHex(preset.params.stamen.color);
			petalColor = rgbToHex(preset.params.petal.color);

			// Load inflorescence params if present
			if (preset.inflorescence) {
				$inflorescenceParams = preset.inflorescence;
			} else {
				// Disable inflorescence mode for flower-only presets
				$inflorescenceParams.enabled = false;
			}

			// Update current preset name for export
			$currentPresetName = selectedPreset;

			// Wait for stores to update, THEN snapshot params
			setTimeout(() => {
				lastLoadedParams = JSON.stringify($allParams);
				isLoadingPreset = false;
			}, 100);
		}
	}

	// Randomize all parameters
	function randomizeParameters() {
		isLoadingPreset = true; // Prevent "custom" detection

		const randomParams = generateRandomFlowerParams();

		// Update all stores
		$diagramParams = randomParams.diagram;
		$receptacleParams = randomParams.receptacle;
		$pistilParams = randomParams.pistil;
		$stamenParams = randomParams.stamen;
		$petalParams = randomParams.petal;
		$inflorescenceParams = randomParams.inflorescence;

		// Update color pickers
		receptacleColor = rgbToHex(randomParams.receptacle.color);
		pistilColor = rgbToHex(randomParams.pistil.color);
		stamenColor = rgbToHex(randomParams.stamen.color);
		petalColor = rgbToHex(randomParams.petal.color);

		// Set preset to custom
		selectedPreset = 'custom';
		$currentPresetName = 'custom';

		setTimeout(() => {
			lastLoadedParams = JSON.stringify($allParams);
			isLoadingPreset = false;
		}, 100);
	}

	// Detect parameter changes to auto-switch to "Custom"
	$effect(() => {
		const currentParams = JSON.stringify($allParams);

		// Skip detection during preset load
		if (isLoadingPreset) return;

		// If params changed and we're not on custom already
		if (
			selectedPreset !== 'custom' &&
			lastLoadedParams !== null &&
			currentParams !== lastLoadedParams
		) {
			selectedPreset = 'custom';
			$currentPresetName = 'custom';
		}
	});

	// Load lily preset on mount
	$effect(() => {
		if (lastLoadedParams === null) {
			loadPreset();
		}
	});
</script>

<div class="parameter-panel" class:open>
	<div class="panel-header">
		<h2 class="text-2xl font-bold text-gray-800">Flower Parameters</h2>
		<div class="button-group">
			<button
				onclick={randomizeParameters}
				class="px-3 py-1 text-sm bg-purple-500 hover:bg-purple-600 text-white rounded transition-colors"
				title="Generate random flower"
			>
				🎲 Random
			</button>
			<button
				onclick={resetToDefaults}
				class="px-3 py-1 text-sm bg-gray-200 hover:bg-gray-300 rounded transition-colors"
			>
				Reset
			</button>
		</div>
	</div>

	<!-- Preset Selector -->
	<div class="preset-selector">
		<label for="preset-select" class="preset-label">Preset:</label>
		<select
			id="preset-select"
			value={selectedPreset}
			onchange={(e) => {
				isLoadingPreset = true; // Set flag BEFORE updating value
				selectedPreset = e.currentTarget.value as PresetName;
				loadPreset();
			}}
			class="preset-dropdown"
		>
			<optgroup label="Single Flowers">
				<option value="lily">{presets['lily'].name}</option>
				<option value="rose">{presets['rose'].name}</option>
				<option value="daisy">{presets['daisy'].name}</option>
				<option value="tulip">{presets['tulip'].name}</option>
				<option value="orchid">{presets['orchid'].name}</option>
			</optgroup>
			<optgroup label="Inflorescences">
				<option value="lily-raceme">{presets['lily-raceme'].name}</option>
				<option value="lavender-spike">{presets['lavender-spike'].name}</option>
				<option value="cherry-umbel">{presets['cherry-umbel'].name}</option>
				<option value="hydrangea-corymb">{presets['hydrangea-corymb'].name}</option>
				<option value="astilbe-plume">{presets['astilbe-plume'].name}</option>
			</optgroup>
			<option value="custom">Custom</option>
		</select>
		{#if selectedPreset !== 'custom' && presets[selectedPreset]}
			<p class="preset-description">{presets[selectedPreset].description}</p>
		{/if}
	</div>

	<!-- Generation Mode Toggle -->
	<details open>
		<summary class="section-header">Generation Mode</summary>
		<div class="section-content">
			<div class="mode-toggle">
				<label class="mode-option">
					<input type="radio" bind:group={$inflorescenceParams.enabled} value={false} />
					<span>Single Flower</span>
				</label>
				<label class="mode-option">
					<input type="radio" bind:group={$inflorescenceParams.enabled} value={true} />
					<span>Inflorescence</span>
				</label>
			</div>
		</div>
	</details>

	<!-- Inflorescence Parameters (conditional) -->
	{#if $inflorescenceParams.enabled}
		<details open>
			<summary class="section-header">Inflorescence Pattern</summary>
			<div class="section-content">
				<div class="param-group">
					<label for="inflo-pattern" class="param-label">Pattern Type</label>
					<select id="inflo-pattern" bind:value={$inflorescenceParams.pattern} class="param-select">
						<optgroup label="Simple Patterns">
							<option value="Raceme">Raceme (stalked along axis)</option>
							<option value="Spike">Spike (sessile along axis)</option>
							<option value="Umbel">Umbel (umbrella-like)</option>
							<option value="Corymb">Corymb (flat-topped)</option>
						</optgroup>
						<optgroup label="Determinate Patterns">
							<option value="Dichasium">Dichasium (Y-branching)</option>
							<option value="Drepanium">Drepanium (spiral helix)</option>
						</optgroup>
						<optgroup label="Compound Patterns">
							<option value="CompoundRaceme">Compound Raceme</option>
							<option value="CompoundUmbel">Compound Umbel</option>
						</optgroup>
					</select>
				</div>

				<div class="param-group">
					<label for="inflo-axis-length">
						<span class="param-label">Axis Length</span>
						<span class="param-value">{$inflorescenceParams.axis_length.toFixed(1)}</span>
					</label>
					<input
						id="inflo-axis-length"
						type="range"
						min="1"
						max="20"
						step="0.5"
						bind:value={$inflorescenceParams.axis_length}
						class="param-slider"
					/>
				</div>

				<div class="param-group">
					<label for="inflo-branch-count">
						<span class="param-label">Branch Count</span>
						<span class="param-value">{$inflorescenceParams.branch_count}</span>
					</label>
					<input
						id="inflo-branch-count"
						type="range"
						min="3"
						max="30"
						bind:value={$inflorescenceParams.branch_count}
						class="param-slider"
					/>
				</div>

				<div class="param-group">
					<label for="inflo-rotation">
						<span class="param-label">Rotation Angle (deg)</span>
						<span class="param-value">{$inflorescenceParams.rotation_angle.toFixed(1)}°</span>
					</label>
					<input
						id="inflo-rotation"
						type="range"
						min="0"
						max="360"
						step="5"
						bind:value={$inflorescenceParams.rotation_angle}
						class="param-slider"
					/>
					<div class="preset-buttons">
						<button
							class="preset-btn"
							onclick={() => ($inflorescenceParams.rotation_angle = 137.5)}
						>
							Golden (137.5°)
						</button>
						<button
							class="preset-btn"
							onclick={() =>
								($inflorescenceParams.rotation_angle =
									360 / $inflorescenceParams.branch_count)}
						>
							Even
						</button>
					</div>
				</div>

				<div class="param-group">
					<label for="inflo-branch-length-top">
						<span class="param-label">Branch Length (Top)</span>
						<span class="param-value">{$inflorescenceParams.branch_length_top.toFixed(2)}</span>
					</label>
					<input
						id="inflo-branch-length-top"
						type="range"
						min="0.1"
						max="3.0"
						step="0.1"
						bind:value={$inflorescenceParams.branch_length_top}
						class="param-slider"
					/>
				</div>

				<div class="param-group">
					<label for="inflo-branch-length-bottom">
						<span class="param-label">Branch Length (Bottom)</span>
						<span class="param-value">{$inflorescenceParams.branch_length_bottom.toFixed(2)}</span>
					</label>
					<input
						id="inflo-branch-length-bottom"
						type="range"
						min="0.1"
						max="3.0"
						step="0.1"
						bind:value={$inflorescenceParams.branch_length_bottom}
						class="param-slider"
					/>
				</div>

				<div class="param-group">
					<label for="inflo-flower-size-top">
						<span class="param-label">Flower Size (Top)</span>
						<span class="param-value">{$inflorescenceParams.flower_size_top.toFixed(2)}</span>
					</label>
					<input
						id="inflo-flower-size-top"
						type="range"
						min="0.3"
						max="1.5"
						step="0.05"
						bind:value={$inflorescenceParams.flower_size_top}
						class="param-slider"
					/>
				</div>

				<div class="param-group">
					<label for="inflo-age-distribution">
						<span class="param-label">Age Distribution</span>
						<span class="param-value">{$inflorescenceParams.age_distribution.toFixed(2)}</span>
					</label>
					<input
						id="inflo-age-distribution"
						type="range"
						min="0"
						max="1"
						step="0.01"
						bind:value={$inflorescenceParams.age_distribution}
						class="param-slider"
					/>
					<p class="param-help">
						💡 0.0 = all buds, 0.5 = natural gradient, 1.0 = all blooms
					</p>
				</div>

				<!-- Recursive pattern parameters (conditional) -->
				{#if isRecursivePattern($inflorescenceParams.pattern)}
					<div class="recursive-params">
						<p class="section-subtitle">Recursive Parameters</p>

						<div class="param-group">
							<label for="inflo-recursion-depth">
								<span class="param-label">Recursion Depth</span>
								<span class="param-value">{$inflorescenceParams.recursion_depth}</span>
							</label>
							<input
								id="inflo-recursion-depth"
								type="range"
								min="1"
								max="5"
								bind:value={$inflorescenceParams.recursion_depth}
								class="param-slider"
							/>
							<p class="param-help">💡 Higher values create more complex structures (may be slower)</p>
						</div>

						<div class="param-group">
							<label for="inflo-branch-ratio">
								<span class="param-label">Branch Ratio</span>
								<span class="param-value">{$inflorescenceParams.branch_ratio.toFixed(2)}</span>
							</label>
							<input
								id="inflo-branch-ratio"
								type="range"
								min="0.3"
								max="0.95"
								step="0.05"
								bind:value={$inflorescenceParams.branch_ratio}
								class="param-slider"
							/>
						</div>

						{#if $inflorescenceParams.pattern === 'Dichasium'}
							<div class="param-group">
								<label for="inflo-angle-div">
									<span class="param-label">Angle Divergence (deg)</span>
									<span class="param-value">{$inflorescenceParams.angle_divergence.toFixed(1)}°</span>
								</label>
								<input
									id="inflo-angle-div"
									type="range"
									min="10"
									max="60"
									step="5"
									bind:value={$inflorescenceParams.angle_divergence}
									class="param-slider"
								/>
							</div>
						{/if}
					</div>
				{/if}
			</div>
		</details>
	{/if}

	<!-- Flower Structure Section -->
	<details open>
		<summary class="section-header">Flower Structure</summary>
		<div class="section-content">
			<div class="param-group">
				<label for="pistil-count">
					<span class="param-label">Pistils</span>
					<span class="param-value">{$diagramParams.pistilCount}</span>
				</label>
				<input
					id="pistil-count"
					type="number"
					min="0"
					max="5"
					bind:value={$diagramParams.pistilCount}
					class="param-input"
				/>
			</div>

			<div class="param-group">
				<label for="stamen-count">
					<span class="param-label">Stamens</span>
					<span class="param-value">{$diagramParams.stamenCount}</span>
				</label>
				<input
					id="stamen-count"
					type="range"
					min="0"
					max="30"
					bind:value={$diagramParams.stamenCount}
					class="param-slider"
				/>
			</div>

			<div class="param-group">
				<label for="petal-count">
					<span class="param-label">Petals</span>
					<span class="param-value">{$diagramParams.petalCount}</span>
				</label>
				<input
					id="petal-count"
					type="range"
					min="3"
					max="30"
					bind:value={$diagramParams.petalCount}
					class="param-slider"
				/>
			</div>

			<div class="param-group">
				<label for="sepal-count">
					<span class="param-label">Sepals</span>
					<span class="param-value">{$diagramParams.sepalCount}</span>
				</label>
				<input
					id="sepal-count"
					type="range"
					min="0"
					max="10"
					bind:value={$diagramParams.sepalCount}
					class="param-slider"
				/>
			</div>

			<div class="param-group">
				<label for="stamen-tilt">
					<span class="param-label">Stamen Tilt (deg)</span>
					<span class="param-value">{$diagramParams.stamenTilt}°</span>
				</label>
				<input
					id="stamen-tilt"
					type="range"
					min="0"
					max="90"
					bind:value={$diagramParams.stamenTilt}
					class="param-slider"
				/>
				<p class="param-help">0° = upright, 90° = spreading</p>
			</div>
		</div>
	</details>

	<!-- Receptacle Section -->
	<details>
		<summary class="section-header">Receptacle</summary>
		<div class="section-content">
			<div class="param-group">
				<label for="rec-height">
					<span class="param-label">Height</span>
					<span class="param-value">{$receptacleParams.height.toFixed(2)}</span>
				</label>
				<input
					id="rec-height"
					type="range"
					min="0.2"
					max="2.0"
					step="0.1"
					bind:value={$receptacleParams.height}
					class="param-slider"
				/>
			</div>

			<div class="param-group">
				<label for="rec-base-radius">
					<span class="param-label">Base Radius</span>
					<span class="param-value">{$receptacleParams.base_radius.toFixed(2)}</span>
				</label>
				<input
					id="rec-base-radius"
					type="range"
					min="0.1"
					max="1.0"
					step="0.05"
					bind:value={$receptacleParams.base_radius}
					class="param-slider"
				/>
			</div>

			<div class="param-group">
				<label for="rec-bulge-radius">
					<span class="param-label">Bulge Radius</span>
					<span class="param-value">{$receptacleParams.bulge_radius.toFixed(2)}</span>
				</label>
				<input
					id="rec-bulge-radius"
					type="range"
					min="0.1"
					max="1.0"
					step="0.05"
					bind:value={$receptacleParams.bulge_radius}
					class="param-slider"
				/>
			</div>

			<div class="param-group">
				<label for="rec-top-radius">
					<span class="param-label">Top Radius</span>
					<span class="param-value">{$receptacleParams.top_radius.toFixed(2)}</span>
				</label>
				<input
					id="rec-top-radius"
					type="range"
					min="0.05"
					max="0.8"
					step="0.05"
					bind:value={$receptacleParams.top_radius}
					class="param-slider"
				/>
			</div>

			<div class="param-group">
				<label for="rec-bulge-pos">
					<span class="param-label">Bulge Position</span>
					<span class="param-value">{$receptacleParams.bulge_position.toFixed(2)}</span>
				</label>
				<input
					id="rec-bulge-pos"
					type="range"
					min="0.0"
					max="1.0"
					step="0.1"
					bind:value={$receptacleParams.bulge_position}
					class="param-slider"
				/>
			</div>

			<div class="param-group">
				<label for="rec-segments">
					<span class="param-label">Segments</span>
					<span class="param-value">{$receptacleParams.segments}</span>
				</label>
				<input
					id="rec-segments"
					type="range"
					min="8"
					max="32"
					step="2"
					bind:value={$receptacleParams.segments}
					class="param-slider"
				/>
			</div>

			<div class="param-group">
				<label for="rec-color">
					<span class="param-label">Color</span>
				</label>
				<input
					id="rec-color"
					type="color"
					value={receptacleColor}
					onchange={(e) => updateReceptacleColor(e.currentTarget.value)}
					class="param-color"
				/>
			</div>
		</div>
	</details>

	<!-- Pistil Section -->
	<details>
		<summary class="section-header">Pistil</summary>
		<div class="section-content">
			<div class="param-group">
				<label for="pistil-length">
					<span class="param-label">Length</span>
					<span class="param-value">{$pistilParams.length.toFixed(2)}</span>
				</label>
				<input
					id="pistil-length"
					type="range"
					min="0.5"
					max="4.0"
					step="0.1"
					bind:value={$pistilParams.length}
					class="param-slider"
				/>
			</div>

			<div class="param-group">
				<label for="pistil-base-radius">
					<span class="param-label">Base Radius</span>
					<span class="param-value">{$pistilParams.base_radius.toFixed(3)}</span>
				</label>
				<input
					id="pistil-base-radius"
					type="range"
					min="0.02"
					max="0.2"
					step="0.01"
					bind:value={$pistilParams.base_radius}
					class="param-slider"
				/>
			</div>

			<div class="param-group">
				<label for="pistil-tip-radius">
					<span class="param-label">Tip Radius</span>
					<span class="param-value">{$pistilParams.tip_radius.toFixed(3)}</span>
				</label>
				<input
					id="pistil-tip-radius"
					type="range"
					min="0.02"
					max="0.2"
					step="0.01"
					bind:value={$pistilParams.tip_radius}
					class="param-slider"
				/>
			</div>

			<div class="param-group">
				<label for="pistil-stigma-radius">
					<span class="param-label">Stigma Radius</span>
					<span class="param-value">{$pistilParams.stigma_radius.toFixed(3)}</span>
				</label>
				<input
					id="pistil-stigma-radius"
					type="range"
					min="0.05"
					max="0.3"
					step="0.01"
					bind:value={$pistilParams.stigma_radius}
					class="param-slider"
				/>
			</div>

			<div class="param-group">
				<label for="pistil-bend">
					<span class="param-label">Pistil Bend</span>
					<span class="param-value">{$pistilParams.pistil_bend.toFixed(2)}</span>
				</label>
				<input
					id="pistil-bend"
					type="range"
					min="0"
					max="1"
					step="0.05"
					bind:value={$pistilParams.pistil_bend}
					class="param-slider"
				/>
				<p class="param-help">💡 0 = straight, 0.5 = gentle curve, 1.0 = dramatic arc</p>
			</div>

			<div class="param-group">
				<label for="pistil-droop">
					<span class="param-label">Pistil Droop</span>
					<span class="param-value">{$pistilParams.pistil_droop.toFixed(2)}</span>
				</label>
				<input
					id="pistil-droop"
					type="range"
					min="-1"
					max="1"
					step="0.05"
					bind:value={$pistilParams.pistil_droop}
					class="param-slider"
				/>
				<p class="param-help">💡 -1 = droop down, 0 = straight, 1 = lift up</p>
			</div>

			<div class="param-group">
				<label for="pistil-segments">
					<span class="param-label">Segments</span>
					<span class="param-value">{$pistilParams.segments}</span>
				</label>
				<input
					id="pistil-segments"
					type="range"
					min="6"
					max="20"
					step="2"
					bind:value={$pistilParams.segments}
					class="param-slider"
				/>
			</div>

			<div class="param-group">
				<label for="pistil-color">
					<span class="param-label">Color</span>
				</label>
				<input
					id="pistil-color"
					type="color"
					value={pistilColor}
					onchange={(e) => updatePistilColor(e.currentTarget.value)}
					class="param-color"
				/>
			</div>
		</div>
	</details>

	<!-- Stamen Section -->
	<details>
		<summary class="section-header">Stamen</summary>
		<div class="section-content">
			<div class="param-group">
				<label for="stamen-filament-length">
					<span class="param-label">Filament Length</span>
					<span class="param-value">{$stamenParams.filament_length.toFixed(2)}</span>
				</label>
				<input
					id="stamen-filament-length"
					type="range"
					min="0.3"
					max="3.0"
					step="0.1"
					bind:value={$stamenParams.filament_length}
					class="param-slider"
				/>
			</div>

			<div class="param-group">
				<label for="stamen-filament-radius">
					<span class="param-label">Filament Radius</span>
					<span class="param-value">{$stamenParams.filament_radius.toFixed(3)}</span>
				</label>
				<input
					id="stamen-filament-radius"
					type="range"
					min="0.02"
					max="0.1"
					step="0.01"
					bind:value={$stamenParams.filament_radius}
					class="param-slider"
				/>
			</div>

			<div class="param-group">
				<label for="stamen-anther-length">
					<span class="param-label">Anther Length</span>
					<span class="param-value">{$stamenParams.anther_length.toFixed(3)}</span>
				</label>
				<input
					id="stamen-anther-length"
					type="range"
					min="0.1"
					max="0.5"
					step="0.05"
					bind:value={$stamenParams.anther_length}
					class="param-slider"
				/>
			</div>

			<div class="param-group">
				<label for="stamen-anther-width">
					<span class="param-label">Anther Width</span>
					<span class="param-value">{$stamenParams.anther_width.toFixed(3)}</span>
				</label>
				<input
					id="stamen-anther-width"
					type="range"
					min="0.05"
					max="0.2"
					step="0.01"
					bind:value={$stamenParams.anther_width}
					class="param-slider"
				/>
			</div>

			<div class="param-group">
				<label for="stamen-bend">
					<span class="param-label">Stamen Bend</span>
					<span class="param-value">{$stamenParams.stamen_bend.toFixed(2)}</span>
				</label>
				<input
					id="stamen-bend"
					type="range"
					min="0"
					max="1"
					step="0.05"
					bind:value={$stamenParams.stamen_bend}
					class="param-slider"
				/>
				<p class="param-help">💡 0 = straight, 0.5 = gentle curve, 1.0 = dramatic arc</p>
			</div>

			<div class="param-group">
				<label for="stamen-droop">
					<span class="param-label">Stamen Droop</span>
					<span class="param-value">{$stamenParams.stamen_droop.toFixed(2)}</span>
				</label>
				<input
					id="stamen-droop"
					type="range"
					min="-1"
					max="1"
					step="0.05"
					bind:value={$stamenParams.stamen_droop}
					class="param-slider"
				/>
				<p class="param-help">💡 -1 = droop down, 0 = straight, 1 = lift up</p>
			</div>

			<div class="param-group">
				<label for="stamen-segments">
					<span class="param-label">Segments</span>
					<span class="param-value">{$stamenParams.segments}</span>
				</label>
				<input
					id="stamen-segments"
					type="range"
					min="6"
					max="20"
					step="2"
					bind:value={$stamenParams.segments}
					class="param-slider"
				/>
			</div>

			<div class="param-group">
				<label for="stamen-color">
					<span class="param-label">Color</span>
				</label>
				<input
					id="stamen-color"
					type="color"
					value={stamenColor}
					onchange={(e) => updateStamenColor(e.currentTarget.value)}
					class="param-color"
				/>
			</div>
		</div>
	</details>

	<!-- Petal Section -->
	<details>
		<summary class="section-header">Petal</summary>
		<div class="section-content">
			<div class="param-group">
				<label for="petal-length">
					<span class="param-label">Length</span>
					<span class="param-value">{$petalParams.length.toFixed(2)}</span>
				</label>
				<input
					id="petal-length"
					type="range"
					min="1.0"
					max="5.0"
					step="0.1"
					bind:value={$petalParams.length}
					class="param-slider"
				/>
			</div>

			<div class="param-group">
				<label for="petal-width">
					<span class="param-label">Width</span>
					<span class="param-value">{$petalParams.width.toFixed(2)}</span>
				</label>
				<input
					id="petal-width"
					type="range"
					min="0.5"
					max="3.0"
					step="0.1"
					bind:value={$petalParams.width}
					class="param-slider"
				/>
			</div>

			<div class="param-group">
				<label for="petal-tip-sharpness">
					<span class="param-label">Tip Sharpness</span>
					<span class="param-value">{$petalParams.tip_sharpness.toFixed(2)}</span>
				</label>
				<input
					id="petal-tip-sharpness"
					type="range"
					min="0.0"
					max="1.0"
					step="0.1"
					bind:value={$petalParams.tip_sharpness}
					class="param-slider"
				/>
			</div>

			<div class="param-group">
				<label for="petal-base-width">
					<span class="param-label">Base Width</span>
					<span class="param-value">{$petalParams.base_width.toFixed(2)}</span>
				</label>
				<input
					id="petal-base-width"
					type="range"
					min="0.1"
					max="1.0"
					step="0.1"
					bind:value={$petalParams.base_width}
					class="param-slider"
				/>
			</div>

			<div class="param-group">
				<label for="petal-curl">
					<span class="param-label">Curl</span>
					<span class="param-value">{$petalParams.curl.toFixed(2)}</span>
				</label>
				<input
					id="petal-curl"
					type="range"
					min="-1.0"
					max="1.0"
					step="0.1"
					bind:value={$petalParams.curl}
					class="param-slider"
				/>
			</div>

			<div class="param-group">
				<label for="petal-twist">
					<span class="param-label">Twist (deg)</span>
					<span class="param-value">{$petalParams.twist.toFixed(1)}</span>
				</label>
				<input
					id="petal-twist"
					type="range"
					min="-45"
					max="45"
					step="5"
					bind:value={$petalParams.twist}
					class="param-slider"
				/>
			</div>

			<div class="param-group">
				<label for="petal-ruffle-freq">
					<span class="param-label">Ruffle Frequency</span>
					<span class="param-value">{$petalParams.ruffle_freq.toFixed(1)}</span>
				</label>
				<input
					id="petal-ruffle-freq"
					type="range"
					min="0.0"
					max="5.0"
					step="0.5"
					bind:value={$petalParams.ruffle_freq}
					class="param-slider"
				/>
			</div>

			<div class="param-group">
				<label for="petal-ruffle-amp">
					<span class="param-label">Ruffle Amplitude</span>
					<span class="param-value">{$petalParams.ruffle_amp.toFixed(2)}</span>
				</label>
				<input
					id="petal-ruffle-amp"
					type="range"
					min="0.0"
					max="0.5"
					step="0.05"
					bind:value={$petalParams.ruffle_amp}
					class="param-slider"
				/>
			</div>

			<div class="param-group">
				<label for="petal-resolution">
					<span class="param-label">Resolution</span>
					<span class="param-value">{$petalParams.resolution}</span>
				</label>
				<input
					id="petal-resolution"
					type="range"
					min="8"
					max="32"
					step="2"
					bind:value={$petalParams.resolution}
					class="param-slider"
				/>
			</div>

			<div class="param-group">
				<label for="petal-color">
					<span class="param-label">Color</span>
				</label>
				<input
					id="petal-color"
					type="color"
					value={petalColor}
					onchange={(e) => updatePetalColor(e.currentTarget.value)}
					class="param-color"
				/>
			</div>
		</div>
	</details>
</div>

<style>
	.parameter-panel {
		width: 320px;
		height: 100vh;
		overflow-y: auto;
		background-color: #f9fafb;
		border-right: 1px solid #e5e7eb;
		padding: 1rem;

		/* Mobile: fixed positioned, slides in from left */
		position: fixed;
		left: 0;
		top: 0;
		z-index: 50;
		transform: translateX(-100%);
		transition: transform 0.3s ease-out;
	}

	/* Desktop: static positioned, always visible */
	@media (min-width: 768px) {
		.parameter-panel {
			position: static;
			transform: translateX(0);
			transition: none;
		}
	}

	/* Mobile: when open */
	.parameter-panel.open {
		transform: translateX(0);
	}

	.panel-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1.5rem;
		padding-bottom: 1rem;
		border-bottom: 2px solid #e5e7eb;
	}

	.button-group {
		display: flex;
		gap: 0.5rem;
	}

	details {
		margin-bottom: 0.5rem;
		background-color: white;
		border-radius: 0.5rem;
		border: 1px solid #e5e7eb;
	}

	details[open] {
		box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1);
	}

	.section-header {
		cursor: pointer;
		padding: 0.75rem 1rem;
		font-weight: 600;
		color: #374151;
		user-select: none;
		transition: background-color 0.15s;
	}

	.section-header:hover {
		background-color: #f3f4f6;
	}

	details[open] .section-header {
		border-bottom: 1px solid #e5e7eb;
	}

	.section-content {
		padding: 1rem;
	}

	.param-group {
		margin-bottom: 1rem;
	}

	.param-group:last-child {
		margin-bottom: 0;
	}

	.param-group label {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 0.25rem;
		font-size: 0.875rem;
		color: #6b7280;
	}

	.param-label {
		font-weight: 500;
	}

	.param-value {
		font-family: 'Courier New', monospace;
		font-size: 0.875rem;
		color: #374151;
		background-color: #f3f4f6;
		padding: 0.125rem 0.5rem;
		border-radius: 0.25rem;
	}

	.param-slider {
		width: 100%;
		height: 1.5rem;
		cursor: pointer;
	}

	.param-input {
		width: 100%;
		padding: 0.5rem;
		border: 1px solid #d1d5db;
		border-radius: 0.375rem;
		font-size: 0.875rem;
	}

	.param-input:focus {
		outline: none;
		border-color: #3b82f6;
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
	}

	.param-color {
		width: 100%;
		height: 2.5rem;
		border: 1px solid #d1d5db;
		border-radius: 0.375rem;
		cursor: pointer;
	}

	/* Custom scrollbar */
	.parameter-panel::-webkit-scrollbar {
		width: 8px;
	}

	.parameter-panel::-webkit-scrollbar-track {
		background: #f3f4f6;
	}

	.parameter-panel::-webkit-scrollbar-thumb {
		background: #d1d5db;
		border-radius: 4px;
	}

	.parameter-panel::-webkit-scrollbar-thumb:hover {
		background: #9ca3af;
	}

	/* Preset Selector Styles */
	.preset-selector {
		margin-bottom: 1rem;
		padding: 1rem;
		background-color: white;
		border-radius: 0.5rem;
		border: 1px solid #e5e7eb;
	}

	.preset-label {
		display: block;
		font-size: 0.875rem;
		font-weight: 600;
		color: #374151;
		margin-bottom: 0.5rem;
	}

	.preset-dropdown {
		width: 100%;
		padding: 0.5rem;
		border: 1px solid #d1d5db;
		border-radius: 0.375rem;
		background-color: white;
		font-size: 0.875rem;
		cursor: pointer;
		transition: border-color 0.15s;
	}

	.preset-dropdown:focus {
		outline: none;
		border-color: #3b82f6;
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
	}

	.preset-description {
		margin-top: 0.5rem;
		font-size: 0.75rem;
		color: #6b7280;
		font-style: italic;
	}

	.param-help {
		margin-top: 0.25rem;
		font-size: 0.75rem;
		color: #9ca3af;
		font-style: italic;
	}

	/* Mode toggle styles */
	.mode-toggle {
		display: flex;
		gap: 0.5rem;
	}

	.mode-option {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 0.5rem;
		border: 2px solid #d1d5db;
		border-radius: 0.375rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.mode-option:has(input:checked) {
		border-color: #3b82f6;
		background-color: #eff6ff;
	}

	.mode-option input[type='radio'] {
		margin-right: 0.5rem;
		cursor: pointer;
	}

	.mode-option span {
		font-size: 0.875rem;
		font-weight: 500;
		color: #374151;
	}

	/* Select dropdown styles */
	.param-select {
		width: 100%;
		padding: 0.5rem;
		border: 1px solid #d1d5db;
		border-radius: 0.375rem;
		background-color: white;
		font-size: 0.875rem;
		cursor: pointer;
		transition: border-color 0.15s;
	}

	.param-select:focus {
		outline: none;
		border-color: #3b82f6;
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
	}

	/* Preset button styles */
	.preset-buttons {
		display: flex;
		gap: 0.5rem;
		margin-top: 0.5rem;
	}

	.preset-btn {
		flex: 1;
		padding: 0.375rem 0.75rem;
		background-color: #f3f4f6;
		border: 1px solid #d1d5db;
		border-radius: 0.25rem;
		font-size: 0.75rem;
		font-weight: 500;
		color: #374151;
		cursor: pointer;
		transition: all 0.15s;
	}

	.preset-btn:hover {
		background-color: #e5e7eb;
		border-color: #9ca3af;
	}

	/* Recursive params section */
	.recursive-params {
		margin-top: 1rem;
		padding-top: 1rem;
		border-top: 1px solid #e5e7eb;
	}

	.section-subtitle {
		font-size: 0.875rem;
		font-weight: 600;
		color: #6b7280;
		margin-bottom: 0.75rem;
	}
</style>
