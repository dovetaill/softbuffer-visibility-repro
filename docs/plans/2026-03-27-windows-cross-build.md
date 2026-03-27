# Windows Cross-Build Script Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add a Linux shell script that builds this crate into a Windows `.exe` and documents how to use it.

**Architecture:** Keep the implementation at the repository root as a small `bash` wrapper around Cargo. Use the GNU Windows Rust target plus `mingw-w64`, then copy the generated `.exe` into `dist/` so users have a stable output path.

**Tech Stack:** Bash, Cargo, rustup, mingw-w64, Rust target `x86_64-pc-windows-gnu`

---

### Task 1: Add the failing smoke test

**Files:**
- Create: `tests/build-windows-help.sh`

**Step 1: Write the failing test**

```bash
bash tests/build-windows-help.sh
```

**Step 2: Run test to verify it fails**

Run: `bash tests/build-windows-help.sh`
Expected: FAIL because `build-windows.sh` does not exist yet

### Task 2: Implement the build script

**Files:**
- Create: `build-windows.sh`
- Modify: `README.md`
- Modify: `.gitignore`

**Step 1: Write minimal implementation**

```bash
#!/usr/bin/env bash
set -euo pipefail

require cargo, rustup, and x86_64-w64-mingw32-gcc
rustup target add x86_64-pc-windows-gnu
export linker env var
cargo build --release --target x86_64-pc-windows-gnu
copy target exe into dist/
```

**Step 2: Run targeted verification**

Run: `bash -n build-windows.sh`
Expected: PASS

Run: `bash tests/build-windows-help.sh`
Expected: PASS

### Task 3: Document usage

**Files:**
- Modify: `README.md`
- Create: `docs/plans/2026-03-27-windows-cross-build-design.md`

**Step 1: Add usage notes**

Document:

- required Linux packages
- target triple used by the script
- final output path

**Step 2: Re-run quick verification**

Run: `bash tests/build-windows-help.sh`
Expected: PASS
