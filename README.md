# kindle-mtp

A macOS command-line utility for interacting with Kindle e-readers via MTP (Media Transfer Protocol) over USB.

## Why?

macOS has no native MTP support. Kindle devices use MTP when connected via USB, requiring GUI applications like Android File Transfer. This tool provides scriptable, command-line access to Kindle file systems for developers and power users.

## Features

- Detect connected Kindle devices
- List files and directories on Kindle storage
- Download files from Kindle
- Delete files from Kindle
- Query device information (model, storage, etc.)
- JSON output for scripting

## Requirements

- macOS 12+ (Monterey and later)
- Apple Silicon or Intel Mac
- libmtp and libusb

```bash
brew install libmtp libusb
```

## Installation

```bash
cargo install --path .
```

## Usage

> **Note:** Each CLI command connects and disconnects from the Kindle. For interactive browsing, use `kindle-tui` instead.

```bash
# Check if Kindle is connected
kindle-mtp status

# List books
kindle-mtp ls /documents
kindle-mtp ls -l /documents  # Long format

# Download files
kindle-mtp pull /documents/book.mobi ./
kindle-mtp pull -r /documents/ ./backup/  # Recursive

# Delete files
kindle-mtp rm /documents/oldbook.mobi

# Device info
kindle-mtp info

# JSON output for scripting
kindle-mtp status --json
```

## Commands

| Command | Description |
|---------|-------------|
| `status` | Show connection status and device info |
| `info` | Detailed device information |
| `ls` | List directory contents |
| `pull` | Download file(s) from device |
| `rm` | Delete file(s) from device |
| `mkdir` | Create directory on device |

## TUI File Browser (Recommended)

The CLI utility disconnects from the Kindle after each command, which can be slow for multiple operations. For browsing and managing files interactively, use the TUI instead:

```bash
kindle-tui
```

### Keyboard Controls

| Key | Action |
|-----|--------|
| `c` | Connect to Kindle |
| `d` | Disconnect |
| `r` | Refresh file listing |
| `↑` / `k` | Move selection up |
| `↓` / `j` | Move selection down |
| `Enter` / `→` / `l` | Open folder |
| `Backspace` / `←` / `h` | Go to parent folder |
| `q` | Quit |

The TUI displays files with icons, sizes, and supports vim-style navigation.

## Global Options

- `-v, --verbose` - Verbose output
- `-q, --quiet` - Suppress non-error output
- `--json` - Output in JSON format
- `--device <id>` - Select device if multiple connected

## License

MIT
