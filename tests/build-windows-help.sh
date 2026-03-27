#!/usr/bin/env bash
set -euo pipefail

project_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
output="$("$project_root/build-windows.sh" --help)"

case "$output" in
    *"x86_64-pc-windows-gnu"*) ;;
    *)
        echo "expected target triple in help output" >&2
        exit 1
        ;;
esac

case "$output" in
    *"dist/softbuffer-visibility-repro-baseline.exe"*) ;;
    *)
        echo "expected baseline output path in help output" >&2
        exit 1
        ;;
esac

case "$output" in
    *"dist/softbuffer-visibility-repro-damage-tracked.exe"*) ;;
    *)
        echo "expected damage-tracked output path in help output" >&2
        exit 1
        ;;
esac
