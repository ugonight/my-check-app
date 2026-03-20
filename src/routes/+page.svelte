<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core"; // ← これだけ追加！

  let morningChecked = $state(false);
  let nightChecked = $state(false);
  let todayChecks: { type: number; time: string }[] = $state([]);

  const today = new Date();

  function isToday(timestamp: string): boolean {
    const d = new Date(timestamp);
    return (
      d.getFullYear() === today.getFullYear() &&
      d.getMonth() === today.getMonth() &&
      d.getDate() === today.getDate()
    );
  }

  onMount(async () => {
    try {
      const checks =
        await invoke<{ type: number; time: string }[]>("get_recent_checks");
      todayChecks = checks.filter((c) => isToday(c.time));
      morningChecked = todayChecks.some((c) => c.type === 0);
      nightChecked = todayChecks.some((c) => c.type === 1);
    } catch (e) {
      console.error(e);
      alert("データ取得失敗: " + e);
    }
  });

  async function checkIn(type: 0 | 1) {
    try {
      await invoke("insert_check", { checkType: type });
      if (type === 0) morningChecked = true;
      else nightChecked = true;
    } catch (e) {
      alert("保存失敗: " + e);
    }
  }
</script>

<div class="p-3 sm:p-5 md:p-8 text-center w-screen min-h-screen flex flex-col items-center justify-center bg-white dark:bg-neutral-900 text-neutral-900 dark:text-neutral-50 transition-colors">
  <h1 class="text-2xl sm:text-3xl md:text-4xl font-bold mb-3 sm:mb-4 md:mb-6">今日のチェック</h1>
  <p class="text-base sm:text-lg md:text-xl mb-8 sm:mb-10 md:mb-12 text-neutral-600 dark:text-neutral-400">{today.toLocaleDateString("ja-JP")}</p>

  <div class="space-y-3 sm:space-y-4 w-full sm:w-auto max-w-xs">
    <button
      onclick={() => checkIn(0)}
      disabled={morningChecked}
      class="w-full sm:w-48 h-12 sm:h-13 md:h-14 inline-flex items-center justify-center rounded-md bg-neutral-950 dark:bg-neutral-50 px-4 sm:px-6 font-medium text-neutral-50 dark:text-neutral-950 shadow-lg shadow-neutral-500/20 dark:shadow-neutral-950/50 enabled:transition enabled:active:scale-95 disabled:cursor-not-allowed disabled:bg-neutral-300 dark:disabled:bg-neutral-700 disabled:text-neutral-500 dark:disabled:text-neutral-400 text-sm sm:text-base"
    >
      {morningChecked ? "朝チェック済み ✓" : "朝のチェック"}
    </button>
    <button
      onclick={() => checkIn(1)}
      disabled={nightChecked}
      class="w-full sm:w-48 h-12 sm:h-13 md:h-14 inline-flex items-center justify-center rounded-md bg-neutral-950 dark:bg-neutral-50 px-4 sm:px-6 font-medium text-neutral-50 dark:text-neutral-950 shadow-lg shadow-neutral-500/20 dark:shadow-neutral-950/50 enabled:transition enabled:active:scale-95 disabled:cursor-not-allowed disabled:bg-neutral-300 dark:disabled:bg-neutral-700 disabled:text-neutral-500 dark:disabled:text-neutral-400 text-sm sm:text-base"
    >
      {nightChecked ? "夜チェック済み ✓" : "夜のチェック"}
    </button>
  </div>
</div>

