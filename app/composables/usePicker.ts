import { open } from "@tauri-apps/plugin-dialog";

/** Sélectionne un dossier via la boîte de dialogue native. */
export async function pickFolder(): Promise<string | null> {
	const result = await open({
		directory: true,
		multiple: false,
		title: "Choisir un dossier"
	});
	return typeof result === "string" ? result : null;
}

/** Sélectionne un fichier via la boîte de dialogue native. */
export async function pickFile(title = "Choisir un fichier", filters: { name: string; extensions: string[] }[] = []): Promise<string | null> {
	const result = await open({
		multiple: false,
		title,
		filters: filters.length ? filters : undefined
	});
	return typeof result === "string" ? result : null;
}
