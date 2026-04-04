use std::fs;

fn main() {
    tauri_build::build();

    // .envファイルから環境変数を読み込む
    let _ = dotenvy::dotenv();

    // ビルド時に環境変数をチェックして埋め込む
    let supabase_url = std::env::var("SUPABASE_URL")
        .expect("SUPABASE_URL environment variable is not set. Please set it in .env file or system environment.");
    let supabase_key = std::env::var("SUPABASE_KEY")
        .expect("SUPABASE_KEY environment variable is not set. Please set it in .env file or system environment.");
    let supabase_jwt_secret = std::env::var("SUPABASE_JWT_SECRET")
        .expect("SUPABASE_JWT_SECRET environment variable is not set. Please set it in .env file or system environment.");

    // コンパイル時にこれらの値がバイナリに埋め込まれる
    println!("cargo:rustc-env=SUPABASE_URL={}", supabase_url);
    println!("cargo:rustc-env=SUPABASE_KEY={}", supabase_key);
    println!(
        "cargo:rustc-env=SUPABASE_JWT_SECRET={}",
        supabase_jwt_secret
    );

    // Android向けにenv.jsonも生成
    if fs::exists("gen/android/app/src/main/assets/").unwrap_or(false) {
        let content = format!(
            r#"{{
  "SUPABASE_URL": "{}",
  "SUPABASE_KEY": "{}"
}}"#,
            supabase_url, supabase_key
        );

        fs::write("gen/android/app/src/main/assets/env.json", content)
            .expect("failed to write env.json");
    }
}
