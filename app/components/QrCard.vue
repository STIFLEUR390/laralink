<template>
	<div class="flex flex-col rounded-2xl border border-(--brand-border) bg-(--brand-surface) p-6">
		<h3 class="flex items-center gap-2 font-heading text-sm font-bold uppercase tracking-widest text-(--brand-muted)">
			<UIcon name="lucide:qr-code" class="size-4" />
			QR code
		</h3>

		<div class="mt-4 flex flex-1 flex-col items-center justify-center gap-3">
			<div
				v-if="qrDataUrl"
				class="rounded-xl bg-white p-3 shadow-sm ring-1 ring-black/5"
			>
				<img :src="qrDataUrl" alt="QR code d'accès au projet" class="size-44" />
			</div>
			<div v-else class="grid size-44 place-items-center rounded-xl border border-dashed border-(--brand-border) bg-(--brand-bg)/40">
				<div class="text-center">
					<UIcon name="lucide:qr-code" class="mx-auto size-8 text-(--brand-muted)" />
					<p class="mt-2 max-w-40 text-xs text-(--brand-muted)">
						Le QR code apparaît quand le projet est en ligne
					</p>
				</div>
			</div>

			<p class="text-center text-xs text-(--brand-muted)">
				Scannez depuis un téléphone ou une tablette<br />connectés au même Wi‑Fi
			</p>
		</div>
	</div>
</template>

<script setup lang="ts">
	import QRCode from "qrcode";
	import { useLaralinkStore } from "../stores/laralink";

	const store = useLaralinkStore();
	const qrDataUrl = ref<string | null>(null);

	watch(
		() => store.status?.url,
		async (url) => {
			if (!url) {
				qrDataUrl.value = null;
				return;
			}
			try {
				qrDataUrl.value = await QRCode.toDataURL(url, {
					width: 220,
					margin: 1,
					errorCorrectionLevel: "M",
					color: { dark: "#0a3a3e", light: "#ffffff" }
				});
			} catch {
				qrDataUrl.value = null;
			}
		},
		{ immediate: true }
	);
</script>
