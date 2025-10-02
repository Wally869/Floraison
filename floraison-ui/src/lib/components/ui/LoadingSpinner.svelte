<script lang="ts">
	/**
	 * Loading Spinner Overlay
	 *
	 * Displays an animated spinner with optional message during loading operations.
	 */

	interface Props {
		/** Whether the spinner is visible */
		visible?: boolean;
		/** Optional loading message to display */
		message?: string;
	}

	let { visible = false, message = 'Generating...' }: Props = $props();
</script>

{#if visible}
	<div class="spinner-overlay">
		<div class="spinner-container">
			<div class="spinner"></div>
			<p class="spinner-message">{message}</p>
		</div>
	</div>
{/if}

<style>
	.spinner-overlay {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background-color: rgba(0, 0, 0, 0.3);
		backdrop-filter: blur(4px);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
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

	.spinner-container {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 1rem;
		background-color: rgba(255, 255, 255, 0.95);
		padding: 2rem 3rem;
		border-radius: 1rem;
		box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);
	}

	.spinner {
		width: 3rem;
		height: 3rem;
		border: 4px solid rgba(0, 0, 0, 0.1);
		border-top-color: #3b82f6;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.spinner-message {
		margin: 0;
		font-size: 0.875rem;
		font-weight: 500;
		color: #555;
		white-space: nowrap;
	}

	/* Ensure spinner is above all other content */
	@media (max-width: 767px) {
		.spinner-overlay {
			z-index: 1001;
		}

		.spinner-container {
			padding: 1.5rem 2rem;
		}
	}
</style>
