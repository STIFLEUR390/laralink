<template>
	<div class="flex flex-col gap-5">
		<!-- En-tête -->
		<header>
			<p class="text-[11px] font-bold uppercase tracking-widest text-(--brand-muted)">
				Développeur
			</p>
			<h1 class="mt-1 font-heading text-3xl font-extrabold tracking-tight">
				À propos
			</h1>
		</header>

		<!-- Carte développeur -->
		<section class="overflow-hidden rounded-2xl border border-(--brand-border) bg-(--brand-surface)">
			<div class="h-20 bg-gradient-to-r from-brand-500/25 via-brand-400/15 to-transparent" />
			<div class="-mt-10 flex flex-col gap-4 px-6 pb-6">
				<div class="flex items-end gap-4">
					<UAvatar
						size="2xl"
						:ui="{
							background: 'bg-gradient-to-br from-brand-500 to-brand-700 text-white',
							rounded: 'rounded-2xl'
						}"
						text="FH"
						class="border-4 border-(--brand-surface) shadow-lg"
					/>
					<div class="pb-1">
						<h2 class="font-heading text-xl font-extrabold tracking-tight">
							Franck Hérold TAMTO TAMKO
						</h2>
						<p class="text-sm font-medium text-(--brand-primary)">
							Full-Stack Developer · Douala, Cameroun
						</p>
					</div>
				</div>

				<div class="max-w-3xl space-y-3 text-sm leading-relaxed text-(--brand-muted)">
					<p>
						Développeur Full-Stack passionné par la conception d'applications web modernes et d'outils qui
						résolvent des problèmes concrets. Diplômé d'un Master en Informatique Industrielle &amp;
						Automatisme (Institut Universitaire de la Côte, 2020), j'accompagne depuis plus de 4 ans la
						conception, l'évolution et le déploiement d'applications web — de l'analyse jusqu'à la mise en
						production.
					</p>
					<p>
						Laralink est né de ma veille quotidienne sur l'écosystème Laravel : un outil desktop qui simplifie
						le lancement de projets Laravel locaux et leur exposition sur le réseau local, pensé pour les
						développeurs qui jonglent entre plusieurs projets et versions PHP.
					</p>
				</div>
			</div>
		</section>

		<!-- Spécialités -->
		<section class="rounded-2xl border border-(--brand-border) bg-(--brand-surface) p-5">
			<h2 class="flex items-center gap-2 font-heading text-sm font-bold uppercase tracking-widest text-(--brand-muted)">
				<UIcon name="lucide:wrench" class="size-4" />
				Spécialités
			</h2>
			<div class="mt-3 flex flex-wrap gap-1.5">
				<span
					v-for="skill in skills"
					:key="skill"
					class="inline-flex items-center gap-1.5 rounded-full border border-(--brand-border) bg-(--brand-bg)/60 px-3 py-1 text-xs font-medium text-(--brand-text)"
				>
					<UIcon :name="skill.icon" class="size-3.5 text-(--brand-primary)" />
					{{ skill.label }}
				</span>
			</div>
		</section>

		<!-- Contact -->
		<section class="rounded-2xl border border-(--brand-border) bg-(--brand-surface) p-5">
			<h2 class="flex items-center gap-2 font-heading text-sm font-bold uppercase tracking-widest text-(--brand-muted)">
				<UIcon name="lucide:mail" class="size-4" />
				Contact
			</h2>
			<div class="mt-3 grid grid-cols-1 gap-2 sm:grid-cols-2 lg:grid-cols-3">
				<button
					v-for="link in links"
					:key="link.label"
					type="button"
					class="group flex items-center gap-3 rounded-xl border border-(--brand-border) bg-(--brand-bg)/40 px-4 py-3 text-left transition-colors hover:border-brand-400 hover:bg-(--brand-primary-soft)/50"
					@click="open(link.href)"
				>
					<span class="grid size-9 shrink-0 place-items-center rounded-lg bg-(--brand-primary-soft) transition-colors group-hover:bg-brand-500/15">
						<UIcon :name="link.icon" class="size-4.5 text-(--brand-primary)" />
					</span>
					<div class="min-w-0">
						<p class="text-sm font-semibold">
							{{ link.label }}
						</p>
						<p class="truncate font-mono text-[11px] text-(--brand-muted)">
							{{ link.value }}
						</p>
					</div>
				</button>
			</div>
		</section>

		<!-- Application -->
		<section class="rounded-2xl border border-(--brand-border) bg-(--brand-surface) p-5">
			<h2 class="flex items-center gap-2 font-heading text-sm font-bold uppercase tracking-widest text-(--brand-muted)">
				<UIcon name="lucide:box" class="size-4" />
				Application
			</h2>
			<div class="mt-3 grid grid-cols-2 gap-2 sm:grid-cols-3 lg:grid-cols-6">
				<div v-for="info in appInfo" :key="info.label" class="rounded-xl border border-(--brand-border) bg-(--brand-bg)/40 px-4 py-3">
					<p class="text-[10px] font-bold uppercase tracking-widest text-(--brand-muted)">
						{{ info.label }}
					</p>
					<p class="mt-1 truncate font-mono text-sm font-semibold">
						{{ info.value }}
					</p>
				</div>
			</div>

			<div class="mt-4 flex flex-wrap items-center justify-between gap-3 rounded-xl border border-(--brand-border) bg-(--brand-bg)/40 px-4 py-3">
				<div class="flex items-center gap-2 text-sm">
					<UIcon name="lucide:github" class="size-4 text-(--brand-muted)" />
					<span class="text-(--brand-muted)">Code source ouvert — licence</span>
					<UBadge color="brand" variant="soft" label="MIT" />
				</div>
				<div class="flex gap-2">
					<UButton
						size="sm"
						color="brand"
						variant="soft"
						icon="lucide:refresh-cw"
						label="Vérifier les mises à jour"
						:loading="updater.checking.value"
						@click="updater.checkForUpdates()"
					/>
					<UButton
						size="sm"
						color="neutral"
						variant="outline"
						icon="lucide:external-link"
						label="Dépôt GitHub"
						@click="open('https://github.com/STIFLEUR390/laralink')"
					/>
				</div>
			</div>
		</section>
	</div>
