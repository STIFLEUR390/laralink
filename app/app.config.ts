export default defineAppConfig({
	app: {
		name: "Laralink",
		description: "Lancez vos projets Laravel sur le réseau local",
		version: "0.1.0"
	},
	ui: {
		colors: {
			primary: "brand",
			neutral: "zinc"
		},
		button: {
			slots: {
				base: "cursor-pointer"
			}
		},
		formField: {
			slots: {
				root: "w-full"
			}
		},
		input: {
			slots: {
				root: "w-full"
			}
		},
		textarea: {
			slots: {
				root: "w-full",
				base: "resize-none"
			}
		},
		accordion: {
			slots: {
				trigger: "cursor-pointer",
				item: "md:py-2"
			}
		}
	}
});
