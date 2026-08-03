import { writeText as tauriWriteText } from "@tauri-apps/plugin-clipboard-manager";
import { isTauriEnv } from "../services/commands";

/** Copie du texte dans le presse-papiers : plugin Tauri natif si disponible, repli navigateur sinon. */
export function useClipboard() {
	async function copy(text: string): Promise<boolean> {
		if (isTauriEnv()) {
			try {
				await tauriWriteText(text);
				return true;
			} catch {
				// repli si le plugin échoue
			}
		}
		try {
			await navigator.clipboard.writeText(text);
			return true;
		} catch {
			try {
				const textarea = document.createElement("textarea");
				textarea.value = text;
				textarea.style.position = "fixed";
				textarea.style.opacity = "0";
				document.body.appendChild(textarea);
				textarea.select();
				const ok = document.execCommand("copy");
				document.body.removeChild(textarea);
				return ok;
			} catch {
				return false;
			}
		}
	}

	return { copy };
}