</template>

<script setup lang="ts">
	import { useUpdater } from "../composables/useUpdater";
	import { commands, isTauriEnv } from "../services/commands";

	definePageMeta({
		layout: "default"
	});

	const updater = useUpdater();

	const skills = [
		{ label: "Laravel", icon: "lucide:layers" },
		{ label: "Vue.js", icon: "lucide:atom" },
		{ label: "Nuxt", icon: "lucide:rocket" },
		{ label: "Inertia.js", icon: "lucide:zap" },
		{ label: "PostgreSQL", icon: "lucide:database" },
		{ label: "APIs RESTful", icon: "lucide:plug" },
		{ label: "Docker", icon: "lucide:container" },
		{ label: "Stripe · Flutterwave · PayPal", icon: "lucide:credit-card" },
		{ label: "Automatisation", icon: "lucide:bot" }
	];

	const links = [
		{ label: "Email", value: "heroldtamko39@gmail.com", icon: "lucide:mail", href: "mailto:heroldtamko39@gmail.com" },
		{ label: "GitHub", value: "@STIFLEUR390", icon: "lucide:github", href: "https://github.com/STIFLEUR390" },
		{ label: "LinkedIn", value: "in/heroldtamko", icon: "lucide:linkedin", href: "https://linkedin.com/in/heroldtamko" },
		{ label: "X (Twitter)", value: "@STIFLEUR390", icon: "lucide:twitter", href: "https://x.com/STIFLEUR390" },
		{ label: "Portfolio", value: "portfolio.aplix.nl", icon: "lucide:globe", href: "https://portfolio.aplix.nl" }
	];

	const appVersion = ref("—");
	const tauriVersion = ref("—");

	onMounted(async () => {
		if (!isTauriEnv()) return;
		try {
			const app = await import("@tauri-apps/api/app");
			appVersion.value = await app.getVersion();
			tauriVersion.value = await app.getTauriVersion();
		} catch {
			// API indisponible : on garde les valeurs par défaut.
		}
	});

	const appInfo = computed(() => [
		{ label: "Version", value: `v${appVersion.value}` },
		{ label: "Tauri", value: tauriVersion.value },
		{ label: "Nuxt", value: "4.5" },
		{ label: "Vue", value: "3.5" },
		{ label: "Base de données", value: "SQLite" },
		{ label: "Licence", value: "MIT" }
	]);

	async function open(url: string) {
		try {
			if (isTauriEnv()) {
				await commands.openUrl(url);
			} else {
				window.open(url, "_blank");
			}
		} catch (e) {
			console.error("Ouverture impossible", e);
		}
	}
</script>
