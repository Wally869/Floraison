<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import * as THREE from 'three';
	import { createScene, type SceneContext } from '$lib/three/scene';
	import { wasmMeshToGeometry } from '$lib/three/mesh-converter';
	import type { MeshData } from '$lib/wasm/floraison';

	// Props
	interface Props {
		mesh: MeshData | null;
	}

	let { mesh = null }: Props = $props();

	// Component state
	let canvas: HTMLCanvasElement;
	let sceneCtx: SceneContext | null = $state(null);
	// Plain variable - not reactive! Three.js state is imperative, not reactive UI state
	let flowerMesh: THREE.Mesh | null = null;

	onMount(() => {
		// Initialize Three.js scene
		sceneCtx = createScene(canvas);
		sceneCtx.animate();

		// $effect will handle mesh rendering when both mesh and sceneCtx are ready
	});

	onDestroy(() => {
		// Cleanup Three.js resources
		if (flowerMesh) {
			flowerMesh.geometry.dispose();
			if (flowerMesh.material instanceof THREE.Material) {
				flowerMesh.material.dispose();
			}
		}
		sceneCtx?.dispose();
	});

	// Update mesh when prop changes
	$effect(() => {
		if (mesh && sceneCtx) {
			updateMesh(mesh);
		}
	});

	function updateMesh(newMesh: MeshData) {
		if (!sceneCtx) return;

		// Remove old mesh from scene (safe - flowerMesh is not reactive)
		if (flowerMesh) {
			sceneCtx.scene.remove(flowerMesh);
			flowerMesh.geometry.dispose();
			if (flowerMesh.material instanceof THREE.Material) {
				flowerMesh.material.dispose();
			}
		}

		// Convert WASM mesh to Three.js geometry
		const geometry = wasmMeshToGeometry(newMesh);

		// Create material (golden color, double-sided for proper rendering)
		const material = new THREE.MeshStandardMaterial({
			color: 0xffcc00,
			side: THREE.DoubleSide,
			metalness: 0.1,
			roughness: 0.7
		});

		// Create mesh and add to scene
		flowerMesh = new THREE.Mesh(geometry, material);
		sceneCtx.scene.add(flowerMesh);

		// Frame camera to show entire flower
		const boundingSphere = geometry.boundingSphere;
		if (boundingSphere) {
			const center = boundingSphere.center;
			const radius = boundingSphere.radius;

			// Position camera at distance based on sphere radius
			const distance = radius * 2.5;
			sceneCtx.camera.position.set(distance, distance, distance);
			sceneCtx.camera.lookAt(center);
			sceneCtx.controls.target.copy(center);
			sceneCtx.controls.update();
		}
	}
</script>

<div class="viewer-container">
	<canvas bind:this={canvas}></canvas>
	{#if !mesh}
		<div class="loading">Loading flower...</div>
	{/if}
</div>

<style>
	.viewer-container {
		width: 100%;
		height: 100%;
		position: relative;
		overflow: hidden;
	}

	canvas {
		width: 100%;
		height: 100%;
		display: block;
	}

	.loading {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		font-size: 1.5rem;
		color: #666;
		pointer-events: none;
	}
</style>
