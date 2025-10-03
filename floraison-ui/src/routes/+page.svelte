<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { browser } from '$app/environment';
	import ThreeViewer from '$lib/components/viewer/ThreeViewer.svelte';
	import ParameterPanel from '$lib/components/ui/ParameterPanel.svelte';
	import LoadingSpinner from '$lib/components/ui/LoadingSpinner.svelte';
	import type { MeshDataLike } from '$lib/wasm/types';
	import { getPositions, getIndices } from '$lib/wasm/types';
	import type { GenerationWorkerManager } from '$lib/workers/manager';
	import { allParams, inflorescenceParams } from '$lib/stores/parameters';

	let mesh: MeshDataLike | null = $state(null);
	let loading = $state(false);
	let error = $state('');

	// Mobile panel state
	let panelOpen = $state(false);

	// Debounce timer for parameter changes
	let debounceTimer: ReturnType<typeof setTimeout> | null = null;

	// Worker manager (initialized in onMount - browser only)
	// Must be $state so $effect re-runs when it's initialized
	let workerManager: GenerationWorkerManager | null = $state(null);

	function togglePanel() {
		panelOpen = !panelOpen;
	}

	function closePanel() {
		panelOpen = false;
	}

	// Initialize worker manager in browser
	onMount(async () => {
		if (browser) {
			const { getWorkerManager } = await import('$lib/workers/manager');
			workerManager = getWorkerManager();
		}
	});

	// Clean up worker on unmount
	onDestroy(() => {
		if (browser) {
			import('$lib/workers/manager').then(({ disposeWorkerManager }) => {
				disposeWorkerManager();
			});
		}
	});

	// Regenerate flower/inflorescence when parameters change (debounced)
	$effect(() => {
		const params = $allParams;
		const infloParams = $inflorescenceParams;

		// Don't regenerate until worker manager is ready
		if (!workerManager) return;

		// Clear existing timer
		if (debounceTimer) {
			clearTimeout(debounceTimer);
		}

		// Set new timer for debounced regeneration
		debounceTimer = setTimeout(async () => {
			if (!workerManager) return;

			loading = true;
			error = '';

			try {
				// Performance profiling
				const startTime = performance.now();

				// Generate using worker
				const generatedMesh = await workerManager.generate(params, infloParams);

				mesh = generatedMesh;

				const endTime = performance.now();
				const genTime = endTime - startTime;

				// Log metrics
				if (mesh) {
					const positions = getPositions(mesh);
					const indices = getIndices(mesh);

					console.log(`✓ ${infloParams.enabled ? 'Inflorescence' : 'Flower'} generated:`, {
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
				console.error(`Failed to generate ${infloParams.enabled ? 'inflorescence' : 'flower'}:`, e);
				error =
					e instanceof Error
						? e.message
						: `Failed to generate ${infloParams.enabled ? 'inflorescence' : 'flower'}`;
			} finally {
				loading = false;
			}
		}, 300);
	});
</script>

<main class="app-container">
	<!-- Hamburger menu button (mobile only) -->
	<button class="hamburger-button md:hidden" onclick={togglePanel} aria-label="Toggle menu">
		<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
			<path d="M3 6h18M3 12h18M3 18h18" />
		</svg>
	</button>

	<div class="split-view">
		<!-- Backdrop overlay (mobile only, when panel open) -->
		{#if panelOpen}
			<div
				class="backdrop md:hidden"
				onclick={closePanel}
				onkeydown={(e) => e.key === 'Escape' && closePanel()}
				role="button"
				tabindex="-1"
				aria-label="Close menu"
			></div>
		{/if}

		<ParameterPanel open={panelOpen} />
		<div class="viewer-container">
			<ThreeViewer {mesh} />
			{#if error}
				<div class="error-banner">
					<p class="text-sm text-red-600">{error}</p>
				</div>
			{/if}
		</div>
	</div>

	<!-- Loading spinner overlay -->
	<LoadingSpinner visible={loading} message="Generating..." />
</main>

<style>
	.app-container {
		width: 100vw;
		height: 100vh;
		overflow: hidden;
	}

	.split-view {
		display: flex;
		width: 100%;
		height: 100%;
		position: relative; /* For backdrop positioning */
	}

	.hamburger-button {
		position: fixed;
		top: 1rem;
		left: 1rem;
		z-index: 60;
		width: 3rem;
		height: 3rem;
		background-color: white;
		border: 1px solid #e5e7eb;
		border-radius: 0.5rem;
		display: flex;
		align-items: center;
		justify-content: center;
		cursor: pointer;
		box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
		transition: all 0.2s;
	}

	.hamburger-button:hover {
		background-color: #f3f4f6;
		box-shadow: 0 6px 12px rgba(0, 0, 0, 0.15);
	}

	.hamburger-button:active {
		transform: scale(0.95);
	}

	.hamburger-button svg {
		width: 1.5rem;
		height: 1.5rem;
	}

	/* Hide hamburger button on desktop (md breakpoint: 768px+) */
	@media (min-width: 768px) {
		.hamburger-button {
			display: none;
		}
	}

	.backdrop {
		position: fixed;
		inset: 0;
		background-color: rgba(0, 0, 0, 0.5);
		z-index: 40;
		animation: fadeIn 0.2s ease-out;
	}

	@keyframes fadeIn {
		from {
			opacity: 0;
		}
		to {
			opacity: 1;
		}
	}

	.viewer-container {
		flex: 1;
		position: relative;
		overflow: hidden;
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
