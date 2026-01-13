# Architecture: kindle-mtp

## Overview

A Rust CLI application for managing Kindle e-reader files via MTP over USB on macOS.

```
┌─────────────────────────────────────────────────────────────┐
│                        CLI Layer                            │
│  (clap argument parsing, output formatting)                 │
├─────────────────────────────────────────────────────────────┤
│                      Command Layer                          │
│  status │ info │ ls │ pull │ rm │ mkdir                     │
├─────────────────────────────────────────────────────────────┤
│                      Device Layer                           │
│  Kindle detection, device abstraction, storage access       │
├─────────────────────────────────────────────────────────────┤
│                    libmtp-rs (Rust bindings)                │
├─────────────────────────────────────────────────────────────┤
│                      libmtp (C library)                     │
├─────────────────────────────────────────────────────────────┤
│                      libusb (USB access)                    │
└─────────────────────────────────────────────────────────────┘
```

## Module Structure

```
src/
├── main.rs              # Entry point
├── lib.rs               # Library exports for testing
├── cli/
│   ├── mod.rs           # CLI module
│   ├── args.rs          # Argument definitions (clap derive)
│   └── output.rs        # Human/JSON output formatting
├── commands/
│   ├── mod.rs           # Command dispatcher
│   ├── status.rs        # Connection status check
│   ├── info.rs          # Detailed device info
│   ├── ls.rs            # List directory contents
│   ├── pull.rs          # Download files
│   ├── rm.rs            # Delete files
│   └── mkdir.rs         # Create directory
├── device/
│   ├── mod.rs           # Device abstraction
│   ├── detect.rs        # MTP device detection
│   ├── kindle.rs        # Kindle-specific vendor/product IDs
│   └── storage.rs       # Storage enumeration
├── fs/
│   ├── mod.rs           # Filesystem operations
│   ├── entry.rs         # File/folder entry types
│   └── path.rs          # MTP path handling
└── error.rs             # Error types, exit codes
```

## Key Design Decisions

### 1. Library Choice: libmtp-rs

See [ADR-001: MTP Library Choice](decisions/001-mtp-library-choice.md)

### 2. Kindle Detection Strategy

Filter MTP devices by Amazon vendor ID (0x1949). Maintain a known product ID list but allow any Amazon device to connect (future-proofing for new Kindle models).

```rust
const AMAZON_VENDOR_ID: u16 = 0x1949;

fn is_kindle(vendor_id: u16) -> bool {
    vendor_id == AMAZON_VENDOR_ID
}
```

### 3. Error Handling

Use `thiserror` for error definitions with explicit exit codes:

```rust
#[derive(Debug, thiserror::Error)]
pub enum KindleError {
    #[error("No Kindle device found")]
    DeviceNotFound,          // Exit code 2

    #[error("File not found: {0}")]
    FileNotFound(String),    // Exit code 3

    #[error("Permission denied")]
    PermissionDenied,        // Exit code 4

    #[error("Storage full")]
    StorageFull,             // Exit code 5

    #[error("Transfer failed: {0}")]
    TransferFailed(String),  // Exit code 6
}

impl KindleError {
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::DeviceNotFound => 2,
            Self::FileNotFound(_) => 3,
            Self::PermissionDenied => 4,
            Self::StorageFull => 5,
            Self::TransferFailed(_) => 6,
        }
    }
}
```

### 4. Output Formatting

Support both human-readable and JSON output via a trait:

```rust
pub trait Outputable {
    fn to_human(&self) -> String;
    fn to_json(&self) -> serde_json::Value;
}

pub fn print_output<T: Outputable>(item: &T, json: bool) {
    if json {
        println!("{}", serde_json::to_string_pretty(&item.to_json()).unwrap());
    } else {
        println!("{}", item.to_human());
    }
}
```

### 5. Path Handling

MTP doesn't use traditional filesystem paths. Objects have numeric IDs and parent relationships. We'll provide a path-like interface that translates to MTP object traversal:

```rust
// User sees:      kindle-mtp ls /documents/books/
// Internal:       find_object_by_path("/documents/books/")
//                 -> traverse: root -> "documents" -> "books"
//                 -> returns object_id for "books" folder
```

## Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| libmtp-rs | 0.7 | MTP device communication |
| clap | 4.x | CLI argument parsing |
| serde | 1.x | Serialization for JSON output |
| serde_json | 1.x | JSON formatting |
| thiserror | 1.x | Error type definitions |

## Build Requirements

System dependencies (via Homebrew):
```bash
brew install libmtp libusb pkg-config
```

