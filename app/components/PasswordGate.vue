<template>
	<div class="mx-auto flex max-w-md flex-col items-center py-16">
		<LaralinkLogo :size="72" class="opacity-90" />

		<h1 class="mt-6 font-heading text-2xl font-extrabold tracking-tight">
			Réglages protégés
		</h1>
		<p class="mt-2 text-center text-sm text-(--brand-muted)">
			La configuration des projets est protégée par mot de passe.<br />
			L'accueil reste accessible sans mot de passe.
		</p>

		<form class="mt-8 w-full" @submit.prevent="submit">
			<UFormField label="Mot de passe" required>
				<UInput
					v-model="password"
					type="password"
					size="lg"
					placeholder="Entrez le mot de passe"
					autofocus
					:disabled="store.lockRemaining > 0"
				>
					<template #leading>
						<UIcon name="lucide:lock-keyhole" class="size-4" />
					</template>
					<template #trailing>
						<UButton
							type="submit"
							size="sm"
							color="brand"
							label="Déverrouiller"
							:disabled="!password || store.lockRemaining > 0"
						/>
					</template>
				</UInput>
			</UFormField>
		</form>

		<div v-if="error" class="mt-4 w-full rounded-lg border border-error/30 bg-error/10 px-3 py-2 text-sm text-error">
			{{ error }}
		</div>

		<div v-if="store.lockRemaining > 0" class="mt-4 flex items-center gap-2 text-sm text-warning">
			<UIcon name="lucide:clock" class="size-4" />
			Trop de tentatives. Réessayez dans {{ store.lockRemaining }} s.
		</div>

		<p class="mt-8 text-xs text-(--brand-muted)">
			Mot de passe stocké sous forme de hash Argon2 dans la base locale.
		</p>
	</div>
</template>

<script setup lang="ts">
	import { useLaralinkStore } from "../stores/laralink";

	const store = useLaralinkStore();
	const emit = defineEmits<{ unlocked: [] }>();

	const password = ref("");
	const error = ref<string | null>(null);
	const busy = ref(false);

	async function submit() {
		if (!password.value || store.lockRemaining > 0) return;
		busy.value = true;
		error.value = null;
		try {
			const result = await store.verifyPassword(password.value);
			if (result.ok) {
				password.value = "";
				emit("unlocked");
			} else if (result.locked) {
				error.value = "Trop de tentatives, veuillez patienter.";
			} else {
				error.value = "Mot de passe incorrect.";
				password.value = "";
			}
		} catch (e) {
			error.value = String(e);
		} finally {
			busy.value = false;
		}
	}
</script>
