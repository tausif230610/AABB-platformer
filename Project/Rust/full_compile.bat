cls
echo windows release-is-compiling
cargo +nightly build --release  --target x86_64-pc-windows-msvc

echo windows dev/debug-is-compiling
cargo +nightly build --target x86_64-pc-windows-msvc

echo web-dev-is-compiling
cargo +nightly build --target wasm32-unknown-emscripten

echo web-release-is-compiling
cargo +nightly build --release --target wasm32-unknown-emscripten

echo full compile done
pause
