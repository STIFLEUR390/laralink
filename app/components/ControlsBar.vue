<template>
	<div class="rounded-2xl border border-(--brand-border) bg-(--brand-surface) p-4">
		<div class="grid grid-cols-2 gap-2">
			<UButton
				block
				size="lg"
				color="brand"
				:icon="store.isBusy ? 'lucide:loader-circle' : 'lucide:play'"
				:loading="store.isBusy"
				:disabled="store.isBusy"
				label="Démarrer"
				@click="store.start()"
			/>
			<UButton
				block
				size="lg"
				color="error"
				variant="outline"
				icon="lucide:square"
				label="Arrêter"
				:disabled="isStopped || store.isBusy"
				@click="store.stop()"
			/>
			<UButton
				block
				size="lg"
				color="neutral"
				variant="outline"
				icon="lucide:rotate-ccw"
				label="Redémarrer"
				:disabled="isStopped || store.isBusy"
				@click="store.restart()"
			/>
			<UButton
				block
				size="lg"
				color="neutral"
				variant="ghost"
				icon="lucide:refresh-cw"
				label="Actualiser"
				@click="store.refresh()"
			/>
		</div>
	</div>
</template>

<script setup lang="ts">
	import { useLaralinkStore } from "../stores/laralink";

	const store = useLaralinkStore();

	const isStopped = computed(() => store.status?.status === "stopped" || store.status?.status === "error");
</script>
