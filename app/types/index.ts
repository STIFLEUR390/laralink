// ---------------------------------------------------------------------------
// Types partagés Laralink (contrat frontend ↔ backend Tauri)
// ---------------------------------------------------------------------------

export type RuntimeType = "system_php" | "custom_php" | "phprs_experimental";
export type ProjectStatus = "stopped" | "starting" | "running" | "error";
export type DbDriver = "mysql" | "pgsql" | "sqlite" | "mariadb";

export interface ProjectSummary {
	id: number;
	name: string;
	slug: string;
	laravelPath: string;
	description: string | null;
	isDefault: boolean;
	status: ProjectStatus;
	runtimeType: RuntimeType | "none";
	runtimeLabel: string;
	lastUrl: string | null;
	port: number | null;
	updatedAt: string;
}

export interface RuntimeInfo {
	id: number;
	projectId: number;
	runtimeType: RuntimeType;
	displayName: string;
	binaryPath: string | null;
	versionLabel: string | null;
	isActive: boolean;
	extraArgs: string | null;
}

export interface NetworkInfo {
	id: number;
	projectId: number;
	bindHost: string;
	preferredPort: number | null;
	lastUsedPort: number | null;
	lastLocalIp: string | null;
	lastPublicUrl: string | null;
}

export interface DatabaseInfo {
	id: number;
	projectId: number;
	driver: DbDriver;
	host: string | null;
	port: number | null;
	databaseName: string;
	username: string | null;
	password: string | null;
	sqlitePath: string | null;
	timeoutSeconds: number;
	isRequired: boolean;
}

export interface PrelaunchInfo {
	id: number;
	projectId: number;
	appPath: string;
	appArgs: string | null;
	isEnabled: boolean;
	waitAfterLaunchMs: number;
}

export interface ProjectDetail extends Omit<ProjectSummary, "runtimeType" | "runtimeLabel" | "lastUrl" | "port"> {
	autoStart: boolean;
	autoOpenBrowser: boolean;
	createdAt: string;
	updatedAt: string;
	runtimes: RuntimeInfo[];
	network: NetworkInfo;
	database: DatabaseInfo;
	prelaunch: PrelaunchInfo | null;
}

export interface RuntimeInput {
	runtimeType: RuntimeType;
	displayName?: string | null;
	binaryPath?: string | null;
	versionLabel?: string | null;
	extraArgs?: string | null;
}

export interface NetworkInput {
	bindHost?: string | null;
	preferredPort?: number | null;
}

export interface DatabaseInput {
	driver: DbDriver;
	host?: string | null;
	port?: number | null;
	databaseName: string;
	username?: string | null;
	password?: string | null;
	sqlitePath?: string | null;
	timeoutSeconds?: number | null;
	isRequired: boolean;
}

export interface PrelaunchInput {
	appPath: string;
	appArgs?: string | null;
	isEnabled: boolean;
	waitAfterLaunchMs?: number | null;
}

export interface ProjectInput {
	name: string;
	slug?: string | null;
	laravelPath: string;
	description?: string | null;
	autoStart: boolean;
	autoOpenBrowser: boolean;
	runtime: RuntimeInput;
	network: NetworkInput;
	database: DatabaseInput;
	prelaunch: PrelaunchInput | null;
}

export interface AppSettingsView {
	hasPassword: boolean;
	theme: "system" | "light" | "dark";
	language: string;
	defaultProjectId: number | null;
	autoLaunchDefaultProject: boolean;
}

export interface VerifyResult {
	ok: boolean;
	locked: boolean;
	remainingSeconds: number;
}

export interface PhpInfo {
	found: boolean;
	path: string;
	version: string | null;
	message: string;
}

export interface CheckResult {
	checkType: "laravel_path" | "artisan_file" | "php_runtime" | "database" | "port" | "network";
	isSuccess: boolean;
	message: string;
	checkedAt: string;
}

export interface SessionInfo {
	id: number;
	projectId: number;
	runtimeId: number | null;
	pid: number | null;
	status: string;
	startedAt: string | null;
	endedAt: string | null;
	localIp: string | null;
	port: number | null;
	url: string | null;
	errorMessage: string | null;
}

export interface LogEntry {
	id: number;
	sessionId: number | null;
	projectId: number;
	level: "info" | "warning" | "error";
	step: string;
	message: string;
	createdAt: string;
}

export interface LocalIp {
	addr: string;
	iface: string;
	score: number;
}

export interface NetworkInfoResult {
	localIps: LocalIp[];
	selected: LocalIp | null;
}

export interface StatusInfo {
	projectId: number;
	status: ProjectStatus;
	pid: number | null;
	port: number | null;
	url: string | null;
	ip: string | null;
	runtimeType: RuntimeType | "none";
	runtimeLabel: string;
	phpVersion: string | null;
	checks: CheckResult[];
}

// ---------------------------------------------------------------------------
// Événements émis par le backend
// ---------------------------------------------------------------------------

export interface StatusEvent {
	projectId: number;
	status: ProjectStatus;
	pid: number | null;
	port: number | null;
	url: string | null;
	ip: string | null;
}

export interface LogEvent {
	sessionId: number | null;
	projectId: number;
	level: "info" | "warning" | "error";
	step: string;
	message: string;
	createdAt: string;
}
