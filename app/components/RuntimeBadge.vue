<template>
	<span
		class="inline-flex items-center gap-1.5 rounded-full border px-2.5 py-0.5 text-xs font-semibold"
		:class="classes"
	>
		<UIcon :name="icon" class="size-3.5" />
		{{ label }}
	</span>
</template>

<script setup lang="ts">
	import type { RuntimeType } from "../types";

	const props = defineProps<{
		type: RuntimeType | "none";
		label?: string;
	}>();

	const config: Record<string, { label: string; icon: string; classes: string }> = {
		system_php: {
			label: "PHP système",
			icon: "lucide:terminal-square",
			classes: "border-brand-300 bg-brand-50 text-brand-700 dark:border-brand-700 dark:bg-brand-950/50 dark:text-brand-300"
		},
		custom_php: {
			label: "PHP personnalisé",
			icon: "lucide:cog",
			classes: "border-brand-300 bg-brand-50 text-brand-700 dark:border-brand-700 dark:bg-brand-950/50 dark:text-brand-300"
		},
		phprs_experimental: {
			label: "phprs expérimental",
			icon: "lucide:flask-conical",
			classes: "border-warning bg-warning/10 text-warning"
		},
		none: {
			label: "Aucun runtime",
			icon: "lucide:circle-slash",
			classes: "border-(--brand-border) bg-(--brand-surface) text-(--brand-muted)"
		}
	};

	const current = computed(() => config[props.type] ?? config.none);
	const label = computed(() => props.label ?? current.value.label);
	const icon = computed(() => current.value.icon);
	const classes = computed(() => current.value.classes);
</script>
