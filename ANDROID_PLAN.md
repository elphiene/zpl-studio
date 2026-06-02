# Android ZPL Printer Tool - Minimal Plan (KISS/DRY/YAGNI)

## Overview
Native Android app in Kotlin using **only what we actually need**. No over-engineering.

## Technology Stack (Minimal)

### Core (Required)
- **Language:** Kotlin
- **UI:** Jetpack Compose (modern, simple, less boilerplate than XML)
- **Printing:** Zebra Link-OS SDK (official, well-documented)
- **Storage:** SharedPreferences for templates (no Room DB needed yet)
- **Regex:** Kotlin stdlib (port our regex from Rust)

### What We're NOT Using (YAGNI)
- ❌ Room Database (SharedPreferences is enough for now)
- ❌ Dependency Injection (Hilt/Koin) - keep it simple
- ❌ Complex architecture patterns (just ViewModels)
- ❌ Retrofit/Networking (offline only)
- ❌ Image loading libraries (we render locally)
- ❌ Analytics, crash reporting (add later if needed)

## Minimal Feature Set

### Phase 1: MVP (Actually Minimal)
1. ✅ Single screen with tabs (Use/Edit)
2. ✅ Load .zpl file from storage
3. ✅ Parse `{{VARIABLES}}` with Kotlin regex
4. ✅ Show form fields for variables
5. ✅ Bluetooth printer selection
6. ✅ Print ZPL via Zebra SDK
7. ✅ Save last used template (SharedPreferences)

### What We're NOT Building Yet (YAGNI)
- ❌ Template library/gallery
- ❌ Cloud sync
- ❌ Template sharing
- ❌ Multiple templates loaded at once
- ❌ Print history
- ❌ Settings screen
- ❌ Dark mode (use system default)
- ❌ Localization (English only for now)

## Simple Architecture

```
MainActivity
├── ViewModel (single file, all logic)
├── UI (Compose screens)
│   ├── UseTab
│   └── EditTab
└── Utilities
    ├── ZplParser.kt (regex for variables)
    ├── TemplateStorage.kt (SharedPreferences wrapper)
    └── ZebraPrinter.kt (SDK wrapper)
```

**That's it.** No repositories, no use cases, no domain layer.

## File Structure (Minimal)

```
app/src/main/java/com/example/zplprinter/
├── MainActivity.kt          # Entry point + ViewModel
├── ui/
│   ├── UseScreen.kt        # Use mode tab
│   ├── EditScreen.kt       # Edit mode tab
│   └── PrinterPicker.kt    # Bluetooth picker dialog
└── utils/
    ├── ZplParser.kt        # Template variable extraction
    ├── TemplateStorage.kt  # Save/load templates
    └── ZebraPrinter.kt     # Print abstraction
```

**6 files total.** That's all we need.

## Code Snippets (Keep It Simple)

### 1. Template Parser (DRY - Reuse our Regex)
```kotlin
// ZplParser.kt - ONE function, that's it
object ZplParser {
    data class Variable(val name: String, val label: String)

    fun parse(zpl: String): List<Variable> {
        val regex = Regex("""\\{\\{([A-Z_][A-Z0-9_]*)(?::([^}]+))?\\}\\}""")
        return regex.findAll(zpl)
            .map { Variable(it.groupValues[1], it.groupValues[2].ifEmpty { it.groupValues[1] }) }
            .distinctBy { it.name }
            .toList()
    }

    fun render(zpl: String, values: Map<String, String>): String {
        var result = zpl
        values.forEach { (key, value) ->
            result = result.replace(Regex("""\\{\\{$key(?::[^}]+)?\\}\\}"""), value)
        }
        return result
    }
}
```

### 2. Simple Storage (KISS)
```kotlin
// TemplateStorage.kt - Just save/load, nothing fancy
class TemplateStorage(private val prefs: SharedPreferences) {
    fun saveTemplate(zpl: String) = prefs.edit().putString("template", zpl).apply()
    fun loadTemplate(): String? = prefs.getString("template", null)
}
```

### 3. Printer Wrapper (One Purpose)
```kotlin
// ZebraPrinter.kt - Thin wrapper over Zebra SDK
class ZebraPrinter {
    fun discoverBluetoothPrinters(): List<String> {
        return BluetoothDiscoverer.findPrinters()
            .map { it.address }
    }

    fun print(address: String, zpl: String): Result<Unit> = runCatching {
        val connection = BluetoothConnection(address)
        connection.open()
        connection.write(zpl.toByteArray())
        connection.close()
    }
}
```

### 4. Single ViewModel (No Over-Engineering)
```kotlin
// MainActivity.kt
class PrinterViewModel : ViewModel() {
    var template by mutableStateOf("")
    var variables by mutableStateOf<List<ZplParser.Variable>>(emptyList())
    var values by mutableStateOf<Map<String, String>>(emptyMap())
    var printers by mutableStateOf<List<String>>(emptyList())

    fun loadTemplate(zpl: String) {
        template = zpl
        variables = ZplParser.parse(zpl)
        values = variables.associate { it.name to "" }
    }

    fun print(printerAddress: String) {
        val rendered = ZplParser.render(template, values)
        ZebraPrinter().print(printerAddress, rendered)
    }
}
```

