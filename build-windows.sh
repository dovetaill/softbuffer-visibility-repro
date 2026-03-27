#!/usr/bin/env bash
set -euo pipefail

target_triple="x86_64-pc-windows-gnu"
binary_names=(
    "softbuffer-visibility-repro-baseline"
    "softbuffer-visibility-repro-damage-tracked"
)
profile="release"
project_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
output_dir="$project_root/dist"

usage() {
    cat <<'EOF'
Usage: ./build-windows.sh [--debug] [--help]

Cross-compile this crate on Linux to a Windows .exe.

Prerequisites:
  - cargo
  - rustup
  - x86_64-w64-mingw32-gcc (usually provided by mingw-w64)
  - Debian/Ubuntu example: sudo apt-get install -y mingw-w64

Target:
  x86_64-pc-windows-gnu

Output:
  dist/softbuffer-visibility-repro-baseline.exe
  dist/softbuffer-visibility-repro-damage-tracked.exe
EOF
}

require_command() {
    local command_name="$1"

    if ! command -v "$command_name" >/dev/null 2>&1; then
        echo "missing required command: $command_name" >&2
        exit 1
    fi
}

if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then
    usage
    exit 0
fi

if [[ "${1:-}" == "--debug" ]]; then
    profile="debug"
elif [[ -n "${1:-}" ]]; then
    echo "unknown argument: ${1:-}" >&2
    usage >&2
    exit 1
fi

require_command cargo
require_command rustup
require_command x86_64-w64-mingw32-gcc

rustup target add "$target_triple"

export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER=x86_64-w64-mingw32-gcc

build_args=(build --target "$target_triple")
if [[ "$profile" == "release" ]]; then
    build_args+=(--release)
fi
for binary_name in "${binary_names[@]}"; do
    build_args+=(--bin "$binary_name")
done

(
    cd "$project_root"
    cargo "${build_args[@]}"
)

mkdir -p "$output_dir"

echo "built Windows binary:"
for binary_name in "${binary_names[@]}"; do
    binary_path="$project_root/target/$target_triple/$profile/$binary_name.exe"
    output_path="$output_dir/$binary_name.exe"
    if [[ ! -f "$binary_path" ]]; then
        echo "build finished but output was not found: $binary_path" >&2
        exit 1
    fi

    cp "$binary_path" "$output_path"
    echo "  $output_path"
done
