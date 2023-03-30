# Apex
Highly hackable & performant taiko client and beatmap editor supporting multiple platforms such as Linux (X11 & Wayland), macOS, Windows and even web using WebAssembly.

## Build Instructions
1. Install the Rust toolchain: https://rustup.rs
2. Clone the repository: `git clone git@github.com:polina4096/apex.git`
3. Navigate to the cloned repository's directory: `cd apex`
4. Compile the program: `cargo build --release`

### WASM support
Install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/), build with: `wasm-pack build --release`.

Host a local web server with: `npm run start`.

## License
Distributed under the MIT license.