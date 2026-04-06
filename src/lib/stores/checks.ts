import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import { getAccessToken } from './auth';

export const todayChecks = writable<{ type: number; time: string }[]>([]);
export const morningChecked = writable(false);
export const nightChecked = writable(false);

// 日付変更時刻を基準に日付を計算
function getAdjustedDate(date: Date): Date {
  const adjusted = new Date(date);
  // constants が未読み込み時はデフォルト値を使用
  const resetHour = localStorage.getItem('DATE_RESET_HOUR') || '4';
  if (adjusted.getHours() < parseInt(resetHour)) {
    adjusted.setDate(adjusted.getDate() - 1);
  }
  return adjusted;
}

function isToday(timestamp: string): boolean {
  const d = getAdjustedDate(new Date(timestamp));
  const t = getAdjustedDate(new Date());
  return (
    d.getFullYear() === t.getFullYear() &&
    d.getMonth() === t.getMonth() &&
    d.getDate() === t.getDate()
  );
}

export async function loadChecks() {
  try {
    const token = await getAccessToken();
    if (!token) {
      return;
    }

    const checks = await invoke<{ type: number; time: string }[]>(
      'get_recent_checks',
      { token },
    );
    const today = checks.filter((c) => isToday(c.time));
    todayChecks.set(today);
    morningChecked.set(today.some((c) => c.type === 0));
    nightChecked.set(today.some((c) => c.type === 1));
  } catch (error) {
    console.error('チェック状況の読み込み失敗:', error);
  }
}
