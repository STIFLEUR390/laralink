-- Sessions de lancement (historique)
CREATE TABLE IF NOT EXISTS runtime_sessions (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  project_id INTEGER NOT NULL,
  runtime_id INTEGER NULL,
  pid INTEGER NULL,
  status TEXT NOT NULL,
  started_at TEXT NULL,
  ended_at TEXT NULL,
  local_ip TEXT NULL,
  port INTEGER NULL,
  url TEXT NULL,
  error_message TEXT NULL,
  created_at TEXT NOT NULL,
  FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
  FOREIGN KEY (runtime_id) REFERENCES project_runtimes(id) ON DELETE SET NULL
);

-- Logs fonctionnels de démarrage
CREATE TABLE IF NOT EXISTS runtime_logs (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  session_id INTEGER NULL,
  project_id INTEGER NOT NULL,
  level TEXT NOT NULL CHECK (level IN ('info', 'warning', 'error')),
  step TEXT NOT NULL,
  message TEXT NOT NULL,
  context_json TEXT NULL,
  created_at TEXT NOT NULL,
  FOREIGN KEY (session_id) REFERENCES runtime_sessions(id) ON DELETE SET NULL,
  FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);

-- Résultats des tests techniques
CREATE TABLE IF NOT EXISTS diagnostic_checks (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  project_id INTEGER NOT NULL,
  check_type TEXT NOT NULL CHECK (check_type IN ('laravel_path', 'artisan_file', 'php_runtime', 'database', 'port', 'network')),
  is_success INTEGER NOT NULL,
  message TEXT NOT NULL,
  checked_at TEXT NOT NULL,
  FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);
