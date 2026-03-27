# Dual Repro Binaries Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Keep the current unconditional `winit + softbuffer` repro as a baseline and add a second Windows binary that simulates Slint-style damage-tracked presentation semantics.

**Architecture:** Extract the shared app state into a small library that exposes a render mode and frame-planning logic. Build two binaries on top of it: one baseline that always presents on redraw, and one damage-tracked variant that skips `present_with_damage()` when no logical dirty region exists.

**Tech Stack:** Rust, `winit`, `softbuffer`, shell build script, Rust unit tests

---

### Task 1: Add failing tests for dual-mode behavior

**Files:**
- Create: `tests/repro_modes.rs`
- Modify: `tests/build-windows-help.sh`

**Step 1: Write the failing tests**

- Add Rust tests that expect:
  - baseline mode to present even when nothing changed between redraws
  - damage-tracked mode to skip present when redraw happens without a logical change
- Update the shell help test to expect two output executables.

**Step 2: Run tests to verify they fail**

Run:

```bash
cargo test
bash tests/build-windows-help.sh
```

Expected: FAIL because the shared library API and dual-output help text do not exist yet.

### Task 2: Implement shared repro core and dual binaries

**Files:**
- Create: `src/lib.rs`
- Create: `src/bin/baseline.rs`
- Create: `src/bin/damage_tracked.rs`
- Delete: `src/main.rs`
- Modify: `Cargo.toml`

**Step 1: Write minimal implementation**

- Add a shared `ReproMode` and frame planner.
- Baseline mode always returns a full-frame present plan.
- Damage-tracked mode only returns a full-frame present plan when the logical content changed.
- Build two binaries that use the shared app runner with different modes.

**Step 2: Run tests to verify they pass**

Run:

```bash
cargo test
```

Expected: PASS

### Task 3: Update docs and Windows build script

**Files:**
- Modify: `README.md`
- Modify: `build-windows.sh`
- Modify: `SOFTBUFFER_ISSUE.md`

**Step 1: Write minimal implementation**

- Make the build script compile both binaries and copy both `.exe` files into `dist/`.
- Document what each binary is meant to demonstrate.

**Step 2: Verify**

Run:

```bash
bash tests/build-windows-help.sh
cargo check
cargo check --target x86_64-pc-windows-gnu
```

Expected: PASS
