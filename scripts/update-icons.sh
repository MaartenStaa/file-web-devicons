#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
PROJECT_DIR=$(dirname "$SCRIPT_DIR")

function main() {
    local tmp
    tmp=$(mktemp -d)

    # Delete temporary dir on exit
    # shellcheck disable=SC2064
    trap "rm -rf '$tmp'" EXIT

    # Download icons, and patch Lua to export them directly
    mkdir -p "$tmp/nvim-web-devicons"
    curl -sSL https://raw.githubusercontent.com/nvim-tree/nvim-web-devicons/master/lua/nvim-web-devicons/icons-default.lua > "$tmp/nvim-web-devicons/icons-default.lua"
    curl -sSL https://raw.githubusercontent.com/nvim-tree/nvim-web-devicons/master/lua/nvim-web-devicons.lua | \
        sed 's/return M/M.icons_by_filename = icons_by_filename;M.icons_by_file_extension = icons_by_file_extension;return M/' > "$tmp/nvim-web-devicons.lua"

    # Call our own Lua script to extract the icons, and turn it into Rust code
    local generated_code_file
    generated_code_file="$tmp/generated-code.txt"
    cp "$SCRIPT_DIR/extract-icons.lua" "$tmp/extract-icons.lua"
    cd "$tmp"
    lua "extract-icons.lua" > "$generated_code_file"

    # Replace the generated code in the Rust source file
    local rust_file
    rust_file="$PROJECT_DIR/crates/file_web_devicon_lib/src/icons.rs"
    local delimiter_pattern
    delimiter_pattern='\/\/ BEGIN GENERATED CODE'

    sed -i '' -n \
        -e "1,/$delimiter_pattern/ p" \
        -e "/$delimiter_pattern/ r $generated_code_file" \
        "$rust_file"

    cd "$PROJECT_DIR"
    cargo fmt
}

main "$@"

