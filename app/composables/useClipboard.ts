/** Copie du texte dans le presse-papiers avec repli. */
export function useClipboard() {
	async function copy(text: string): Promise<boolean> {
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
