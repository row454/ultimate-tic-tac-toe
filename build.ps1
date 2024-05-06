cargo build --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/ultimate_tic_tac_toe.wasm --out-dir web_build --target web --no-typescript