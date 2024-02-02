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

HTML=$(cat <<- END
<html lang="en">
<head>
    <meta charset="utf-8">
    <title>wizgame</title>
    <style>
        html,
        body,
        canvas {
            margin: 0px;
            padding: 0px;
            width: 100%;
            height: 100%;
            overflow: hidden;
            position: absolute;
            z-index: 0;
        }
    </style>
</head>
<body style="margin: 0; padding: 0; height: 100vh; width: 100vw;">
    <canvas id="glcanvas" tabindex='1' hidden></canvas>
    <script src="https://not-fl3.github.io/miniquad-samples/mq_js_bundle.js"></script>
    <script type="module">
        import init, { set_wasm } from "./wizgame.js";
        async function impl_run() {
            let wbg = await init();
            miniquad_add_plugin({
                register_plugin: (a) => (a.wbg = wbg),
                on_init: () => set_wasm(wasm_exports),
                version: "0.0.1",
                name: "wbg",
            });
            load("./wizgame_bg.wasm");
        }
        window.run = function() {
            document.getElementById("run-container").remove();
            document.getElementById("glcanvas").removeAttribute("hidden");
            document.getElementById("glcanvas").focus();
            impl_run();
        }
    </script>
    <div id="run-container" style="display: flex; justify-content: center; align-items: center; height: 100%; flex-direction: column;">
        <p>Game can't play audio unless a button has been clicked.</p>
        <button onclick="run()">Run Game</button>
    </div>
</body>
</html>
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

# Create index from the HTML variable
echo "$HTML" > wbindgen/index.html