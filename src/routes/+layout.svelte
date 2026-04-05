<script lang="ts">
  import "./layout.css";
  import { onMount } from "svelte";
  import {
    initializeAuth,
    setupAuthListener,
  } from "$lib/stores/auth";
  import AuthOffcanvas from "$lib/components/AuthOffcanvas.svelte";

  const { children } = $props();

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

    // セッションリスナー設定
    const unsubscribe = setupAuthListener();

    return () => {
      mediaQuery.removeEventListener("change", handleChange);
      unsubscribe();
    };
  });
</script>

<div
  class="text-center w-screen min-h-screen flex flex-col items-center justify-center bg-white dark:bg-neutral-900 text-neutral-900 dark:text-neutral-50 transition-colors relative"
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
