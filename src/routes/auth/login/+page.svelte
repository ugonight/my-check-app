<script lang="ts">
  import { signInWithGoogle, isLoading, authError } from '$lib/stores/auth';
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';
  import { supabaseClient } from '$lib/supabase';

  let isInitializing = true;

  onMount(async () => {
    // セッション確認：既にログイン済みなら / へリダイレクト
    const { data } = await supabaseClient.auth.getSession();
    if (data.session) {
      goto('/');
    }
    isInitializing = false;
  });

  const handleGoogleSignIn = async () => {
    await signInWithGoogle();
  };
</script>

<div class="flex flex-col items-center justify-center min-h-screen bg-white dark:bg-neutral-900 p-6">
  {#if isInitializing}
    <p class="text-neutral-700 dark:text-neutral-300">ロード中...</p>
  {:else}
    <div class="max-w-sm w-full space-y-6">
      <div class="text-center">
        <h1 class="text-3xl font-bold text-neutral-900 dark:text-white mb-2">My Check App</h1>
        <p class="text-neutral-600 dark:text-neutral-400">チェック履歴をログインして管理</p>
      </div>

      <div class="bg-neutral-50 dark:bg-neutral-800 rounded-lg p-6 space-y-4">
        <button
          on:click={handleGoogleSignIn}
          disabled={$isLoading}
          class="w-full bg-white dark:bg-neutral-700 text-neutral-900 dark:text-white border border-neutral-300 dark:border-neutral-600 py-3 px-4 rounded-lg font-semibold hover:bg-neutral-100 dark:hover:bg-neutral-600 transition disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
        >
          {#if $isLoading}
            <span class="inline-block animate-spin">⏳</span>
            ログイン中...
          {:else}
            <svg class="w-5 h-5" viewBox="0 0 24 24">
              <path fill="currentColor" d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z"/>
              <path fill="currentColor" d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z"/>
              <path fill="currentColor" d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z"/>
              <path fill="currentColor" d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z"/>
            </svg>
            Google でログイン
          {/if}
        </button>
      </div>

      {#if $authError}
        <div class="bg-red-100 dark:bg-red-900 border border-red-400 dark:border-red-700 text-red-700 dark:text-red-100 px-4 py-3 rounded">
          <p class="font-semibold">ログインエラー</p>
          <p class="text-sm">{$authError}</p>
        </div>
      {/if}

      <p class="text-center text-sm text-neutral-600 dark:text-neutral-400">
        Google でログインしてご利用ください
      </p>
    </div>
  {/if}
</div>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
  }
</style>
