# ADR-001: MTP Library Choice

**Status**: Accepted (validated by spike 2026-01-13)
**Date**: 2026-01-13
**Decision**: Use libmtp-rs (Rust bindings to libmtp)

## Context

We need MTP (Media Transfer Protocol) support to communicate with Kindle devices over USB on macOS. macOS has no native MTP support, so we require a third-party library.

## Options Considered

### Option 1: libmtp-rs (Rust bindings to C libmtp)

**Description**: High-level Rust bindings wrapping the established C libmtp library.

| Pros | Cons |
|------|------|
| Active development (updated Oct 2025) | Alpha status |
| Core features implemented (detect, list, send, receive, delete) | Some features incomplete (playlists, thumbnails) |
| Wraps battle-tested C library | Requires system dependency (libmtp via Homebrew) |
| Cross-platform (macOS, Linux) | ~40% documentation coverage |
| Safe Rust API | |

**Repository**: https://github.com/quebin31/libmtp-rs
**Crates.io**: https://crates.io/crates/libmtp-rs
**Version**: 0.7.x

### Option 2: libmtp-sys (Low-level FFI)

**Description**: Raw FFI bindings to libmtp C library.

| Pros | Cons |
|------|------|
| Direct access to all libmtp functions | Unsafe Rust, requires manual memory management |
| Stable, minimal abstraction | No idiomatic Rust API |
| Used by libmtp-rs internally | Steep learning curve |

### Option 3: Pure Rust Implementation (jeandudey/libmtp)

**Description**: An in-progress port of libmtp to pure Rust.

| Pros | Cons |
|------|------|
| No C dependencies | Not production-ready (explicitly stated) |
| Pure Rust safety | Only ~5% Rust code completed |
| | No timeline for completion |

**Repository**: https://github.com/jeandudey/libmtp

### Option 4: winmtp (Windows only)

**Description**: Rust wrapper over Windows WPD/MTP API.

| Pros | Cons |
|------|------|
| Native Windows integration | Windows only - **not viable for macOS target** |

### Option 5: Custom Implementation

**Description**: Build MTP protocol support from scratch using rusb (Rust libusb bindings).

| Pros | Cons |
|------|------|
| No external dependencies | Massive effort (MTP spec is complex) |
| Full control | No existing reference implementation |
| | Months of development for questionable benefit |

## Decision

**Use libmtp-rs (Option 1)**

## Rationale

1. **Feature completeness**: libmtp-rs implements all operations we need:
   - Device detection (`detect_raw_devices`)
   - Device info (`manufacturer_name`, `model_name`, `serial_number`)
   - Storage access (`Storage`, `StoragePool`)
   - File operations (list, send, receive, delete)
   - Folder operations (list, create)

2. **Proven foundation**: Wraps libmtp, which has 15+ years of development and wide device compatibility.

3. **macOS support**: No documented issues with macOS. libmtp works on Unix-like systems.

4. **Active maintenance**: Recent commits (Oct 2025), responsive maintainer.

5. **Acceptable risk**: Alpha status is a concern, but:
   - Our use case covers well-implemented features
   - We can contribute fixes if needed
   - Fallback to libmtp-sys exists if blocking issues arise

## Consequences

### Positive
- Fast path to working implementation
- Benefit from libmtp's device quirk handling
- Idiomatic Rust API

### Negative
- System dependency on libmtp (requires `brew install libmtp`)
- Alpha status may mean API changes
- Depends on C library maintenance

### Risks to Mitigate
- **Spike validation**: ✅ Completed - libmtp-rs compiles and links correctly on macOS Apple Silicon
- **Pin dependency version**: Lock libmtp-rs version in Cargo.toml
- **Document workarounds**: If we hit libmtp-rs bugs, document them for future reference

## Validation (2026-01-13)

**Full spike validation completed with physical Kindle device.**

Spike executed successfully:
- libmtp-rs 0.7.7 compiles on macOS Sequoia (Apple Silicon)
- System libmtp 1.1.22 links correctly via pkg-config
- Device detection API works (returns expected error when no device attached)
- No USB permission issues encountered

**Device testing:**
- Kindle Scribe 32GB detected (vendor=0x1949, product=0x9981)
- Device info retrieved: Manufacturer "Amazon", Model "Kindle Paperwhite Signature Edition"
- Storage enumerated: 27.34 GB total, 26.09 GB free
- Root folder listing successful: documents, fonts, audible, voice, etc.

API quirks discovered during spike (documented in architecture.md):
- Some method names differ from docs (e.g., `get_friendly_name()` not `friendly_name()`)
- `Filetype` enum doesn't implement `PartialEq`
- Return types differ in some cases (`Option` vs `Result`)

## Alternatives Rejected

- **libmtp-sys**: Too low-level, would require significant unsafe code
- **Pure Rust port**: Not ready, no ETA
- **winmtp**: Wrong platform
- **Custom implementation**: Disproportionate effort vs. benefit

## References

- [libmtp-rs documentation](https://docs.rs/libmtp-rs/latest/libmtp_rs/)
- [libmtp-rs GitHub](https://github.com/quebin31/libmtp-rs)
- [libmtp C library](http://libmtp.sourceforge.net/)
- [MTP specification](https://en.wikipedia.org/wiki/Media_Transfer_Protocol)
