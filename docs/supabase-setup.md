# Supabase セットアップガイド

## テーブル一覧

### 1. `daily_checks` テーブル

チェック履歴を記録するメインテーブル。

| カラム名 | 型 | 説明 |
|---------|-----|------|
| id | serial (PK) | 自動採番 |
| user_id | uuid (FK) | auth.users.id への外部キー |
| time | timestamptz | 記録した時刻（タイムゾーン対応） |
| type | integer | 0=朝、1=夜 |

**セットアップ**:

```sql
-- 既存テーブルを削除（Google認証導入時：既存データはリセット）
DROP TABLE IF EXISTS daily_checks CASCADE;

-- テーブル作成（user_id は必須）
CREATE TABLE daily_checks (
  id SERIAL PRIMARY KEY,
  user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
  time TIMESTAMPTZ DEFAULT NOW(),
  type INTEGER NOT NULL CHECK (type IN (0, 1))
);

-- インデックス作成
CREATE INDEX idx_daily_checks_user_id ON daily_checks(user_id);

-- RLS 有効化
ALTER TABLE daily_checks ENABLE ROW LEVEL SECURITY;

-- ユーザーは自分のチェックのみ参照可能
CREATE POLICY "Users can view their own checks" ON daily_checks
  FOR SELECT USING (auth.uid() = user_id);

-- ユーザーは自分のチェックのみ作成可能
CREATE POLICY "Users can insert their own checks" ON daily_checks
  FOR INSERT WITH CHECK (auth.uid() = user_id);

-- ユーザーは自分のチェックのみ更新可能
CREATE POLICY "Users can update their own checks" ON daily_checks
  FOR UPDATE USING (auth.uid() = user_id);

-- ユーザーは自分のチェックのみ削除可能
CREATE POLICY "Users can delete their own checks" ON daily_checks
  FOR DELETE USING (auth.uid() = user_id);
```

---

## テーブル構造

### 2. `constants` テーブル

定数情報を一元管理するテーブル。アプリケーションが起動時に読み込み、定数として使用します。

| カラム名 | 型 | 説明 |
|---------|-----|------|
| id | bigint (PK) | 自動採番 |
| key | text (UNIQUE) | 定数のキー（例：MORNING_START） |
| value | text | 値 |
| description | text | 説明 |
| created_at | timestamp | 作成日時 |

### デフォルトデータ

```json
[
  { "key": "MORNING_START", "value": "6", "description": "朝チェック開始時刻" },
  { "key": "MORNING_END", "value": "12", "description": "朝チェック終了時刻" },
  { "key": "MORNING_NOTIFY", "value": "8", "description": "朝チェック通知時刻" },
  { "key": "NIGHT_START", "value": "18", "description": "夜チェック開始時刻" },
  { "key": "NIGHT_END", "value": "1", "description": "夜チェック終了時刻（翌日）" },
  { "key": "NIGHT_NOTIFY", "value": "22", "description": "夜チェック通知時刻" },
  { "key": "DATE_RESET_HOUR", "value": "4", "description": "日付リセット時刻" }
]
```

---

## セットアップ手順

### 1. Supabaseプロジェクト作成

1. [Supabase](https://supabase.com/) にアクセス
2. 新規プロジェクトを作成し、Project URLとAPI Keyを記録

### 2. daily_checks テーブル作成

Supabaseダッシュボード → **SQL Editor** で以下を実行：

```sql
-- 既存テーブルを削除（Google認証導入時：既存データはリセット）
DROP TABLE IF EXISTS daily_checks CASCADE;

-- テーブル作成（user_id は必須）
CREATE TABLE daily_checks (
  id SERIAL PRIMARY KEY,
  user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
  time TIMESTAMPTZ DEFAULT NOW(),
  type INTEGER NOT NULL CHECK (type IN (0, 1))
);

-- インデックス作成
CREATE INDEX idx_daily_checks_user_id ON daily_checks(user_id);

-- RLS 有効化
ALTER TABLE daily_checks ENABLE ROW LEVEL SECURITY;

-- ユーザーは自分のチェックのみ参照可能
CREATE POLICY "Users can view their own checks" ON daily_checks
  FOR SELECT USING (auth.uid() = user_id);

-- ユーザーは自分のチェックのみ作成可能
CREATE POLICY "Users can insert their own checks" ON daily_checks
  FOR INSERT WITH CHECK (auth.uid() = user_id);

-- ユーザーは自分のチェックのみ更新可能
CREATE POLICY "Users can update their own checks" ON daily_checks
  FOR UPDATE USING (auth.uid() = user_id);

-- ユーザーは自分のチェックのみ削除可能
CREATE POLICY "Users can delete their own checks" ON daily_checks
  FOR DELETE USING (auth.uid() = user_id);
```

### 3. constants テーブル作成

```sql
CREATE TABLE constants (
  id BIGSERIAL PRIMARY KEY,
  key TEXT UNIQUE NOT NULL,
  value TEXT NOT NULL,
  description TEXT,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT TIMEZONE('UTC'::text, NOW()) NOT NULL
);

-- インデックス作成
CREATE INDEX idx_constants_key ON constants(key);

-- RLS 有効化
ALTER TABLE constants ENABLE ROW LEVEL SECURITY;

-- 読み取り許可
CREATE POLICY "Allow public read" ON constants
  FOR SELECT USING (true);

-- 書き込み禁止（INSERT / UPDATE / DELETE を個別に作成）
CREATE POLICY "Prevent public insert" ON constants
  FOR INSERT WITH CHECK (false);

CREATE POLICY "Prevent public update" ON constants
  FOR UPDATE WITH CHECK (false);

CREATE POLICY "Prevent public delete" ON constants
  FOR DELETE USING (false);
```

### 4. デフォルトデータ挿入

```sql
INSERT INTO constants (key, value, description) VALUES
('MORNING_START', "6", '朝チェック開始時刻'),
('MORNING_END', "12", '朝チェック終了時刻'),
('MORNING_NOTIFY', "8", '朝チェック通知時刻'),
('NIGHT_START', "18", '夜チェック開始時刻'),
('NIGHT_END', "1", '夜チェック終了時刻（翌日）'),
('NIGHT_NOTIFY', "22", '夜チェック通知時刻'),
('DATE_RESET_HOUR', "4", '日付リセット時刻');
```

### 5. 環境変数設定

`src-tauri/.env` を作成・編集：

```env
SUPABASE_URL=https://<your-project>.supabase.co
SUPABASE_KEY=<your-anon-key>
```

---

## 管理と変更

- **編集**: Supabaseコンソール上の「テーブルエディタ」から直接編集可能
- **反映**: アプリ再起動時に最新の設定を読み込む
- **バージョン管理**: 設定変更履歴が必要な場合は `time_settings_history` テーブルの追加を検討
