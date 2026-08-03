<template>
	<div class="rounded-2xl border border-(--brand-border) bg-(--brand-surface) p-6">
		<div class="flex items-center justify-between">
			<h3 class="flex items-center gap-2 font-heading text-sm font-bold uppercase tracking-widest text-(--brand-muted)">
				<UIcon name="lucide:network" class="size-4" />
				Réseau local
			</h3>
			<UButton
				size="xs"
				variant="ghost"
				color="neutral"
				icon="lucide:refresh-cw"
				@click="store.loadNetwork()"
			/>
		</div>

		<div class="mt-4 space-y-3">
			<!-- Adresse IP -->
			<div>
				<p class="mb-1 text-[11px] font-semibold uppercase tracking-wide text-(--brand-muted)">Adresse IP</p>
				<div class="flex items-center gap-2">
					<span class="relative flex size-2">
						<span v-if="store.network.selected" class="absolute inline-flex size-full animate-ping rounded-full bg-success opacity-60" />
						<span class="relative inline-flex size-2 rounded-full" :class="store.network.selected ? 'bg-success' : 'bg-error'" />
					</span>
					<code class="font-mono text-sm font-semibold">
						{{ store.network.selected?.addr ?? "Aucune IP détectée" }}
					</code>
				</div>
				<p v-if="store.network.selected" class="mt-0.5 text-[11px] text-(--brand-muted)">
					Interface : {{ store.network.selected.iface }}
				</p>
			</div>

			<!-- Port -->
			<div>
				<p class="mb-1 text-[11px] font-semibold uppercase tracking-wide text-(--brand-muted)">Port</p>
				<code class="font-mono text-sm font-semibold">
					{{ store.status?.port ?? "—" }}
				</code>
			</div>

			<!-- URL -->
			<div>
				<p class="mb-1 text-[11px] font-semibold uppercase tracking-wide text-(--brand-muted)">URL d'accès</p>
				<div class="flex items-center gap-1.5">
					<code
						class="min-w-0 flex-1 truncate rounded-lg border border-(--brand-border) bg-(--brand-bg)/60 px-2.5 py-1.5 font-mono text-sm"
						:class="url ? 'text-brand-700 dark:text-brand-300' : 'text-(--brand-muted)'"
					>
						{{ url ?? "Démarrez le projet pour obtenir l'URL" }}
					</code>
					<UTooltip text="Copier l'URL">
						<UButton
							size="sm"
							color="neutral"
							variant="outline"
							icon="lucide:copy"
							:disabled="!url"
							@click="copyUrl"
						/>
					</UTooltip>
					<UTooltip text="Ouvrir dans le navigateur">
						<UButton
							size="sm"
							color="neutral"
							variant="outline"
							icon="lucide:external-link"
							:disabled="!url"
							@click="openBrowser"
						/>
					</UTooltip>
				</div>
			</div>
		</div>
	</div>
</template>

<script setup lang="ts">
	import { useLaralinkStore } from "../stores/laralink";
	import { useClipboard } from "../composables/useClipboard";
	import { commands } from "../services/commands";

	const store = useLaralinkStore();
	const { copy } = useClipboard();
	const toast = useToast();

	const url = computed(() => store.status?.url ?? null);

	async function copyUrl() {
		if (!url.value) return;
		const ok = await copy(url.value);
		toast.add({
			title: ok ? "URL copiée" : "Copie impossible",
			description: ok ? url.value : "Utilisez la sélection manuelle",
			color: ok ? "success" : "error",
			icon: ok ? "lucide:clipboard-check" : "lucide:clipboard-x"
		});
	}

	async function openBrowser() {
		if (!url.value) return;
		await commands.openUrl(url.value);
	}
</script>
