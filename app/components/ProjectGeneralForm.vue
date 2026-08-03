<template>
	<div class="rounded-2xl border border-(--brand-border) bg-(--brand-surface) p-5">
		<div class="flex items-start gap-3">
			<div class="grid size-10 shrink-0 place-items-center rounded-xl bg-(--brand-primary-soft)">
				<UIcon name="lucide:info" class="size-5 text-(--brand-primary)" />
			</div>
			<div class="min-w-0 flex-1">
				<h3 class="font-heading text-sm font-bold">Informations générales</h3>
				<p class="mt-0.5 text-xs text-(--brand-muted)">Identité du projet et chemin Laravel.</p>
			</div>
		</div>

		<div class="mt-4 space-y-4">
			<UFormField label="Nom du projet" required>
				<UInput v-model="model.name" placeholder="Mon application Laravel" />
			</UFormField>

			<UFormField label="Chemin du projet Laravel" required hint="Doit contenir le fichier artisan">
				<div class="flex gap-2">
					<UInput v-model="model.laravelPath" placeholder="C:\laragon\www\mon-projet" class="flex-1" />
					<UButton
						variant="outline"
						color="neutral"
						icon="lucide:folder-open"
						:label="isSmall ? undefined : 'Parcourir'"
						@click="browse"
					/>
				</div>
			</UFormField>

			<UFormField label="Description" optional>
				<UTextarea v-model="model.description" rows="2" placeholder="Description courte du projet…" />
			</UFormField>

			<div class="grid grid-cols-1 gap-3 sm:grid-cols-2">
				<div class="flex items-center justify-between rounded-xl border border-(--brand-border) px-3 py-2.5">
					<div>
						<p class="text-sm font-medium">Démarrage auto</p>
						<p class="text-[11px] text-(--brand-muted)">Lancer au démarrage de Laralink</p>
					</div>
					<USwitch v-model="model.autoStart" />
				</div>
				<div class="flex items-center justify-between rounded-xl border border-(--brand-border) px-3 py-2.5">
					<div>
						<p class="text-sm font-medium">Ouvrir le navigateur</p>
						<p class="text-[11px] text-(--brand-muted)">À la fin du démarrage</p>
					</div>
					<USwitch v-model="model.autoOpenBrowser" />
				</div>
			</div>
		</div>
	</div>
</template>

<script setup lang="ts">
	import { pickFolder } from "../composables/usePicker";

	const model = defineModel<{
		name: string;
		laravelPath: string;
		description: string | null;
		autoStart: boolean;
		autoOpenBrowser: boolean;
	}>({ required: true });

	const isSmall = ref(false);

	async function browse() {
		const folder = await pickFolder();
		if (folder) {
			model.value.laravelPath = folder;
		}
	}
</script>
