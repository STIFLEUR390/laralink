<template>
	<div class="rounded-2xl border border-(--brand-border) bg-(--brand-surface) p-5">
		<div class="flex items-start gap-3">
			<div class="grid size-10 shrink-0 place-items-center rounded-xl bg-(--brand-primary-soft)">
				<UIcon name="lucide:rocket" class="size-5 text-(--brand-primary)" />
			</div>
			<div class="min-w-0 flex-1">
				<h3 class="font-heading text-sm font-bold">Application de pré-lancement</h3>
				<p class="mt-0.5 text-xs text-(--brand-muted)">
					Facultatif : exécutée avant les vérifications (ex. Laragon, WampServer, Docker).
				</p>
			</div>
			<USwitch v-model="enabled" aria-label="Activer le pré-lancement" />
		</div>

		<div v-if="enabled" class="mt-4 space-y-4">
			<UFormField label="Chemin de l'application" required>
				<div class="flex gap-2">
					<UInput v-model="appPath" placeholder="C:\laragon\laragon.exe" class="flex-1" />
					<UButton variant="outline" color="neutral" icon="lucide:folder-open" :label="isSmall ? undefined : 'Parcourir'" @click="browse" />
				</div>
			</UFormField>

			<UFormField label="Arguments" optional>
				<UInput v-model="appArgs" placeholder="--start" />
			</UFormField>

			<UFormField label="Attente après lancement (ms)">
				<UInput v-model="waitMs" type="number" min="0" step="500" />
			</UFormField>
		</div>

		<p v-else class="mt-3 text-xs text-(--brand-muted)">
			Pré-lancement désactivé pour ce projet.
		</p>
	</div>
</template>

<script setup lang="ts">
	import { pickFile } from "../composables/usePicker";
	import type { PrelaunchInput } from "../types";

	const model = defineModel<PrelaunchInput>({ required: true });
	const isSmall = ref(false);

	const enabled = computed({
		get: () => model.value.isEnabled,
		set: (v) => {
			model.value.isEnabled = v;
		}
	});

	const appPath = computed({ get: () => model.value.appPath, set: (v) => (model.value.appPath = v) });
	const appArgs = computed({ get: () => model.value.appArgs ?? "", set: (v) => (model.value.appArgs = v || null) });

	const waitMs = computed({
		get: () => String(model.value.waitAfterLaunchMs ?? 5000),
		set: (v) => {
			const n = Number.parseInt(v, 10);
			model.value.waitAfterLaunchMs = Number.isNaN(n) ? 5000 : Math.max(0, n);
		}
	});

	async function browse() {
		const file = await pickFile("Choisir l'application", [{ name: "Exécutable", extensions: ["exe", "bat", "cmd"] }]);
		if (file) {
			model.value.appPath = file;
		}
	}
</script>
