package com.zplprinter.utils

import android.bluetooth.BluetoothAdapter
import android.bluetooth.BluetoothDevice
import android.bluetooth.BluetoothSocket
import android.util.Log
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import java.util.UUID

/**
 * Zebra printer wrapper - Basic Bluetooth implementation
 *
 * PRODUCTION NOTE: For production use, integrate Zebra Link-OS SDK:
 * 1. Download ZSDK_ANDROID_API.jar from Zebra's developer portal
 * 2. Add to android/app/libs/ directory
 * 3. Add to build.gradle.kts: implementation(files("libs/ZSDK_ANDROID_API.jar"))
 * 4. Replace this stub with full SDK implementation (see git history)
 *
 * This stub uses basic Bluetooth SPP (Serial Port Profile) for testing.
 */
class ZebraPrinter {

    data class PrinterInfo(
        val name: String,
        val address: String
    )

    /**
     * Discover paired Bluetooth printers
     * Returns already-paired devices that might be printers
     */
    suspend fun discoverPrinters(): List<PrinterInfo> = withContext(Dispatchers.IO) {
        try {
            val bluetoothAdapter = BluetoothAdapter.getDefaultAdapter()
            if (bluetoothAdapter == null || !bluetoothAdapter.isEnabled) {
                return@withContext emptyList()
            }

            // Get paired devices - user must pair printer first
            val pairedDevices = bluetoothAdapter.bondedDevices
            pairedDevices
                .filter { device ->
                    // Filter for likely printer devices
                    device.name?.contains("zebra", ignoreCase = true) == true ||
                    device.name?.contains("printer", ignoreCase = true) == true ||
                    device.bluetoothClass?.majorDeviceClass == 0x0600 // Imaging class
                }
                .map { device ->
                    PrinterInfo(
                        name = device.name ?: "Unknown Device",
                        address = device.address
                    )
                }
        } catch (e: SecurityException) {
            Log.e("ZebraPrinter", "Bluetooth permission denied", e)
            emptyList()
        } catch (e: Exception) {
            Log.e("ZebraPrinter", "Failed to discover printers", e)
            emptyList()
        }
    }

    /**
     * Print ZPL to Bluetooth printer using SPP
     * Uses standard Bluetooth Serial Port Profile (UUID: 00001101-0000-1000-8000-00805F9B34FB)
     */
    suspend fun print(address: String, zpl: String): Result<Unit> = withContext(Dispatchers.IO) {
        var socket: BluetoothSocket? = null
        try {
            val bluetoothAdapter = BluetoothAdapter.getDefaultAdapter()
                ?: return@withContext Result.failure(Exception("Bluetooth not available"))

            val device: BluetoothDevice = bluetoothAdapter.getRemoteDevice(address)

            // Standard SPP UUID for serial communication
            val uuid = UUID.fromString("00001101-0000-1000-8000-00805F9B34FB")

            socket = device.createRfcommSocketToServiceRecord(uuid)
            socket.connect()

            // Send ZPL data
            socket.outputStream.write(zpl.toByteArray())
            socket.outputStream.flush()

            Log.i("ZebraPrinter", "Successfully sent ${zpl.length} bytes to printer")
            Result.success(Unit)
        } catch (e: SecurityException) {
            Log.e("ZebraPrinter", "Bluetooth permission denied", e)
            Result.failure(Exception("Bluetooth permission denied. Grant Bluetooth permissions."))
        } catch (e: Exception) {
            Log.e("ZebraPrinter", "Print failed", e)
            Result.failure(Exception("Failed to print: ${e.message}"))
        } finally {
            try {
                socket?.close()
            } catch (e: Exception) {
                Log.e("ZebraPrinter", "Failed to close socket", e)
            }
        }
    }
}
