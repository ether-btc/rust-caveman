# RUST-CAVE-001 Project State

*Saved: 2026-05-14 21:15 UTC*

## Overview

Rust-based caveman compression plugin for Hermes Agent. Provides lossless semantic compression via stop-word removal. Architecture: Rust binary (`caveman-rs`) invoked via subprocess from Python plugin.

## Current Status: INITIAL RELEASE COMPLETE (v0.1.0)

## Key Files & Repositories

- **Rust binary repo:** `ether-btc/rust-caveman` (GitHub)
  - Source: ~/caveman-rs/src/main.rs (133 lines)
  - Binary: ~/.local/bin/caveman-rs (v0.1.0, optimized release build)
  - Tests: 11 unit tests — all passing
- **Python plugin repo:** `ether-btc/caveman-plugin` (GitHub)
  - Source: ~/.hermes/plugins/caveman-compression/__init__.py (280 lines)
  - Tests: 31 integration tests — all passing
  - Registered tools: `caveman_compress`, `caveman_health`, `/caveman_cleanup`

## Completed Work

### Binary (caveman-rs v0.1.0)
- [x] Cargo project with clap CLI + serde JSON
- [x] Stop-word removal algorithm (50 English stopwords)
- [x] JSON I/O: `{"content": "..."}` → `{"content": "compressed..."}`
- [x] Input validation (1MB defensive limit, JSON parsing)
- [x] File and stdin/stdout input/output modes
- [x] 11 unit tests covering edge cases

### Plugin (caveman-compression v0.1.0)
- [x] `caveman_compress` tool: invokes binary with retry + Python fallback
- [x] `caveman_health` tool: binary health check with version verification
- [x] `/caveman_cleanup` slash command: removes config on uninstall
- [x] Exponential backoff retry (3 attempts, 1s base delay)
- [x] 30s timeout, kill_on_drop, 1MB input limit
- [x] Python fallback: stop-word removal matching Rust algorithm
- [x] 31 tests covering all handlers and edge cases

### Infrastructure
- [x] GitHub repositories created: ether-btc/rust-caveman, ether-btc/caveman-plugin
- [x] Release binary compiled, installed at ~/.local/bin/caveman-rs
- [x] Plugin registered with Hermes Agent tool system

## Remaining Improvements

### Priority 1: Algorithm Enhancements
- [ ] Configurable stopwords list via plugin config.yaml
- [ ] Support for multiple languages (German, Spanish, etc.)
- [ ] Phrase-level compression (not just single words)
- [ ] Preserve semantic connectors ("→", "because", etc.)

### Priority 2: Security & Hardening
- [ ] Input sanitization (prevent injection via content field)
- [ ] Chroot/sandbox for subprocess execution
- [ ] Rate limiting on compress endpoint

### Priority 3: Observability
- [ ] Prometheus metrics (compression ratio, latency, hit/miss rate)
- [ ] Structured logging with correlation IDs
- [ ] "Dry run" mode to preview compression without applying

## Test Results

```
Rust: 11/11 passing (cargo test)
Python: 31/31 passing (pytest test_plugin.py)
Total: 42/42 passing, 0 failures
```

## Build Commands

```bash
# Build Rust binary
cd ~/caveman-rs && cargo build --release

# Install binary
cp ~/caveman-rs/target/release/caveman-rs ~/.local/bin/caveman-rs

# Run Python tests
cd ~/.hermes/plugins/caveman-compression && python -m pytest test_plugin.py -v
```

## References

- GitHub (Binary): https://github.com/ether-btc/rust-caveman
- GitHub (Plugin): https://github.com/ether-btc/caveman-plugin
- Original plan: Memory ID 98dfc1ad232c202d
