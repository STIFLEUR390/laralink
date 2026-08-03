-- Projets Laravel gérés
CREATE TABLE IF NOT EXISTS projects (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
  slug TEXT NOT NULL UNIQUE,
  laravel_path TEXT NOT NULL,
  description TEXT NULL,
  is_default INTEGER NOT NULL DEFAULT 0,
  auto_start INTEGER NOT NULL DEFAULT 0,
  auto_open_browser INTEGER NOT NULL DEFAULT 0,
  status TEXT NOT NULL DEFAULT 'stopped',
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

-- Runtimes PHP par projet
CREATE TABLE IF NOT EXISTS project_runtimes (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  project_id INTEGER NOT NULL,
  runtime_type TEXT NOT NULL CHECK (runtime_type IN ('system_php', 'custom_php', 'phprs_experimental')),
  display_name TEXT NOT NULL,
  binary_path TEXT NULL,
  version_label TEXT NULL,
  is_active INTEGER NOT NULL DEFAULT 0,
  extra_args TEXT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);

-- Paramètres réseau d'un projet (1:1)
CREATE TABLE IF NOT EXISTS project_networks (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  project_id INTEGER NOT NULL UNIQUE,
  bind_host TEXT NOT NULL DEFAULT '0.0.0.0',
  preferred_port INTEGER NULL,
  last_used_port INTEGER NULL,
  last_local_ip TEXT NULL,
  last_public_url TEXT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);

-- Paramètres base de données d'un projet (1:1)
CREATE TABLE IF NOT EXISTS project_databases (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  project_id INTEGER NOT NULL UNIQUE,
  driver TEXT NOT NULL CHECK (driver IN ('mysql', 'pgsql', 'sqlite', 'mariadb')),
  host TEXT NULL,
  port INTEGER NULL,
  database_name TEXT NOT NULL,
  username TEXT NULL,
  password TEXT NULL,
  sqlite_path TEXT NULL,
  timeout_seconds INTEGER NOT NULL DEFAULT 15,
  is_required INTEGER NOT NULL DEFAULT 1,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);

-- Application de pré-lancement facultative (1:0..1)
CREATE TABLE IF NOT EXISTS project_prelaunch_apps (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  project_id INTEGER NOT NULL UNIQUE,
  app_path TEXT NOT NULL,
  app_args TEXT NULL,
  is_enabled INTEGER NOT NULL DEFAULT 0,
  wait_after_launch_ms INTEGER NOT NULL DEFAULT 5000,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);
