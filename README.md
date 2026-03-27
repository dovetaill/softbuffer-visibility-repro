# softbuffer visibility repro

Minimal Windows visibility/presentation repros built on `winit + softbuffer`.

## Variants

This crate now builds two binaries:

- `softbuffer-visibility-repro-baseline.exe`
  - Control group.
  - Every `RedrawRequested` fills the full buffer and calls `present()`.
- `softbuffer-visibility-repro-damage-tracked.exe`
  - Closer to  Slint software behavior.
  - A redraw only calls `present()` when the logical content changed.
  - Redraws without a logical dirty region intentionally skip present.

## Controls

- `Space`: toggle the entire window between blue and red
- `Esc`: exit

Neither binary requests redraws on move events. They only redraw when the color changes or when the OS decides to deliver a redraw.

## Why there are two binaries

The baseline binary is the pure `winit + softbuffer` comparison point.

The damage-tracked binary models the more interesting case for Slint: a redraw can happen, but the renderer may still choose not to present because it thinks there is no dirty region. That makes it useful for checking whether the Windows stale-region symptom depends on damage-tracked rendering semantics rather than on `softbuffer` alone.

## Reproduction on Windows

1. Run one of the binaries.
2. Move part of the window outside the monitor bounds.
3. Press `Space` while the window is still partially off screen.
4. Move the window back fully on screen.

## What to look for

- Baseline:
  - If a redraw happens, it always re-presents the full frame.
- Damage-tracked:
  - If a redraw happens without a logical change after the toggle has already been presented, the app logs that it skipped present.
  - This is the closer approximation of the  Slint software path.

## Build a Windows `.exe` From Linux

This repository includes a helper script for Linux cross-compilation:

```bash
./build-windows.sh
```

The script targets `x86_64-pc-windows-gnu` and writes the final binary to:

```text
dist/softbuffer-visibility-repro-baseline.exe
dist/softbuffer-visibility-repro-damage-tracked.exe
```

### Prerequisites

- `cargo`
- `rustup`
- `mingw-w64`

On Debian or Ubuntu:

```bash
sudo apt-get install -y mingw-w64
rustup target add x86_64-pc-windows-gnu
```

Then run:

```bash
./build-windows.sh
```
