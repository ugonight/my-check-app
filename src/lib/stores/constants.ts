import { readable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

let setConstants: (v: Record<string, string>) => void;

export const constants = readable<Record<string, string>>({}, (set) => {
  setConstants = set;
  return () => {};
});

export async function loadConstants() {
  try {
    const settings = await invoke<any[]>('get_constants');
    const settingsMap = Object.fromEntries(
      settings.map(s => [s.key, s.value])
    );
    setConstants(settingsMap);
  } catch (error) {
    console.error('Failed to load constants:', error);
  }
}