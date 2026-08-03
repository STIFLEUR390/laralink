<template>
	<div class="flex flex-col gap-5">
		<div class="rounded-2xl border border-(--brand-border) bg-(--brand-surface) p-5">
			<h3 class="flex items-center gap-2 font-heading text-sm font-bold uppercase tracking-widest text-(--brand-muted)">
				<UIcon name="lucide:palette" class="size-4" />
				Apparence
			</h3>
			<div class="mt-4 grid grid-cols-1 gap-3 sm:grid-cols-2">
				<UFormField label="Thème">
					<USelect
						:model-value="theme"
						:items="themeOptions"
						value-key="value"
						@update:model-value="onThemeChange"
					/>
				</UFormField>
				<UFormField label="Langue">
					<UInput model-value="Français" disabled class="opacity-60" />
				</UFormField>
			</div>
		</div>

		<div class="rounded-2xl border border-(--brand-border) bg-(--brand-surface) p-5">
			<h3 class="flex items-center gap-2 font-heading text-sm font-bold uppercase tracking-widest text-(--brand-muted)">
				<UIcon name="lucide:play-circle" class="size-4" />
				Démarrage
			</h3>
			<div class="mt-4 space-y-4">
				<UFormField label="Projet par défaut" hint="Sélectionné au lancement de l'application">
					<USelect
						:model-value="defaultProjectId"
						:items="projectOptions"
						value-key="value"
						:placeholder="store.projects.length ? 'Aucun projet par défaut' : 'Aucun projet configuré'"
						@update:model-value="onDefaultProjectChange"
					/>
				</UFormField>

				<div class="flex items-center justify-between rounded-xl border border-(--brand-border) px-3 py-2.5">
					<div>
						<p class="text-sm font-medium">Démarrage automatique</p>
						<p class="text-[11px] text-(--brand-muted)">Lancer le projet par défaut à l'ouverture de Laralink</p>
					</div>
					<USwitch :model-value="autoLaunch" @update:model-value="onAutoLaunchChange" />
				</div>
			</div>
		</div>

		<div class="rounded-2xl border border-(--brand-border) bg-(--brand-surface) p-5">
			<h3 class="flex items-center gap-2 font-heading text-sm font-bold uppercase tracking-widest text-(--brand-muted)">
				<UIcon name="lucide:shield-check" class="size-4" />
				Mot de passe des réglages
			</h3>
			<p class="mt-2 text-sm text-(--brand-muted)">
				{{ store.settings?.hasPassword
					? "Les réglages sont actuellement protégés par mot de passe."
					: "Aucun mot de passe défini : les réglages sont librement accessibles." }}
			</p>

			<form class="mt-4 space-y-4" @submit.prevent="changePassword">
				<UFormField v-if="store.settings?.hasPassword" label="Mot de passe actuel" required>
					<UInput v-model="currentPassword" type="password" placeholder="••••••••" autocomplete="current-password" />
				</UFormField>
				<div class="grid grid-cols-1 gap-3 sm:grid-cols-2">
					<UFormField label="Nouveau mot de passe" :required="!!newPassword || !store.settings?.hasPassword">
						<UInput v-model="newPassword" type="password" placeholder="Minimum 4 caractères" autocomplete="new-password" />
					</UFormField>
					<UFormField label="Confirmation">
						<UInput v-model="confirmPassword" type="password" placeholder="Répétez le mot de passe" autocomplete="new-password" />
					</UFormField>
				</div>

				<div class="flex gap-2">
					<UButton
						color="brand"
						type="submit"
						icon="lucide:key-round"
						:label="saving ? 'Enregistrement…' : (store.settings?.hasPassword ? 'Changer le mot de passe' : 'Définir un mot de passe')"
						:loading="saving"
						:disabled="!newPassword || newPassword !== confirmPassword"
					/>
					<UButton
						v-if="store.settings?.hasPassword"
						color="neutral"
						variant="ghost"
						icon="lucide:lock-open"
						label="Supprimer la protection"
						:disabled="!currentPassword"
						@click="removePassword"
					/>
				</div>
				<p v-if="newPassword && newPassword !== confirmPassword" class="text-xs text-error">
					Les mots de passe ne correspondent pas.
				</p>
			</form>
		</div>
	</div>
</template>

<script setup lang="ts">
	import { useLaralinkStore } from "../stores/laralink";
	import { commands } from "../services/commands";

	const store = useLaralinkStore();
	const toast = useToast();
	const colorMode = useColorMode();

	const theme = computed(() => store.settings?.theme ?? "system");
	const defaultProjectId = computed(() => store.settings?.defaultProjectId ?? null);
	const autoLaunch = computed(() => store.settings?.autoLaunchDefaultProject ?? false);

	const themeOptions = [
		{ value: "system", label: "Système" },
		{ value: "light", label: "Clair" },
		{ value: "dark", label: "Sombre" }
	];

	const projectOptions = computed(() => [
		{ value: null, label: "— Aucun —" },
		...store.projects.map((p) => ({ value: p.id, label: p.name }))
	]);

	async function onThemeChange(value: string | null) {
		if (!value) return;
		colorMode.preference = value;
		await store.saveSettings({ theme: value as "light" | "dark" | "system" });
		toast.add({ title: "Thème mis à jour", color: "success", icon: "lucide:check" });
	}

	async function onDefaultProjectChange(value: number | null) {
		await store.saveSettings({ defaultProjectId: value });
	}

	async function onAutoLaunchChange(value: boolean) {
		await store.saveSettings({ autoLaunchDefaultProject: value });
	}

	const currentPassword = ref("");
	const newPassword = ref("");
	const confirmPassword = ref("");
	const saving = ref(false);

	async function changePassword() {
		if (newPassword.value.length < 4) {
			toast.add({ title: "Mot de passe trop court", description: "Minimum 4 caractères.", color: "error", icon: "lucide:triangle-alert" });
			return;
		}
		saving.value = true;
		try {
			await commands.setPassword(currentPassword.value || null, newPassword.value);
			toast.add({ title: "Mot de passe enregistré", color: "success", icon: "lucide:shield-check" });
			currentPassword.value = "";
			newPassword.value = "";
			confirmPassword.value = "";
			await store.loadSettings();
		} catch (e) {
			toast.add({ title: "Mot de passe actuel incorrect", description: String(e), color: "error", icon: "lucide:triangle-alert" });
		} finally {
			saving.value = false;
		}
	}

	async function removePassword() {
		saving.value = true;
		try {
			await commands.setPassword(currentPassword.value || null, "");
			toast.add({ title: "Protection retirée", color: "success", icon: "lucide:lock-open" });
			await store.loadSettings();
		} catch (e) {
			toast.add({ title: "Impossible", description: String(e), color: "error", icon: "lucide:triangle-alert" });
		} finally {
			saving.value = false;
		}
	}
</script>
