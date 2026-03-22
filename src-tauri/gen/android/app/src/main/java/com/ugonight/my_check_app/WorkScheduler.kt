package com.ugonight.my_check_app

import android.content.Context
import androidx.work.ExistingWorkPolicy
import androidx.work.OneTimeWorkRequestBuilder
import androidx.work.WorkManager
import kotlinx.serialization.Serializable
import kotlinx.serialization.decodeFromString
import kotlinx.serialization.json.Json
import okhttp3.OkHttpClient
import okhttp3.Request
import java.time.Duration
import java.time.LocalDateTime
import java.util.concurrent.TimeUnit

@Serializable
data class Constant(
    val key: String,
    val value: String
)

object WorkScheduler {

    fun schedule(context: Context) {
        val (morning, night) = fetchNotifyTimes(context)
        val delay = calculateNextDelay(morning, night)

        val request =
            OneTimeWorkRequestBuilder<CheckWorker>()
                .setInitialDelay(delay, TimeUnit.MILLISECONDS)
                .build()

        WorkManager.getInstance(context)
            .enqueueUniqueWork(
                "daily_check",
                ExistingWorkPolicy.REPLACE,
                request
            )
    }

    private fun fetchNotifyTimes(context: Context): Pair<Int, Int> {
        val baseUrl = EnvLoader.url(context)
        val key = EnvLoader.key(context)

        val url = "$baseUrl/constants"

        val request = Request.Builder()
            .url(url)
            .addHeader("apikey", key)
            .addHeader("Authorization", "Bearer $key")
            .build()

        val response = OkHttpClient().newCall(request).execute()
        val body = response.body?.string() ?: "[]"

        val list = Json.decodeFromString<List<Constant>>(body)

        val morning = list.first { it.key == "MORNING_NOTIFY" }.value.toInt()
        val night = list.first { it.key == "NIGHT_NOTIFY" }.value.toInt()

        return morning to night
    }

    private fun calculateNextDelay(morning: Int, night: Int): Long {
        val now = LocalDateTime.now()

        val todayMorning = now.withHour(morning).withMinute(0).withSecond(0)
        val todayNight = now.withHour(night).withMinute(0).withSecond(0)

        val candidates = listOf(todayMorning, todayNight)
            .map { if (it.isAfter(now)) it else it.plusDays(1) }

        val next = candidates.minBy { Duration.between(now, it).toMillis() }

        return Duration.between(now, next).toMillis()
    }
}