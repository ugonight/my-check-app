package com.ugonight.my_check_app

import android.content.Context
import org.json.JSONObject

object EnvLoader {

    private var cache: JSONObject? = null

    fun load(context: Context): JSONObject {
        if (cache != null) return cache!!

        val input = context.assets.open("env.json")
        val text = input.bufferedReader().use { it.readText() }

        cache = JSONObject(text)
        return cache!!
    }

    fun url(context: Context) = load(context).getString("SUPABASE_URL")
    fun key(context: Context) = load(context).getString("SUPABASE_KEY")
}