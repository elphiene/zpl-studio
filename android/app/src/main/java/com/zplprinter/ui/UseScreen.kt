package com.zplprinter.ui

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.zplprinter.PrinterViewModel

/**
 * Use mode screen - fill in template variables
 * KISS: Simple form, nothing fancy
 */
@Composable
fun UseScreen(
    vm: PrinterViewModel,
    onPrintClick: () -> Unit
) {
    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(16.dp)
            .verticalScroll(rememberScrollState())
    ) {
        if (vm.template.isEmpty()) {
            // No template loaded
            Text(
                text = "No template loaded",
                style = MaterialTheme.typography.bodyLarge
            )
            Spacer(modifier = Modifier.height(8.dp))
            Text(
                text = "Click 'Load Template' to load a .zpl file",
                style = MaterialTheme.typography.bodyMedium,
                color = MaterialTheme.colorScheme.onSurfaceVariant
            )
        } else {
            // Show variables form
            Text(
                text = "Template Variables",
                style = MaterialTheme.typography.titleMedium
            )

            Spacer(modifier = Modifier.height(16.dp))

            vm.variables.forEach { variable ->
                OutlinedTextField(
                    value = vm.values[variable.name] ?: "",
                    onValueChange = { vm.updateValue(variable.name, it) },
                    label = { Text(variable.label) },
                    modifier = Modifier.fillMaxWidth(),
                    singleLine = true
                )
                Spacer(modifier = Modifier.height(8.dp))
            }

            Spacer(modifier = Modifier.height(16.dp))

            // Print button
            Button(
                onClick = onPrintClick,
                modifier = Modifier.fillMaxWidth()
            ) {
                Text("🖨 Print Label")
            }

            Spacer(modifier = Modifier.height(8.dp))

            // Clear button
            OutlinedButton(
                onClick = {
                    vm.values = vm.variables.associate { it.name to "" }
                    vm.statusMessage = "Fields cleared"
                },
                modifier = Modifier.fillMaxWidth()
            ) {
                Text("🔄 Clear")
            }
        }
    }
}
