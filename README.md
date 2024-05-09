# ejs

## Build
```bash
trunk build --release
wasm-opt --strip-debug dist/*.wasm -o dist/*.wasm
wasm-opt dist/*.wasm -Os -o dist/*.wasm
```

## Todo
- Further reduce WASM binary size
    - try reducing size of `css` use
    - micro optimizations
    - find a small/fast markdown-to-html parser
- A List (top-down) for the Projects tab
