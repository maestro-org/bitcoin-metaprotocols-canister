# Bitcoin Metaprotocols Canister

[ICP canister](https://internetcomputer.org/docs/building-apps/essentials/canisters) for Bitcoin metaprotocols. Born out if a collaboration with Maestro and Liquidium.

## Prerequisites
To get started, ensure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [DFX](https://internetcomputer.org/docs/building-apps/developer-tools/dfx/)
- WebAssembly target for Rust:

```bash
rustup target add wasm32-unknown-unknown
```

## Running locally

If you want to test your project locally, you can use the following commands:

```bash
# Start local subnet in a dedicated terminal
dfx start --clean
```

```bash
# [optional] Generate did
dfx generate

# [optiona] Build
dfx build

# Deploy
dfx deploy

# Test
dfx canister call --update bitcoin-metaprotocols-canister-backend get_inscriptions
```

### Roadmap

- [ ] Inscriptions by address with collection symbol and floor price
- [ ] Cleanup
