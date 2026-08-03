<template>
	<div class="flex flex-col gap-5">
		<!-- Barre d'actions -->
		<div class="flex flex-wrap items-center justify-between gap-3">
			<div>
				<h2 class="font-heading text-lg font-bold tracking-tight">
					{{ isNew ? "Nouveau projet" : `Modifier — ${project.name}` }}
				</h2>
				<p v-if="!isNew" class="text-xs text-(--brand-muted)">slug : {{ project.slug }}</p>
			</div>
			<div class="flex gap-2">
				<UButton
					v-if="!isNew"
					color="error"
					variant="outline"
					icon="lucide:trash-2"
					label="Supprimer"
					@click="confirmDelete = true"
				/>
				<UButton color="neutral" variant="ghost" icon="lucide:x" label="Annuler" @click="emit('cancelled')" />
				<UButton
					color="brand"
					icon="lucide:save"
					:label="saving ? 'Enregistrement…' : 'Enregistrer'"
					:loading="saving"
					@click="save"
				/>
			</div>
		</div>

		<ProjectGeneralForm v-model="form" />
		<ProjectRuntimeForm v-model="form.runtime" />
		<ProjectNetworkForm v-model="form.network" />
		<ProjectDatabaseForm v-model="form.database" />
		<ProjectPrelaunchForm v-model="form.prelaunch" />

		<UModal v-model:open="confirmDelete">
			<UCard>
				<template #header>
					<h3 class="font-heading font-bold">Supprimer le projet ?</h3>
				</template>
				<p class="text-sm text-(--brand-muted)">
					« {{ project?.name ?? "Ce projet" }} » et toute sa configuration (runtime, réseau, base de données, historique)
					seront définitivement supprimés.
				</p>
				<template #footer>
					<div class="flex justify-end gap-2">
						<UButton color="neutral" variant="ghost" label="Annuler" @click="confirmDelete = false" />
						<UButton color="error" icon="lucide:trash-2" label="Supprimer" :loading="deleting" @click="remove" />
					</div>
				</template>
			</UCard>
		</UModal>
	</div>
</template>

<script setup lang="ts">
	import { useLaralinkStore } from "../stores/laralink";
	import { commands } from "../services/commands";
	import type { DatabaseInput, NetworkInput, PrelaunchInput, ProjectDetail, ProjectInput, RuntimeInput } from "../types";

	const props = defineProps<{
		project: ProjectDetail | null;
	}>();
	const emit = defineEmits<{
		saved: [project: ProjectDetail];
		cancelled: [];
		deleted: [id: number];
	}>();

	const store = useLaralinkStore();
	const toast = useToast();

	const isNew = computed(() => !props.project);
	const saving = ref(false);
	const deleting = ref(false);
	const confirmDelete = ref(false);

	const emptyRuntime = (): RuntimeInput => ({
		runtimeType: "system_php",
		displayName: null,
		binaryPath: null,
		versionLabel: null,
		extraArgs: null
	});

	const emptyNetwork = (): NetworkInput => ({
		bindHost: "0.0.0.0",
		preferredPort: null
	});

	const emptyDatabase = (): DatabaseInput => ({
		driver: "mysql",
		host: "127.0.0.1",
		port: 3306,
		databaseName: "laravel",
		username: "root",
		password: null,
		sqlitePath: null,
		timeoutSeconds: 15,
		isRequired: true
	});

	const emptyPrelaunch = (): PrelaunchInput => ({
		appPath: "",
		appArgs: null,
		isEnabled: false,
		waitAfterLaunchMs: 5000
	});

	function buildForm(p: ProjectDetail | null): ProjectInput {
		if (!p) {
			return {
				name: "",
				laravelPath: "",
				description: null,
				autoStart: false,
				autoOpenBrowser: false,
				runtime: emptyRuntime(),
				network: emptyNetwork(),
				database: emptyDatabase(),
				prelaunch: emptyPrelaunch()
			};
		}
		const active = p.runtimes.find((r) => r.isActive) ?? p.runtimes[0];
		return {
			name: p.name,
			laravelPath: p.laravelPath,
			description: p.description,
			autoStart: p.autoStart,
			autoOpenBrowser: p.autoOpenBrowser,
			runtime: active
				? {
						runtimeType: active.runtimeType,
						displayName: active.displayName,
						binaryPath: active.binaryPath,
						versionLabel: active.versionLabel,
						extraArgs: active.extraArgs
					}
				: emptyRuntime(),
			network: {
				bindHost: p.network?.bindHost ?? "0.0.0.0",
				preferredPort: p.network?.preferredPort ?? null
			},
			database: p.database
				? {
						driver: p.database.driver,
						host: p.database.host,
						port: p.database.port,
						databaseName: p.database.databaseName,
						username: p.database.username,
						password: p.database.password,
						sqlitePath: p.database.sqlitePath,
						timeoutSeconds: p.database.timeoutSeconds,
						isRequired: p.database.isRequired
					}
				: emptyDatabase(),
			prelaunch: p.prelaunch
				? {
						appPath: p.prelaunch.appPath,
						appArgs: p.prelaunch.appArgs,
						isEnabled: p.prelaunch.isEnabled,
						waitAfterLaunchMs: p.prelaunch.waitAfterLaunchMs
					}
				: emptyPrelaunch()
		};
	}

	const form = ref<ProjectInput>(buildForm(props.project));

	watch(
		() => props.project?.id,
		() => {
			form.value = buildForm(props.project);
		}
	);

	function validate(): string | null {
		if (!form.value.name.trim()) return "Le nom du projet est requis.";
		if (!form.value.laravelPath.trim()) return "Le chemin du projet Laravel est requis.";
		if (
			form.value.runtime.runtimeType === "custom_php"
			&& !form.value.runtime.binaryPath?.trim()
		) {
			return "Un chemin d'exécutable PHP est requis pour le runtime personnalisé.";
		}
		if (form.value.runtime.runtimeType !== "sqlite" && !form.value.database.databaseName.trim() && form.value.database.isRequired) {
			return "Le nom de la base de données est requis.";
		}
		return null;
	}

	async function save() {
		const error = validate();
		if (error) {
			toast.add({ title: "Formulaire incomplet", description: error, color: "error", icon: "lucide:triangle-alert" });
			return;
		}
		saving.value = true;
		try {
			let saved: ProjectDetail;
			if (props.project) {
				saved = await commands.updateProject(props.project.id, form.value);
				toast.add({ title: "Projet mis à jour", color: "success", icon: "lucide:check-circle-2" });
			} else {
				saved = await commands.createProject(form.value);
				toast.add({ title: "Projet créé", description: saved.name, color: "success", icon: "lucide:check-circle-2" });
			}
			await store.loadProjects(false);
			emit("saved", saved);
		} catch (e) {
			toast.add({ title: "Enregistrement impossible", description: String(e), color: "error", icon: "lucide:triangle-alert" });
		} finally {
			saving.value = false;
		}
	}

	async function remove() {
		if (!props.project) return;
		deleting.value = true;
		try {
			await commands.deleteProject(props.project.id);
			toast.add({ title: "Projet supprimé", color: "success", icon: "lucide:trash-2" });
			confirmDelete.value = false;
			emit("deleted", props.project.id);
			await store.loadProjects();
		} catch (e) {
			toast.add({ title: "Suppression impossible", description: String(e), color: "error", icon: "lucide:triangle-alert" });
		} finally {
			deleting.value = false;
		}
	}
</script>
