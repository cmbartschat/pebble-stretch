# Stretch Watchface

Minimalist watchface with animated digits

<img alt="Preview of watchface" src="https://apps.repebble.com/og/c99a31383623444d98b7e777.png" width="300" height="157" />

Install: https://apps.repebble.com/c99a31383623444d98b7e777

## Development

### Prerequisites

1. Cargo: https://rustup.rs/
2. Pebble SDK: https://developer.repebble.com/sdk/
3. Cargo Pebble: `cargo install --git https://codeberg.org/filmroellchen/cargo-pebble.git`
4. Rust targets: `rustup target add thumbv7m-none-eabi thumbv7em-none-eabi thumbv8m.main-none-eabi`

```sh
cargo pebble build
pebble install --emulator emery
```
