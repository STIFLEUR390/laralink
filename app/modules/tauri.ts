import * as tauriCore from "@tauri-apps/api/core";
import * as tauriEvent from "@tauri-apps/api/event";
import * as tauriDialog from "@tauri-apps/plugin-dialog";
import { addImports, defineNuxtModule } from "nuxt/kit";

const capitalize = (name: string) => {
	return name.charAt(0).toUpperCase() + name.slice(1);
};

const tauriModules = [
	{ module: tauriCore, prefix: "Core", importPath: "@tauri-apps/api/core" },
	{ module: tauriEvent, prefix: "Event", importPath: "@tauri-apps/api/event" },
	{ module: tauriDialog, prefix: "Dialog", importPath: "@tauri-apps/plugin-dialog" }
];

export default defineNuxtModule<ModuleOptions>({
	meta: {
		name: "nuxt-tauri",
		configKey: "tauri"
	},
	defaults: {
		prefix: "useTauri"
	},
	setup(options) {
		tauriModules.forEach(({ module, prefix, importPath }) => {
			Object.keys(module).filter((name) => name !== "default").forEach((name) => {
				const prefixedName = `${options.prefix}${prefix}` || "";
				const as = prefixedName ? prefixedName + capitalize(name) : name;
				addImports({ from: importPath, name, as });
			});
		});
	}
});
