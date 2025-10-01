<script lang="ts">
	import { onMount } from 'svelte';
	import ThreeViewer from '$lib/components/viewer/ThreeViewer.svelte';
	import ParameterPanel from '$lib/components/ui/ParameterPanel.svelte';
	import { loadWasm, FlowerGenerator, type MeshData } from '$lib/wasm/loader';
	import { allParams } from '$lib/stores/parameters';

	let mesh: MeshData | null = $state(null);
	let loading = $state(true);
	let regenerating = $state(false);
	let error = $state('');
	let generator: FlowerGenerator | null = null;

	// Debounce timer for parameter changes
	let debounceTimer: ReturnType<typeof setTimeout> | null = null;

	// Initialize WASM and generator
	onMount(async () => {
		try {
			await loadWasm();
			generator = new FlowerGenerator();
			loading = false;
		} catch (e) {
			console.error('Failed to initialize Floraison:', e);
			error = e instanceof Error ? e.message : 'Failed to load flower generator';
			loading = false;
		}
	});

	// Regenerate flower when parameters change (debounced)
	$effect(() => {
		const params = $allParams;

		// Don't regenerate until WASM is loaded
		if (!generator || loading) return;

		// Clear existing timer
		if (debounceTimer) {
			clearTimeout(debounceTimer);
		}

		// Set new timer for debounced regeneration
		debounceTimer = setTimeout(async () => {
			regenerating = true;
			error = '';

			try {
				const paramsJson = JSON.stringify(params);
				console.log('Generating flower with params:', paramsJson);

				// Performance profiling
				const startTime = performance.now();
				mesh = generator!.generate_flower(paramsJson);
				const endTime = performance.now();
				const genTime = endTime - startTime;

				// Log metrics
				if (mesh) {
					const positions = mesh.positions();
					const indices = mesh.indices();

					console.log('✓ Flower generated:', {
						time: `${genTime.toFixed(2)}ms`,
						vertices: positions.length / 3,
						triangles: indices.length / 3
					});

					// Warn if generation is slow
					if (genTime > 1000) {
						console.warn('⚠ Slow generation detected (>1s). Consider reducing mesh resolution.');
					}
				}
			} catch (e) {
				console.error('Failed to generate flower:', e);
				error = e instanceof Error ? e.message : 'Failed to generate flower';
			} finally {
				regenerating = false;
			}
		}, 300);
	});
</script>

<main class="app-container">
	{#if loading}
		<div class="loading-container">
			<p class="text-xl text-gray-700">Loading Floraison...</p>
		</div>
	{:else if error && !mesh}
		<div class="error-container">
			<div class="text-center">
				<p class="text-xl text-red-600 mb-2">Error</p>
				<p class="text-gray-600">{error}</p>
			</div>
		</div>
	{:else}
		<div class="split-view">
			<ParameterPanel />
			<div class="viewer-container">
				<ThreeViewer {mesh} />
				{#if regenerating}
					<div class="regenerating-indicator">
						<div class="spinner"></div>
						<span>Generating...</span>
					</div>
				{/if}
				{#if error}
					<div class="error-banner">
						<p class="text-sm text-red-600">{error}</p>
					</div>
				{/if}
			</div>
		</div>
	{/if}
</main>

<style>
	.app-container {
		width: 100vw;
		height: 100vh;
		overflow: hidden;
	}

	.loading-container,
	.error-container {
		display: flex;
		align-items: center;
		justify-content: center;
		height: 100%;
	}

	.split-view {
		display: flex;
		width: 100%;
		height: 100%;
	}

	.viewer-container {
		flex: 1;
		position: relative;
		overflow: hidden;
	}

	.regenerating-indicator {
		position: absolute;
		top: 1rem;
		left: 50%;
		transform: translateX(-50%);
		display: flex;
		align-items: center;
		gap: 0.5rem;
		background-color: rgba(59, 130, 246, 0.9);
		color: white;
		padding: 0.5rem 1rem;
		border-radius: 0.5rem;
		font-size: 0.875rem;
		font-weight: 500;
		box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
		z-index: 10;
	}

	.spinner {
		width: 1rem;
		height: 1rem;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: white;
		border-radius: 50%;
		animation: spin 0.6s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.error-banner {
		position: absolute;
		bottom: 1rem;
		left: 50%;
		transform: translateX(-50%);
		background-color: rgba(254, 226, 226, 0.95);
		padding: 0.75rem 1.5rem;
		border-radius: 0.5rem;
		border: 1px solid #fca5a5;
		box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
		z-index: 10;
	}
</style>
