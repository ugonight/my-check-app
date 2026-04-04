import { createClient } from '@supabase/supabase-js';

// Supabase URL と API Key
// 実際の値は src-tauri/.env から取得
const SUPABASE_URL = 'https://kxlqfoansbiymasmhvie.supabase.co';
const SUPABASE_KEY = 'sb_publishable_IGryZxJpy6G5rca_x0Kcow_-rYT8Q3p';

// Supabase クライアント初期化
export const supabaseClient = createClient(SUPABASE_URL, SUPABASE_KEY);

export default supabaseClient;
