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
    curl -sSL https://raw.githubusercontent.com/nvim-tree/nvim-web-devicons/master/lua/nvim-web-devicons.lua | \
        sed 's/get_icon = get_icon,/get_icon = get_icon,icons_by_filename = icons_by_filename, icons_by_file_extension = icons_by_file_extension,/' > "$tmp/nvim-web-devicons.lua"

    # Call our own Lua script to extract the icons, and turn it into Rust code
    local generated_code_file
    generated_code_file="$tmp/generated-code.txt"
    lua "$SCRIPT_DIR/extract-icons.lua" < "$tmp/nvim-web-devicons.lua" > "$generated_code_file"
    # echo "$generated_code" > "$generated_code_file"

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

