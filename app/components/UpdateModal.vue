<template>
	<UModal
		:open="!!updateInfo"
		:close="!downloading"
		@update:open="(open: boolean) => { if (!open) dismiss(); }"
	>
		<template #header>
			<div class="flex items-center gap-3">
				<span class="grid size-10 shrink-0 place-items-center rounded-xl bg-(--brand-primary-soft)">
					<UIcon :name="downloading ? 'lucide:loader-circle' : 'lucide:download-cloud'" class="size-5 text-(--brand-primary)" :class="downloading ? 'animate-spin' : ''" />
				</span>
				<div>
					<h3 class="font-heading text-base font-extrabold tracking-tight">
						Mise à jour disponible
					</h3>
					<p class="text-xs text-(--brand-muted)">
						v{{ info?.currentVersion }} → v{{ info?.version }}
					</p>
				</div>
			</div>
		</template>

		<template #body>
			<div class="flex flex-col gap-4">
				<p class="text-sm leading-relaxed text-(--brand-muted)">
					Une nouvelle version de Laralink est disponible. Télécharger et installer maintenant ?
				</p>

				<div v-if="info?.body" class="max-h-40 overflow-y-auto rounded-xl border border-(--brand-border) bg-(--brand-bg)/50 p-3">
					<p class="text-[10px] font-bold uppercase tracking-widest text-(--brand-muted)">
						Notes de version
					</p>
					<p class="mt-1 whitespace-pre-wrap font-mono text-xs text-(--brand-text)">
						{{ info.body }}
					</p>
				</div>

				<UProgress
					v-if="downloading"
					:value="progress"
					:label="`Téléchargement… ${progress}%`"
					label-position="top"
					size="sm"
				/>

				<p v-if="error && !downloading" class="flex items-center gap-2 text-xs text-error">
					<UIcon name="lucide:triangle-alert" class="size-4" />
					{{ error }}
				</p>
			</div>
		</template>

		<template #footer>
			<div class="flex justify-end gap-2">
				<UButton
					color="neutral"
					variant="ghost"
					label="Plus tard"
					:disabled="downloading"
					@click="dismiss()"
				/>
				<UButton
					color="brand"
					icon="lucide:download"
					label="Mettre à jour"
					:loading="downloading"
					:disabled="downloading"
					@click="installAndRelaunch()"
				/>
			</div>
		</template>
	</UModal>
</template>

<script setup lang="ts">
	import { useUpdater } from "../composables/useUpdater";

	const { updateInfo, downloading, progress, error, installAndRelaunch, dismiss } = useUpdater();
	const info = computed(() => updateInfo.value);
</script>
