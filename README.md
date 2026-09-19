# AdamantiumJson

Official JSON package for Adamantium. It supports objects, arrays, strings,
numbers, booleans, `null`, validation, pretty/compact serialization and safe
file reading and writing. Build the WASI package with:

```text
cargo build --release --target wasm32-wasip1
```

The command ABI and exported functions are declared in
`adamantium_packet.toml`. Malformed input returns a `json_error` package error.
