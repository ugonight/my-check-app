<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { constants } from "$lib/stores/constants";

  let allChecks: { type: number; time: string }[] = $state([]);
  let weekData: { date: Date; hasMorning: boolean; hasNight: boolean }[] =
    $state([]);
  let loading = $state(true);

  // 日付変更時刻を基準に日付を計算
  function getAdjustedDate(date: Date): Date {
    const adjusted = new Date(date);
    if (adjusted.getHours() < parseInt($constants.DATE_RESET_HOUR)) {
      adjusted.setDate(adjusted.getDate() - 1);
    }
    return adjusted;
  }

  // 指定日のチェック状況を取得
  function getChecksForDate(date: Date): {
    hasMorning: boolean;
    hasNight: boolean;
  } {
    const dayChecks = allChecks.filter((c) => {
      const checkDate = getAdjustedDate(new Date(c.time));
      return (
        checkDate.getFullYear() === date.getFullYear() &&
        checkDate.getMonth() === date.getMonth() &&
        checkDate.getDate() === date.getDate()
      );
    });

    return {
      hasMorning: dayChecks.some((c) => c.type === 0),
      hasNight: dayChecks.some((c) => c.type === 1),
    };
  }

  // 週間データを生成（月曜日開始）
  function generateWeekData() {
    const today = getAdjustedDate(new Date());

    // 今週の月曜日を取得
    const dayOfWeek = today.getDay();
    const daysToMonday = dayOfWeek === 0 ? 6 : dayOfWeek - 1; // 日曜日は6, 月曜日は0
    const monday = new Date(today);
    monday.setDate(monday.getDate() - daysToMonday);

    const week = [];
    // 月曜日から日曜日まで7日間
    for (let i = 0; i < 7; i++) {
      const date = new Date(monday);
      date.setDate(date.getDate() + i);
      const checks = getChecksForDate(date);
      week.push({
        date,
        hasMorning: checks.hasMorning,
        hasNight: checks.hasNight,
      });
    }

    weekData = week;
  }

  onMount(async () => {
    try {
      const checks =
        await invoke<{ type: number; time: string }[]>("get_recent_checks");
      allChecks = checks;
      generateWeekData();
    } catch (e) {
      console.error(e);
      alert("データ取得失敗: " + e);
    } finally {
      loading = false;
    }
  });

  function formatDate(date: Date): string {
    return date.toLocaleDateString("ja-JP", {
      month: "numeric",
      day: "numeric",
      weekday: "short",
    });
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

  function isToday(date: Date): boolean {
    const today = getAdjustedDate(new Date());
    return (
      date.getFullYear() === today.getFullYear() &&
      date.getMonth() === today.getMonth() &&
      date.getDate() === today.getDate()
    );
  }
</script>

<div class="absolute top-8 left-3 sm:top-8 sm:left-5">
  <a
    href="/"
    class="px-3 sm:px-4 py-2 text-sm sm:text-base rounded-md bg-neutral-200 dark:bg-neutral-700 hover:bg-neutral-300 dark:hover:bg-neutral-600 transition-colors"
  >
    ← 今日
  </a>
</div>
<h1 class="text-2xl sm:text-3xl md:text-4xl font-bold mb-6 sm:mb-8">
  週間チェック状況
</h1>

{#if loading}
  <p class="text-neutral-500 dark:text-neutral-400">読み込み中...</p>
{:else}
  <div class="w-full max-w-2xl grid grid-cols-1 sm:grid-cols-2 gap-3 sm:gap-4">
    {#each weekData as day (day.date.toISOString())}
      <div
        class={`p-4 sm:p-5 rounded-lg border-2 transition-colors ${
          isToday(day.date)
            ? "border-blue-500 bg-blue-50 dark:bg-blue-950/30"
            : "border-neutral-200 dark:border-neutral-700 bg-neutral-50 dark:bg-neutral-800"
        }`}
      >
        <div class="flex items-center justify-between">
          <div class="text-left">
            <p class="font-semibold text-sm sm:text-base">
              {formatDate(day.date)}
            </p>
            {#if isToday(day.date)}
              <p class="text-xs text-blue-600 dark:text-blue-400">今日</p>
            {/if}
          </div>
          <div class="flex gap-2">
            <div
              class={`w-10 h-10 sm:w-12 sm:h-12 flex items-center justify-center rounded-md font-semibold text-sm border-2 transition-colors ${
                day.hasMorning
                  ? "bg-green-100 dark:bg-green-900/40 border-green-500 text-green-700 dark:text-green-300"
                  : "bg-neutral-100 dark:bg-neutral-700 border-neutral-300 dark:border-neutral-600 text-neutral-500 dark:text-neutral-400"
              }`}
            >
              {day.hasMorning ? "✓" : "-"}
            </div>
            <div
              class={`w-10 h-10 sm:w-12 sm:h-12 flex items-center justify-center rounded-md font-semibold text-sm border-2 transition-colors ${
                day.hasNight
                  ? "bg-green-100 dark:bg-green-900/40 border-green-500 text-green-700 dark:text-green-300"
                  : "bg-neutral-100 dark:bg-neutral-700 border-neutral-300 dark:border-neutral-600 text-neutral-500 dark:text-neutral-400"
              }`}
            >
              {day.hasNight ? "✓" : "-"}
            </div>
          </div>
        </div>
        <div
          class="text-xs text-neutral-500 dark:text-neutral-400 mt-2 text-left"
        >
          <p>
            朝: {day.hasMorning ? `${getMorningTimeDisplay()} 完了` : "未実施"}
          </p>
          <p>夜: {day.hasNight ? `${getNightTimeDisplay()} 完了` : "未実施"}</p>
        </div>
      </div>
    {/each}
  </div>

  <!-- 統計情報 -->
  {#if weekData.length > 0}
    <div
      class="mt-8 sm:mt-10 p-4 sm:p-6 rounded-lg bg-neutral-100 dark:bg-neutral-800 w-full max-w-2xl"
    >
      <h2 class="font-semibold mb-4 text-lg">この週の統計</h2>
      <div class="grid grid-cols-2 sm:grid-cols-3 gap-3 sm:gap-4">
        <div class="text-center">
          <p
            class="text-2xl sm:text-3xl font-bold text-green-600 dark:text-green-400"
          >
            {weekData.filter((d) => d.hasMorning).length}
          </p>
          <p class="text-xs sm:text-sm text-neutral-600 dark:text-neutral-400">
            朝チェック
          </p>
        </div>
        <div class="text-center">
          <p
            class="text-2xl sm:text-3xl font-bold text-green-600 dark:text-green-400"
          >
            {weekData.filter((d) => d.hasNight).length}
          </p>
          <p class="text-xs sm:text-sm text-neutral-600 dark:text-neutral-400">
            夜チェック
          </p>
        </div>
        <div class="text-center">
          <p
            class="text-2xl sm:text-3xl font-bold text-blue-600 dark:text-blue-400"
          >
            {weekData.filter((d) => d.hasMorning && d.hasNight).length}
          </p>
          <p class="text-xs sm:text-sm text-neutral-600 dark:text-neutral-400">
            完全達成
          </p>
        </div>
      </div>
    </div>
  {/if}
{/if}
