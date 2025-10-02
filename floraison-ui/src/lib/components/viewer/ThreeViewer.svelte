<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import * as THREE from 'three';
	import { createScene, type SceneContext } from '$lib/three/scene';
	import { wasmMeshToGeometry } from '$lib/three/mesh-converter';
	import type { MeshDataLike } from '$lib/wasm/types';
	import ViewerControls from './ViewerControls.svelte';
	import { viewerSettings } from '$lib/stores/viewer';
	import { exportToGLB, generateFilename } from '$lib/three/exporter';
	import { currentPresetName } from '$lib/stores/parameters';

	// Props
	interface Props {
		mesh: MeshDataLike | null;
	}

	let { mesh = null }: Props = $props();

	// Component state
	let canvas: HTMLCanvasElement;
	let sceneCtx: SceneContext | null = $state(null);
	// Plain variable - not reactive! Three.js state is imperative, not reactive UI state
	let flowerMesh: THREE.Mesh | null = null;
	let meshCenter = new THREE.Vector3(0, 0, 0); // Track mesh center for camera reset
	let isFirstMeshLoad = $state(true);

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

	// React to viewer settings changes
	$effect(() => {
		if (!sceneCtx) return;

		const settings = $viewerSettings;
		sceneCtx.setBackgroundColor(settings.backgroundColor);
		sceneCtx.setAmbientIntensity(settings.ambientIntensity);
		sceneCtx.setDirectionalIntensity(settings.directionalIntensity);
		sceneCtx.setAmbientColor(settings.ambientColor);
		sceneCtx.setDirectionalColor(settings.directionalColor);
		sceneCtx.setHemisphereSkyColor(settings.hemisphereSkyColor);
		sceneCtx.setHemisphereGroundColor(settings.hemisphereGroundColor);
		sceneCtx.setExposure(settings.exposure);
		sceneCtx.toggleAxesHelper(settings.showAxes);
		sceneCtx.toggleShadows(settings.enableShadows);

		// Update wireframe on existing mesh
		if (flowerMesh && flowerMesh.material instanceof THREE.MeshPhysicalMaterial) {
			flowerMesh.material.wireframe = settings.wireframe;
		}
	});

	/**
	 * Frame the mesh in the camera view
	 * Centers and zooms camera to show entire flower
	 */
	function frameMeshInView(geometry: THREE.BufferGeometry) {
		if (!sceneCtx) return;

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

	function updateMesh(newMesh: MeshDataLike) {
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

		// Compute bounding box for ground positioning and orbit centering
		geometry.computeBoundingBox();
		if (geometry.boundingBox) {
			const minY = geometry.boundingBox.min.y;
			sceneCtx.positionGround(minY);

			// Always update orbit target to center of bounding box (especially vertical center)
			// This makes the camera orbit around the middle of the mesh rather than its base
			geometry.boundingBox.getCenter(meshCenter);
			sceneCtx.controls.target.copy(meshCenter);
			sceneCtx.controls.update();
		}

		// Create enhanced material with translucency (organic petal appearance)
		const material = new THREE.MeshPhysicalMaterial({
			vertexColors: true, // Use per-vertex colors from geometry
			side: THREE.DoubleSide,

			// Base PBR properties
			metalness: 0.0, // Petals are not metallic
			roughness: 0.6, // Slightly rough surface

			// Translucency (subsurface scattering approximation)
			transmission: 0.0, // No full transparency
			thickness: 0.5, // Controls SSS depth
			ior: 1.4, // Index of refraction (organic material)

			// Sheen for petal-like appearance
			sheen: 0.5,
			sheenRoughness: 0.5,
			sheenColor: new THREE.Color(0xffeecc), // Slight warm tint

			// Clearcoat for waxy surface
			clearcoat: 0.3,
			clearcoatRoughness: 0.4,

			wireframe: $viewerSettings.wireframe
		});

		// Create mesh and add to scene
		flowerMesh = new THREE.Mesh(geometry, material);
		flowerMesh.castShadow = true;
		flowerMesh.receiveShadow = true; // Petals can shadow each other
		sceneCtx.scene.add(flowerMesh);

		// Only auto-frame camera on first load
		// This prevents camera jumping when user is exploring the flower
		if (isFirstMeshLoad) {
			frameMeshInView(geometry);
			isFirstMeshLoad = false;
		}
	}

	function handleResetCamera() {
		if (sceneCtx) {
			// Reset camera to default position but look at mesh center
			sceneCtx.camera.position.set(10, 10, 10);
			sceneCtx.camera.lookAt(meshCenter);
			sceneCtx.controls.target.copy(meshCenter);
			sceneCtx.controls.update();
		}
	}

	function handleFrameFlower() {
		if (flowerMesh && sceneCtx) {
			const geometry = flowerMesh.geometry;
			geometry.computeBoundingSphere();
			frameMeshInView(geometry);
		}
	}

	function handleExport() {
		if (!flowerMesh) {
			console.warn('No flower to export');
			return;
		}

		const filename = generateFilename($currentPresetName);

		exportToGLB(flowerMesh, {
			filename,
			onSuccess: () => {
				console.log('Flower exported successfully!');
			},
			onError: (error) => {
				console.error('Export failed:', error);
			}
		});
	}
</script>

<div class="viewer-container">
	<canvas bind:this={canvas}></canvas>
	{#if !mesh}
		<div class="loading">Loading flower...</div>
	{/if}
	<ViewerControls
		onResetCamera={handleResetCamera}
		onFrameFlower={handleFrameFlower}
		onExport={handleExport}
	/>
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
