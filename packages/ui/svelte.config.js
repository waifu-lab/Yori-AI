import adapter from '@sveltejs/adapter-auto';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	compilerOptions: {
		runes: true
	},
	vitePlugin: {
		dynamicCompileOptions({ filename }) {
			if (filename.includes('node_modules')) {
				return { runes: undefined };
			}
		}
	},
	kit: {
		adapter: adapter()
	}
};

export default config;
