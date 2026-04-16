import type { StorybookConfig } from '@storybook/sveltekit';
import path from 'path';

const config: StorybookConfig = {
	stories: ['../src/**/*.mdx', '../src/**/*.stories.@(js|ts|svelte)'],
	addons: [
		'@storybook/addon-svelte-csf',
		'@chromatic-com/storybook',
		'@storybook/addon-vitest',
		'@storybook/addon-a11y',
		'@storybook/addon-docs',
		'@storybook/addon-themes'
	],
	framework: '@storybook/sveltekit',

	// Vite 8 uses rolldown which cannot resolve relative imports from within
	// virtual modules. This plugin strips the virtual-module prefix/query from
	// the importer and resolves relative .svelte imports against the real file
	// path on disk (needed for @storybook/svelte static files).
	viteFinal: async (config) => {
		return {
			...config,
			plugins: [
				...(config.plugins ?? []),
				{
					name: 'storybook-svelte-virtual-resolve',
					enforce: 'pre' as const,
					resolveId(id: string, importer?: string) {
						if (!importer || !id.startsWith('./') || !id.endsWith('.svelte')) return;

						// Strip virtual-module: prefix if present
						let base = importer.startsWith('virtual-module:')
							? importer.slice('virtual-module:'.length)
							: importer;

						// Strip query params (?id=0 etc.)
						const q = base.indexOf('?');
						if (q !== -1) base = base.slice(0, q);

						// Only act on imports coming from inside @storybook/svelte
						if (!base.includes('@storybook' + '/svelte')) return;

						return path.resolve(path.dirname(base), id);
					}
				}
			]
		};
	}
};
export default config;
