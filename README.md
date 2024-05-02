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
    - maybe better (more conventional) implementation for themes
    - maybe try modularizing styling (separate for each element, not global)
- Maybe a top-down interface for the Projects tab
