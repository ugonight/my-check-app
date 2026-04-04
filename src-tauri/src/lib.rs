use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use tauri::command;
use uuid::Uuid;

mod auth;
use auth::{extract_user_id, verify_jwt};

fn get_supabase_url() -> String {
    env!("SUPABASE_URL").to_string()
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
        .insert_header("Authorization", format!("Bearer {}", supabase_key)))
}

// JWT トークンを使用する認証クライアント
fn create_authenticated_client_with_token(token: &str) -> Result<Postgrest, String> {
    let supabase_url = get_supabase_url();
    let supabase_key = get_supabase_key();

    if supabase_url.is_empty() || supabase_key.is_empty() {
        return Err("Supabaseの設定が不足しています。.envファイルをご確認ください。".to_string());
    }

    // JWT トークン検証
    let user_id = extract_user_id(token)?;

    Ok(Postgrest::new(&supabase_url)
        .insert_header("apikey", &supabase_key)
        .insert_header("Authorization", format!("Bearer {}", token)))
}

// 直近30件取得（トークン必須）
#[command]
async fn get_recent_checks(token: String) -> Result<Vec<DailyCheck>, String> {
    let client = create_authenticated_client_with_token(&token)?;

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

// チェック記録（INSERT、トークン必須）
#[command]
async fn insert_check(token: String, check_type: i32) -> Result<(), String> {
    if check_type != 0 && check_type != 1 {
        return Err("typeは0または1のみ".to_string());
    }

    // JWT トークン検証と user_id 抽出
    let user_id = extract_user_id(&token)?;

    let client = create_authenticated_client_with_token(&token)?;
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
        serde_json::from_str(&body).map_err(|e| format!("JSONパース失敗: {}", e))?;

    Ok(settings)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_recent_checks,
            insert_check,
            get_constants
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
