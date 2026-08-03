<template>
	<div class="rounded-2xl border border-(--brand-border) bg-(--brand-surface) p-5">
		<div class="flex items-start gap-3">
			<div class="grid size-10 shrink-0 place-items-center rounded-xl bg-(--brand-primary-soft)">
				<UIcon name="lucide:terminal-square" class="size-5 text-(--brand-primary)" />
			</div>
			<div class="min-w-0 flex-1">
				<h3 class="font-heading text-sm font-bold">Runtime PHP</h3>
				<p class="mt-0.5 text-xs text-(--brand-muted)">
					Chaque projet peut utiliser sa propre version de PHP.
				</p>
			</div>
		</div>

		<div class="mt-4 space-y-4">
			<UFormField label="Type de runtime" required>
				<USelect
					v-model="runtimeType"
					:items="runtimeOptions"
					value-key="value"
					:disabled="busy"
				/>
			</UFormField>

			<UAlert
				v-if="runtimeType === 'phprs_experimental'"
				title="Runtime expérimental"
				description="Le support Laravel de phprs est annoncé comme « planned » : le démarrage n'est pas garanti."
				color="warning"
				icon="lucide:flask-conical"
				variant="soft"
			/>

			<UFormField v-if="runtimeType !== 'system_php'" label="Chemin de l'exécutable PHP" required :hint="binaryHint">
				<div class="flex gap-2">
					<UInput v-model="binaryPath" placeholder="C:\php\php.exe" class="flex-1" />
					<UButton variant="outline" color="neutral" icon="lucide:folder-search" :label="isSmall ? undefined : 'Parcourir'" @click="browse" />
				</div>
			</UFormField>

			<UFormField label="Version PHP" hint="Version détectée du runtime">
				<div class="flex gap-2">
					<UInput v-model="versionLabel" placeholder="8.3" class="flex-1" />
					<UButton
						variant="outline"
						color="neutral"
						icon="lucide:scan-search"
						:label="isSmall ? undefined : 'Détecter'"
						:loading="busy"
						@click="detectVersion"
					/>
				</div>
			</UFormField>

			<UFormField label="Arguments supplémentaires" optional hint="Ex. -d memory_limit=256M (optionnel)">
				<UInput v-model="extraArgs" placeholder="-d error_reporting=E_ALL" />
			</UFormField>

			<div v-if="detected" class="rounded-lg border px-3 py-2 text-sm" :class="detected.found ? 'border-success/30 bg-success/5 text-success' : 'border-error/30 bg-error/5 text-error'">
				{{ detected.message }}
			</div>
		</div>
	</div>
</template>

<script setup lang="ts">
	import { commands } from "../services/commands";
	import { pickFile } from "../composables/usePicker";
	import type { PhpInfo, RuntimeInput, RuntimeType } from "../types";

	const model = defineModel<RuntimeInput>({ required: true });
	const isSmall = ref(false);

	const busy = ref(false);
	const detected = ref<PhpInfo | null>(null);

	const runtimeOptions = [
		{ value: "system_php", label: "PHP système (PATH)" },
		{ value: "custom_php", label: "PHP personnalisé (chemin explicite)" },
		{ value: "phprs_experimental", label: "phprs — expérimental" }
	];

	const runtimeType = computed<RuntimeType>({
		get: () => model.value.runtimeType,
		set: (v) => {
			model.value.runtimeType = v;
			detected.value = null;
		}
	});

	const binaryPath = computed({
		get: () => model.value.binaryPath ?? "",
		set: (v) => {
			model.value.binaryPath = v;
		}
	});

	const versionLabel = computed({
		get: () => model.value.versionLabel ?? "",
		set: (v) => {
			model.value.versionLabel = v;
		}
	});

	const extraArgs = computed({
		get: () => model.value.extraArgs ?? "",
		set: (v) => {
			model.value.extraArgs = v;
		}
	});

	const binaryHint = computed(() => {
		if (!binaryPath.value) return "Sélectionnez l'exécutable php.exe ou php";
		return null;
	});

	async function browse() {
		const file = await pickFile("Choisir l'exécutable PHP", [
			{ name: "Exécutable PHP", extensions: ["exe", ""] },
			{ name: "Tous les fichiers", extensions: ["*"] }
		]);
		if (file) {
			model.value.binaryPath = file;
			await detectVersion();
		}
	}

	async function detectVersion() {
		if (!runtimeType.value || runtimeType.value === "phprs_experimental") return;
		busy.value = true;
		detected.value = null;
		try {
			const info = runtimeType.value === "custom_php"
				? await commands.validateCustomPhp(binaryPath.value || "")
				: await commands.detectSystemPhp();
			detected.value = info;
			if (info.found && info.version) {
				model.value.versionLabel = info.version;
			}
		} catch (e) {
			detected.value = { found: false, path: "", version: null, message: String(e) };
		} finally {
			busy.value = false;
		}
	}
</script>
