package com.ugonight.my_check_app

import android.content.Context
import androidx.work.PeriodicWorkRequestBuilder
import androidx.work.ExistingPeriodicWorkPolicy
import androidx.work.WorkManager
import java.time.Duration
import java.time.LocalDateTime
import java.util.concurrent.TimeUnit

object WorkScheduler {

    fun schedule(context: Context) {
        // val request =
        //     PeriodicWorkRequestBuilder<CheckWorker>(1, TimeUnit.DAYS)
        //         .setInitialDelay(calculateDelay(), TimeUnit.MILLISECONDS)
        //         .build()
        // 15 分ごとに実行
        val request = PeriodicWorkRequestBuilder<CheckWorker>(
            15, TimeUnit.MINUTES
        ).build()

        WorkManager.getInstance(context).enqueueUniquePeriodicWork(
            "daily_check",
            ExistingPeriodicWorkPolicy.UPDATE,
            request
        )
    }

    private fun calculateDelay(): Long {
        val now = LocalDateTime.now()
        val target = now.withHour(21).withMinute(0).withSecond(0)

        val next = if (now > target) target.plusDays(1) else target
        return Duration.between(now, next).toMillis()
    }
}