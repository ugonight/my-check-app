package com.ugonight.my_check_app

import android.app.NotificationChannel
import android.app.NotificationManager
import android.content.Context
import androidx.core.app.NotificationCompat
import androidx.work.WorkerParameters
import java.time.Duration
import java.time.LocalDate
import java.time.LocalDateTime
import kotlinx.serialization.Serializable
import okhttp3.OkHttpClient
import okhttp3.Request
import androidx.work.CoroutineWorker
import kotlinx.serialization.json.Json

class CheckWorker(context: Context, params: WorkerParameters) : CoroutineWorker(context, params) {
    override suspend fun doWork(): Result {
        return try {
            val constants = fetchConstants()
            val now = LocalDateTime.now()

            val isMorning = isInMorningRange(now, constants)
            val isNight = isInNightRange(now, constants)

            val targetType =
                    when {
                        isMorning -> 0
                        isNight -> 1
                        else -> null
                    }

            if (targetType != null) {
                val isChecked = fetchCheck(targetType)

                if (!isChecked) {
                    val message =
                            when (targetType) {
                                0 -> "朝のチェックがまだです"
                                1 -> "夜のチェックがまだです"
                                else -> "チェックがまだです"
                            }
                    showNotification(message)
                }
            }

            val delay = calculateNextDelay(now, constants)
            WorkScheduler.scheduleNext(applicationContext, delay)

            Result.success()
        } catch (e: Exception) {
            Result.retry()
        }
    }

    // -----------------------------
    // Supabase constants の取得
    // -----------------------------
    @Serializable data class Constant(val key: String, val value: String)

    private val client = OkHttpClient()

    private fun fetchConstants(): Map<String, Int> {
        val baseUrl = EnvLoader.url(applicationContext)
        val key = EnvLoader.key(applicationContext)

        val url = "$baseUrl/constants"

        val request =
                Request.Builder()
                        .url(url)
                        .addHeader("apikey", key)
                        .addHeader("Authorization", "Bearer $key")
                        .build()
        val body =
                client.newCall(request).execute().use { response ->
                    if (!response.isSuccessful) {
                        throw Exception("HTTP error: ${response.code}")
                    }
                    response.body?.string() ?: "[]"
                }
        val list = Json.decodeFromString<List<Constant>>(body)
        return list.associate { it.key to it.value.toInt() }
    }

    // -----------------------------
    // 朝の時間帯判定
    // -----------------------------
    private fun isInMorningRange(now: LocalDateTime, c: Map<String, Int>): Boolean {
        val start = c["MORNING_START"] ?: 6
        val end = c["MORNING_END"] ?: 12
        return now.hour in start until end
    }

    // -----------------------------
    // 夜の時間帯判定（翌日またぎ対応）
    // -----------------------------
    private fun isInNightRange(now: LocalDateTime, c: Map<String, Int>): Boolean {
        val start = c["NIGHT_START"] ?: 18
        val end = c["NIGHT_END"] ?: 1

        return if (start < end) {
            now.hour in start until end
        } else {
            now.hour >= start || now.hour < end
        }
    }

    // -----------------------------
    // daily_checks の type を見て判定
    // -----------------------------
    private fun fetchCheck(type: Int): Boolean {
        val baseUrl = EnvLoader.url(applicationContext)
        val key = EnvLoader.key(applicationContext)

        val today = LocalDate.now().toString()
        val tomorrow = LocalDate.now().plusDays(1).toString()
        val tz = "%2B09:00"

        val url =
                "$baseUrl/daily_checks" +
                        "?time=gte.${today}T00:00:00$tz" +
                        "&time=lt.${tomorrow}T00:00:00$tz" +
                        "&type=eq.$type"

        val request =
                Request.Builder()
                        .url(url)
                        .addHeader("apikey", key)
                        .addHeader("Authorization", "Bearer $key")
                        .build()

        val body =
                client.newCall(request).execute().use { response ->
                    if (!response.isSuccessful) {
                        throw Exception("HTTP error: ${response.code}")
                    }
                    response.body?.string() ?: "[]"
                }

        return body != "[]"
    }

    // -----------------------------
    // 通知
    // -----------------------------
    private fun showNotification(message: String) {
        val manager =
                applicationContext.getSystemService(Context.NOTIFICATION_SERVICE) as
                        NotificationManager

        val channelId = "check_channel"

        val channel =
                NotificationChannel(channelId, "チェック通知", NotificationManager.IMPORTANCE_DEFAULT)
        if (manager.getNotificationChannel(channelId) == null) {
            manager.createNotificationChannel(channel)
        }

        val notification =
                NotificationCompat.Builder(applicationContext, channelId)
                        .setContentTitle("未チェックです")
                        .setContentText(message)
                        .setSmallIcon(android.R.drawable.ic_dialog_info)
                        .build()

        manager.notify(1, notification)
    }

    private fun calculateNextDelay(now: LocalDateTime, c: Map<String, Int>): Long {
        val morning = c["MORNING_NOTIFY"] ?: 9
        val night = c["NIGHT_NOTIFY"] ?: 21

        val todayMorning = now.withHour(morning).withMinute(0).withSecond(0)
        val todayNight = now.withHour(night).withMinute(0).withSecond(0)

        val candidates =
                listOf(todayMorning, todayNight).map { if (it.isAfter(now)) it else it.plusDays(1) }

        val next = candidates.minBy { Duration.between(now, it).toMillis() }

        return Duration.between(now, next).toMillis()
    }
}
