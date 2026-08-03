-- Paramètres globaux de l'application (1 seule ligne, id=1)
CREATE TABLE IF NOT EXISTS app_settings (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  settings_password_hash TEXT NULL,
  theme TEXT NOT NULL DEFAULT 'system',
  language TEXT NOT NULL DEFAULT 'fr',
  default_project_id INTEGER NULL,
  auto_launch_default_project INTEGER NOT NULL DEFAULT 0,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

INSERT INTO app_settings (theme, language, created_at, updated_at)
SELECT 'system', 'fr', datetime('now'), datetime('now')
WHERE NOT EXISTS (SELECT 1 FROM app_settings);
