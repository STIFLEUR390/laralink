import { defineStore } from "pinia";
import { listen } from "@tauri-apps/api/event";
import { isPermissionGranted, requestPermission } from "@tauri-apps/plugin-notification";
import { commands, EVENTS, isTauriEnv } from "../services/commands";
import type {
	AppSettingsView,
	CheckResult,
	LocalIp,
	LogEntry,
	NetworkInfoResult,
	ProjectStatus,
	ProjectSummary,
	StatusInfo,
	VerifyResult
} from "../types";

export const useLaralinkStore = defineStore("laralink", () => {
	// --- Projets ----------------------------------------------------------
	const projects = ref<ProjectSummary[]>([]);
	const activeProjectId = ref<number | null>(null);
	const loading = ref(false);

	// --- Statut & exécution ----------------------------------------------
	const status = ref<StatusInfo | null>(null);
	const logs = ref<LogEntry[]>([]);
	const checks = ref<CheckResult[]>([]);
	const starting = ref(false);

	// --- Réseau -----------------------------------------------------------
	const network = ref<NetworkInfoResult>({ localIps: [], selected: null });

	// --- Réglages & sécurité ---------------------------------------------
	const settings = ref<AppSettingsView | null>(null);
	const unlocked = ref(false);
	const lockRemaining = ref(0);
	const lockTimer = ref<ReturnType<typeof setInterval> | null>(null);

	const activeProject = computed(() =>
		projects.value.find((p) => p.id === activeProjectId.value) ?? null
	);
	const isBusy = computed(() => status.value?.status === "starting" || starting.value);
	const hasProjects = computed(() => projects.value.length > 0);

	// --- Actions ----------------------------------------------------------

	async function loadProjects(selectDefault = true) {
		loading.value = true;
		try {
			projects.value = await commands.listProjects();
			if (!activeProjectId.value || !projects.value.some((p) => p.id === activeProjectId.value)) {
				const def = projects.value.find((p) => p.isDefault) ?? projects.value[0] ?? null;
				activeProjectId.value = def ? def.id : null;
			} else if (selectDefault) {
				// on conserve la sélection utilisateur
			}
			await loadNetwork();
		} finally {
			loading.value = false;
		}
	}

	async function selectProject(id: number) {
		activeProjectId.value = id;
		await refreshStatus();
	}

	async function loadStatus() {
		if (!activeProjectId.value) {
			status.value = null;
			return;
		}
		status.value = await commands.getStatus(activeProjectId.value);
		if (status.value) {
			checks.value = status.value.checks;
		}
	}

	async function loadLogs() {
		if (!activeProjectId.value) return;
		logs.value = await commands.getLogs(activeProjectId.value, null, 400);
	}

	async function refreshStatus() {
		await Promise.all([loadStatus(), loadLogs()]);
	}

	async function loadNetwork() {
		network.value = await commands.getNetworkInfo();
	}

	async function start() {
		if (!activeProjectId.value || isBusy.value) return;
		starting.value = true;
		try {
			await commands.startProject(activeProjectId.value);
		} finally {
			starting.value = false;
		}
	}

	async function stop() {
		if (!activeProjectId.value) return;
		await commands.stopProject(activeProjectId.value);
		await refreshStatus();
	}

	async function restart() {
		if (!activeProjectId.value) return;
		starting.value = true;
		try {
			await commands.restartProject(activeProjectId.value);
		} finally {
			starting.value = false;
		}
	}

	async function refresh() {
		await Promise.all([loadProjects(false), refreshStatus()]);
	}

	async function setActiveProject(id: number) {
		await commands.setActiveProject(id);
		await loadProjects(false);
	}

	// --- Réglages ---------------------------------------------------------

	async function loadSettings() {
		settings.value = await commands.getAppSettings();
	}

	async function verifyPassword(password: string): Promise<VerifyResult> {
		const result = await commands.verifyPassword(password);
		if (result.locked && result.remainingSeconds > 0) {
			lockRemaining.value = result.remainingSeconds;
			startLockCountdown();
		}
		if (result.ok) {
			unlocked.value = true;
		}
		return result;
	}

	function startLockCountdown() {
		if (lockTimer.value) clearInterval(lockTimer.value);
		lockTimer.value = setInterval(() => {
			lockRemaining.value = Math.max(0, lockRemaining.value - 1);
			if (lockRemaining.value === 0 && lockTimer.value) {
				clearInterval(lockTimer.value);
				lockTimer.value = null;
			}
		}, 1000);
	}

	async function saveSettings(patch: Partial<Pick<AppSettingsView, "theme" | "defaultProjectId" | "autoLaunchDefaultProject">>) {
		settings.value = await commands.saveSettings(patch);
	}

	// --- Événements backend ----------------------------------------------

	let unlistenStatus: (() => void) | null = null;
	let unlistenLog: (() => void) | null = null;

	async function listenToBackend() {
		if (!unlistenStatus) {
			unlistenStatus = await listen<{ projectId: number; status: ProjectStatus }>(EVENTS.status, async (event) => {
				const { projectId, status: newStatus } = event.payload;
				const project = projects.value.find((p) => p.id === projectId);
				if (project) {
					project.status = newStatus;
				}
				if (activeProjectId.value === projectId) {
					await loadStatus();
				}
			});
		}
		if (!unlistenLog) {
			unlistenLog = await listen<LogEntry>(EVENTS.log, (event) => {
				if (activeProjectId.value === event.payload.projectId) {
					logs.value.push(event.payload);
					if (logs.value.length > 500) {
						logs.value = logs.value.slice(-400);
					}
				}
			});
		}
	}

	// --- Init -------------------------------------------------------------

	async function ensureNotificationPermission() {
		if (!isTauriEnv()) return;
		try {
			if (!(await isPermissionGranted())) {
				await requestPermission();
			}
		} catch {
			// silencieux : la notification reste optionnelle
		}
	}

	async function init() {
		if (isTauriEnv()) {
			await Promise.all([loadProjects(), loadSettings(), listenToBackend(), ensureNotificationPermission()]);
			await refreshStatus();
		} else {
			// Mode aperçu navigateur (sans backend Tauri)
			projects.value = [
				{
					id: 1,
					name: "Mon projet Laravel",
					slug: "mon-projet-laravel",
					laravelPath: "/home/dev/mon-projet",
					description: null,
					isDefault: true,
					status: "running",
					runtimeType: "system_php",
					runtimeLabel: "PHP système",
					lastUrl: "http://192.168.1.42:8000",
					port: 8000,
					updatedAt: new Date().toISOString()
				},
				{
					id: 2,
					name: "Blog Laravel 11",
					slug: "blog-laravel-11",
					laravelPath: "C:\\laragon\\www\\blog",
					description: null,
					isDefault: false,
					status: "stopped",
					runtimeType: "custom_php",
					runtimeLabel: "PHP 8.3 personnalisé",
					lastUrl: null,
					port: null,
					updatedAt: new Date().toISOString()
				}
			];
			activeProjectId.value = 1;
			status.value = {
				projectId: 1,
				status: "running",
				pid: 12456,
				port: 8000,
				url: "http://192.168.1.42:8000",
				ip: "192.168.1.42",
				runtimeType: "system_php",
				runtimeLabel: "PHP système",
				phpVersion: "8.3.14",
				checks: [
					{ checkType: "database", isSuccess: true, message: "Base de données OK.", checkedAt: new Date().toISOString() },
					{ checkType: "php_runtime", isSuccess: true, message: "PHP 8.3.14 (php)", checkedAt: new Date().toISOString() },
					{ checkType: "artisan_file", isSuccess: true, message: "Fichier artisan présent.", checkedAt: new Date().toISOString() }
				]
			};
			network.value = {
				localIps: [
					{ addr: "192.168.1.42", iface: "wlp2s0", score: 30 },
					{ addr: "10.0.0.5", iface: "enp3s0", score: 25 }
				],
				selected: { addr: "192.168.1.42", iface: "wlp2s0", score: 30 }
			};
			settings.value = { hasPassword: false, theme: "system", language: "fr", defaultProjectId: 1, autoLaunchDefaultProject: false };
			unlocked.value = true;
			logs.value = [
				{ id: 1, sessionId: 7, projectId: 1, level: "info", step: "start", message: "Démarrage du projet…", createdAt: new Date(Date.now() - 9000).toISOString() },
				{ id: 2, sessionId: 7, projectId: 1, level: "info", step: "database", message: "Base de données OK.", createdAt: new Date(Date.now() - 8000).toISOString() },
				{ id: 3, sessionId: 7, projectId: 1, level: "info", step: "runtime", message: "Runtime : PHP système (8.3.14)", createdAt: new Date(Date.now() - 7000).toISOString() },
				{ id: 4, sessionId: 7, projectId: 1, level: "info", step: "port", message: "Port choisi : 8000", createdAt: new Date(Date.now() - 6000).toISOString() },
				{ id: 5, sessionId: 7, projectId: 1, level: "info", step: "launch", message: "Serveur PHP démarré (pid 12456). Attente de la disponibilité…", createdAt: new Date(Date.now() - 5000).toISOString() },
				{ id: 6, sessionId: 7, projectId: 1, level: "info", step: "network", message: "URL réseau : http://192.168.1.42:8000", createdAt: new Date(Date.now() - 4000).toISOString() },
				{ id: 7, sessionId: 7, projectId: 1, level: "info", step: "process", message: "INFO  Server running on [http://0.0.0.0:8000].", createdAt: new Date(Date.now() - 2000).toISOString() },
				{ id: 8, sessionId: 7, projectId: 1, level: "info", step: "process", message: "INFO  Watching for file changes.", createdAt: new Date(Date.now() - 1000).toISOString() }
			];
		}
	}

	return {
		projects,
		activeProjectId,
		activeProject,
		loading,
		status,
		logs,
		checks,
		starting,
		isBusy,
		hasProjects,
		network,
		settings,
		unlocked,
		lockRemaining,
		init,
		loadProjects,
		selectProject,
		refreshStatus,
		refresh,
		loadLogs,
		loadNetwork,
		start,
		stop,
		restart,
		setActiveProject,
		loadSettings,
		verifyPassword,
		saveSettings
	};
});
