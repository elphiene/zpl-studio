package com.zplprinter.utils

import android.content.Context
import android.content.SharedPreferences

/**
 * Simple template storage using SharedPreferences
 * KISS: No database, just save/load one template
 */
class TemplateStorage(context: Context) {
    private val prefs: SharedPreferences = context.getSharedPreferences(
        "zpl_printer_prefs",
        Context.MODE_PRIVATE
    )

    fun saveTemplate(zpl: String) {
        prefs.edit().putString(KEY_TEMPLATE, zpl).apply()
    }

    fun loadTemplate(): String? {
        return prefs.getString(KEY_TEMPLATE, null)
    }

    fun saveFileName(name: String) {
        prefs.edit().putString(KEY_FILENAME, name).apply()
    }

    fun loadFileName(): String? {
        return prefs.getString(KEY_FILENAME, null)
    }

    companion object {
        private const val KEY_TEMPLATE = "template"
        private const val KEY_FILENAME = "filename"
    }
}