## Spike Results (2026-01-13)

Executed validation spike to test libmtp-rs on macOS.

### Results

| Goal | Status | Notes |
|------|--------|-------|
| libmtp-rs compiles on macOS | **PASS** | Compiled on Apple Silicon (arm64) |
| libmtp links correctly | **PASS** | System libmtp 1.1.22 via Homebrew |
| Device detection works | **PASS** | Returns `NoDeviceAttached` when no device |
| Kindle detected | **PASS** | Kindle Scribe 32GB (0x1949:0x9981) |
| Read device info | **PASS** | Manufacturer, model, friendly name all work |
| List files | **PASS** | Root folder contents listed successfully |

### Device Tested

- **Hardware**: Kindle Scribe 32GB (USB product ID 0x9981)
- **Reported Model**: Kindle Paperwhite Signature Edition (firmware quirk?)
- **Storage**: 27.34 GB total, 26.09 GB free
- **Root folders**: documents, fonts, audible, voice, screenshots, system, .bcache

### Key Findings

1. **libmtp-rs API differs from documentation** - Several method names differ:
   - `friendly_name()` → `get_friendly_name()`
   - `max_capacity()` → `maximum_capacity()`
   - `free_space()` → `free_space_in_bytes()`
   - `open_uncached()` returns `Option`, not `Result`
   - `storage_pool()` returns `StoragePool` directly
   - `Filetype` doesn't implement `PartialEq` (use `matches!` macro)

2. **No USB permission prompts** - On macOS Sequoia, no special permissions were required to access libusb

3. **System dependencies work** - `brew install libmtp` installs libmtp 1.1.22 and libusb 1.0.29

### Recommendation

**Proceed with implementation.** All spike goals achieved. The technical approach is fully validated.

---

## Risk Areas

### Primary Risk: Kindle/macOS MTP Compatibility

**Concern**: libmtp-rs is alpha software, and Kindle devices may have MTP quirks.

**Status**: Partially mitigated by spike (see above).

**Remaining validation**:
1. Detect connected Kindle device
2. Read device info (manufacturer, model, serial)
3. List files in `/documents/`
4. Download a single file

### Secondary Risk: USB Permissions on macOS

**Concern**: Modern macOS may require special entitlements or permissions for USB access.

**Mitigation**:
- Test as unsigned CLI tool first
- Document any required permissions/prompts
- Consider code signing if needed for distribution

### Tertiary Risk: Large File Transfers

**Concern**: Progress reporting and error handling for large files.

**Mitigation**:
- Implement progress callbacks if libmtp-rs supports them
- Add transfer resumption if feasible
- Clear error messages for interrupted transfers

## Testing Strategy

### Unit Tests
- Path parsing logic
- Output formatting
- Error code mapping
- Kindle ID detection

### Integration Tests (require device)
- Device detection
- File listing
- File download
- File deletion

### Manual Testing Checklist
- [ ] Intel Mac
- [ ] Apple Silicon Mac
- [ ] Different Kindle models (if available)
- [ ] Large files (>100MB)
- [ ] Special characters in filenames
- [ ] Device disconnect during operation

## Known Limitations

### 1. Kindle Disconnects After Each Command

**Issue**: On macOS, the Kindle USB connection resets after each command completes.

**Cause**: When libmtp releases the MTP device handle, macOS resets the USB port. This is a limitation of how MTP works on macOS - the OS doesn't maintain persistent MTP connections like it does for mass storage devices.

**Workaround**: Reconnect the Kindle between commands, or unplug and replug the USB cable.

**Status**: Cannot be fixed in software - this is a macOS/MTP limitation.

### 2. libmtp Debug Output

**Issue**: libmtp prints "Device 0 (VID=xxxx and PID=xxxx) is a..." to stdout during device detection.

**Cause**: This is hardcoded in libmtp's device detection code, not debug output that can be suppressed via environment variables.

**Workaround**: Redirect stdout if needed: `kindle-mtp status 2>&1 | grep -v "^Device 0"`

### 3. Read-Only Mode

**Issue**: Write operations (mkdir, rm, push) are not implemented.

**Reason**: User requested read-only mode to prevent accidental data loss on the Kindle.

## Future Considerations

Not in scope for v1, but worth noting:

1. **Upload support** (`push` command) - Listed in spec but not in initial goals
2. **Recursive download** - `pull -r` for backing up entire directories
3. **Multi-device support** - `--device` flag for multiple Kindles
4. **Progress bars** - Better UX for large transfers
5. **Completion scripts** - Bash/Zsh completions
