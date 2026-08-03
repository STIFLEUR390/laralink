<template>
	<div class="rounded-2xl border border-(--brand-border) bg-(--brand-surface) p-5">
		<div class="flex items-start gap-3">
			<div class="grid size-10 shrink-0 place-items-center rounded-xl bg-(--brand-primary-soft)">
				<UIcon name="lucide:network" class="size-5 text-(--brand-primary)" />
			</div>
			<div class="min-w-0 flex-1">
				<h3 class="font-heading text-sm font-bold">Réseau</h3>
				<p class="mt-0.5 text-xs text-(--brand-muted)">
					Liaison du serveur et port préféré.
				</p>
			</div>
		</div>

		<div class="mt-4 space-y-4">
			<UFormField label="Adresse de liaison (bind)" hint="0.0.0.0 expose le serveur sur le réseau local">
				<USelect
					v-model="bindHost"
					:items="[
						{ value: '0.0.0.0', label: '0.0.0.0 — Toutes les interfaces (recommandé)' },
						{ value: '127.0.0.1', label: '127.0.0.1 — Local uniquement' }
					]"
					value-key="value"
				/>
			</UFormField>

			<UFormField label="Port préféré" hint="Laissez vide pour un port automatique (8000+)" optional>
				<UInput
					v-model="portInput"
					type="number"
					min="1"
					max="65535"
					placeholder="8000"
				/>
			</UFormField>
		</div>
	</div>
</template>

<script setup lang="ts">
	import type { NetworkInput } from "../types";

	const model = defineModel<NetworkInput>({ required: true });

	const bindHost = computed({
		get: () => model.value.bindHost ?? "0.0.0.0",
		set: (v) => {
			model.value.bindHost = v;
		}
	});

	const portInput = computed<string>({
		get: () => (model.value.preferredPort != null ? String(model.value.preferredPort) : ""),
		set: (v) => {
			const n = Number.parseInt(v, 10);
			model.value.preferredPort = v.trim() === "" || Number.isNaN(n) ? null : n;
		}
	});
</script>