### 5. Simple Compose UI
```kotlin
// UseScreen.kt - Keep it dead simple
@Composable
fun UseScreen(vm: PrinterViewModel) {
    Column {
        // Variables form
        vm.variables.forEach { variable ->
            OutlinedTextField(
                value = vm.values[variable.name] ?: "",
                onValueChange = { vm.values += (variable.name to it) },
                label = { Text(variable.label) }
            )
        }

        // Print button
        Button(onClick = { vm.print(selectedPrinter) }) {
            Text("Print")
        }
    }
}
```

## Build.gradle (Minimal Dependencies)

```gradle
dependencies {
    // Compose (standard)
    implementation "androidx.compose.ui:ui:1.5.4"
    implementation "androidx.compose.material3:material3:1.1.2"
    implementation "androidx.activity:activity-compose:1.8.0"

    // Zebra SDK (only one we need)
    implementation "com.zebra.sdk:ZSDK_ANDROID_API:3.0.3174"

    // That's it. No Room, no Retrofit, no Hilt.
}
```

## Permissions (Only What's Needed)

```xml
<manifest>
    <!-- Bluetooth only -->
    <uses-permission android:name="android.permission.BLUETOOTH_CONNECT" />
    <uses-permission android:name="android.permission.BLUETOOTH_SCAN" />

    <!-- File access -->
    <uses-permission android:name="android.permission.READ_EXTERNAL_STORAGE" />
</manifest>
```

## Development Timeline (Realistic)

### Week 1: Setup + Parsing
- [ ] Create Android project
- [ ] Add Zebra SDK
- [ ] Port ZplParser.kt (2 functions)
- [ ] Test with example template

### Week 2: UI
- [ ] Build UseScreen (form)
- [ ] Build EditScreen (text input)
- [ ] Add tab navigation
- [ ] File picker integration

### Week 3: Printing
- [ ] Bluetooth discovery
- [ ] Printer selection dialog
- [ ] Test print with real printer
- [ ] Error handling

### Week 4: Polish
- [ ] Template persistence
- [ ] Basic validation
- [ ] Loading states
- [ ] APK optimization

**Total: 4 weeks, one developer.**

## What Makes This KISS/DRY/YAGNI?

### KISS (Keep It Simple)
- ✅ 6 files, not 60
- ✅ One ViewModel, not 10
- ✅ SharedPreferences, not Room
- ✅ Direct SDK calls, no abstraction layers
- ✅ Compose for UI (less code than XML)

### DRY (Don't Repeat Yourself)
- ✅ Reuse regex from Rust implementation
- ✅ Same variable syntax `{{VAR:Label}}`
- ✅ One parser, used everywhere
- ✅ Shared template format with desktop

### YAGNI (You Aren't Gonna Need It)
- ✅ No database (files are fine)
- ✅ No network code (offline only)
- ✅ No complex architecture (ViewModel is enough)
- ✅ No multi-module setup
- ✅ No preview rendering (Zebra SDK has it if needed)

## Comparison: Over-Engineered vs. KISS

| Feature | Over-Engineered ❌ | KISS Approach ✅ |
|---------|-------------------|------------------|
| Storage | Room + DAOs + Entities | SharedPreferences |
| DI | Hilt + Modules | Simple constructors |
| Architecture | Clean Arch (5 layers) | ViewModel only |
| Networking | Retrofit + OkHttp | None (offline) |
| Image Loading | Coil/Glide | Not needed yet |
| Files | 40+ files | 6 files |
| Dependencies | 20+ libraries | 3 libraries |
| Lines of Code | 5000+ | ~500 |

## Testing Strategy (Minimal)

```kotlin
// ZplParserTest.kt - Just test what matters
class ZplParserTest {
    @Test
    fun `parse extracts variables`() {
        val zpl = "^FD{{NAME:Full Name}}^FS"
        val vars = ZplParser.parse(zpl)
        assertEquals(1, vars.size)
        assertEquals("NAME", vars[0].name)
        assertEquals("Full Name", vars[0].label)
    }

    @Test
    fun `render replaces variables`() {
        val zpl = "^FD{{NAME}}^FS"
        val result = ZplParser.render(zpl, mapOf("NAME" to "John"))
        assertEquals("^FDJohn^FS", result)
    }
}
```

That's it. Test the parser. Bluetooth and printing? Test manually with real hardware.

## Launch Checklist

- [ ] Test with Zebra ZQ610 (our target printer)
- [ ] Test on Android 7.0+ (API 24+)
- [ ] APK size < 10MB
- [ ] No crashes on template load
- [ ] Bluetooth pairing works
- [ ] Variables render correctly
- [ ] Autosave works

## What We'll Add Later (If Needed)

**Only add these IF users actually ask:**
- Template gallery (if users want to manage multiple)
- Cloud backup (if users lose templates)
- Print preview (if users complain about waste)
- Print history (if users need to reprint)
- Settings (if defaults don't work)
- Dark mode (if users complain)

**Start simple. Add complexity only when actually needed.**

## Success Criteria

1. ✅ Works with same .zpl files as desktop
2. ✅ Prints to Zebra printers via Bluetooth
3. ✅ APK < 10MB
4. ✅ < 1000 lines of code
5. ✅ No external dependencies except Zebra SDK
6. ✅ Can build and maintain by ONE developer

---

**Bottom line:** We don't need Clean Architecture, we don't need a database, we don't need 20 libraries. We need to load a template, fill in fields, and print. That's it.
