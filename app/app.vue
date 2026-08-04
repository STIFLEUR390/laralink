<template>
	<Html class="overflow-x-hidden">
		<Body class="font-sans antialiased">
			<UApp>
				<NuxtLayout>
					<NuxtPage />
				</NuxtLayout>
				<UpdateModal />
			</UApp>
		</Body>
	</Html>
</template>

<script setup lang="ts">
	import { useUpdater } from "./composables/useUpdater";

	const { checkForUpdates } = useUpdater();

	onMounted(() => {
		// Vérification silencieuse au démarrage : la modale s'affiche si une
		// mise à jour est disponible, sans interrompre l'utilisation de l'app.
		const timer = setTimeout(() => checkForUpdates(true), 2500);
		onUnmounted(() => clearTimeout(timer));
	});
</script>
