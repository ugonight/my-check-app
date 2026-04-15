import { writable, derived } from 'svelte/store';
import { supabaseClient } from '../supabase';
import type { AuthUser, Session } from '../types';
import { invoke } from '@tauri-apps/api/core';
import { openUrl } from '@tauri-apps/plugin-opener';
import { listen } from '@tauri-apps/api/event';

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
        redirectTo: 'mycheckapp://my-check-app/auth-callback',
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
 * Deep-link イベントをリッスンしてトークンを処理
 * Tauri アプリが google 認可後にリダイレクトされた時に実行される
 */
export async function setupDeepLinkListener() {
  const unlisteners: (() => void)[] = [];

  const processDeepLink = async (uri: string) => {
    console.log('Processing deep link:', uri);

    // URI の形式：mycheckapp://my-check-app/auth-callback#access_token=...&refresh_token=...
    const hashIndex = uri.indexOf('#');
    if (hashIndex > -1) {
      const hashParams = new URLSearchParams(uri.substring(hashIndex + 1));
      const access_token = hashParams.get('access_token');
      const refresh_token = hashParams.get('refresh_token');

      if (access_token && refresh_token) {
        isLoading.set(true);
        authError.set(null);

        try {
          const { error } = await supabaseClient.auth.setSession({
            access_token,
            refresh_token,
          });

          if (error) {
            console.error('セッション設定エラー:', error);
            authError.set(error.message);
            return;
          }

          console.log('セッション確立成功（Deep-link経由）');
        } catch (err) {
          const message = typeof err === 'string' ? err : (err as Error).message;
          authError.set(message || 'セッション確立に失敗しました');
          console.error('セッション確立例外:', err);
        } finally {
          isLoading.set(false);
        }
      }
    }
  };

  try {
    // window.location から fragment を読み取る（フォールバック）
    console.log('Current location:', window.location.href);
    if (window.location.hash) {
      console.log('Fragment found in window.location:', window.location.hash);
      const hashParams = new URLSearchParams(window.location.hash.substring(1));
      const access_token = hashParams.get('access_token');
      const refresh_token = hashParams.get('refresh_token');

      if (access_token && refresh_token) {
        console.log('Tokens found in window.location, setting session...');
        await processDeepLink(window.location.href);
      }
    }

    // single-instance プラグイン経由の deep-link イベント（Windows）
    const unlistenDeepLinkUri = await listen<string>('deep-link-uri', async (event) => {
      console.log('Deep link URI received (single-instance):', event.payload);
      await processDeepLink(event.payload);
    });
    unlisteners.push(unlistenDeepLinkUri);

    // Tauri deep-link プラグインがアプリに渡す URL をリッスン
    // イベント名は "tauri://protocol-invoked" または プラグインが定義するもの
    const unlistenProtocol = await listen<{ protocol: string; path: string }>('tauri://protocol-invoked', async (event) => {
      console.log('Protocol invoked:', event.payload);

      // Protocol が mycheckapp の場合は処理
      if (event.payload.protocol === 'mycheckapp') {
        // path から fragment を抽出（パス: /auth-callback#access_token=...）
        const uri = `mycheckapp://my-check-app${event.payload.path}`;
        await processDeepLink(uri);
      }
    });
    unlisteners.push(unlistenProtocol);

    // すべてのリスナーを一度にアンサブスクライブする関数を返す
    return () => {
      unlisteners.forEach((unlisten) => unlisten());
    };
  } catch (err) {
    console.warn('Deep-link listener 設定スキップ（非 Tauri 環境）:', err);
    // 非 Tauri 環境（ブラウザ）では listen に対応していないため無視
    return () => {};
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
