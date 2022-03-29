# polkapad-smarts

#### Install rustup
`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

#### Update rustup and install nighlty toolchain
`rustup update`

`rustup update nightly`

#### Install WASM compiler plugin for rustup
`rustup target add wasm32-unknown-unknown --toolchain nightly`

### Build
1. `cd /contracts/token` or `cd /contracts/wallet`
2. `RUSTFLAGS="-C link-args=--import-memory" cargo +nightly build --release --target=wasm32-unknown-unknown`
3. `wasm-proc --path ./target/wasm32-unknown-unknown/release/[project_name].wasm`

### Deploy contract to the test-env via Gear GUI
1. Go to https://idea.gear-tech.io
2. Click on 'Upload program'
3. Choose `./target/wasm32-unknown-unknown/release/[project_name].wasm` file
4. (Optional) Choose meta-data `./target/wasm32-unknown-unknown/release/[project_name].meta.wasm` file
5. Click on 'Upload program'
