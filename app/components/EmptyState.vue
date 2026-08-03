<template>
	<div class="rounded-2xl border border-dashed border-(--brand-border) bg-(--brand-surface) px-8 py-14 text-center">
		<LaralinkLogo :size="64" class="mx-auto opacity-80" />
		<h2 class="mt-6 font-heading text-xl font-extrabold tracking-tight">
			Aucun projet configuré
		</h2>
		<p class="mx-auto mt-2 max-w-md text-sm text-(--brand-muted)">
			Ajoutez votre premier projet Laravel pour le lancer sur votre réseau local et le rendre accessible par URL et QR code.
		</p>
		<div class="mt-6 flex justify-center gap-2">
			<UButton
				color="brand"
				icon="lucide:plus"
				label="Configurer un projet"
				to="/settings?new=1"
			/>
		</div>
	</div>
</template>
