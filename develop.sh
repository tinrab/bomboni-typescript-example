#!/bin/bash

set -euo pipefail
current_path="$(realpath $0)"
current_dir="$(dirname $current_path)"

function wasm() {
	rm -rf ./pkg
	RUSTFLAGS="-C opt-level=s" \
		wasm-pack build --mode no-install \
		--out-name "wasm" \
		--out-dir "./pkg/node" \
		--target nodejs \
		--no-pack \
		--release
	RUSTFLAGS="-C opt-level=s" \
		wasm-pack build --mode no-install \
		--out-name "wasm" \
		--out-dir "./pkg/web" \
		--target web \
		--no-pack \
		--release
}

function test() {
	bun test
}

function help() {
	echo "Usage: $(basename "$0") [OPTIONS]

Commands:
  wasm           Pack WASM
  help           Show help
"
}

if [[ $1 =~ ^(wasm|test|help)$ ]]; then
	"$@"
else
	help
	exit 1
fi
