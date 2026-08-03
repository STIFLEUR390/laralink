<template>
	<div class="overflow-hidden rounded-2xl border border-(--brand-border) bg-[#0f1210] text-zinc-200 dark:bg-[#0d0f0e]">
		<div class="flex items-center justify-between border-b border-white/5 px-4 py-2.5">
			<div class="flex items-center gap-2">
				<span class="flex gap-1.5">
					<span class="size-2.5 rounded-full bg-error/70" />
					<span class="size-2.5 rounded-full bg-warning/70" />
					<span class="size-2.5 rounded-full bg-success/70" />
				</span>
				<p class="ml-2 font-mono text-xs text-zinc-500">laralink://logs</p>
			</div>
			<div class="flex items-center gap-1">
				<UButton size="xs" variant="ghost" color="neutral" icon="lucide:trash-2" :disabled="!store.logs.length" @click="clearLogs" />
			</div>
		</div>

		<div
			ref="logContainer"
			class="h-64 overflow-y-auto px-4 py-3 font-mono text-[12px] leading-relaxed"
		>
			<div v-if="!store.logs.length" class="py-8 text-center text-zinc-600">
				Aucun log pour le moment — lancez le projet pour voir la sortie du serveur.
			</div>
			<div v-for="log in store.logs" :key="log.id" class="flex gap-2 py-0.5">
				<span class="shrink-0 text-zinc-600">{{ formatTime(log.createdAt) }}</span>
				<span class="w-20 shrink-0 font-semibold" :class="levelClass(log.level)">
					{{ log.level.toUpperCase() }}
				</span>
				<span class="shrink-0 text-brand-300/70">{{ log.step }}</span>
				<span class="min-w-0 break-words text-zinc-300">{{ log.message }}</span>
			</div>
		</div>
	</div>
</template>

<script setup lang="ts">
	import { useLaralinkStore } from "../stores/laralink";

	const store = useLaralinkStore();
	const logContainer = ref<HTMLElement | null>(null);

	function levelClass(level: string) {
		const map: Record<string, string> = {
			info: "text-brand-300",
			warning: "text-warning",
			error: "text-error"
		};
		return map[level] ?? "text-zinc-400";
	}

	function formatTime(iso: string) {
		const d = new Date(iso);
		return d.toLocaleTimeString("fr-FR", { hour12: false });
	}

	function clearLogs() {
		store.logs = [];
	}

	watch(
		() => store.logs.length,
		() => {
			nextTick(() => {
				if (logContainer.value) {
					logContainer.value.scrollTop = logContainer.value.scrollHeight;
				}
			});
		}
	);
</script>
