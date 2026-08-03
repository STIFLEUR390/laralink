<template>
	<div class="flex items-center gap-3">
		<div class="flex flex-col gap-1.5">
			<label class="text-[11px] font-bold uppercase tracking-widest text-(--brand-muted)">
				Projet actif
			</label>
			<USelectMenu
				v-if="store.projects.length > 1"
				v-model="selectedId"
				:items="items"
				value-key="id"
				class="w-72"
				:disabled="store.isBusy"
				@update:model-value="onSelect"
			/>
			<div v-else-if="store.activeProject" class="flex items-center gap-2 text-sm font-semibold">
				<LaralinkLogo :size="20" />
				{{ store.activeProject.name }}
			</div>
		</div>
	</div>
</template>

<script setup lang="ts">
	import { useLaralinkStore } from "../stores/laralink";

	const store = useLaralinkStore();
	const selectedId = ref<number | null>(null);

	const items = computed(() =>
		store.projects.map((p) => ({
			id: p.id,
			label: p.name,
			icon: p.isDefault ? "lucide:star" : "lucide:folder-git-2",
			description: p.runtimeLabel
		}))
	);

	watch(
		() => store.activeProjectId,
		(id) => {
			selectedId.value = id;
		},
		{ immediate: true }
	);

	async function onSelect(value: { id: number } | null) {
		if (value && value.id !== store.activeProjectId.value) {
			await store.selectProject(value.id);
		}
	}
</script>
