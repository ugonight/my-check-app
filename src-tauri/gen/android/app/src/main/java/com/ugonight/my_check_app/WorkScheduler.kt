package com.ugonight.my_check_app

import android.content.Context
import androidx.work.ExistingWorkPolicy
import androidx.work.OneTimeWorkRequestBuilder
import androidx.work.WorkManager
import kotlinx.serialization.Serializable
import java.util.concurrent.TimeUnit

@Serializable
data class Constant(
    val key: String,
    val value: String
)

object WorkScheduler {

    fun schedule(context: Context) {
        // 初回起動（即実行）
        scheduleNext(context, 0)
    }

    fun scheduleNext(context: Context, delay: Long) {
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
}