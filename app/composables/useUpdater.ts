import { relaunch } from "@tauri-apps/plugin-process";
import { check } from "@tauri-apps/plugin-updater";
import { isTauriEnv } from "../services/commands";

interface UpdateInfo {
	currentVersion: string
	version: string
	body?: string
}

/** État singleton partagé entre la vérification au démarrage et le bouton « À propos ». */
const checking = ref(false);
const downloading = ref(false);
const progress = ref(0);
const error = ref<string | null>(null);
const updateInfo = ref<UpdateInfo | null>(null);

/**
 * Vérifie les mises à jour (manifeste GitHub + vérification de signature).
 * - Si une mise à jour est disponible : `updateInfo` est rempli (la modale s'affiche).
 * - Si rien : un toast est affiché sauf si `silent` (vérification automatique au démarrage).
 */
export function useUpdater() {
	const toast = useToast();

	async function checkForUpdates(silent = false) {
		if (!isTauriEnv() || checking.value || downloading.value) return;
		checking.value = true;
		error.value = null;
		try {
			const update = await check();
			if (update) {
				updateInfo.value = {
					currentVersion: update.currentVersion,
					version: update.version,
					body: update.body
				};
				return;
			}
			if (!silent) {
				toast.add({ title: "Laralink est à jour", description: "Aucune mise à jour disponible.", color: "success", icon: "lucide:check-check" });
			}
		} catch (e) {
			error.value = String(e);
			if (!silent) {
				toast.add({ title: "Vérification impossible", description: String(e), color: "error", icon: "lucide:triangle-alert" });
			}
		} finally {
			checking.value = false;
		}
	}

	function dismiss() {
		updateInfo.value = null;
		error.value = null;
		progress.value = 0;
	}

	async function installAndRelaunch() {
		const info = updateInfo.value;
		if (!info || downloading.value) return;
		downloading.value = true;
		progress.value = 0;
		try {
			const update = await check();
			if (!update) {
				dismiss();
				return;
			}
			let downloaded = 0;
			let total = 0;
			await update.downloadAndInstall((event) => {
				if (event.event === "Started") {
					total = event.data.contentLength ?? 0;
				} else if (event.event === "Progress") {
					downloaded += event.data.chunkLength;
					if (total > 0) {
						progress.value = Math.min(100, Math.round((downloaded / total) * 100));
					}
				} else if (event.event === "Finished") {
					progress.value = 100;
				}
			});
			// Redémarre l'application sur la nouvelle version.
			await relaunch();
		} catch (e) {
			error.value = String(e);
			toast.add({ title: "Mise à jour impossible", description: String(e), color: "error", icon: "lucide:triangle-alert" });
			downloading.value = false;
		}
	}

	return {
		checking,
		downloading,
		progress,
		error,
		updateInfo,
		checkForUpdates,
		installAndRelaunch,
		dismiss
	};
}
