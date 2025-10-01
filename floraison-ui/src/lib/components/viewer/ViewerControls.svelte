<script lang="ts">
	import { viewerSettings } from '$lib/stores/viewer';

	interface Props {
		onResetCamera: () => void;
	}

	let { onResetCamera }: Props = $props();

	let expanded = $state(false);
</script>

<div class="viewer-controls">
	<button class="toggle-button" onclick={() => (expanded = !expanded)} aria-label="Toggle viewer controls">
		<svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor">
			<path d="M12 5v.01M12 12v.01M12 19v.01M12 6a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2z" />
		</svg>
	</button>

	{#if expanded}
		<div class="controls-panel">
			<div class="panel-header">
				<h3>Viewer Settings</h3>
			</div>

			<div class="control-group">
				<label class="toggle-control">
					<input type="checkbox" bind:checked={$viewerSettings.showAxes} />
					<span>Show Axes</span>
				</label>

				<label class="toggle-control">
					<input type="checkbox" bind:checked={$viewerSettings.wireframe} />
					<span>Wireframe</span>
				</label>
			</div>

			<div class="control-group">
				<label class="color-control">
					<span>Background</span>
					<input type="color" bind:value={$viewerSettings.backgroundColor} />
				</label>
			</div>

			<div class="control-group">
				<label class="slider-control">
					<span>Ambient Light</span>
					<input
						type="range"
						min="0"
						max="1"
						step="0.1"
						bind:value={$viewerSettings.ambientIntensity}
					/>
					<span class="value">{$viewerSettings.ambientIntensity.toFixed(1)}</span>
				</label>

				<label class="slider-control">
					<span>Directional Light</span>
					<input
						type="range"
						min="0"
						max="1"
						step="0.1"
						bind:value={$viewerSettings.directionalIntensity}
					/>
					<span class="value">{$viewerSettings.directionalIntensity.toFixed(1)}</span>
				</label>
			</div>

			<div class="control-group">
				<button class="action-button" onclick={onResetCamera}>
					Reset Camera
				</button>
			</div>
		</div>
	{/if}
</div>

<style>
	.viewer-controls {
		position: absolute;
		top: 1rem;
		right: 1rem;
		z-index: 20;
		display: flex;
		flex-direction: column;
		align-items: flex-end;
		gap: 0.5rem;
	}

	.toggle-button {
		width: 2.5rem;
		height: 2.5rem;
		background-color: rgba(255, 255, 255, 0.95);
		border: 1px solid rgba(0, 0, 0, 0.1);
		border-radius: 0.5rem;
		display: flex;
		align-items: center;
		justify-content: center;
		cursor: pointer;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
		transition: all 0.2s;
	}

	.toggle-button:hover {
		background-color: rgba(255, 255, 255, 1);
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
	}

	.icon {
		width: 1.25rem;
		height: 1.25rem;
		stroke-width: 2;
	}

	.controls-panel {
		background-color: rgba(255, 255, 255, 0.95);
		border: 1px solid rgba(0, 0, 0, 0.1);
		border-radius: 0.5rem;
		padding: 1rem;
		min-width: 16rem;
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
		backdrop-filter: blur(10px);
	}

	.panel-header {
		margin-bottom: 0.75rem;
		padding-bottom: 0.75rem;
		border-bottom: 1px solid rgba(0, 0, 0, 0.1);
	}

	.panel-header h3 {
		margin: 0;
		font-size: 0.875rem;
		font-weight: 600;
		color: #333;
	}

	.control-group {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		margin-bottom: 0.75rem;
		padding-bottom: 0.75rem;
		border-bottom: 1px solid rgba(0, 0, 0, 0.05);
	}

	.control-group:last-child {
		border-bottom: none;
		margin-bottom: 0;
		padding-bottom: 0;
	}

	.toggle-control {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.875rem;
		cursor: pointer;
	}

	.toggle-control input[type='checkbox'] {
		width: 1rem;
		height: 1rem;
		cursor: pointer;
	}

	.toggle-control span {
		color: #555;
	}

	.color-control {
		display: flex;
		justify-content: space-between;
		align-items: center;
		font-size: 0.875rem;
	}

	.color-control span {
		color: #555;
	}

	.color-control input[type='color'] {
		width: 3rem;
		height: 2rem;
		border: 1px solid rgba(0, 0, 0, 0.1);
		border-radius: 0.25rem;
		cursor: pointer;
	}

	.slider-control {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
		font-size: 0.875rem;
	}

	.slider-control > span:first-child {
		color: #555;
		font-weight: 500;
	}

	.slider-control input[type='range'] {
		width: 100%;
		cursor: pointer;
	}

	.slider-control .value {
		align-self: flex-end;
		font-size: 0.75rem;
		color: #888;
		font-family: monospace;
	}

	.action-button {
		width: 100%;
		padding: 0.5rem 1rem;
		background-color: #3b82f6;
		color: white;
		border: none;
		border-radius: 0.375rem;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: background-color 0.2s;
	}

	.action-button:hover {
		background-color: #2563eb;
	}
</style>
