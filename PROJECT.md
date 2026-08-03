## Vision produit

### Intitulé

**Laralink** est une application desktop Tauri permettant de gérer plusieurs projets Laravel locaux, chacun avec sa propre version PHP, sa configuration base de données et ses paramètres réseau, afin de les démarrer puis de les rendre accessibles aux appareils connectés au même réseau local. Le stockage local des configurations et états d’exécution est assuré par SQLite avec migrations versionnées. [youtube](https://www.youtube.com/watch?v=kQU3kdfAXpI)

### Finalité

L’application doit simplifier le lancement d’un projet Laravel sur une machine hôte Windows, vérifier les prérequis techniques, détecter l’adresse réseau utile, puis afficher une URL et un QR code utilisables sur téléphone, tablette ou autre poste connecté au même Wi‑Fi. Le support multi-projets et multi-runtimes PHP permet aussi de gérer des applications Laravel différentes avec des versions PHP distinctes dans un seul outil. [dev](https://dev.to/armanrahman/how-to-host-a-laravel-script-on-a-local-lan-network-and-access-it-from-any-connected-device-4jf6)

## Cahier des charges

### Objectifs

L’application doit :
- Gérer plusieurs projets Laravel locaux.
- Permettre d’associer à chaque projet une version PHP différente.
- Lancer un projet Laravel sur le réseau local.
- Vérifier la base de données avant le lancement.
- Afficher une page d’accueil claire avec URL et QR code.
- Protéger la configuration par mot de passe.
- Stocker durablement les paramètres en SQLite. [v2.tauri](https://v2.tauri.app/plugin/sql/)

### Périmètre fonctionnel v1

La v1 couvre :
- Desktop Tauri pour Windows en priorité.
- Deux pages principales : Accueil et Settings.
- Sélecteur de projet actif.
- Multi-projets Laravel.
- Multi-runtimes PHP.
- Runtime PHP standard utilisateur.
- Runtime PHP détecté automatiquement si disponible.
- Runtime `phprs` en mode expérimental, non par défaut, car le support Laravel est encore annoncé comme “planned”. [crates](https://crates.io/crates/phprs)

Ne sont pas inclus en v1 :
- Publication Internet publique.
- HTTPS automatique.
- Exécution simultanée de plusieurs projets en service actif garanti.
- Gestion d’équipe multi-utilisateur.
- Déploiement cloud. [dev](https://dev.to/armanrahman/how-to-host-a-laravel-script-on-a-local-lan-network-and-access-it-from-any-connected-device-4jf6)

### Utilisateurs

- **Administrateur local** : configure les projets, les runtimes PHP, la DB, les ports et les options de lancement. [v2.tauri](https://v2.tauri.app/plugin/sql/)
- **Utilisateur réseau** : consulte seulement l’application Laravel exposée via navigateur ou QR code sur le LAN. [stackoverflow](https://stackoverflow.com/questions/69283840/laravel-how-access-laravel-app-from-local-network/69283981)

### Écrans

#### 1. Accueil

La page d’accueil reste accessible sans mot de passe. Elle doit afficher le projet actif, son état de démarrage, la version PHP active, l’URL réseau et le QR code. [stackoverflow](https://stackoverflow.com/questions/69283840/laravel-how-access-laravel-app-from-local-network/69283981)

Fonctions attendues :
- Sélection du projet actif.
- Affichage du statut global : arrêt, démarrage, prêt, erreur.
- Affichage de l’IP locale.
- Affichage du port actif.
- Affichage de l’URL complète.
- Génération d’un QR code.
- Affichage du runtime utilisé : `system_php`, `custom_php`, ou `phprs_experimental`.
- Affichage des checks : DB OK, PHP OK, Laravel OK.
- Boutons : Démarrer, Arrêter, Redémarrer, Actualiser. [crates](https://crates.io/crates/phprs)

#### 2. Settings

La page Settings est protégée par mot de passe. Elle permet de créer, modifier, supprimer et dupliquer des projets. [v2.tauri](https://v2.tauri.app/plugin/sql/)

Sections attendues :
- Informations générales du projet.
- Chemin du projet Laravel.
- Paramètres de runtime PHP.
- Paramètres réseau.
- Paramètres base de données.
- Application de pré-lancement facultative.
- Sécurité.
- Tests et diagnostics. [youtube](https://www.youtube.com/watch?v=kQU3kdfAXpI)

### Gestion multi-projets

Chaque projet doit être indépendant et posséder :
- son nom,
- son chemin Laravel,
- son runtime PHP actif,
- sa configuration DB,
- son port préféré,
- son éventuelle application de pré-lancement,
- ses derniers états de lancement. [youtube](https://www.youtube.com/watch?v=kQU3kdfAXpI)

Règles :
- un seul projet est actif à la fois en v1,
- un projet peut être défini par défaut au lancement de l’application,
- la duplication d’un projet doit copier tous ses paramètres sauf les états runtime. [youtube](https://www.youtube.com/watch?v=kQU3kdfAXpI)

### Gestion multi-PHP

Chaque projet doit pouvoir utiliser un runtime PHP différent. Le runtime est sélectionné dans la fiche projet. [dev](https://dev.to/armanrahman/how-to-host-a-laravel-script-on-a-local-lan-network-and-access-it-from-any-connected-device-4jf6)

Types de runtime :
- `system_php` : PHP trouvé dans le PATH système ou auto-détecté,
- `custom_php` : chemin fourni explicitement vers un exécutable PHP,
- `phprs_experimental` : moteur expérimental optionnel. [crates](https://crates.io/crates/phprs)

Règles métier :
- si `custom_php` est choisi, l’exécutable doit être vérifié avant enregistrement,
- si `system_php` est choisi, l’application doit tester la commande PHP disponible,
- si `phprs_experimental` est choisi, un avertissement doit indiquer que le support Laravel n’est pas garanti. Le crate `phprs` se présente comme interpréteur/VM PHP en Rust, mais le support Laravel est encore indiqué comme “planned”. [crates](https://crates.io/crates/phprs)

### Démarrage d’un projet

Au lancement d’un projet :
1. Charger la configuration du projet.
2. Vérifier le chemin Laravel.
3. Vérifier la présence du fichier `artisan`.
4. Lancer l’application préalable si configurée.
5. Attendre la disponibilité de la base de données.
6. Vérifier le runtime PHP choisi.
7. Choisir un port libre.
8. Lancer Laravel avec un binding réseau adapté.
9. Détecter l’IPv4 locale.
10. Afficher l’URL et le QR code. [stackoverflow](https://stackoverflow.com/questions/65875127/how-to-run-a-laravel-serve-that-can-be-possible-run-also-on-lan-not-only-in-my-p/65877094)

La commande standard recommandée pour un runtime PHP classique est `php artisan serve --host=0.0.0.0 --port={port}` ou son équivalent avec un chemin PHP personnalisé. L’URL à communiquer aux utilisateurs du réseau doit ensuite être basée sur l’adresse IPv4 locale réelle de la machine. [wd5](https://wd5.ru/laravel/artisan-serve-na-proizvolnom-portu-ili-ip/)

### Vérification base de données

L’application doit pouvoir vérifier que la DB cible est joignable avant le lancement final. Cette logique évite d’annoncer un projet “prêt” alors que l’application Laravel ne pourra pas réellement démarrer correctement. [v2.tauri](https://v2.tauri.app/plugin/sql/)

Champs minimums :
- driver,
- host,
- port,
- database,
- username,
- password,
- timeout. [v2.tauri](https://v2.tauri.app/plugin/sql/)

### Sécurité locale

Les Settings doivent être protégés par mot de passe. Le mot de passe ne doit pas être stocké en clair mais sous forme de hash dans SQLite. [v2.tauri](https://v2.tauri.app/plugin/sql/)

Règles :
- accès libre à l’accueil,
- accès protégé aux settings,
- possibilité future de changer le mot de passe,
- masquage visuel des champs sensibles,
- délai temporaire après plusieurs échecs, recommandé. [v2.tauri](https://v2.tauri.app/plugin/sql/)

## Structure fonctionnelle

Je te propose cette structure logique de l’application :

- **Shell App**
  - Accueil
  - Settings
- **Project Manager**
  - Création projet
  - Sélection projet
  - Duplication projet
  - Suppression projet
- **Runtime Manager**
  - Détection PHP
  - Validation runtime
  - Gestion `phprs_experimental`
- **Launcher Engine**
  - Pré-lanceur
  - Check DB
  - Check PHP
  - Détection port
  - Lancement Laravel
  - Arrêt des processus
- **Network Info**
  - Détection IP
  - Construction URL
  - QR code
- **Persistence**
  - SQLite
  - Migrations
  - Logs runtime [docs](https://docs.rs/tauri-plugin-sql/latest/src/tauri_plugin_sql/lib.rs.html)

## Structure technique

Je te recommande cette arborescence cible :

```text
laralink/
├── src/
│   ├── app/
│   │   ├── pages/
│   │   │   ├── HomePage.vue
│   │   │   └── SettingsPage.vue
│   │   ├── components/
│   │   │   ├── ProjectSelector.vue
│   │   │   ├── RuntimeBadge.vue
│   │   │   ├── StatusPanel.vue
│   │   │   ├── NetworkCard.vue
│   │   │   ├── QrCodeCard.vue
│   │   │   ├── PasswordGate.vue
│   │   │   ├── ProjectForm.vue
│   │   │   ├── RuntimeForm.vue
│   │   │   ├── DatabaseForm.vue
│   │   │   └── PrelaunchForm.vue
│   │   ├── stores/
│   │   │   ├── app.ts
│   │   │   ├── projects.ts
│   │   │   └── runtime.ts
│   │   ├── services/
│   │   │   ├── db.ts
│   │   │   ├── commands.ts
│   │   │   └── validators.ts
│   │   └── types/
│   │       ├── project.ts
│   │       ├── runtime.ts
│   │       └── database.ts
│   └── main.ts
├── src-tauri/
│   ├── src/
│   │   ├── lib.rs
│   │   ├── commands/
│   │   │   ├── projects.rs
│   │   │   ├── runtime.rs
│   │   │   ├── launcher.rs
│   │   │   ├── network.rs
│   │   │   ├── security.rs
│   │   │   └── diagnostics.rs
│   │   ├── services/
│   │   │   ├── process_manager.rs
│   │   │   ├── php_runtime_service.rs
│   │   │   ├── laravel_launcher.rs
│   │   │   ├── db_checker.rs
│   │   │   ├── port_scanner.rs
│   │   │   └── network_detector.rs
│   │   ├── models/
│   │   │   ├── project.rs
│   │   │   ├── runtime.rs
│   │   │   └── log.rs
│   │   └── migrations/
│   │       ├── 0001_init.sql
│   │       ├── 0002_projects.sql
│   │       ├── 0003_runtimes.sql
│   │       ├── 0004_databases.sql
│   │       └── 0005_logs.sql
│   ├── capabilities/
│   └── tauri.conf.json
└── package.json
```

Cette structure colle bien à Tauri 2, à ses permissions/capabilities, et à un stockage SQLite avec migrations versionnées. La doc du plugin SQL précise que les migrations peuvent être définies et ajoutées via `add_migrations`, avec version, description, SQL et type de migration. [docs](https://docs.rs/tauri-plugin-sql/latest/src/tauri_plugin_sql/lib.rs.html)

## Tables SQLite

Je te propose une base unique SQLite, par exemple `laralink.db`, avec les tables suivantes. SQLite et les migrations sont adaptés à ce besoin de configuration locale persistante dans Tauri. [docs](https://docs.rs/tauri-plugin-sql/latest/src/tauri_plugin_sql/lib.rs.html)

### 1. `app_settings`

Stocke les paramètres globaux de l’application.

```sql
CREATE TABLE app_settings (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  settings_password_hash TEXT NOT NULL,
  theme TEXT DEFAULT 'system',
  language TEXT DEFAULT 'fr',
  default_project_id INTEGER NULL,
  auto_launch_default_project INTEGER NOT NULL DEFAULT 0,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);
```

Rôle :
- mot de passe global des settings,
- préférences générales,
- projet par défaut. [v2.tauri](https://v2.tauri.app/plugin/sql/)

### 2. `projects`

Stocke les projets Laravel gérés par l’application.

```sql
CREATE TABLE projects (
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
```

Rôle :
- identité métier du projet,
- chemin Laravel,
- préférences de lancement,
- état courant simplifié. [youtube](https://www.youtube.com/watch?v=kQU3kdfAXpI)

### 3. `project_runtimes`

Stocke les runtimes PHP disponibles par projet.

```sql
CREATE TABLE project_runtimes (
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
```

Rôle :
- rattacher plusieurs runtimes à un projet,
- activer un runtime à la fois,
- permettre plusieurs versions PHP par projet. [crates](https://crates.io/crates/phprs)

### 4. `project_networks`

Stocke les paramètres réseau d’un projet.

```sql
CREATE TABLE project_networks (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  project_id INTEGER NOT NULL,
  bind_host TEXT NOT NULL DEFAULT '0.0.0.0',
  preferred_port INTEGER NULL,
  last_used_port INTEGER NULL,
  last_local_ip TEXT NULL,
  last_public_url TEXT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);
```

Rôle :
- config réseau,
- mémoire du dernier port et de la dernière URL.  
Pour Laravel sur LAN, l’app doit bien lier sur `0.0.0.0`, puis afficher aux clients l’IP locale réelle, pas `0.0.0.0`. [stackoverflow](https://stackoverflow.com/questions/65875127/how-to-run-a-laravel-serve-that-can-be-possible-run-also-on-lan-not-only-in-my-p/65877094)

### 5. `project_databases`

Stocke les paramètres DB d’un projet.

```sql
CREATE TABLE project_databases (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  project_id INTEGER NOT NULL,
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
```

Rôle :
- centraliser les paramètres de test de disponibilité DB,
- gérer MySQL/MariaDB/PostgreSQL/SQLite. [v2.tauri](https://v2.tauri.app/plugin/sql/)

### 6. `project_prelaunch_apps`

Stocke l’application préalable facultative comme Laragon ou WampServer.

```sql
CREATE TABLE project_prelaunch_apps (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  project_id INTEGER NOT NULL,
  app_path TEXT NOT NULL,
  app_args TEXT NULL,
  is_enabled INTEGER NOT NULL DEFAULT 0,
  wait_after_launch_ms INTEGER NOT NULL DEFAULT 5000,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);
```

Rôle :
- lancer un environnement avant le check DB ou PHP,
- attendre une fenêtre raisonnable avant les vérifications. [dev](https://dev.to/armanrahman/how-to-host-a-laravel-script-on-a-local-lan-network-and-access-it-from-any-connected-device-4jf6)

### 7. `runtime_sessions`

Stocke l’état runtime courant et les sessions de lancement.

```sql
CREATE TABLE runtime_sessions (
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
```

Rôle :
- garder l’historique des sessions,
- mémoriser PID, port, URL et erreurs. [youtube](https://www.youtube.com/watch?v=kQU3kdfAXpI)

### 8. `runtime_logs`

Stocke les logs fonctionnels de démarrage.

```sql
CREATE TABLE runtime_logs (
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
```

Rôle :
- journaliser les étapes métier,
- afficher des diagnostics clairs dans l’UI. [youtube](https://www.youtube.com/watch?v=kQU3kdfAXpI)

### 9. `diagnostic_checks`

Stocke les résultats de tests techniques.

```sql
CREATE TABLE diagnostic_checks (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  project_id INTEGER NOT NULL,
  check_type TEXT NOT NULL CHECK (check_type IN ('laravel_path', 'artisan_file', 'php_runtime', 'database', 'port', 'network')),
  is_success INTEGER NOT NULL,
  message TEXT NOT NULL,
  checked_at TEXT NOT NULL,
  FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);
```

Rôle :
- garder une trace exploitable des derniers tests,
- alimenter les badges “DB OK”, “PHP OK”, “Laravel OK”. [dev](https://dev.to/armanrahman/how-to-host-a-laravel-script-on-a-local-lan-network-and-access-it-from-any-connected-device-4jf6)

## Relations conseillées

Voici le schéma logique :

- `app_settings` → référence un `default_project_id`
- `projects` → table centrale
- `projects` 1→N `project_runtimes`
- `projects` 1→1 `project_networks`
- `projects` 1→1 `project_databases`
- `projects` 1→0..1 `project_prelaunch_apps`
- `projects` 1→N `runtime_sessions`
- `runtime_sessions` 1→N `runtime_logs`
- `projects` 1→N `diagnostic_checks` [docs](https://docs.rs/tauri-plugin-sql/latest/src/tauri_plugin_sql/lib.rs.html)

## Exemples de profils

Tu pourras avoir par exemple :

- Projet A : Laravel 10, PHP 8.1 via Laragon, MySQL local, port 8000.
- Projet B : Laravel 11, PHP 8.2 custom, MariaDB, port 8010.
- Projet C : projet test, runtime `phprs_experimental`, SQLite, port auto. [dev](https://dev.to/armanrahman/how-to-host-a-laravel-script-on-a-local-lan-network-and-access-it-from-any-connected-device-4jf6)

