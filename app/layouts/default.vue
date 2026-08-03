<template>
	<div class="flex min-h-screen bg-(--brand-bg)">
		<!-- Barre latérale -->
		<aside class="fixed inset-y-0 left-0 z-30 flex w-64 flex-col border-r border-(--brand-border) bg-(--brand-surface)">
			<!-- Marque -->
			<NuxtLink to="/" class="flex items-center gap-3 px-5 py-5">
				<LaralinkLogo :size="34" />
				<div class="leading-tight">
					<p class="font-heading text-base font-extrabold tracking-tight">Laralink</p>
					<p class="text-xs text-(--brand-muted)">Laravel · LAN</p>
				</div>
			</NuxtLink>

			<!-- Navigation -->
			<nav class="mt-2 flex flex-col gap-1 px-3">
				<NuxtLink
					v-for="item in navItems"
					:key="item.to"
					:to="item.to"
					class="group flex items-center gap-3 rounded-lg px-3 py-2.5 text-sm font-medium transition-colors"
					:class="route.path === item.to
						? 'bg-(--brand-primary-soft) text-(--brand-text)'
						: 'text-(--brand-muted) hover:bg-(--brand-primary-soft)/50 hover:text-(--brand-text)'"
				>
					<UIcon :name="item.icon" class="size-5" />
					<span>{{ item.label }}</span>
					<span
						v-if="item.to === '/settings' && lockedHint"
						class="ml-auto inline-flex items-center gap-1 rounded-full px-2 py-0.5 text-[10px] font-semibold uppercase tracking-wider"
						:class="lockedHint === 'locked' ? 'bg-warning/15 text-warning' : 'bg-success/15 text-success'"
					>
						<UIcon :name="lockedHint === 'locked' ? 'lucide:lock' : 'lucide:lock-open'" class="size-3" />
						{{ lockedHint === "locked" ? "protégé" : "ouvert" }}
					</span>
				</NuxtLink>
			</nav>

			<div class="flex-1" />

			<!-- État du projet actif -->
			<div class="mx-3 mb-3 rounded-xl border border-(--brand-border) bg-(--brand-bg)/60 p-3">
				<p class="mb-2 text-[10px] font-bold uppercase tracking-widest text-(--brand-muted)">Projet actif</p>
				<template v-if="store.activeProject">
					<div class="flex items-center gap-2">
						<span
							class="size-2.5 rounded-full"
							:class="statusDotClass"
						/>
						<p class="truncate text-sm font-semibold">{{ store.activeProject.name }}</p>
					</div>
					<p class="mt-1 truncate font-mono text-xs text-(--brand-muted)">
						{{ statusLabel }}
					</p>
				</template>
				<p v-else class="text-sm text-(--brand-muted)">Aucun projet configuré.</p>
			</div>

			<!-- Thème -->
			<div class="flex items-center justify-between border-t border-(--brand-border) px-4 py-3">
				<p class="text-xs text-(--brand-muted)">Thème</p>
				<div class="flex gap-1">
					<UTooltip text="Mode clair">
						<UButton
							size="sm"
							variant="ghost"
							color="neutral"
							:class="colorMode.preference === 'light' ? 'bg-(--brand-primary-soft)' : ''"
							icon="lucide:sun"
							@click="colorMode.preference = 'light'"
						/>
					</UTooltip>
					<UTooltip text="Système">
						<UButton
							size="sm"
							variant="ghost"
							color="neutral"
							:class="colorMode.preference === 'system' ? 'bg-(--brand-primary-soft)' : ''"
							icon="lucide:monitor"
							@click="colorMode.preference = 'system'"
						/>
					</UTooltip>
					<UTooltip text="Mode sombre">
						<UButton
							size="sm"
							variant="ghost"
							color="neutral"
							:class="colorMode.preference === 'dark' ? 'bg-(--brand-primary-soft)' : ''"
							icon="lucide:moon"
							@click="colorMode.preference = 'dark'"
						/>
					</UTooltip>
				</div>
			</div>
		</aside>

		<!-- Contenu -->
		<main class="ml-64 min-w-0 flex-1">
			<div class="relative z-10 mx-auto max-w-5xl px-8 py-8">
				<slot />
			</div>
		</main>
	</div>
</template>

<script setup lang="ts">
	import { useRoute } from "vue-router";
	import { useLaralinkStore } from "../stores/laralink";

	const route = useRoute();
	const store = useLaralinkStore();
	const colorMode = useColorMode();

	const navItems = [
		{ to: "/", label: "Accueil", icon: "lucide:home" },
		{ to: "/settings", label: "Réglages", icon: "lucide:settings" }
	];

	const lockedHint = computed(() => {
		if (!store.settings) return null;
		if (!store.settings.hasPassword) return "open";
		return store.unlocked ? "open" : "locked";
	});

	const statusLabel = computed(() => {
		const status = store.status?.status ?? store.activeProject?.status ?? "stopped";
		const map: Record<string, string> = {
			stopped: "À l'arrêt",
			starting: "Démarrage…",
			running: store.status?.url ?? "En ligne",
			error: "Erreur"
		};
		return map[status] ?? status;
	});

	const statusDotClass = computed(() => {
		const status = store.status?.status ?? store.activeProject?.status ?? "stopped";
		const map: Record<string, string> = {
			stopped: "bg-(--brand-muted)",
			starting: "animate-pulse bg-warning",
			running: "bg-success",
			error: "bg-error"
		};
		return map[status] ?? "bg-(--brand-muted)";
	});

	onMounted(async () => {
		if (!store.projects.length) {
			await store.init();
		}
	});
</script>
