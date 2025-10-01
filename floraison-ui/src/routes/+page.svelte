<script lang="ts">
	import { onMount } from 'svelte';
	import ThreeViewer from '$lib/components/viewer/ThreeViewer.svelte';
	import { loadWasm, FlowerGenerator, type MeshData } from '$lib/wasm/loader';

	let mesh: MeshData | null = $state(null);
	let loading = $state(true);
	let error = $state('');

	onMount(async () => {
		try {
			// Load WASM module
			await loadWasm();

			// Create flower generator
			const generator = new FlowerGenerator();

			// Generate lily flower
			mesh = generator.generate_lily();

			loading = false;
		} catch (e) {
			console.error('Failed to initialize Floraison:', e);
			error = e instanceof Error ? e.message : 'Failed to load flower generator';
			loading = false;
		}
	});
</script>

<main class="w-screen h-screen">
	{#if loading}
		<div class="flex items-center justify-center h-full">
			<p class="text-xl text-gray-700">Loading Floraison...</p>
		</div>
	{:else if error}
		<div class="flex items-center justify-center h-full">
			<div class="text-center">
				<p class="text-xl text-red-600 mb-2">Error</p>
				<p class="text-gray-600">{error}</p>
			</div>
		</div>
	{:else}
		<ThreeViewer {mesh} />
	{/if}
</main>
