package com.zplprinter

import android.content.Intent
import android.net.Uri
import android.os.Bundle
import android.widget.Toast
import androidx.activity.ComponentActivity
import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.activity.compose.setContent
import androidx.activity.result.contract.ActivityResultContracts
import androidx.compose.foundation.layout.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.unit.dp
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewmodel.compose.viewModel
import com.zplprinter.ui.EditScreen
import com.zplprinter.ui.UseScreen
import com.zplprinter.utils.TemplateStorage
import com.zplprinter.utils.ZebraPrinter
import com.zplprinter.utils.ZplParser
import kotlinx.coroutines.launch

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            MaterialTheme {
                MainScreen()
            }
        }
    }
}

/**
 * Single ViewModel for entire app - KISS: No complex architecture
 */
class PrinterViewModel : ViewModel() {
    var template by mutableStateOf("")
    var fileName by mutableStateOf<String?>(null)
    var variables by mutableStateOf<List<ZplParser.Variable>>(emptyList())
    var values by mutableStateOf<Map<String, String>>(emptyMap())
    var printers by mutableStateOf<List<ZebraPrinter.PrinterInfo>>(emptyList())
    var selectedPrinter by mutableStateOf<String?>(null)
    var isLoading by mutableStateOf(false)
    var statusMessage by mutableStateOf("")

    fun loadTemplate(zpl: String, name: String?) {
        template = zpl
        fileName = name
        variables = ZplParser.parse(zpl)
        values = variables.associate { it.name to "" }
    }

    fun updateValue(variableName: String, value: String) {
        values = values + (variableName to value)
    }

    fun getRenderedZpl(): String {
        return ZplParser.render(template, values)
    }
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun MainScreen(vm: PrinterViewModel = viewModel()) {
    val context = LocalContext.current
    val storage = remember { TemplateStorage(context) }
    val printer = remember { ZebraPrinter() }
    val scope = rememberCoroutineScope()

    var selectedTab by remember { mutableIntStateOf(0) }
    var showPrinterDialog by remember { mutableStateOf(false) }

    // File picker launcher
    val filePickerLauncher = rememberLauncherForActivityResult(
        ActivityResultContracts.GetContent()
    ) { uri: Uri? ->
        uri?.let {
            try {
                val zpl = context.contentResolver.openInputStream(it)
                    ?.bufferedReader()?.use { reader -> reader.readText() }
                if (zpl != null) {
                    val name = it.lastPathSegment ?: "template.zpl"
                    vm.loadTemplate(zpl, name)
                    storage.saveTemplate(zpl)
                    storage.saveFileName(name)
                    vm.statusMessage = "Template loaded"
                }
            } catch (e: Exception) {
                Toast.makeText(context, "Error loading file", Toast.LENGTH_SHORT).show()
            }
        }
    }

    // Load saved template on start
    LaunchedEffect(Unit) {
        storage.loadTemplate()?.let { zpl ->
            val name = storage.loadFileName()
            vm.loadTemplate(zpl, name)
        }
    }

    Scaffold(
        topBar = {
            TopAppBar(
                title = { Text("ZPL Printer Tool") },
                actions = {
                    // Load Template button
                    TextButton(onClick = { filePickerLauncher.launch("*/*") }) {
                        Text("📁 Load Template")
                    }
                }
            )
        }
    ) { padding ->
        Column(
            modifier = Modifier
                .fillMaxSize()
                .padding(padding)
        ) {
            // Template filename
            vm.fileName?.let { name ->
                Surface(
                    modifier = Modifier.fillMaxWidth(),
                    color = MaterialTheme.colorScheme.secondaryContainer
                ) {
                    Text(
                        text = "📄 $name",
                        modifier = Modifier.padding(8.dp),
                        style = MaterialTheme.typography.bodySmall
                    )
                }
            }

            // Tab selector
            TabRow(selectedTabIndex = selectedTab) {
                Tab(
                    selected = selectedTab == 0,
                    onClick = { selectedTab = 0 },
                    text = { Text("📄 Use") }
                )
                Tab(
                    selected = selectedTab == 1,
                    onClick = { selectedTab = 1 },
                    text = { Text("✏ Edit") }
                )
            }

            // Tab content
            when (selectedTab) {
                0 -> UseScreen(
                    vm = vm,
                    onPrintClick = { showPrinterDialog = true }
                )
                1 -> EditScreen(
                    vm = vm,
                    onSave = {
                        storage.saveTemplate(vm.template)
                        vm.loadTemplate(vm.template, vm.fileName)
                        vm.statusMessage = "Saved"
                    },
                    onPrintClick = { showPrinterDialog = true }
                )
            }

            // Status bar
            if (vm.statusMessage.isNotEmpty()) {
                Surface(
                    modifier = Modifier.fillMaxWidth(),
                    color = MaterialTheme.colorScheme.surfaceVariant
                ) {
                    Text(
                        text = vm.statusMessage,
                        modifier = Modifier.padding(8.dp),
                        style = MaterialTheme.typography.bodySmall
                    )
                }
            }
        }
    }

    // Printer selection dialog
    if (showPrinterDialog) {
        AlertDialog(
            onDismissRequest = { showPrinterDialog = false },
            title = { Text("Select Printer") },
            text = {
                Column {
                    if (vm.isLoading) {
                        CircularProgressIndicator()
                        Text("Discovering printers...")
                    } else if (vm.printers.isEmpty()) {
                        Text("No printers found. Make sure Bluetooth is enabled.")
                        Button(onClick = {
                            scope.launch {
                                vm.isLoading = true
                                vm.printers = printer.discoverPrinters()
                                vm.isLoading = false
                            }
                        }) {
                            Text("🔄 Scan Again")
                        }
                    } else {
                        vm.printers.forEach { p ->
                            RadioButton(
                                selected = vm.selectedPrinter == p.address,
                                onClick = { vm.selectedPrinter = p.address }
                            )
                            Text(p.name)
                        }
                    }
                }
            },
            confirmButton = {
                Button(
                    onClick = {
                        scope.launch {
                            vm.selectedPrinter?.let { address ->
                                val zpl = if (selectedTab == 0) {
                                    vm.getRenderedZpl()
                                } else {
                                    vm.template
                                }
                                vm.isLoading = true
                                val result = printer.print(address, zpl)
                                vm.isLoading = false
                                if (result.isSuccess) {
                                    vm.statusMessage = "Printed successfully"
                                    Toast.makeText(context, "Printed!", Toast.LENGTH_SHORT).show()
                                } else {
                                    vm.statusMessage = "Print failed: ${result.exceptionOrNull()?.message}"
                                    Toast.makeText(context, "Print failed", Toast.LENGTH_SHORT).show()
                                }
                                showPrinterDialog = false
                            }
                        }
                    },
                    enabled = vm.selectedPrinter != null && !vm.isLoading
                ) {
                    Text("Print")
                }
            },
            dismissButton = {
                TextButton(onClick = { showPrinterDialog = false }) {
                    Text("Cancel")
                }
            }
        )

        // Auto-discover on dialog open
        LaunchedEffect(showPrinterDialog) {
            if (showPrinterDialog && vm.printers.isEmpty()) {
                vm.isLoading = true
                vm.printers = printer.discoverPrinters()
                vm.isLoading = false
            }
        }
    }
}
