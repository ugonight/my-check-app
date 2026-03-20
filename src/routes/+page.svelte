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

<div class="p-5 text-center">
  <h1 class="text-2xl font-bold mb-4">今日のチェック</h1>
  <p class="text-lg mb-8">{today.toLocaleDateString("ja-JP")}</p>

  <div class="space-y-3">
    <button
      onclick={() => checkIn(0)}
      disabled={morningChecked}
      class="inline-flex h-12 items-center justify-center rounded-md bg-neutral-950 px-6 font-medium text-neutral-50 shadow-lg shadow-neutral-500/20 enabled:transition enabled:active:scale-95 disabled:cursor-not-allowed disabled:bg-neutral-300 disabled:text-neutral-500"
    >
      {morningChecked ? "朝チェック済み ✓" : "朝のチェック"}
    </button>
    <button
      onclick={() => checkIn(1)}
      disabled={nightChecked}
      class="inline-flex h-12 items-center justify-center rounded-md bg-neutral-950 px-6 font-medium text-neutral-50 shadow-lg shadow-neutral-500/20 enabled:transition enabled:active:scale-95 disabled:cursor-not-allowed disabled:bg-neutral-300 disabled:text-neutral-500"
    >
      {nightChecked ? "夜チェック済み ✓" : "夜のチェック"}
    </button>
  </div>
</div>
