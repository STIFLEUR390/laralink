<p align="center">
  <img width="150" src="./public/logo.png" alt="Laralink Logo">
</p>

<h1 align="center">LARALINK</h1>

<p align="center">
  Gérez et lancez vos projets Laravel locaux sur votre réseau local —<br />
  URL d'accès et QR code pour téléphone, tablette et autre poste du même Wi‑Fi.
</p>

<p align="center">
  <img src="https://img.shields.io/github/package-json/v/STIFLEUR390/laralink" />
  <img src="https://img.shields.io/github/license/STIFLEUR390/laralink" />
</p>

---

**Laralink** est une application desktop [Tauri 2](https://v2.tauri.app) construite avec [Nuxt 4](https://nuxt.com) et [Nuxt UI](https://ui4.nuxt.dev). Elle simplifie le lancement d'un projet Laravel sur une machine hôte (Windows en priorité), vérifie les prérequis techniques, détecte l'adresse réseau utile, puis affiche une URL et un QR code utilisables sur n'importe quel appareil du même réseau.

## Fonctionnalités

- 🗂️ **Multi-projets Laravel** — chaque projet a sa propre config (chemin, runtime PHP, base de données, port, pré-lancement).
- 🐘 **Multi-runtimes PHP** — `system_php` (PATH), `custom_php` (chemin explicite) ou `phprs_experimental` (avertissement, support Laravel « planned »).
- 🚀 **Launcher intégré** — vérification DB → validation PHP → port libre → `php artisan serve --host=0.0.0.0 --port={port}`, avec logs temps réel.
- 🌐 **Réseau local** — détection IPv4, URL construite automatiquement, QR code scannable.
- 🩺 **Diagnostics** — chemin Laravel, fichier `artisan`, PHP, base de données, port, réseau.
- 🔒 **Réglages protégés** — mot de passe haché (Argon2), verrouillage temporaire après échecs répétés.
- 💾 **Stockage SQLite** — migrations versionnées, historique des sessions et logs.
- ⌨️ **CLI** — `laralink start <slug>` / `laralink stop <slug>` depuis le terminal.

## Démarrage rapide

```sh
# Prérequis : bun, Rust, et les dépendances système Tauri (voir v2.tauri.app)
bun install
bun run tauri:dev
```

## Prise en charge Windows

Laralink cible **Windows en priorité** (ainsi que Linux). Le code Rust est cross-platform :

- arrêt des processus via `taskkill /PID /T /F` et détection via `tasklist`,
- fenêtres console masquées (`CREATE_NO_WINDOW`) lors du lancement de PHP,
- chemins et exécutables gérés avec `std::path`.

### Build Windows (installateurs NSIS + MSI)

Le build natif Windows est réalisé par le pipeline **GitHub Actions** (`.github/workflows/release.yml`) sur un runner `windows-latest` — il produit l'installateur `Laralink_<version>_x64-setup.exe`, le MSI, leurs signatures, et régénère `latest.json` pour l'updater.

```sh
# 1. Taguer une version (ex. v0.1.1) et pousser :
git tag v0.1.1 && git push origin v0.1.1

# 2. Le workflow build les assets Windows + Linux et crée une release brouillon.
#    Publiez-la depuis l'onglet Releases quand les checks sont verts.
```

> 💡 Les secrets GitHub `TAURI_SIGNING_PRIVATE_KEY` / `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`
> doivent être configurés pour signer les installateurs (clé générée via `bunx tauri signer generate`).

### Build local sous Windows

```sh
bun install
bun run tauri:dev      # développement
bun run tauri:build    # installateur NSIS/MSI
```

Prérequis Windows : [WebView2](https://developer.microsoft.com/microsoft-edge/webview2/) (installé par l'installateur), PHP dans le PATH (ou runtime personnalisé), et les outils [Visual Studio Build Tools](https://learn.microsoft.com/cpp/build/vscpp-step-0-installation) pour Rust.

## Structure

```text
app/            # Frontend Nuxt 4 (pages, composants, store Pinia)
src-tauri/      # Backend Rust (commandes, services, migrations SQLite)
├── migrations/ # Migrations SQL versionnées
└── src/
    ├── commands/   # Commandes Tauri (projets, runtime, launcher, réseau, sécurité, diagnostics)
    └── services/   # Process manager, PHP runtime, launcher, db checker, port scanner, réseau
```

## Scripts

| Script | Description |
|---|---|
| `bun run dev` | Serveur Nuxt de développement |
| `bun run tauri:dev` | Application Tauri en mode développement |
| `bun run tauri:build` | Build de production (Linux : AppImage/deb/rpm — Windows : NSIS/MSI) |
| `cargo test` (dans `src-tauri/`) | Tests unitaires (services + migrations) |
| GitHub Actions | `ci.yml` (tests) + `release.yml` (build multi-plateforme + release) |

## Licence

MIT © 2024–2026 [STIFLEUR390](https://github.com/STIFLEUR390)
