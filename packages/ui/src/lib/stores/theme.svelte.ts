type Theme = 'dark' | 'light';

function createThemeStore() {
	let current = $state<Theme>('dark');

	function apply(theme: Theme) {
		current = theme;
		if (typeof document !== 'undefined') {
			document.documentElement.setAttribute('data-theme', theme);
		}
	}

	function toggle() {
		apply(current === 'dark' ? 'light' : 'dark');
	}

	function init(savedTheme?: Theme) {
		const preferred =
			savedTheme ??
			(typeof window !== 'undefined' && window.matchMedia('(prefers-color-scheme: light)').matches ? 'light' : 'dark');
		apply(preferred);
	}

	return {
		get current() {
			return current;
		},
		get isDark() {
			return current === 'dark';
		},
		apply,
		toggle,
		init
	};
}

export const themeStore = createThemeStore();
