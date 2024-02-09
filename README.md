# Slimebook

Install dependencies:
1. `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` or install the Rust toolchain using another method
2. `cargo install mdbook`
3. `cargo install near-cli-rs`

Build: `mdbook build`

Deploy to mainnet: `NEAR_ENV=mainnet npx web4-deploy out/html web4.slimebook.near --nearfs`
Deploy to test: `NEAR_ENV=testnet npx web4-deploy out/html web4.slimebook.testnet --nearfs`

Add `--deploy-contract` option when you deploy it for the first time.

Built with [NEAR](https://near.org), [Web4](https://web4.near.page/), [mdBook](https://rust-lang.github.io/mdBook/), and 💚
