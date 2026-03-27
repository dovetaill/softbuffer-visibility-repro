Title: Win32: stale newly exposed regions when redraws do not re-present the existing frame

## Reproducer

Minimal crate:

- [`Cargo.toml`](./Cargo.toml)
- [`src/lib.rs`](./src/lib.rs)
- [`src/bin/baseline.rs`](./src/bin/baseline.rs)
- [`src/bin/damage_tracked.rs`](./src/bin/damage_tracked.rs)

This crate intentionally contains two variants:

- `softbuffer-visibility-repro-baseline`
  - control group
  - every redraw fills the full buffer and calls `present()`
- `softbuffer-visibility-repro-damage-tracked`
  - closer to the Slint software renderer path
  - redraws without a logical dirty region skip `present()`

## Steps to reproduce

1. Run the `damage-tracked` repro on Windows.
2. Move part of the window outside the monitor bounds.
3. Press `Space` while the window is still partially off-screen.
4. Move the window fully back onto the monitor.

## Expected

The entire window should show the new color after step 3 / step 4.

## Observed

The region that was outside the monitor can remain on the old color until something triggers a redraw that actually re-presents the buffer.

In other words, the visible part updates, but the newly exposed region does not necessarily reflect the most recently presented frame.

## Why I think the interesting part is the redraw/present contract

The baseline binary is intentionally kept as a control group. It may not reproduce the issue consistently, because it unconditionally presents the full frame on every redraw.

The damage-tracked binary is closer to the behavior I saw in Slint software rendering: a redraw can happen, but if the renderer decides there is no logical dirty region then it does not call `present()` again.

The relevant pieces I found while reading the code are:

- `winit` on Windows implements `request_redraw()` via `RedrawWindow(..., RDW_INTERNALPAINT)`.
- Microsoft documents that `RDW_INTERNALPAINT` can produce a `WM_PAINT` without an update region, and that such an internal paint is only generated once until the window is invalidated again.
- `softbuffer`'s Win32 backend presents by `BitBlt`-ing the requested damage rectangles to the window DC.
- `BitBlt` is clipped by the destination DC, so when the window is partially off-screen only the currently visible portion is actually copied to the window.

That means a `present()` while the window is partially off-screen may only copy the currently visible subset of the frame. If a later redraw does not cause a new `present()`, the newly exposed region can stay stale until some other redraw path re-presents the buffer.

## Relevant source locations

Current `softbuffer` Win32 backend:

- `src/backends/win32.rs`
- `BufferImpl::present_with_damage`

Current `winit` Windows backend:

- `src/platform_impl/windows/window.rs`
- `Window::request_redraw`
- `src/platform_impl/windows/event_loop.rs`
- `WM_PAINT` handling

## Question

Is this something `softbuffer` should handle on Win32, or is this a documented limitation of the current model when downstream renderers use damage-tracked redraw semantics?

At the moment it looks difficult for downstream users to discover this on their own, because the important distinction is not just "did I get a redraw?" but also "did that redraw actually re-present the existing frame to the window DC?"

If this is expected, it would be helpful to document the required remedy for downstreams on Windows.
