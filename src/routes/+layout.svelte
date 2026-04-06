<script lang="ts">
  import "./layout.css";
  import { onMount } from "svelte";
  import {
    initializeAuth,
    setupAuthListener,
    setupDeepLinkListener,
    isAuthenticated,
    isLoading,
  } from "$lib/stores/auth";
  import { loadConstants } from "$lib/stores/constants";
  import { loadChecks } from "$lib/stores/checks";
  import AuthOffcanvas from "$lib/components/AuthOffcanvas.svelte";
  import { goto } from "$app/navigation";

  const { children } = $props();

  let wasLoading = false;

  onMount(() => {
    // テーマ設定
    const savedTheme = localStorage.getItem("theme");
    const isDark =
      savedTheme === "dark" ||
      (savedTheme === null &&
        window.matchMedia("(prefers-color-scheme: dark)").matches);

    document.documentElement.classList.toggle("dark", isDark);

    // Listen for OS theme changes
    const mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");
    const handleChange = (e: MediaQueryListEvent) => {
      if (!localStorage.getItem("theme")) {
        document.documentElement.classList.toggle("dark", e.matches);
      }
    };

    mediaQuery.addEventListener("change", handleChange);

    // 認証初期化
    initializeAuth();

    // 定数読み込み
    loadConstants();

    // セッションリスナー設定
    const unsubscribe = setupAuthListener();

    // Deep-link イベントリスナー設定
    let unlistenDeepLink: (() => void) | null = null;
    setupDeepLinkListener().then((unlisten) => {
      unlistenDeepLink = unlisten;
    });

    // Deep-link による認証完了時の自動遷移
    let unsubscribeAuth: (() => void) | null = null;
    let unsubscribeCheckAuth: (() => void) | null = null;

    unsubscribeAuth = isLoading.subscribe((loading) => {
      if (wasLoading && !loading) {
        // ローディング終了時に認証状態を確認
        unsubscribeCheckAuth = isAuthenticated.subscribe((authenticated) => {
          if (authenticated) {
            // チェック状況を読み込む
            loadChecks();
            goto("/");
          }
          // 1回実行したらアンサブスクライブ
          unsubscribeCheckAuth?.();
        });
      }
      wasLoading = loading;
    });

    return () => {
      mediaQuery.removeEventListener("change", handleChange);
      unsubscribe();
      unlistenDeepLink?.();
      unsubscribeAuth?.();
      unsubscribeCheckAuth?.();
    };
  });
</script>

<div
  class="text-center w-full min-h-screen flex flex-col items-center justify-center bg-white dark:bg-neutral-900 text-neutral-900 dark:text-neutral-50 transition-colors relative"
>
  <AuthOffcanvas />

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
