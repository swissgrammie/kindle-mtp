# Specification: kindle-mtp

## Overview
A macOS command-line utility for interacting with Kindle e-readers via MTP (Media Transfer Protocol) over USB. Enables file transfer, library management, and device queries without requiring Android File Transfer or other GUI tools.

## Problem Statement
macOS has no native MTP support. Kindle devices use MTP when connected via USB. Current solutions require GUI applications or are unreliable. Developers and power users need scriptable, command-line access to Kindle file systems.

## Goals
- [ ] Detect connected Kindle devices
- [ ] List files and directories on Kindle storage
- [ ] Download files from Kindle
- [ ] Delete files from Kindle
- [ ] Query device information (model, storage capacity, etc.)

## Non-Goals
- Converting ebook formats (use Calibre for that)
- Managing Kindle collections/metadata database
- Wireless transfer (USB only)
- Supporting non-Kindle MTP devices (initially)

## User Stories

### US-1: Device Detection
As a user, I want to check if my Kindle is connected and recognized, so I can verify the connection before transferring files.

```bash
kindle-mtp status
# Output: Kindle Paperwhite (5th Gen) connected - 2.8GB free of 4GB
```

### US-2: List Contents
As a user, I want to list the contents of my Kindle, so I can see what books are already on the device.

```bash
kindle-mtp ls /documents
kindle-mtp ls -l /documents  # Long format with sizes/dates
```


### US-4: Download Files
As a user, I want to download files from my Kindle, so I can back them up or transfer to another device.

```bash
kindle-mtp pull /documents/mybook.mobi ./
kindle-mtp pull -r /documents/ ./kindle-backup/  # Recursive
```

### US-5: Delete Files
As a user, I want to delete files from my Kindle, so I can free up space.

```bash
kindle-mtp rm /documents/oldbook.mobi
```

### US-6: Device Info
As a user, I want to query device details, so I can verify I'm working with the right device.

```bash
kindle-mtp info
# Output:
#   Device: Kindle Paperwhite
#   Serial: G000XXXX
#   Storage: Internal (4GB)
#   Free: 2.8GB
#   Firmware: 5.x.x
```

## Technical Requirements

### Platform
- macOS 12+ (Monterey and later)
- Apple Silicon support

### Language Options 

Use **Rust** - Performance, safety, good libmtp bindings exist


### Key Dependencies
- **libmtp** - The de facto MTP library (C library)
  - Install: `brew install libmtp`
  - Provides: device detection, file operations, metadata
- **libusb** - USB access (libmtp dependency)
  - Install: `brew install libusb`

### Kindle-Specific Considerations
- Kindle uses MTP but with some quirks
- Device presents as "Internal Storage" 
- Books typically in `/documents/` directory
- Supported formats: .mobi, .azw, .azw3, .pdf, .txt
- May need USB vendor/product ID filtering for Kindle detection

### Known Kindle USB IDs (reference)
```
Amazon Kindle:
  Vendor ID:  0x1949 (Amazon)
  Product IDs vary by model:
    - Basic: 0x0004
    - Paperwhite: 0x0005 (varies by gen)
    - Oasis: 0x0006 (varies)
    - Scribe: [needs research]
```

## Interface Design

### CLI Structure
```
kindle-mtp <command> [options] [arguments]

Commands:
  status    Show connection status and device info
  info      Detailed device information
  ls        List directory contents
  pull      Download file(s) from device
  rm        Delete file(s) from device
  mkdir     Create directory on device
  help      Show help for a command

Global Options:
  -v, --verbose    Verbose output
  -q, --quiet      Suppress non-error output
  --json           Output in JSON format (for scripting)
  --device <id>    Select device if multiple connected
```

### Exit Codes
- 0: Success
- 1: General error
- 2: Device not found
- 3: File not found
- 4: Permission denied
- 5: Storage full
- 6: Transfer failed

### Output Formats
Default: Human-readable
`--json`: Machine-parseable JSON for scripting

## Error Handling

### Common Errors
1. **No device found** - Clear message, suggest checking USB connection
2. **Permission denied** - May need to unlock Kindle or trust computer
3. **File exists** - Prompt or use `--force` flag
4. **Storage full** - Show available space, suggest cleanup
5. **Unsupported format** - Warn but allow (Kindle may still accept)

## Success Criteria
- [ ] Detects Kindle devices reliably on macOS
- [ ] Can list files on connected Kindle
- [ ] Can download a file from Kindle
- [ ] Handles device disconnect gracefully
- [ ] Works on both Intel and Apple Silicon Macs
- [ ] CLI is intuitive (follows common patterns like `adb`, `scp`)


## References
- [libmtp documentation](http://libmtp.sourceforge.net/)
- [MTP specification](https://en.wikipedia.org/wiki/Media_Transfer_Protocol)
- [Kindle supported formats](https://www.amazon.com/gp/help/customer/display.html?nodeId=G5WYD9SAF7PGXRNA)
- [pymtp](https://github.com/emdete/pymtp) - Python bindings (reference)
- [go-mtpfs](https://github.com/hanwen/go-mtpfs) - Go MTP filesystem (reference)

## Appendix: Research Notes

### libmtp Basic Flow
```c
// Pseudocode for typical libmtp usage
LIBMTP_Init();
device = LIBMTP_Get_First_Device();
LIBMTP_Get_Storage(device, LIBMTP_STORAGE_SORTBY_NOTSORTED);
files = LIBMTP_Get_Files_And_Folders(device, storage_id, parent_id);
LIBMTP_Send_File_From_File(device, local_path, remote_path, ...);
LIBMTP_Release_Device(device);
```

### macOS USB Permissions
- May need entitlements for USB access in sandboxed apps
- CLI tool without sandbox should work with libusb
- User may need to "Trust" computer on Kindle (if applicable)
