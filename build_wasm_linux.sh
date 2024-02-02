#!/usr/bin/env bash

set -e

HELP_STRING=$(cat <<- END
	usage: build_wasm.sh PROJECT_NAME [--release]

	Build script for combining a Macroquad project with wasm-bindgen,
	allowing integration with the greater wasm-ecosystem.

	example: build_wasm.sh flappy-bird

	This'll go through the following steps:

	    1. Build as target 'wasm32-unknown-unknown'
	    2. Create the directory 'wbindgen' if it doesn't already exist
	    3. Run wasm-bindgen with output into the wbindgen directory
	    4. Apply patches to the output js file (detailed here: https://github.com/not-fl3/macroquad/issues/212#issuecomment-835276147)

	Required arguments:

	    PROJECT_NAME            The name of the artifact/target/project

	Arguments:

	    --release               Build in release mode


	Author: Tom Solberg <me@sbg.dev>
	Version: 0.1
END
)

PROJECT_NAME="wizgame"

# Build
cargo build --target wasm32-unknown-unknown --release

# Generate bindgen outputs
mkdir -p wbindgen
wasm-bindgen --target web --out-dir wbindgen/ target/wasm32-unknown-unknown/release/$PROJECT_NAME.wasm

# Shim to tie the thing together
sed -i "s/import \* as __wbg_star0 from 'env';//" wbindgen/$PROJECT_NAME.js
sed -i "s/let wasm;/let wasm; export const set_wasm = (w) => wasm = w;/" wbindgen/$PROJECT_NAME.js
sed -i "s/imports\['env'\] = __wbg_star0;/return imports.wbg\;/" wbindgen/$PROJECT_NAME.js
sed -i "s/const imports = __wbg_get_imports();/return __wbg_get_imports();/" wbindgen/$PROJECT_NAME.js