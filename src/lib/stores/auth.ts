import { writable, derived } from 'svelte/store';
import { supabaseClient } from '../supabase';
import type { AuthUser, Session } from '../types';
import { invoke } from '@tauri-apps/api/core';
import { openUrl } from '@tauri-apps/plugin-opener';

// 認証状態
export const authUser = writable<AuthUser | null>(null);
export const session = writable<Session | null>(null);
export const isAuthenticated = derived([authUser, session], ([$authUser, $session]) => {
  return !!$authUser && !!$session;
});

// ローディング状態
export const isLoading = writable(false);

// エラーメッセージ
export const authError = writable<string | null>(null);

/**
 * 認証初期化
 * localStorage から認証状態を復元し、トークン検証を行う
 */
export async function initializeAuth() {
  isLoading.set(true);
  try {
    // Supabase の getSession() でセッション確認
    const { data, error } = await supabaseClient.auth.getSession();

    if (error) {
      console.error('セッション確認エラー:', error.message);
      authError.set(error.message);
      return;
    }

    if (data.session && data.session.user) {
      session.set(data.session as Session);
      authUser.set(data.session.user as AuthUser);
    } else {
      // セッション無し
      authUser.set(null);
      session.set(null);
    }
  } catch (err) {
    console.error('認証初期化エラー:', err);
    authError.set('認証状態の確認に失敗しました');
  } finally {
    isLoading.set(false);
  }
}

/**
 * Google OAuth ログイン
 */
export async function signInWithGoogle() {
  isLoading.set(true);
  authError.set(null);

  try {
    const { data, error } = await supabaseClient.auth.signInWithOAuth({
      provider: 'google',
      options: {
        redirectTo: 'tauri://my-check-app/auth-callback',
        skipBrowserRedirect: true,
      },
    });

    if (error) {
      authError.set(error.message);
      console.error('Google ログインエラー:', error);
      return;
    }

    // 外部ブラウザで開く
    if (data.url) {
      await openUrl(data.url);
    }
  } catch (err) {
    const message = typeof err === 'string' ? err : (err as Error).message;
    authError.set(message || 'ログインに失敗しました');
    console.error('Google ログイン例外:', err);
  } finally {
    isLoading.set(false);
  }
}

/**
 * ログアウト
 */
export async function logout() {
  isLoading.set(true);
  authError.set(null);

  try {
    const { error } = await supabaseClient.auth.signOut();

    if (error) {
      authError.set(error.message);
      console.error('ログアウトエラー:', error);
      return;
    }

    // ストア更新
    authUser.set(null);
    session.set(null);
  } catch (err) {
    const message = typeof err === 'string' ? err : (err as Error).message;
    authError.set(message || 'ログアウトに失敗しました');
    console.error('ログアウト例外:', err);
  } finally {
    isLoading.set(false);
  }
}

/**
 * セッションリスナー設定
 * トークン更新やセッション変更を監視
 */
export function setupAuthListener() {
  const { data: subscription } = supabaseClient.auth.onAuthStateChange(
    (event, currentSession) => {
      console.log('Auth state changed:', event);

      if (currentSession && currentSession.user) {
        session.set(currentSession as Session);
        authUser.set(currentSession.user as AuthUser);
      } else {
        authUser.set(null);
        session.set(null);
      }
    }
  );

  // アンサブスクライブ関数を返す
  return () => {
    subscription?.unsubscribe();
  };
}

/**
 * 現在のアクセストークンを取得
 */
export async function getAccessToken(): Promise<string | null> {
  const { data, error } = await supabaseClient.auth.getSession();

  if (error || !data.session) {
    console.error('トークン取得エラー:', error);
    return null;
  }

  return data.session.access_token;
}
