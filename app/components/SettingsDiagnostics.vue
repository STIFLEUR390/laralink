<template>
	<div class="flex flex-col gap-5">
		<div class="rounded-2xl border border-(--brand-border) bg-(--brand-surface) p-5">
			<div class="flex flex-wrap items-center justify-between gap-3">
				<div>
					<h3 class="flex items-center gap-2 font-heading text-sm font-bold uppercase tracking-widest text-(--brand-muted)">
						<UIcon name="lucide:stethoscope" class="size-4" />
						Tests et diagnostics
					</h3>
					<p class="mt-1 text-xs text-(--brand-muted)">
						Vérifie le chemin Laravel, artisan, PHP, la base de données, le port et le réseau.
					</p>
				</div>
				<UButton
					color="brand"
					icon="lucide:activity"
					label="Lancer les tests"
					:loading="running"
					:disabled="!selectedProjectId"
					@click="run"
				/>
			</div>

			<div class="mt-4 space-y-2">
				<template v-if="running">
					<div v-for="i in 6" :key="i" class="flex animate-pulse items-center gap-3 rounded-xl border border-(--brand-border) px-3 py-2.5">
						<span class="size-3 rounded-full bg-(--brand-border)" />
						<span class="h-3 w-40 rounded bg-(--brand-border)" />
					</div>
				</template>

				<div
					v-for="check in results"
					:key="check.checkType"
					class="flex items-start gap-3 rounded-xl border px-3 py-2.5"
					:class="check.isSuccess ? 'border-success/30 bg-success/5' : 'border-error/30 bg-error/5'"
				>
					<UIcon
						:name="check.isSuccess ? 'lucide:check-circle-2' : 'lucide:x-circle'"
						class="mt-0.5 size-4.5 shrink-0"
						:class="check.isSuccess ? 'text-success' : 'text-error'"
					/>
					<div class="min-w-0">
						<p class="text-sm font-semibold">{{ label(check.checkType) }}</p>
						<p class="text-xs" :class="check.isSuccess ? 'text-success' : 'text-error'">
							{{ check.message }}
						</p>
						<p class="mt-0.5 font-mono text-[10px] text-(--brand-muted)">
							{{ new Date(check.checkedAt).toLocaleString("fr-FR") }}
						</p>
					</div>
				</div>

				<div v-if="!running && !results.length" class="rounded-xl border border-dashed border-(--brand-border) px-4 py-8 text-center text-sm text-(--brand-muted)">
					<UIcon name="lucide:scan-line" class="mx-auto size-6" />
					<p class="mt-2">Aucun test exécuté pour ce projet.</p>
				</div>
			</div>
		</div>
	</div>
</template>

<script setup lang="ts">
	import { useLaralinkStore } from "../stores/laralink";
	import { commands } from "../services/commands";
	import type { CheckResult } from "../types";

	const store = useLaralinkStore();
	const running = ref(false);
	const results = ref<CheckResult[]>([]);

	const selectedProjectId = computed(() => store.activeProjectId);

	const labels: Record<string, string> = {
		laravel_path: "Chemin du projet",
		artisan_file: "Fichier artisan",
		php_runtime: "Runtime PHP",
		database: "Base de données",
		port: "Port",
		network: "Réseau local"
	};

	function label(key: string) {
		return labels[key] ?? key;
	}

	async function run() {
		if (!selectedProjectId.value || running.value) return;
		running.value = true;
		results.value = [];
		try {
			results.value = await commands.runDiagnostics(selectedProjectId.value);
			await store.refreshStatus();
		} catch (e) {
			results.value = [{ checkType: "laravel_path", isSuccess: false, message: String(e), checkedAt: new Date().toISOString() }];
		} finally {
			running.value = false;
		}
	}

	onMounted(async () => {
		if (selectedProjectId.value) {
			results.value = await commands.getDiagnostics(selectedProjectId.value, 12);
		}
	});
</script>
