<script lang="ts">
	import './layout.css';
	import { onMount } from 'svelte';
	import { initializeAuth, setupAuthListener, authUser, logout, isAuthenticated } from '$lib/stores/auth';
	import { goto } from '$app/navigation';

	const { children } = $props();

	onMount(() => {
		// テーマ設定
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

		// 認証初期化
		initializeAuth();

		// セッションリスナー設定
		const unsubscribe = setupAuthListener();

		return () => {
			mediaQuery.removeEventListener('change', handleChange);
			unsubscribe();
		};
	});

	const handleLogout = async () => {
		await logout();
		goto('/auth/login');
	};
</script>

<div class="text-center w-screen min-h-screen flex flex-col items-center justify-center bg-white dark:bg-neutral-900 text-neutral-900 dark:text-neutral-50 transition-colors">
	<!-- Header with user info -->
	<header class="w-full border-b border-neutral-200 dark:border-neutral-800 bg-white dark:bg-neutral-950 sticky top-0 z-50">
		<div class="max-w-4xl mx-auto px-4 py-4 flex justify-between items-center">
			<h1 class="text-xl font-bold">My Check App</h1>

			{#if $isAuthenticated && $authUser}
				<div class="flex items-center gap-4">
					<span class="text-sm text-neutral-600 dark:text-neutral-400">
						{$authUser.email}
					</span>
					<button
						on:click={handleLogout}
						class="px-3 py-1 bg-neutral-200 dark:bg-neutral-700 rounded hover:bg-neutral-300 dark:hover:bg-neutral-600 transition text-sm"
					>
						ログアウト
					</button>
				</div>
			{:else}
				<a
					href="/auth/login"
					class="px-3 py-1 bg-blue-500 text-white rounded hover:bg-blue-600 transition text-sm"
				>
					ログイン
				</a>
			{/if}
		</div>
	</header>

	<!-- Main content -->
	<main class="flex-1 w-full">
		{@render children()}
	</main>
</div>

<style>
	:global(body) {
		margin: 0;
		padding: 0;
	}
</style>

