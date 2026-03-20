use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use tauri::command;

fn get_supabase_url() -> String {
    env!("SUPABASE_URL").to_string()
}

fn get_supabase_key() -> String {
    env!("SUPABASE_KEY").to_string()
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct DailyCheck {
    id: i32,
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
        .insert_header("Authorization", format!("Bearer {}", supabase_key)))
}

// 直近30件取得（Svelte側で「今日」判定）
#[command]
async fn get_recent_checks() -> Result<Vec<DailyCheck>, String> {
    let client = create_authenticated_client()?;

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
        serde_json::from_str(&body).map_err(|e| format!("JSONパース失敗: {}", e))?;

    Ok(checks)
}

// チェック記録（INSERT）
#[command]
async fn insert_check(check_type: i32) -> Result<(), String> {
    if check_type != 0 && check_type != 1 {
        return Err("typeは0または1のみ".to_string());
    }

    let client = create_authenticated_client()?;
    let payload = serde_json::json!({ "type": check_type });

    client
        .from("daily_checks")
        .insert(payload.to_string())
        .execute()
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_recent_checks, insert_check])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
