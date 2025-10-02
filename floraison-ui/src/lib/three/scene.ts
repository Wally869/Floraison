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
	setAmbientColor: (color: string) => void;
	setDirectionalColor: (color: string) => void;
	setHemisphereSkyColor: (color: string) => void;
	setHemisphereGroundColor: (color: string) => void;
	setExposure: (value: number) => void;
	toggleAxesHelper: (visible: boolean) => void;
	toggleShadows: (enabled: boolean) => void;
	positionGround: (minY: number) => void;
	updateShadowCamera: (boundingBox: THREE.Box3) => void;
	captureSquareScreenshot: (filename: string, onSuccess?: () => void, onError?: (error: Error) => void) => void;
	resetCamera: () => void;
}

/**
 * Create a Three.js scene with camera, lights, and controls
 *
 * @param canvas - Canvas element to render to
 * @returns Scene context with all necessary objects and methods
 */
export function createScene(canvas: HTMLCanvasElement): SceneContext {
	// Create scene with light brown background (earthy tone)
	const scene = new THREE.Scene();
	scene.background = new THREE.Color(0x714B4B);

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
		antialias: true,
		preserveDrawingBuffer: true // Required for screenshots/canvas capture
	});
	renderer.setSize(canvas.clientWidth, canvas.clientHeight);
	renderer.setPixelRatio(window.devicePixelRatio);

	// Enable shadows with highest quality (VSM = softest, most realistic)
	renderer.shadowMap.enabled = true;
	renderer.shadowMap.type = THREE.VSMShadowMap;

	// Enable tone mapping for realistic lighting (industry standard)
	renderer.toneMapping = THREE.ACESFilmicToneMapping;
	renderer.toneMappingExposure = 1.0;
	renderer.outputColorSpace = THREE.SRGBColorSpace;

	// Add lighting - hemisphere light for natural outdoor ambient
	const hemisphereLight = new THREE.HemisphereLight(
		0x87ceeb, // Sky color (light blue)
		0x8b7355, // Ground color (brownish earth)
		0.6
	);
	scene.add(hemisphereLight);

	// Main directional light (key light)
	const dirLight = new THREE.DirectionalLight(0xffffff, 1.2);
	dirLight.position.set(5, 10, 5);
	dirLight.castShadow = true;

	// Configure shadow camera for maximum quality
	dirLight.shadow.mapSize.width = 4096;  // Ultra-high resolution
	dirLight.shadow.mapSize.height = 4096;
	dirLight.shadow.camera.near = 0.5;
	dirLight.shadow.camera.far = 50;
	dirLight.shadow.camera.left = -20;
	dirLight.shadow.camera.right = 20;
	dirLight.shadow.camera.top = 20;
	dirLight.shadow.camera.bottom = -20;
	dirLight.shadow.bias = -0.001;  // Reduce shadow acne
	dirLight.shadow.normalBias = 0.02;  // Surface-aware bias for complex geometry
	dirLight.shadow.radius = 3;  // Wider penumbra for softer edge
	dirLight.shadow.blurSamples = 25;  // More blur samples for smoothness

	scene.add(dirLight);

	// Fill light (softer, opposite side, no shadows)
	const fillLight = new THREE.DirectionalLight(0xffffff, 0.4);
	fillLight.position.set(-5, 5, -5);
	scene.add(fillLight);

	// Add ground plane for shadow reception
	const groundGeometry = new THREE.PlaneGeometry(60, 60);
	const groundMaterial = new THREE.ShadowMaterial({ opacity: 0.3 });
	const ground = new THREE.Mesh(groundGeometry, groundMaterial);
	ground.rotation.x = -Math.PI / 2;
	ground.position.y = 0; // Will be repositioned dynamically based on mesh bounds
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
		hemisphereLight.intensity = intensity;
	}

	function setDirectionalIntensity(intensity: number) {
		dirLight.intensity = intensity;
	}

	function setAmbientColor(color: string) {
		// For hemisphere light, set sky color
		hemisphereLight.color = new THREE.Color(color);
	}

	function setDirectionalColor(color: string) {
		dirLight.color = new THREE.Color(color);
		fillLight.color = new THREE.Color(color);
	}

	function setHemisphereSkyColor(color: string) {
		hemisphereLight.color = new THREE.Color(color);
	}

	function setHemisphereGroundColor(color: string) {
		hemisphereLight.groundColor = new THREE.Color(color);
	}

	function setExposure(value: number) {
		renderer.toneMappingExposure = value;
	}

	function toggleAxesHelper(visible: boolean) {
		axesHelper.visible = visible;
	}

	function toggleShadows(enabled: boolean) {
		renderer.shadowMap.enabled = enabled;
		ground.visible = enabled;
	}

	function positionGround(minY: number) {
		// Position ground at or slightly below the lowest vertex
		ground.position.y = minY - 0.1;
	}

	function updateShadowCamera(boundingBox: THREE.Box3) {
		// Dynamically adjust shadow camera frustum to fit mesh bounds
		// Add 20% padding to ensure shadows are fully captured
		const size = new THREE.Vector3();
		boundingBox.getSize(size);
		const center = new THREE.Vector3();
		boundingBox.getCenter(center);

		const maxDim = Math.max(size.x, size.y, size.z);
		const frustumSize = maxDim * 0.6; // 60% of max dimension with padding

		dirLight.shadow.camera.left = -frustumSize;
		dirLight.shadow.camera.right = frustumSize;
		dirLight.shadow.camera.top = frustumSize;
		dirLight.shadow.camera.bottom = -frustumSize;
		dirLight.shadow.camera.far = maxDim * 2.5; // Ensure far plane covers entire scene

		dirLight.shadow.camera.updateProjectionMatrix();
	}

	function captureSquareScreenshot(
		filename: string,
		onSuccess?: () => void,
		onError?: (error: Error) => void
	) {
		try {
			// Create temporary canvas for square render
			const tempCanvas = document.createElement('canvas');
			const size = 2048; // High quality square resolution
			tempCanvas.width = size;
			tempCanvas.height = size;

			// Create temporary renderer
			const tempRenderer = new THREE.WebGLRenderer({
				canvas: tempCanvas,
				antialias: true,
				preserveDrawingBuffer: true
			});
			tempRenderer.setSize(size, size);
			tempRenderer.setPixelRatio(1); // Use 1:1 for consistent quality

			// Copy renderer settings from main renderer
			tempRenderer.shadowMap.enabled = renderer.shadowMap.enabled;
			tempRenderer.shadowMap.type = renderer.shadowMap.type;
			tempRenderer.toneMapping = renderer.toneMapping;
			tempRenderer.toneMappingExposure = renderer.toneMappingExposure;
			tempRenderer.outputColorSpace = renderer.outputColorSpace;

			// Create environment map for temp renderer (textures are tied to WebGL context)
			const tempPMREMGenerator = new PMREMGenerator(tempRenderer);
			tempPMREMGenerator.compileEquirectangularShader();
			const tempEnvScene = new THREE.Scene();
			tempEnvScene.background = new THREE.Color(0xffffff);
			const tempEnvMap = tempPMREMGenerator.fromScene(tempEnvScene).texture;

			// Store original scene environment
			const originalEnvironment = scene.environment;
			const originalEnvironmentIntensity = scene.environmentIntensity;

			// Set temporary environment map
			scene.environment = tempEnvMap;
			scene.environmentIntensity = 0.5;

			// Store original camera aspect ratio
			const originalAspect = camera.aspect;

			// Set camera to square aspect ratio
			camera.aspect = 1.0;
			camera.updateProjectionMatrix();

			// Render twice: first pass initializes shadows, second pass captures final image
			// This ensures shadow maps are properly generated for the new renderer
			tempRenderer.render(scene, camera);
			tempRenderer.render(scene, camera); // Second render with shadows ready

			// Restore original camera aspect
			camera.aspect = originalAspect;
			camera.updateProjectionMatrix();

			// Restore original scene environment
			scene.environment = originalEnvironment;
			scene.environmentIntensity = originalEnvironmentIntensity;

			// Cleanup temp environment resources
			tempPMREMGenerator.dispose();

			// Capture as blob and download
			tempCanvas.toBlob(
				(blob) => {
					if (!blob) {
						const error = new Error('Failed to capture canvas as blob');
						console.error('Screenshot failed:', error);
						onError?.(error);
						tempRenderer.dispose();
						return;
					}

					// Create download link
					const url = URL.createObjectURL(blob);
					const link = document.createElement('a');
					link.href = url;
					link.download = filename;
					document.body.appendChild(link);
					link.click();
					document.body.removeChild(link);

					// Cleanup
					URL.revokeObjectURL(url);
					tempRenderer.dispose();

					onSuccess?.();
				},
				'image/png',
				1.0 // Maximum quality
			);
		} catch (error) {
			console.error('Screenshot failed:', error);
			const errorObj = error instanceof Error ? error : new Error(String(error));
			onError?.(errorObj);
		}
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
		setAmbientColor,
		setDirectionalColor,
		setHemisphereSkyColor,
		setHemisphereGroundColor,
		setExposure,
		toggleAxesHelper,
		toggleShadows,
		positionGround,
		updateShadowCamera,
		captureSquareScreenshot,
		resetCamera
	};
}
