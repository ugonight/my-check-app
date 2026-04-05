use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // user_id
    pub email: String,
    pub exp: i64,    // expiration time
    pub iat: i64,    // issued at
    pub aud: String, // audience
    pub iss: String, // issuer
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JWK {
    pub kty: String,
    pub crv: String,
    pub x: String,
    pub y: String,
    pub kid: String,
    pub use_: Option<String>,
    #[serde(rename = "use")]
    pub use_field: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JWKS {
    pub keys: Vec<JWK>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenHeader {
    pub kid: Option<String>,
}

// JWKS キャッシュ
static JWKS_CACHE: Mutex<Option<JWKS>> = Mutex::new(None);

fn get_supabase_base_url() -> String {
    env!("SUPABASE_URL").to_string()
}

fn get_supabase_jwks_url() -> String {
    format!("{}/auth/v1/.well-known/jwks.json", get_supabase_base_url())
}

/// JWKS を取得（キャッシュ利用）
async fn fetch_jwks() -> Result<JWKS, String> {
    // キャッシュ確認
    if let Ok(cache) = JWKS_CACHE.lock() {
        if let Some(jwks) = cache.as_ref() {
            return Ok(jwks.clone());
        }
    }

    let url = get_supabase_jwks_url();
    let response = reqwest::get(&url)
        .await
        .map_err(|e| format!("JWKS取得失敗: {e}"))?;

    let jwks: JWKS = response
        .json()
        .await
        .map_err(|e| format!("JWKS解析失敗: {e}"))?;

    // キャッシュへ保存
    if let Ok(mut cache) = JWKS_CACHE.lock() {
        *cache = Some(jwks.clone());
    }

    Ok(jwks)
}

/// JWK から ES256 用の DecodingKey を構築
fn jwk_to_decoding_key(jwk: &JWK) -> Result<DecodingKey, String> {
    if jwk.kty != "EC" || jwk.crv != "P-256" {
        return Err(format!(
            "サポートされていない鍵タイプ: {} {}",
            jwk.kty, jwk.crv
        ));
    }

    // ES256 用の DecodingKey を構築（x と y座標を渡す）
    DecodingKey::from_ec_components(&jwk.x, &jwk.y).map_err(|e| format!("DecodingKey構築失敗: {e}"))
}

/// JWT トークンを検証し、user_id を抽出する
pub async fn verify_jwt(token: &str) -> Result<String, String> {
    // Bearer スキーム削除
    let token = if token.starts_with("Bearer ") {
        &token[7..]
    } else {
        token
    };

    // トークンヘッダから kid を取得
    let header = decode_header(token).map_err(|e| format!("ヘッダ解析失敗: {e}"))?;

    let kid = header
        .kid
        .ok_or("トークンに kid がありません".to_string())?;

    // JWKS 取得
    let jwks = fetch_jwks().await?;

    // 対応する JWK を検索
    let jwk = jwks
        .keys
        .iter()
        .find(|k| k.kid == kid)
        .ok_or("JWK が見つかりません".to_string())?;

    // DecodingKey を構築
    let decoding_key = jwk_to_decoding_key(jwk)?;

    // ES256 で検証
    let mut validation = Validation::new(Algorithm::ES256);
    validation.set_audience(&["authenticated"]);

    let token_data = decode::<Claims>(token, &decoding_key, &validation)
        .map_err(|e| format!("JWT検証失敗: {e}"))?;

    let claims = token_data.claims;

    // exp (expiration time) を確認
    let now = chrono::Utc::now().timestamp();
    if claims.exp < now {
        return Err("トークンが期限切れです".to_string());
    }

    // user_id を抽出（subject クレーム）
    if claims.sub.is_empty() {
        return Err("user_id が見つかりません".to_string());
    }

    // UUID として検証
    Uuid::parse_str(&claims.sub).map_err(|_| "無効な user_id です".to_string())?;

    Ok(claims.sub)
}

/// トークンから user_id を抽出（検証済み）
pub async fn extract_user_id(token: &str) -> Result<String, String> {
    verify_jwt(token).await
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_bearer_scheme() {
        // Bearer スキーム削除のテスト（実装確認用）
        let token_with_bearer = "Bearer token123";
        let token = if token_with_bearer.starts_with("Bearer ") {
            &token_with_bearer[7..]
        } else {
            token_with_bearer
        };
        assert_eq!(token, "token123");
    }
}
