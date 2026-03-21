package com.ugonight.my_check_app

import android.app.NotificationChannel
import android.app.NotificationManager
import android.content.Context
import androidx.core.app.NotificationCompat
import androidx.work.Worker
import androidx.work.WorkerParameters
import java.time.LocalDate
import okhttp3.OkHttpClient
import okhttp3.Request

class CheckWorker(context: Context, params: WorkerParameters) : Worker(context, params) {

    override fun doWork(): Result {
        val prefs = applicationContext.getSharedPreferences("app", Context.MODE_PRIVATE)

        return try {
            val isChecked = fetchCheck()

            if (!isChecked) {
                showNotification()
            }

            Result.success()
        } catch (e: Exception) {
            Result.retry()
        }
    }

    private fun fetchCheck(): Boolean {
        val baseUrl = EnvLoader.url(applicationContext)
        val key = EnvLoader.key(applicationContext)

        // 今日と明日（JST）
        val today = LocalDate.now().toString() // 2025-02-21
        val tomorrow = LocalDate.now().plusDays(1).toString()

        // +09:00 は URL エンコードして %2B09:00 にする
        val tz = "%2B09:00"

        val url =
                "$baseUrl/daily_checks" +
                        "?time=gte.${today}T00:00:00$tz" +
                        "&time=lt.${tomorrow}T00:00:00$tz"

        val request =
                Request.Builder()
                        .url(url)
                        .addHeader("apikey", key)
                        .addHeader("Authorization", "Bearer $key")
                        .build()

        val response = OkHttpClient().newCall(request).execute()
        val body = response.body?.string()

        return body != "[]"
    }

    private fun showNotification() {
        val manager =
                applicationContext.getSystemService(Context.NOTIFICATION_SERVICE) as
                        NotificationManager

        val channelId = "check_channel"

        val channel =
                NotificationChannel(channelId, "チェック通知", NotificationManager.IMPORTANCE_DEFAULT)
        manager.createNotificationChannel(channel)

        val notification =
                NotificationCompat.Builder(applicationContext, channelId)
                        .setContentTitle("未チェックです")
                        .setContentText("今日のチェックをしてください")
                        .setSmallIcon(android.R.drawable.ic_dialog_info)
                        .build()

        manager.notify(1, notification)
    }
}
