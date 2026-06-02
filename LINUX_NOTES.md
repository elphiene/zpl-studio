# Linux Support Notes

## CUPS Configuration

ZPL Printer Tool uses CUPS (Common Unix Printing System) on Linux. CUPS is the standard printing system for most Linux distributions.

### Prerequisites

Make sure CUPS is installed and running:
```bash
# Check if CUPS is installed
sudo systemctl status cups

# Install CUPS if needed (Ubuntu/Debian)
sudo apt-get install cups libcups2-dev

# Install CUPS if needed (Fedora/RHEL)
sudo dnf install cups cups-devel
```

### Zebra Printer Setup

To use a Zebra thermal printer with CUPS:

1. **Install Raw Queue Driver**:
   ```bash
   sudo apt-get install printer-driver-escpr
   ```

2. **Add Printer via CUPS Web Interface**:
   - Navigate to `http://localhost:631`
   - Go to Administration → Add Printer
   - Select your Zebra printer (USB or network)
   - Choose "Raw Queue" as the driver/PPD
   - Set defaults as needed

3. **Or use lpadmin command line**:
   ```bash
   # For USB printer
   sudo lpadmin -p ZebraZPL -v usb://Zebra/ZTC%20ZD420-203dpi%20ZPL -E -m raw

   # For network printer
   sudo lpadmin -p ZebraZPL -v socket://192.168.1.100:9100 -E -m raw

   # Set as default
   sudo lpadmin -d ZebraZPL
   ```

### Raw Printing Mode

ZPL printers MUST be configured as "Raw Queue" printers to accept ZPL commands directly without CUPS processing the data.

**Important**: Standard print drivers will not work with ZPL. The printer must be configured to accept raw data streams.

### Troubleshooting

**Printer not listed?**
- Ensure CUPS is running: `systemctl status cups`
- Check printer connection: `lpstat -p -d`
- Verify printer is set to raw mode: `lpadmin -p <printer> -o printer-op-policy=default`

**Permission denied?**
- Add your user to the lpadmin group:
  ```bash
  sudo usermod -a -G lpadmin $USER
  # Log out and back in for changes to take effect
  ```

**Print job not executing?**
- Check CUPS error log: `sudo tail -f /var/log/cups/error_log`
- Verify raw printing is allowed: edit `/etc/cups/cups-browsed.conf` and add `CreateIPPPrinterQueues All`

### AppImage Usage

The Linux build is distributed as an AppImage:

```bash
# Make executable
chmod +x ZPL_Printer_Tool-*.AppImage

# Run directly
./ZPL_Printer_Tool-*.AppImage

# Or integrate with system
./ZPL_Printer_Tool-*.AppImage --appimage-extract
sudo mv squashfs-root /opt/zpl-printer
sudo ln -s /opt/zpl-printer/AppRun /usr/local/bin/zpl-printer
```

### Known Limitations

1. **Network Discovery**: CUPS network printer discovery may be slower than Windows
2. **Driver Compatibility**: Some proprietary Zebra drivers are Windows-only
3. **Configuration**: Requires manual CUPS setup (no auto-detection of raw queues)

### Recommended Setup

For best results on Linux:
- Use USB connection for simplicity
- Configure printer as raw queue before using the tool
- Test with simple ZPL: `echo "^XA^FDTest^FS^XZ" | lp -d <printer-name>`
