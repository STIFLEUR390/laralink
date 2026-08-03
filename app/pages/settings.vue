<template>
	<div class="flex flex-col gap-5">
		<header>
			<p class="text-[11px] font-bold uppercase tracking-widest text-(--brand-muted)">Administration</p>
			<h1 class="mt-1 font-heading text-3xl font-extrabold tracking-tight">Réglages</h1>
		</header>

		<PasswordGate v-if="locked" @unlocked="store.unlocked = true" />

		<template v-else>
			<UTabs v-model="tab" :items="tabs" class="w-full">
				<template #default="{ item }">
					<UIcon :name="item.icon" class="size-4" />
					{{ item.label }}
				</template>
			</UTabs>

			<!-- Onglet : Projets -->
			<div v-if="tab === 'projects'" class="grid grid-cols-1 gap-5 xl:grid-cols-[320px_1fr]">
				<!-- Liste des projets -->
				<div class="flex h-fit flex-col gap-3">
					<div class="flex items-center justify-between px-1">
						<p class="text-xs font-bold uppercase tracking-widest text-(--brand-muted)">Projets</p>
						<UButton size="xs" color="brand" variant="outline" icon="lucide:plus" label="Nouveau" @click="createNew" />
					</div>

					<div class="flex flex-col gap-1.5">
						<button
							v-for="p in store.projects"
							:key="p.id"
							type="button"
							class="group flex items-center gap-3 rounded-xl border px-3 py-2.5 text-left transition-colors"
							:class="selectedId === p.id
								? 'border-brand-400 bg-(--brand-primary-soft)/60'
								: 'border-(--brand-border) bg-(--brand-surface) hover:border-brand-300'"
							@click="select(p.id)"
						>
							<span class="grid size-8 shrink-0 place-items-center rounded-lg bg-(--brand-primary-soft)">
								<UIcon name="lucide:folder-git-2" class="size-4" :class="selectedId === p.id ? 'text-(--brand-primary)' : 'text-(--brand-muted)'" />
							</span>
							<div class="min-w-0 flex-1">
								<p class="flex items-center gap-1.5 truncate text-sm font-semibold">
									{{ p.name }}
									<UIcon v-if="p.isDefault" name="lucide:star" class="size-3 fill-warning text-warning" />
								</p>
								<p class="truncate text-[11px] text-(--brand-muted)">{{ p.runtimeLabel }}</p>
							</div>
							<span class="size-2 shrink-0 rounded-full" :class="statusDot(p.status)" />
						</button>

						<div v-if="!store.projects.length" class="rounded-xl border border-dashed border-(--brand-border) px-4 py-8 text-center text-sm text-(--brand-muted)">
							Aucun projet. Créez le premier avec « Nouveau ».
						</div>
					</div>

					<!-- Actions sur le projet sélectionné -->
					<div v-if="selected" class="mt-1 flex flex-col gap-1.5 rounded-xl border border-(--brand-border) bg-(--brand-surface) p-3">
						<p class="px-1 text-[10px] font-bold uppercase tracking-widest text-(--brand-muted)">Actions</p>
						<div class="grid grid-cols-2 gap-1.5">
							<UButton size="sm" color="neutral" variant="outline" icon="lucide:copy" label="Dupliquer" @click="duplicate(selected)" />
							<UButton size="sm" color="neutral" variant="outline" icon="lucide:star" :label="selected.isDefault ? 'Défaut' : 'Définir défaut'" @click="setDefault(selected)" />
							<UButton size="sm" color="brand" variant="soft" icon="lucide:play" label="Démarrer" @click="start(selected)" />
							<UButton size="sm" color="error" variant="outline" icon="lucide:square" label="Arrêter" @click="stop(selected)" />
						</div>
					</div>
				</div>

				<!-- Formulaire -->
				<ProjectForm
					:project="selectedDetail"
					@saved="onSaved"
					@cancelled="selectedId = store.projects[0]?.id ?? null"
					@deleted="onDeleted"
				/>
			</div>

			<!-- Onglet : Sécurité -->
			<div v-else-if="tab === 'security'">
				<SettingsSecurity />
			</div>

			<!-- Onglet : Diagnostics -->
			<div v-else-if="tab === 'diagnostics'">
				<SettingsDiagnostics />
			</div>
		</template>
	</div>
</template>

<script setup lang="ts">
	import { useLaralinkStore } from "../stores/laralink";
	import { commands } from "../services/commands";
	import type { ProjectDetail, ProjectSummary } from "../types";

	definePageMeta({
		layout: "default"
	});

	const store = useLaralinkStore();
	const toast = useToast();
	const route = useRoute();

	const tab = ref<"projects" | "security" | "diagnostics">("projects");
	const selectedId = ref<number | null>(null);
	const selectedDetail = ref<ProjectDetail | null>(null);
	const loadingDetail = ref(false);

	const tabs = [
		{ label: "Projets", icon: "lucide:folder-git-2", value: "projects" },
		{ label: "Sécurité", icon: "lucide:shield-check", value: "security" },
		{ label: "Diagnostics", icon: "lucide:stethoscope", value: "diagnostics" }
	];

	const locked = computed(() => {
		if (!store.settings) return true;
		return store.settings.hasPassword && !store.unlocked;
	});

	const selected = computed(() => store.projects.find((p) => p.id === selectedId.value) ?? null);

	function statusDot(status: string) {
		const map: Record<string, string> = {
			stopped: "bg-(--brand-muted)",
			starting: "animate-pulse bg-warning",
			running: "bg-success",
			error: "bg-error"
		};
		return map[status] ?? "bg-(--brand-muted)";
	}

	watch(
		() => store.projects,
		(list) => {
			if (!list.some((p) => p.id === selectedId.value)) {
				selectedId.value = list[0]?.id ?? null;
			}
		},
		{ immediate: true }
	);

	watch(
		selectedId,
		async (id) => {
			if (id == null) {
				selectedDetail.value = null;
				return;
			}
			loadingDetail.value = true;
			try {
				selectedDetail.value = await commands.getProject(id);
			} catch {
				selectedDetail.value = null;
			} finally {
				loadingDetail.value = false;
			}
		},
		{ immediate: true }
	);

	async function select(id: number) {
		selectedId.value = id;
	}

	function createNew() {
		selectedId.value = null;
		selectedDetail.value = null;
	}

	function onSaved(saved: ProjectDetail) {
		selectedId.value = saved.id;
		selectedDetail.value = saved;
	}

	function onDeleted(_id: number) {
		selectedId.value = store.projects[0]?.id ?? null;
	}

	async function duplicate(project: ProjectSummary) {
		try {
			const copy = await commands.duplicateProject(project.id);
			await store.loadProjects(false);
			selectedId.value = copy.id;
			selectedDetail.value = copy;
			toast.add({ title: "Projet dupliqué", description: copy.name, color: "success", icon: "lucide:copy" });
		} catch (e) {
			toast.add({ title: "Duplication impossible", description: String(e), color: "error", icon: "lucide:triangle-alert" });
		}
	}

	async function setDefault(project: ProjectSummary) {
		await store.setActiveProject(project.id);
		toast.add({ title: "Projet par défaut", description: project.name, color: "success", icon: "lucide:star" });
	}

	async function start(project: ProjectSummary) {
		await commands.startProject(project.id);
	}

	async function stop(project: ProjectSummary) {
		await commands.stopProject(project.id);
	}

	onMounted(async () => {
		await store.init();
		// Route ?new=1 → créer un projet directement.
		if (route.query.new) {
			createNew();
		}
	});
</script>
