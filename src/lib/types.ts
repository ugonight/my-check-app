// Authentication Types
export interface AuthUser {
  id: string;  // UUID
  email: string;
  user_metadata?: {
    name?: string;
    avatar_url?: string;
    provider?: string;
  };
  app_metadata?: {
    provider?: string;
    providers?: string[];
  };
}

export interface Session {
  access_token: string;
  refresh_token: string;
  expires_in: number;
  expires_at?: number;
  token_type: string;
  user: AuthUser;
}

// Daily Check Types
export interface DailyCheck {
  id: number;
  user_id: string;  // UUID
  type: number;     // 0 = morning, 1 = night
  time: string;     // ISO timestamp
}

// Constants Types
export interface Constants {
  key: string;
  value: string;
  description?: string;
}

// API Response Types
export interface ApiResponse<T> {
  data?: T;
  error?: string;
  status: number;
}
