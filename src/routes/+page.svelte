<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';   // ← これだけ追加！

  let morningChecked = $state(false);
  let nightChecked = $state(false);
  let todayChecks: { type: number; time: string }[] = $state([]);

  const today = new Date();

  function isToday(timestamp: string): boolean {
    const d = new Date(timestamp);
    return d.getFullYear() === today.getFullYear() &&
           d.getMonth() === today.getMonth() &&
           d.getDate() === today.getDate();
  }

  onMount(async () => {
    try {
      const checks = await invoke<{ type: number; time: string }[]>('get_recent_checks');
      todayChecks = checks.filter(c => isToday(c.time));
      morningChecked = todayChecks.some(c => c.type === 0);
      nightChecked = todayChecks.some(c => c.type === 1);
    } catch (e) {
      console.error(e);
      alert('データ取得失敗: ' + e);
    }
  });

  async function checkIn(type: 0 | 1) {
    try {
      await invoke('insert_check', { checkType: type });
      if (type === 0) morningChecked = true;
      else nightChecked = true;
    } catch (e) {
      alert('保存失敗: ' + e);
    }
  }
</script>

<!-- HTML部分は前回と同じ（ボタンなど） -->
<div style="padding: 20px; text-align: center; font-family: sans-serif;">
  <h1>今日のチェック</h1>
  <p style="font-size: 1.2em;">{today.toLocaleDateString('ja-JP')}</p>

  <button onclick={() => checkIn(0)} disabled={morningChecked} style="...">
    {morningChecked ? '朝チェック済み ✓' : '朝のチェック'}
  </button>
  <button onclick={() => checkIn(1)} disabled={nightChecked} style="...">
    {nightChecked ? '夜チェック済み ✓' : '夜のチェック'}
  </button>
</div>