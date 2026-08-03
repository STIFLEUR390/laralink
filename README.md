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
| `bun run tauri:build` | Build de production |
| `cargo test` (dans `src-tauri/`) | Tests unitaires (services + migrations) |

## Licence

MIT © 2024–2026 [STIFLEUR390](https://github.com/STIFLEUR390)
