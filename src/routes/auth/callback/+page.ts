import { goto } from '$app/navigation';
import { supabaseClient } from '$lib/supabase';

export async function load() {
  // フラグメント（#access_token=...）から手動セッション確立
  const hash = typeof window !== 'undefined' ? window.location.hash : '';

  if (hash.includes('access_token')) {
    const params = new URLSearchParams(hash.substring(1));
    const access_token = params.get('access_token');
    const refresh_token = params.get('refresh_token');

    if (access_token && refresh_token) {
      const { error } = await supabaseClient.auth.setSession({
        access_token,
        refresh_token,
      });

      if (error) {
        console.error('セッション設定エラー:', error);
        goto('/auth/login');
        return;
      }

      console.log('セッション確立成功');
      goto('/');
      return;
    }
  }

  // フラグメントがない場合の フォールバック
  const { data, error } = await supabaseClient.auth.getSession();

  if (error || !data.session) {
    console.error('コールバックエラー:', error);
    goto('/auth/login');
    return;
  }

  console.log('セッション確立:', data.session.user.email);
  goto('/');
}
