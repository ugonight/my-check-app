<script lang="ts">
	import './layout.css';
	import { onMount } from 'svelte';

	const { children } = $props();

	onMount(() => {
		const savedTheme = localStorage.getItem('theme');
		const isDark =
			savedTheme === 'dark' ||
			(savedTheme === null && window.matchMedia('(prefers-color-scheme: dark)').matches);

		document.documentElement.classList.toggle('dark', isDark);

		// Listen for OS theme changes
		const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
		const handleChange = (e: MediaQueryListEvent) => {
			if (!localStorage.getItem('theme')) {
				document.documentElement.classList.toggle('dark', e.matches);
			}
		};

		mediaQuery.addEventListener('change', handleChange);

		return () => {
			mediaQuery.removeEventListener('change', handleChange);
		};
	});
</script>

<div class="text-center w-screen min-h-screen flex flex-col items-center justify-center bg-white dark:bg-neutral-900 text-neutral-900 dark:text-neutral-50 transition-colors">
	{@render children()}
</div>

