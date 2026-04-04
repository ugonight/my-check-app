<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { constants, loadConstants } from "$lib/stores/constants";
  import { isAuthenticated, getAccessToken } from "$lib/stores/auth";
  import { goto } from "$app/navigation";

  let morningChecked = $state(false);
  let nightChecked = $state(false);
  let todayChecks: { type: number; time: string }[] = $state([]);

  const today = new Date();

  // 日付変更時刻を基準に日付を計算
  function getAdjustedDate(date: Date): Date {
    const adjusted = new Date(date);
    if (adjusted.getHours() < parseInt($constants.DATE_RESET_HOUR)) {
      adjusted.setDate(adjusted.getDate() - 1);
    }
    return adjusted;
  }

  function isToday(timestamp: string): boolean {
    const d = getAdjustedDate(new Date(timestamp));
    const t = getAdjustedDate(today);
    return (
      d.getFullYear() === t.getFullYear() &&
      d.getMonth() === t.getMonth() &&
      d.getDate() === t.getDate()
    );
  }

  function isInMorningWindow(): boolean {
    const hour = today.getHours();
    return (
      hour >= parseInt($constants.MORNING_START) &&
      hour < parseInt($constants.MORNING_END)
    );
  }

  function isInNightWindow(): boolean {
    const hour = today.getHours();
    return (
      hour >= parseInt($constants.NIGHT_START) ||
      hour < parseInt($constants.NIGHT_END)
    );
  }

  function getTimeDisplay(start: number, end: number): string {
    if (start > end) {
      end += 24;
    }
    return `${start}:00～${end}:00`;
  }

  function getMorningTimeDisplay(): string {
    return getTimeDisplay(parseInt($constants.MORNING_START), parseInt($constants.MORNING_END));
  }

  function getNightTimeDisplay(): string {
    return getTimeDisplay(parseInt($constants.NIGHT_START), parseInt($constants.NIGHT_END));
  }

  onMount(async () => {
    // 認証チェック
    if (!$isAuthenticated) {
      goto("/auth/login");
      return;
    }

    try {
      const token = await getAccessToken();
      if (!token) {
        goto("/auth/login");
        return;
      }

      const checks =
        await invoke<{ type: number; time: string }[]>("get_recent_checks", { token });
      todayChecks = checks.filter((c) => isToday(c.time));
      morningChecked = todayChecks.some((c) => c.type === 0);
      nightChecked = todayChecks.some((c) => c.type === 1);
      await loadConstants();
    } catch (e) {
      console.error(e);
      alert("データ取得失敗: " + e);
    }
  });

  async function checkIn(type: 0 | 1) {
    const isValid = type === 0 ? isInMorningWindow() : isInNightWindow();
    if (!isValid) {
      const timeStr =
        type === 0 ? getMorningTimeDisplay() : getNightTimeDisplay();
      alert(`指定の時間帯（${timeStr}）のみ押せます`);
      return;
    }
    try {
      const token = await getAccessToken();
      if (!token) {
        goto("/auth/login");
        return;
      }

      await invoke("insert_check", { token, checkType: type });
      if (type === 0) morningChecked = true;
      else nightChecked = true;
    } catch (e) {
      alert("保存失敗: " + e);
    }
  }
</script>

<div class="absolute top-8 right-3 sm:top-8 sm:right-5">
  <a
    href="/week"
    class="px-3 sm:px-4 py-2 text-sm sm:text-base rounded-md bg-neutral-200 dark:bg-neutral-700 hover:bg-neutral-300 dark:hover:bg-neutral-600 transition-colors"
  >
    週間表示
  </a>
</div>
<h1 class="text-2xl sm:text-3xl md:text-4xl font-bold mb-3 sm:mb-4 md:mb-6">
  今日のチェック
</h1>
<p
  class="text-base sm:text-lg md:text-xl mb-8 sm:mb-10 md:mb-12 text-neutral-600 dark:text-neutral-400"
>
  {today.toLocaleDateString("ja-JP")}
</p>

<div class="space-y-3 sm:space-y-4 w-full sm:w-auto max-w-xs">
  <div>
    <button
      onclick={() => checkIn(0)}
      disabled={morningChecked || !isInMorningWindow()}
      class="w-full sm:w-48 h-12 sm:h-13 md:h-14 inline-flex items-center justify-center rounded-md bg-neutral-950 dark:bg-neutral-50 px-4 sm:px-6 font-medium text-neutral-50 dark:text-neutral-950 shadow-lg shadow-neutral-500/20 dark:shadow-neutral-950/50 enabled:transition enabled:active:scale-95 disabled:cursor-not-allowed disabled:bg-neutral-300 dark:disabled:bg-neutral-700 disabled:text-neutral-500 dark:disabled:text-neutral-400 text-sm sm:text-base"
    >
      {morningChecked ? "朝チェック済み ✓" : "朝のチェック"}
    </button>
    <p class="text-xs text-neutral-500 dark:text-neutral-500 mt-1">
      {getMorningTimeDisplay()}に押せます
    </p>
  </div>
  <div>
    <button
      onclick={() => checkIn(1)}
      disabled={nightChecked || !isInNightWindow()}
      class="w-full sm:w-48 h-12 sm:h-13 md:h-14 inline-flex items-center justify-center rounded-md bg-neutral-950 dark:bg-neutral-50 px-4 sm:px-6 font-medium text-neutral-50 dark:text-neutral-950 shadow-lg shadow-neutral-500/20 dark:shadow-neutral-950/50 enabled:transition enabled:active:scale-95 disabled:cursor-not-allowed disabled:bg-neutral-300 dark:disabled:bg-neutral-700 disabled:text-neutral-500 dark:disabled:text-neutral-400 text-sm sm:text-base"
    >
      {nightChecked ? "夜チェック済み ✓" : "夜のチェック"}
    </button>
    <p class="text-xs text-neutral-500 dark:text-neutral-500 mt-1">
      {getNightTimeDisplay()}に押せます
    </p>
  </div>
</div>
