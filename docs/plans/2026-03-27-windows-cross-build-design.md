# Windows Cross-Build Script Design

## Goal

Add a Linux shell script that cross-compiles this Rust repro into a Windows `.exe` with the fewest moving parts.

## Context

The crate is a small `winit + softbuffer` repro with a single binary target. There is no existing build automation for Windows artifacts.

## Decision

Use Rust's `x86_64-pc-windows-gnu` target with `mingw-w64`.

This is the most direct option for a small Linux-hosted repro:

- no Docker requirement
- no MSVC SDK setup
- one command for users after installing `mingw-w64`

## Script Behavior

The script should:

- verify `cargo`, `rustup`, and `x86_64-w64-mingw32-gcc`
- ensure the Rust Windows GNU target is installed
- build in `release` mode by default
- support `--debug` for local troubleshooting
- copy the final artifact to `dist/softbuffer-visibility-repro.exe`

## Error Handling

Fail early with a clear message when a required command is missing or when Cargo does not produce the expected `.exe`.

## Validation

Use a small shell smoke test for `--help` output and then validate the script with `bash -n` and the smoke test itself.
