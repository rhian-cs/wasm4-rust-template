all:
	cargo build --release

	wasm-opt -Oz target/wasm32-unknown-unknown/release/rust_wasm4_template.wasm \
    -o target/wasm32-unknown-unknown/release/rust_wasm4_template.wasm

setup:
	rustup target add wasm32-unknown-unknown

size: all
	du -b target/wasm32-unknown-unknown/release/rust_wasm4_template.wasm

run: all
	w4 run-native target/wasm32-unknown-unknown/release/rust_wasm4_template.wasm

bundle: all
	w4 bundle target/wasm32-unknown-unknown/release/rust_wasm4_template.wasm --title "rust_wasm4_template" --html dist/index.html

# Remember to check when https://github.com/tschaub/gh-pages/issues/354 is closed
deploy: bundle
	npx gh-pages@3.0.0 -d dist
