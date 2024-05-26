cargo build --package frontend --target=wasm32-unknown-unknown
wasm-bindgen --no-typescript --target=web --out-dir=target/front ./target/wasm32-unknown-unknown/debug/frontend.wasm
cargo run --package backend