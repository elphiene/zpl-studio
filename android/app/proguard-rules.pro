# Add project specific ProGuard rules here.
# Keep Zebra SDK classes
-keep class com.zebra.sdk.** { *; }
-dontwarn com.zebra.sdk.**

# Keep Compose runtime
-keep class androidx.compose.** { *; }

# Keep data classes used with Compose
-keepclassmembers class ** {
    @androidx.compose.runtime.Stable *;
}
