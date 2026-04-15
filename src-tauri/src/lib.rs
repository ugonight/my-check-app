use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use tauri::command;
use tauri::Manager;

mod auth;
use auth::verify_jwt;

fn get_supabase_url() -> String {
    // .env の SUPABASE_URL を取得し、/rest/v1 を追加
    let base_url = env!("SUPABASE_URL").to_string();
    format!("{base_url}/rest/v1")
}

fn get_supabase_key() -> String {
    env!("SUPABASE_KEY").to_string()
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct DailyCheck {
    id: i32,
    user_id: String, // UUID as String
    time: String,
    #[serde(rename = "type")]
    check_type: i32, // Rustの"type"予約語回避
}

fn create_authenticated_client() -> Result<Postgrest, String> {
    let supabase_url = get_supabase_url();
    let supabase_key = get_supabase_key();

    if supabase_url.is_empty() || supabase_key.is_empty() {
        return Err("Supabaseの設定が不足しています。.envファイルをご確認ください。".to_string());
    }

    Ok(Postgrest::new(&supabase_url)
        .insert_header("apikey", &supabase_key)
        .insert_header("Authorization", format!("Bearer {supabase_key}")))
}

// JWT トークンを使用する認証クライアント
async fn create_authenticated_client_with_token(token: &str) -> Result<Postgrest, String> {
    let supabase_url = get_supabase_url();
    let supabase_key = get_supabase_key();

    if supabase_url.is_empty() || supabase_key.is_empty() {
        return Err("Supabaseの設定が不足しています。.envファイルをご確認ください。".to_string());
    }

    // JWT トークン検証
    let _user_id = verify_jwt(token).await?;

    Ok(Postgrest::new(&supabase_url)
        .insert_header("apikey", &supabase_key)
        .insert_header("Authorization", format!("Bearer {token}")))
}

// 直近30件取得（トークン必須）
#[command]
async fn get_recent_checks(token: String) -> Result<Vec<DailyCheck>, String> {
    let client = create_authenticated_client_with_token(&token).await?;

    let resp = client
        .from("daily_checks")
        .select("*")
        .order("time.desc") // 新しい順
        .limit(30)
        .execute()
        .await
        .map_err(|e| e.to_string())?;

    let body = resp.text().await.map_err(|e| e.to_string())?;
    let checks: Vec<DailyCheck> =
        serde_json::from_str(&body).map_err(|e| format!("JSONパース失敗: {e}"))?;

    Ok(checks)
}

// チェック記録（INSERT、トークン必須）
#[command]
async fn insert_check(token: String, check_type: i32) -> Result<(), String> {
    if check_type != 0 && check_type != 1 {
        return Err("typeは0または1のみ".to_string());
    }

    // JWT トークン検証と user_id 抽出
    let user_id = verify_jwt(&token).await?;

    let client = create_authenticated_client_with_token(&token).await?;
    let payload = serde_json::json!({
        "user_id": user_id,
        "type": check_type
    });

    client
        .from("daily_checks")
        .insert(payload.to_string())
        .execute()
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Constants {
    pub key: String,
    pub value: String,
    pub description: Option<String>,
}

#[command]
async fn get_constants() -> Result<Vec<Constants>, String> {
    let client = create_authenticated_client()?;

    let resp = client
        .from("constants")
        .select("*")
        .execute()
        .await
        .map_err(|e| e.to_string())?;

    let body = resp.text().await.map_err(|e| e.to_string())?;
    let settings: Vec<Constants> =
        serde_json::from_str(&body).map_err(|e| format!("JSONパース失敗: {e}"))?;

    Ok(settings)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default();

    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_single_instance::init(|app, argv, _cwd| {
          println!("a new app instance was opened with {argv:?} and the deep link event was already triggered");

          // argv から mycheckapp:// スキームのディープリンク URL を探す
          if let Some(deep_link_uri) = argv.iter().find(|arg| arg.starts_with("mycheckapp://")) {
            use tauri::Emitter;

            println!("Deep link URI found: {}", deep_link_uri);
            // フロント側に deep-link-uri イベントで通知
            let _ = app.emit("deep-link-uri", deep_link_uri);
          }
        }));
    }

    builder = builder
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_recent_checks,
            insert_check,
            get_constants
        ])
        .setup(|app| {
            // Deep-link プラグインは protocol registration を自動で行う
            // フロント側が deep-link イベントをキャッチして処理する
            #[cfg(any(windows, target_os = "linux"))]
            {
                use tauri_plugin_deep_link::DeepLinkExt;
                app.deep_link().register_all()?;
            }

            // Android では Intent から URI を抽出する
            #[cfg(target_os = "android")]
            {
                use tauri::Emitter;
                use tauri_plugin_deep_link::DeepLinkExt;

                eprintln!("Android: Deep-link setup starting");

                // スタートアップ時の deep-link を取得
                match app.deep_link().get_current() {
                    Ok(Some(urls)) => {
                        eprintln!("Android: Deep links found on startup: {:?}", urls);
                        if let Some(deep_link_url) = urls.first() {
                            eprintln!("Android: Emitting deep-link-uri: {}", deep_link_url);
                            let _ = app.emit("deep-link-uri", deep_link_url.to_string());
                        }
                    }
                    Ok(None) => {
                        eprintln!("Android: No deep links found on startup");
                    }
                    Err(e) => {
                        eprintln!("Android: Error getting current deep link: {}", e);
                    }
                }

                // 実行中の deep-link activation をリッスン
                let app_handle = app.handle().clone();
                app.deep_link().on_open_url(move |request| {
                    if let Some(url) = request.urls().first() {
                        eprintln!("Android: Deep link request received: {}", url);
                        let _ = app_handle.emit("deep-link-uri", url.to_string());
                    }
                });
            }

            Ok(())
        });

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
