/**
 * Three.js Scene Setup
 *
 * Creates and manages a Three.js scene with camera, lights, and orbit controls.
 */

import * as THREE from 'three';
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js';

export interface SceneContext {
	scene: THREE.Scene;
	camera: THREE.PerspectiveCamera;
	renderer: THREE.WebGLRenderer;
	controls: OrbitControls;
	animate: () => void;
	dispose: () => void;
}

/**
 * Create a Three.js scene with camera, lights, and controls
 *
 * @param canvas - Canvas element to render to
 * @returns Scene context with all necessary objects and methods
 */
export function createScene(canvas: HTMLCanvasElement): SceneContext {
	// Create scene with gray background
	const scene = new THREE.Scene();
	scene.background = new THREE.Color(0xf0f0f0);

	// Create perspective camera
	const camera = new THREE.PerspectiveCamera(
		50, // FOV
		canvas.clientWidth / canvas.clientHeight, // aspect
		0.1, // near
		1000 // far
	);
	camera.position.set(10, 10, 10);
	camera.lookAt(0, 0, 0);

	// Create WebGL renderer
	const renderer = new THREE.WebGLRenderer({
		canvas,
		antialias: true
	});
	renderer.setSize(canvas.clientWidth, canvas.clientHeight);
	renderer.setPixelRatio(window.devicePixelRatio);

	// Add lighting
	const ambientLight = new THREE.AmbientLight(0xffffff, 0.5);
	scene.add(ambientLight);

	const dirLight = new THREE.DirectionalLight(0xffffff, 0.8);
	dirLight.position.set(5, 10, 5);
	scene.add(dirLight);

	// Add orbit controls
	const controls = new OrbitControls(camera, renderer.domElement);
	controls.enableDamping = true;
	controls.dampingFactor = 0.05;
	controls.screenSpacePanning = false;
	controls.minDistance = 5;
	controls.maxDistance = 50;

	// Animation loop
	let animationId: number;

	function animate() {
		animationId = requestAnimationFrame(animate);
		controls.update();
		renderer.render(scene, camera);
	}

	// Cleanup function
	function dispose() {
		cancelAnimationFrame(animationId);
		renderer.dispose();
		controls.dispose();
	}

	return {
		scene,
		camera,
		renderer,
		controls,
		animate,
		dispose
	};
}
