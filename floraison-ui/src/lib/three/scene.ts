/**
 * Three.js Scene Setup
 *
 * Creates and manages a Three.js scene with camera, lights, and orbit controls.
 */

import * as THREE from 'three';
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js';
import { PMREMGenerator } from 'three';

export interface SceneContext {
	scene: THREE.Scene;
	camera: THREE.PerspectiveCamera;
	renderer: THREE.WebGLRenderer;
	controls: OrbitControls;
	animate: () => void;
	dispose: () => void;
	// Viewer control methods
	setBackgroundColor: (color: string) => void;
	setAmbientIntensity: (intensity: number) => void;
	setDirectionalIntensity: (intensity: number) => void;
	toggleAxesHelper: (visible: boolean) => void;
	toggleShadows: (enabled: boolean) => void; // NEW
	resetCamera: () => void;
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

	// Enable shadows with soft shadow mapping
	renderer.shadowMap.enabled = true;
	renderer.shadowMap.type = THREE.PCFSoftShadowMap;

	// Add lighting
	const ambientLight = new THREE.AmbientLight(0xffffff, 0.5);
	scene.add(ambientLight);

	const dirLight = new THREE.DirectionalLight(0xffffff, 0.8);
	dirLight.position.set(5, 10, 5);
	dirLight.castShadow = true;

	// Configure shadow camera for optimal shadow quality
	dirLight.shadow.mapSize.width = 2048;
	dirLight.shadow.mapSize.height = 2048;
	dirLight.shadow.camera.near = 0.5;
	dirLight.shadow.camera.far = 50;
	dirLight.shadow.camera.left = -10;
	dirLight.shadow.camera.right = 10;
	dirLight.shadow.camera.top = 10;
	dirLight.shadow.camera.bottom = -10;
	dirLight.shadow.bias = -0.001;

	scene.add(dirLight);

	// Add ground plane for shadow reception
	const groundGeometry = new THREE.PlaneGeometry(30, 30);
	const groundMaterial = new THREE.ShadowMaterial({ opacity: 0.3 });
	const ground = new THREE.Mesh(groundGeometry, groundMaterial);
	ground.rotation.x = -Math.PI / 2;
	ground.position.y = -5;
	ground.receiveShadow = true;
	scene.add(ground);

	// Setup environment map for reflections (neutral studio lighting)
	const pmremGenerator = new PMREMGenerator(renderer);
	pmremGenerator.compileEquirectangularShader();

	// Create simple neutral environment
	const envScene = new THREE.Scene();
	envScene.background = new THREE.Color(0xffffff);
	const envMap = pmremGenerator.fromScene(envScene).texture;
	scene.environment = envMap;
	scene.environmentIntensity = 0.5; // Subtle reflections

	pmremGenerator.dispose();

	// Add axes helper (initially hidden)
	const axesHelper = new THREE.AxesHelper(10);
	axesHelper.visible = false;
	scene.add(axesHelper);

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

	// Viewer control methods
	function setBackgroundColor(color: string) {
		scene.background = new THREE.Color(color);
	}

	function setAmbientIntensity(intensity: number) {
		ambientLight.intensity = intensity;
	}

	function setDirectionalIntensity(intensity: number) {
		dirLight.intensity = intensity;
	}

	function toggleAxesHelper(visible: boolean) {
		axesHelper.visible = visible;
	}

	function toggleShadows(enabled: boolean) {
		renderer.shadowMap.enabled = enabled;
		ground.visible = enabled;
	}

	function resetCamera() {
		camera.position.set(10, 10, 10);
		camera.lookAt(0, 0, 0);
		controls.target.set(0, 0, 0);
		controls.update();
	}

	return {
		scene,
		camera,
		renderer,
		controls,
		animate,
		dispose,
		setBackgroundColor,
		setAmbientIntensity,
		setDirectionalIntensity,
		toggleAxesHelper,
		toggleShadows,
		resetCamera
	};
}
