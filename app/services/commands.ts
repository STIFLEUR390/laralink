import { invoke } from "@tauri-apps/api/core";import type {
	AppSettingsView,
	CheckResult,
	DatabaseInput,
	LogEntry,
	NetworkInfo,
	NetworkInfoResult,
	PhpInfo,
	ProjectDetail,
	ProjectInput,
	ProjectSummary,
	RuntimeInput,
	SessionInfo,
	StatusInfo,
	VerifyResult
} from "../types";

/**
 * Couche d'accès aux commandes Tauri. Chaque fonction correspond à une
 * commande Rust enregistrée dans `src-tauri/src/lib.rs`.
 */
export const commands = {
	listProjects: () => invoke<ProjectSummary[]>("list_projects"),
	getProject: (id: number) => invoke<ProjectDetail>("get_project", { id }),
	createProject: (input: ProjectInput) => invoke<ProjectDetail>("create_project", { input }),
	updateProject: (id: number, input: ProjectInput) => invoke<ProjectDetail>("update_project", { id, input }),
	deleteProject: (id: number) => invoke<void>("delete_project", { id }),
	duplicateProject: (id: number) => invoke<ProjectDetail>("duplicate_project", { id }),
	setActiveProject: (id: number) => invoke<void>("set_active_project", { id }),

	detectSystemPhp: () => invoke<PhpInfo>("detect_system_php"),
	validateCustomPhp: (path: string) => invoke<PhpInfo>("validate_custom_php", { path }),
	testProjectRuntime: (projectId: number) => invoke<PhpInfo>("test_project_runtime", { projectId }),

	startProject: (projectId: number) => invoke<SessionInfo>("start_project", { projectId }),
	stopProject: (projectId: number) => invoke<void>("stop_project", { projectId }),
	restartProject: (projectId: number) => invoke<SessionInfo>("restart_project", { projectId }),
	getStatus: (projectId: number) => invoke<StatusInfo>("get_status", { projectId }),
	getSessions: (projectId: number, limit?: number) => invoke<SessionInfo[]>("get_sessions", { projectId, limit }),
	getLogs: (projectId: number, sessionId?: number | null, limit?: number) =>
		invoke<LogEntry[]>("get_logs", { projectId, sessionId, limit }),

	getNetworkInfo: () => invoke<NetworkInfoResult>("get_network_info"),
	getProjectNetwork: (projectId: number) => invoke<NetworkInfo>("get_project_network", { projectId }),
	openUrl: (url: string) => invoke<void>("open_url", { url }),

	getAppSettings: () => invoke<AppSettingsView>("get_app_settings"),
	saveSettings: (settings: Partial<Pick<AppSettingsView, "theme" | "language" | "defaultProjectId" | "autoLaunchDefaultProject">>) =>
		invoke<AppSettingsView>("save_settings", settings),
	verifyPassword: (password: string) => invoke<VerifyResult>("verify_password", { password }),
	setPassword: (currentPassword: string | null, newPassword: string) =>
		invoke<void>("set_password", { currentPassword, newPassword }),

	runDiagnostics: (projectId: number) => invoke<CheckResult[]>("run_diagnostics", { projectId }),
	getDiagnostics: (projectId: number, limit?: number) => invoke<CheckResult[]>("get_diagnostics", { projectId, limit }),
	testDatabase: (input: DatabaseInput) => invoke<{ ok: boolean; message: string }>("test_database_connection", { input })
};

/** Événements émis par le backend. */
export const EVENTS = {
	status: "laralink://status",
	log: "laralink://log"
} as const;

export type { RuntimeInput };

/** Vrai environnement Tauri (desktop) ou navigateur (aperçu). */
export function isTauriEnv(): boolean {
	return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}
