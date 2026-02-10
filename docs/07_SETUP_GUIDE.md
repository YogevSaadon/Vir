# Vassnian — Developer Setup Guide

## What you need to install

### 1. Rust (the language + compiler + package manager)
Everything comes in one install. No separate IDE or editor needed.

**Install Rust:**
```bash
# Mac / Linux — run this in terminal:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Windows — download and run:
# https://rustup.rs (click the big button)
```

This gives you:
- `rustc` — the Rust compiler
- `cargo` — the build tool + package manager (like npm for JavaScript)
- `rustup` — manages Rust versions

**Verify it works:**
```bash
rustc --version
cargo --version
```

### 2. A code editor (pick one)

| Editor | Why | Install |
|--------|-----|---------|
| **VS Code** (recommended) | Free, great Rust support | https://code.visualstudio.com |
| **RustRover** | JetBrains IDE for Rust | https://www.jetbrains.com/rust |
| **Zed** | Fast, built-in Rust support | https://zed.dev |

**If using VS Code**, install these extensions:
- `rust-analyzer` — autocomplete, errors, go-to-definition (ESSENTIAL)
- `Even Better TOML` — for Cargo.toml files
- `CodeLLDB` — for debugging

### 3. That's it for MVP!

Macroquad is NOT a separate download. Cargo downloads it automatically when you build.

```toml
[dependencies]
macroquad = "0.4"
```

**No engine download. No installer. No editor. Just `cargo build`.**

---

## How to build and run

```bash
cd vassnian/
cargo run
# First time takes 1-2 min (downloads deps). After that, seconds.
```

---

## Platform-specific setup (LATER — not for MVP)

### Web (WASM) — for Itch.io:
```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./web --target web ./target/wasm32-unknown-unknown/release/vassnian_game.wasm
```

### Android — for Google Play:
```bash
# Need Android Studio installed for SDK/NDK
rustup target add aarch64-linux-android
cargo install cargo-apk
cargo apk build --release
```

### iOS — for App Store:
```
⚠️ REQUIRES A MAC + Xcode
⚠️ Apple Developer account: $99/year
rustup target add aarch64-apple-ios
cargo build --release --target aarch64-apple-ios
```

---

## Summary

| Thing | How | When |
|-------|-----|------|
| **Rust + Cargo** | Install via rustup.rs | Now |
| **VS Code + rust-analyzer** | Install manually | Now |
| **Macroquad, serde, rand** | Cargo downloads on `cargo build` | Automatic |
| **wasm-bindgen** | Install when ready for web | Later |
| **Android Studio** | Install when ready for mobile | Later |
| **Xcode** | Install when ready for iOS (Mac only) | Later |

---

## FAQ

**Q: Do I download Macroquad like Godot?**
No. It's code, not an app. Cargo handles it.

**Q: Where's the scene editor?**
There isn't one. Game = code + JSON. Edit in VS Code.

**Q: Can I switch engines later?**
Yes. Only `vassnian_game` uses Macroquad. Engine + content crates are pure Rust.

**Q: How big is the install?**
~500MB Rust toolchain. ~100MB dependencies on first build.
