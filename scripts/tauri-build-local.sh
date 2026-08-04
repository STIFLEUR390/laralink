#!/usr/bin/env bash
# Build local : charge la clé de signature updater si elle existe.
#
# La clé privée générée par `bunx tauri signer generate` est stockée par défaut
# dans ~/.tauri/laralink.key. Si elle est protégée par un mot de passe,
# définissez-le via TAURI_SIGNING_PRIVATE_KEY_PASSWORD (ou exportez-le dans
# votre shell) avant d'exécuter ce script.
#
#   TAURI_SIGNING_PRIVATE_KEY_PASSWORD="mon-mot-de-passe" bun run tauri:build:local
set -euo pipefail

KEY="${TAURI_SIGNING_PRIVATE_KEY_PATH:-$HOME/.tauri/laralink.key}"
if [[ -f "$KEY" ]]; then
	export TAURI_SIGNING_PRIVATE_KEY_PATH="$KEY"
	echo "→ Clé de signature updater : $KEY"
else
	echo "⚠  Clé de signature introuvable ($KEY) — le build échouera à l'étape updater." >&2
fi

bun run tauri build "$@"
