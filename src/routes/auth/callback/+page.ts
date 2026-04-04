import { goto } from '$app/navigation';
import { supabaseClient } from '$lib/supabase';

export async function load() {
  // URL からセッションを確立するため、自動的に Supabase がハンドルします
  const { data, error } = await supabaseClient.auth.getSession();

  if (error) {
    console.error('コールバックエラー:', error);
    // エラーの場合はログインページへリダイレクト
    goto('/auth/login');
    return;
  }

  if (data.session) {
    // セッション確立後、ホームページへリダイレクト
    console.log('セッション確立:', data.session.user.email);
    goto('/');
  } else {
    // セッションがない場合はログインページへ
    goto('/auth/login');
  }
}
