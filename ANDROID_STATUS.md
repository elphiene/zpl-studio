# Android Implementation Status

## Current Status: **Ready for Testing (but on hold)**

The Android version of ZPL Printer Tool has been implemented following KISS/DRY/YAGNI principles. The implementation is complete and builds successfully, but is set aside for future testing.

## What's Built

### Core Files (6 files)
1. **ZplParser.kt** - Template parsing with flexible variable regex
2. **TemplateStorage.kt** - SharedPreferences wrapper for template persistence
3. **ZebraPrinter.kt** - Basic Bluetooth printer implementation
4. **MainActivity.kt** - Single ViewModel with tab navigation
5. **UseScreen.kt** - Dynamic form generation from template variables
6. **EditScreen.kt** - Scrollable ZPL code editor

### Key Features
- ✅ Template loading via file picker (.zpl files)
- ✅ Variable parsing: `{{Detail}}`, `{{Name:Full Name}}`, `{{Tracking ID}}`, etc.
- ✅ Flexible regex: Supports uppercase, lowercase, spaces, special characters
- ✅ Autosave to SharedPreferences
- ✅ Bluetooth printer discovery (paired devices)
- ✅ Basic Bluetooth printing via SPP (Serial Port Profile)
- ✅ Same template format as desktop version (DRY)
- ✅ Jetpack Compose UI with Material 3
- ✅ Targets Android 7.0+ (API 24)

### Build Status
- ✅ Compiles successfully
- ✅ All Gradle dependencies resolved
- ✅ GitHub Actions workflow configured
- ✅ APK artifact generation ready

## Architecture

### KISS (Keep It Simple, Stupid)
- **Single ViewModel**: No complex MVVM layers, repositories, or use cases
- **No DI framework**: Direct instantiation, no Hilt/Dagger overhead
- **6 core files**: Target ~1000 LOC vs typical 5000+ LOC enterprise apps

### DRY (Don't Repeat Yourself)
- **Same regex pattern**: Desktop and Android use identical variable parsing
- **Unified template format**: `.zpl` files work across all platforms
- **Shared logic**: Template parsing, rendering duplicated but consistent

### YAGNI (You Aren't Gonna Need It)
- **SharedPreferences**: Not Room database - simple key-value storage
- **Basic Bluetooth**: No Zebra SDK dependency (see below)
- **No analytics**: No Firebase, crash reporting, or telemetry
- **No networking**: No API calls, sync, or cloud features

## Printing Implementation

### Current: Basic Bluetooth SPP
The app uses standard Android Bluetooth APIs with Serial Port Profile:
- **Pros**: No external dependencies, works with most Zebra printers, simple
- **Cons**: Limited to paired devices, no printer status feedback, no advanced features

### Optional: Zebra Link-OS SDK
For production use with advanced features, integrate the official SDK:

1. **Download SDK**
   - Go to https://www.zebra.com/us/en/support-downloads/software/developer-tools/link-os-sdk.html
   - Download `ZSDK_ANDROID_API.jar` (latest version)

2. **Add to Project**
   ```bash
   mkdir -p android/app/libs
   cp ZSDK_ANDROID_API.jar android/app/libs/
   ```

3. **Update build.gradle.kts**
   ```kotlin
   dependencies {
       // ... existing dependencies
       implementation(files("libs/ZSDK_ANDROID_API.jar"))
   }
   ```

4. **Replace ZebraPrinter.kt**
   - Check git history for full SDK implementation
   - Commit `71ca176` has the original Zebra SDK code
   - Replace stub with full implementation

### Zebra SDK Features
The full SDK provides:
- Printer discovery over Wi-Fi and Bluetooth
- Printer status monitoring (paper out, etc.)
- Configuration management
- Firmware updates
- Better error handling

## Variable Syntax (Updated)

The regex pattern was updated to be more flexible:

### Old Pattern (Too Restrictive)
```regex
\{\{([A-Z_][A-Z0-9_]*)(?::([^}]+))?\}\}
```
Only matched: `{{NAME}}`, `{{TRACKING_ID}}`

### New Pattern (Flexible)
```regex
\{\{([^:}]+?)(?::([^}]+))?\}\}
```
Matches any characters except `:` and `}`

### Examples
- `{{NAME}}` - Simple uppercase
- `{{Detail}}` - Mixed case
- `{{Cherrys Labs Tracking ID}}` - Spaces and mixed case
- `{{NAME:Full Name}}` - With display label
- `{{tracking-id}}` - Lowercase with hyphen

## Testing Instructions

### Prerequisites
1. Android device running Android 7.0+ (API 24)
2. Zebra Bluetooth printer paired with the device
3. `.zpl` template file to test with

### Steps
1. **Download APK** from GitHub Actions artifacts
2. **Install APK** on device (enable "Install from unknown sources")
3. **Open app** - should see Use/Edit tabs
4. **Load template** - tap "Load Template" button
5. **Check variables** - form fields should appear for each variable
6. **Fill in values** - enter test data
7. **Select printer** - tap printer icon, choose paired Zebra printer
8. **Print** - tap Print button, check printer output

### Expected Behavior
- Template loads successfully
- Variables parsed correctly (including spaces/lowercase)
- Form generates input fields dynamically
- Edit mode shows scrollable ZPL code
- Printer discovery shows paired devices
- Print sends ZPL via Bluetooth

### Troubleshooting
- **No printers found**: Ensure Bluetooth printer is paired in Android settings first
- **Print fails**: Check Bluetooth permissions are granted
- **Variables not parsed**: Check template uses `{{variable}}` syntax
- **App crashes**: Check Android version is 7.0+

## Future Enhancements (Not Yet Needed)

Following YAGNI, these features were intentionally excluded but could be added:

1. **Template Library**: Browse/download templates from cloud
2. **Print History**: Track what was printed and when
3. **Multi-printer Support**: Save favorite printers
4. **QR Code Generation**: Generate QR codes in templates
5. **Image Integration**: Add photos/logos to labels
6. **Batch Printing**: Print multiple labels at once
7. **Template Variables Export**: Share templates with variables
8. **Cloud Sync**: Sync templates across devices

## Known Limitations

1. **No Wi-Fi Printing**: Only Bluetooth (upgrade to Zebra SDK for Wi-Fi)
2. **No Printer Status**: Can't detect paper out, low battery, etc.
3. **No Preview**: Desktop has ZPL preview, Android doesn't
4. **Paired Devices Only**: Must pair printer in Android settings first
5. **No Template Validation**: Doesn't check ZPL syntax before printing

## Build Artifacts

When GitHub Actions runs, it generates:
- **Artifact Name**: `zpl-printer-android-{commit-sha}`
- **Contents**: `app-release.apk`
- **Retention**: 30 days
- **Size**: ~3-5 MB

## Git History

Key commits for Android implementation:
- `71ca176` - Initial Android with Zebra SDK
- `5f4793a` - Added gradle.properties (AndroidX fix)
- `619cfe2` - Replaced Zebra SDK with basic Bluetooth
- `[current]` - Updated regex for flexible variable names

## Next Steps (When Ready to Continue)

1. **Test on real device** with Zebra printer
2. **Verify variable parsing** with various templates
3. **Test Bluetooth printing** end-to-end
4. **Consider Zebra SDK** if advanced features needed
5. **Add to README** as available platform
6. **Create release** with Windows, Linux, and Android builds

## Contact & Support

For issues with Android implementation:
- Check this document first
- Review git commit history for implementation details
- Test with basic Bluetooth SPP on standard Zebra printers
- Consider Zebra SDK for production features
