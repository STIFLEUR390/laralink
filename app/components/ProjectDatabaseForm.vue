<template>
	<div class="rounded-2xl border border-(--brand-border) bg-(--brand-surface) p-5">
		<div class="flex items-start gap-3">
			<div class="grid size-10 shrink-0 place-items-center rounded-xl bg-(--brand-primary-soft)">
				<UIcon name="lucide:database" class="size-5 text-(--brand-primary)" />
			</div>
			<div class="min-w-0 flex-1">
				<h3 class="font-heading text-sm font-bold">Base de données</h3>
				<p class="mt-0.5 text-xs text-(--brand-muted)">
					Vérifiée avant chaque lancement pour éviter une erreur au démarrage.
				</p>
			</div>
			<USwitch v-model="isRequired" aria-label="Vérification requise" />
		</div>

		<div class="mt-4 space-y-4">
			<UFormField label="Driver">
				<USelect
					v-model="driver"
					:items="driverOptions"
					value-key="value"
					:disabled="!isRequired"
				/>
			</UFormField>

			<template v-if="isRequired && isServerDriver">
				<div class="grid grid-cols-1 gap-3 sm:grid-cols-2">
					<UFormField label="Hôte">
						<UInput v-model="host" placeholder="127.0.0.1" />
					</UFormField>
					<UFormField label="Port">
						<UInput v-model="port" type="number" :placeholder="String(defaultPort)" />
					</UFormField>
				</div>
				<UFormField label="Nom de la base" required>
					<UInput v-model="databaseName" placeholder="laravel" />
				</UFormField>
				<div class="grid grid-cols-1 gap-3 sm:grid-cols-2">
					<UFormField label="Utilisateur">
						<UInput v-model="username" placeholder="root" />
					</UFormField>
					<UFormField label="Mot de passe">
						<UInput v-model="password" type="password" placeholder="••••••••" />
					</UFormField>
				</div>
			</template>

			<template v-else-if="isRequired && driver === 'sqlite'">
				<UFormField label="Chemin du fichier SQLite" required hint="Relatif au dossier du projet ou chemin absolu">
					<div class="flex gap-2">
						<UInput v-model="sqlitePath" placeholder="database/database.sqlite" class="flex-1" />
						<UButton variant="outline" color="neutral" icon="lucide:folder-open" :label="isSmall ? undefined : 'Parcourir'" @click="browse" />
					</div>
				</UFormField>
			</template>

			<div class="flex items-center gap-2">
				<UFormField label="Délai de connexion (s)" class="w-40">
					<UInput v-model="timeout" type="number" min="1" max="120" />
				</UFormField>
				<UButton
					class="mt-6"
					variant="outline"
					color="neutral"
					icon="lucide:plug-zap"
					:label="isSmall ? undefined : 'Tester la connexion'"
					:loading="testing"
					:disabled="!isRequired"
					@click="test"
				/>
			</div>

			<div v-if="testResult" class="rounded-lg border px-3 py-2 text-sm" :class="testResult.ok ? 'border-success/30 bg-success/5 text-success' : 'border-error/30 bg-error/5 text-error'">
				{{ testResult.message }}
			</div>
		</div>
	</div>
</template>

<script setup lang="ts">
	import { commands } from "../services/commands";
	import { pickFile } from "../composables/usePicker";
	import type { DatabaseInput, DbDriver } from "../types";

	const model = defineModel<DatabaseInput>({ required: true });
	const isSmall = ref(false);

	const testing = ref(false);
	const testResult = ref<{ ok: boolean; message: string } | null>(null);

	const driverOptions = [
		{ value: "mysql", label: "MySQL" },
		{ value: "mariadb", label: "MariaDB" },
		{ value: "pgsql", label: "PostgreSQL" },
		{ value: "sqlite", label: "SQLite" }
	];

	const driver = computed<DbDriver>({
		get: () => model.value.driver,
		set: (v) => {
			model.value.driver = v;
			testResult.value = null;
		}
	});

	const isServerDriver = computed(() => driver.value === "mysql" || driver.value === "mariadb" || driver.value === "pgsql");
	const defaultPort = computed(() => (driver.value === "pgsql" ? 5432 : 3306));

	const isRequired = computed({
		get: () => model.value.isRequired,
		set: (v) => {
			model.value.isRequired = v;
		}
	});

	const host = computed({ get: () => model.value.host ?? "", set: (v) => (model.value.host = v || null) });
	const port = computed({
		get: () => (model.value.port != null ? String(model.value.port) : ""),
		set: (v) => {
			const n = Number.parseInt(v, 10);
			model.value.port = v.trim() === "" || Number.isNaN(n) ? null : n;
		}
	});
	const databaseName = computed({ get: () => model.value.databaseName, set: (v) => (model.value.databaseName = v) });
	const username = computed({ get: () => model.value.username ?? "", set: (v) => (model.value.username = v || null) });
	const password = computed({ get: () => model.value.password ?? "", set: (v) => (model.value.password = v || null) });
	const sqlitePath = computed({ get: () => model.value.sqlitePath ?? "", set: (v) => (model.value.sqlitePath = v || null) });
	const timeout = computed({
		get: () => String(model.value.timeoutSeconds ?? 15),
		set: (v) => {
			const n = Number.parseInt(v, 10);
			model.value.timeoutSeconds = Number.isNaN(n) ? 15 : n;
		}
	});

	async function browse() {
		const file = await pickFile("Choisir la base SQLite", [{ name: "Base SQLite", extensions: ["sqlite", "db", "sqlite3"] }]);
		if (file) {
			model.value.sqlitePath = file;
		}
	}

	async function test() {
		if (testing.value) return;
		testing.value = true;
		testResult.value = null;
		try {
			// Utilise le check réel via diagnostics sur un projet temporaire serait
			// trop lourd : on passe par une commande dédiée au prochain run.
			const payload = {
				driver: model.value.driver,
				host: model.value.host,
				port: model.value.port,
				databaseName: model.value.databaseName,
				username: model.value.username,
				password: model.value.password,
				sqlitePath: model.value.sqlitePath,
				timeoutSeconds: model.value.timeoutSeconds,
				isRequired: true
			};
			// La commande test_database est exposée par le backend.
			const result = await commands.testDatabase(payload);
			testResult.value = result;
		} catch (e) {
			testResult.value = { ok: false, message: String(e) };
		} finally {
			testing.value = false;
		}
	}
</script>
