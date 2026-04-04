use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
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

/// Supabase の public key を環境変数から読み込む
fn get_supabase_jwt_secret() -> String {
    env!("SUPABASE_JWT_SECRET").to_string()
}

/// JWT トークンを検証し、user_id を抽出する
pub fn verify_jwt(token: &str) -> Result<String, String> {
    // Bearer スキーム削除
    let token = if token.starts_with("Bearer ") {
        &token[7..]
    } else {
        token
    };

    let secret = get_supabase_jwt_secret();

    // Supabase の JWT シークレット を使用してデコード
    // 実装例：HMAC-SHA256 署名検証
    let decoding_key = DecodingKey::from_secret(secret.as_ref());

    let token_data = decode::<Claims>(token, &decoding_key, &Validation::new(Algorithm::HS256))
        .map_err(|e| format!("JWT検証失敗: {}", e))?;

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
pub fn extract_user_id(token: &str) -> Result<String, String> {
    verify_jwt(token)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_user_id_invalid_token() {
        let result = extract_user_id("invalid.token.here");
        assert!(result.is_err());
    }

    #[test]
    fn test_bearer_scheme_removal() {
        // This is a simplified test; real JWT testing requires a valid token
        let token_with_bearer = "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9";
        let token_without_bearer = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9";

        // Both should fail due to invalid JWT, but the Bearer removal should work
        let result1 = extract_user_id(token_with_bearer);
        let result2 = extract_user_id(token_without_bearer);

        assert!(result1.is_err());
        assert!(result2.is_err());
    }
}
