<template>
	<svg
		:width="size"
		:height="size"
		viewBox="0 0 128 128"
		fill="none"
		xmlns="http://www.w3.org/2000/svg"
		role="img"
		aria-label="Laralink logo"
		class="laralink-logo"
	>
		<!-- Ring -->
		<circle cx="64" cy="64" r="42" class="mark-stroke" opacity="0.18" />
		<!-- Network branches -->
		<path d="M64 34V64" class="accent-stroke" />
		<path d="M64 64L38 80" class="mark-stroke" />
		<path d="M64 64L90 80" class="mark-stroke" />
		<!-- Stylized L -->
		<path d="M50 42V82C50 88 54 92 60 92H82" class="accent-stroke" />
		<!-- Nodes -->
		<circle cx="64" cy="64" r="9" class="accent" />
		<circle cx="38" cy="80" r="7" class="mark-fill" />
		<circle cx="90" cy="80" r="7" class="mark-fill" />
		<circle cx="64" cy="34" r="7" class="mark-fill" />
	</svg>
</template>

<script setup lang="ts">
	defineProps<{
		size?: number;
	}>();
</script>

<style scoped>
	.laralink-logo {
		color: var(--brand-text);
		--logo-accent: var(--brand-primary);
	}

	.mark-stroke {
		stroke: currentColor;
		stroke-width: 8;
		stroke-linecap: round;
		stroke-linejoin: round;
	}

	.mark-fill {
		fill: currentColor;
	}

	.accent {
		fill: var(--logo-accent, currentColor);
	}

	.accent-stroke {
		stroke: var(--logo-accent, currentColor);
		stroke-width: 8;
		stroke-linecap: round;
		stroke-linejoin: round;
	}
</style>
