package com.zplprinter.utils

/**
 * ZPL template parser - DRY: Reuses regex from desktop version
 * Extracts {{VARIABLE:Label}} placeholders and renders templates
 */
object ZplParser {
    data class Variable(
        val name: String,
        val label: String
    )

    /**
     * Parse ZPL template and extract variables
     * Matches: {{VARIABLE}} or {{VARIABLE:Display Label}}
     * Supports any characters in variable names (including spaces, lowercase, etc.)
     */
    fun parse(zpl: String): List<Variable> {
        val regex = Regex("""\\{\\{([^:}]+?)(?::([^}]+))?\\}\\}""")
        return regex.findAll(zpl)
            .map { match ->
                Variable(
                    name = match.groupValues[1].trim(),
                    label = match.groupValues[2].trim().ifEmpty { match.groupValues[1].trim() }
                )
            }
            .distinctBy { it.name }
            .toList()
    }

    /**
     * Render template by replacing variables with values
     * Handles variable names with special characters
     */
    fun render(zpl: String, values: Map<String, String>): String {
        var result = zpl
        values.forEach { (key, value) ->
            // Escape special regex characters in the key
            val escapedKey = Regex.escape(key)
            // Replace {{VARIABLE}} or {{VARIABLE:Label}}
            result = result.replace(
                Regex("""\\{\\{$escapedKey(?::[^}]+)?\\}\\}"""),
                value
            )
        }
        return result
    }
}
