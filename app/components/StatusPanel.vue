<template>
	<div class="relative overflow-hidden rounded-2xl border border-(--brand-border) bg-(--brand-surface) p-6">
		<!-- Bandeau décoratif -->
		<div class="absolute inset-x-0 top-0 h-1" :class="accentBarClass" />

		<div class="flex items-start justify-between gap-4">
			<div class="flex items-center gap-4">
				<!-- Indicateur d'état -->
				<div class="relative grid size-14 shrink-0 place-items-center">
					<span
						class="absolute inset-0 rounded-full"
						:class="[pulseClass, ringClass]"
					/>
					<span class="relative size-4 rounded-full" :class="dotClass" />
				</div>

				<div>
					<p class="text-[11px] font-bold uppercase tracking-widest text-(--brand-muted)">
						État du serveur
					</p>
					<h2 class="mt-0.5 font-heading text-2xl font-extrabold tracking-tight">
						{{ statusLabel }}
					</h2>
					<div class="mt-2 flex flex-wrap items-center gap-2">
						<RuntimeBadge :type="store.status?.runtimeType ?? 'none'" :label="store.status?.runtimeLabel" />
						<UBadge v-if="store.status?.phpVersion" variant="subtle" color="neutral" :label="`PHP ${store.status.phpVersion}`" />
						<span v-if="store.status?.pid" class="font-mono text-xs text-(--brand-muted)">
							pid {{ store.status.pid }}
						</span>
					</div>
				</div>
			</div>
		</div>

		<!-- Checks -->
		<div class="mt-5 grid grid-cols-1 gap-2 sm:grid-cols-3">
			<div
				v-for="check in checkItems"
				:key="check.key"
				class="flex items-center gap-2.5 rounded-xl border px-3 py-2.5"
				:class="check.classes"
			>
				<UIcon :name="check.icon" class="size-4.5 shrink-0" />
				<div class="min-w-0">
					<p class="text-xs font-semibold">{{ check.label }}</p>
					<p class="truncate text-[11px]" :class="check.subClass">{{ check.message }}</p>
				</div>
			</div>
		</div>

		<!-- Barre de progression pendant le démarrage -->
		<UProgress
			v-if="store.isBusy"
			class="mt-4"
			:value="undefined"
			indeterminate
			size="xs"
		/>
	</div>
</template>

<script setup lang="ts">
	import { useLaralinkStore } from "../stores/laralink";
	import type { CheckResult } from "../types";

	const store = useLaralinkStore();

	const statusLabel = computed(() => {
		const map: Record<string, string> = {
			stopped: "À l'arrêt",
			starting: "Démarrage en cours…",
			running: "Prêt — en ligne",
			error: "Erreur de lancement"
		};
		return map[store.status?.status ?? "stopped"] ?? "À l'arrêt";
	});

	const dotClass = computed(() => {
		const map: Record<string, string> = {
			stopped: "bg-(--brand-muted)",
			starting: "bg-warning",
			running: "bg-success",
			error: "bg-error"
		};
		return map[store.status?.status ?? "stopped"];
	});

	const ringClass = computed(() => {
		const map: Record<string, string> = {
			stopped: "bg-(--brand-muted)/10",
			starting: "bg-warning/10",
			running: "bg-success/15",
			error: "bg-error/10"
		};
		return map[store.status?.status ?? "stopped"];
	});

	const pulseClass = computed(() =>
		store.status?.status === "starting" ? "animate-ping" : ""
	);

	const accentBarClass = computed(() => {
		const map: Record<string, string> = {
			stopped: "bg-(--brand-muted)",
			starting: "bg-warning",
			running: "bg-success",
			error: "bg-error"
		};
		return map[store.status?.status ?? "stopped"];
	});

	function checkValue(key: string): CheckResult | undefined {
		return store.checks.find((c) => c.checkType === key);
	}

	const checkItems = computed(() => {
		const defs = [
			{ key: "database", label: "Base de données", okMsg: "OK", failMsg: "Injoignable" },
			{ key: "php_runtime", label: "PHP", okMsg: "OK", failMsg: "Introuvable" },
			{ key: "artisan_file", label: "Laravel", okMsg: "OK", failMsg: "Artisan manquant" }
		];
		return defs.map((def) => {
			const c = checkValue(def.key);
			const ok = c ? c.isSuccess : null;
			return {
				...def,
				ok,
				message: c ? c.message : "Non testé",
				icon: ok === null ? "lucide:minus-circle" : ok ? "lucide:check-circle-2" : "lucide:x-circle",
				classes: ok === null
					? "border-(--brand-border) bg-(--brand-bg)/40 text-(--brand-muted)"
					: ok
						? "border-success/30 bg-success/5 text-(--brand-text)"
						: "border-error/30 bg-error/5 text-error",
				subClass: ok === null ? "text-(--brand-muted)" : ok ? "text-success" : "text-error"
			};
		});
	});
</script>
