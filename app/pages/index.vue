<template>
	<div class="flex flex-col gap-5">
		<!-- En-tête -->
		<header class="flex flex-wrap items-end justify-between gap-4">
			<div>
				<p class="text-[11px] font-bold uppercase tracking-widest text-(--brand-muted)">
					{{ new Date().toLocaleDateString("fr-FR", { weekday: "long", day: "numeric", month: "long" }) }}
				</p>
				<h1 class="mt-1 font-heading text-3xl font-extrabold tracking-tight">
					Accueil
				</h1>
			</div>
			<ProjectSelector v-if="store.hasProjects" />
		</header>

		<EmptyState v-if="!store.hasProjects" />

		<template v-else>
			<StatusPanel />

			<ControlsBar />

			<div class="grid grid-cols-1 gap-5 lg:grid-cols-2">
				<NetworkCard />
				<QrCard />
			</div>

			<LogsPanel />
		</template>
	</div>
</template>

<script setup lang="ts">
	import { useLaralinkStore } from "../stores/laralink";

	definePageMeta({
		layout: "default"
	});

	const store = useLaralinkStore();

	onMounted(async () => {
		await store.init();
	});
</script>
