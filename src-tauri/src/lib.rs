use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use tauri::command;

const SUPABASE_URL: &str = "https://kxlqfoansbiymasmhvie.supabase.co/rest/v1"; // ← 自分のプロジェクトURLに変更
const SUPABASE_KEY: &str = "sb_publishable_IGryZxJpy6G5rca_x0Kcow_-rYT8Q3p"; // ← Supabaseのanon key（Project Settings → API → anon public）に変更

#[derive(Serialize, Deserialize, Debug, Clone)]
struct DailyCheck {
    id: i32,
    time: String,
    #[serde(rename = "type")]
    check_type: i32, // Rustの"type"予約語回避
}

// 直近30件取得（Svelte側で「今日」判定）
#[command]
async fn get_recent_checks() -> Result<Vec<DailyCheck>, String> {
    let client = Postgrest::new(SUPABASE_URL)
        .insert_header("apikey", SUPABASE_KEY)
        .insert_header("Authorization", format!("Bearer {}", SUPABASE_KEY));

    let resp = client
        .from("daily_checks")
        .select("*")
        .order("time.desc") // 新しい順
        .limit(30)
        .execute()
        .await
        .map_err(|e| e.to_string())?;

    let body = resp.text().await.map_err(|e| e.to_string())?;
    println!("Supabaseからのレスポンス: {}", body);
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

    let client = Postgrest::new(SUPABASE_URL)
        .insert_header("apikey", SUPABASE_KEY)
        .insert_header("Authorization", format!("Bearer {}", SUPABASE_KEY));

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
