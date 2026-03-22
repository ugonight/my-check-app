$headers = @{
  "apikey" = "sb_publishable_IGryZxJpy6G5rca_x0Kcow_-rYT8Q3p"
  "Authorization" = "Bearer sb_publishable_IGryZxJpy6G5rca_x0Kcow_-rYT8Q3p"
}

$todayJST = (Get-Date).ToString("yyyy-MM-dd")
$tomorrowJST = (Get-Date).AddDays(1).ToString("yyyy-MM-dd")

# +09:00 → %2B09:00 に変換
$tz = "%2B09:00"

$url = "https://kxlqfoansbiymasmhvie.supabase.co/rest/v1/daily_checks?time=gte.${todayJST}T00:00:00$tz&time=lt.${tomorrowJST}T00:00:00$tz"

# $url = "https://kxlqfoansbiymasmhvie.supabase.co/rest/v1/constants"
(Invoke-WebRequest -Uri $url -Headers $headers).Content
