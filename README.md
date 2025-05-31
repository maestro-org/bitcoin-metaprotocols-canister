# Bitcoin Metaprotocols Canister

[ICP canister](https://internetcomputer.org/docs/building-apps/essentials/canisters) for Bitcoin metaprotocols. Born out a collaboration with Maestro and Liquidium.

## Prerequisites

To get started, ensure you have the following installed:

-   [Rust](https://www.rust-lang.org/tools/install)
-   [DFX](https://internetcomputer.org/docs/building-apps/developer-tools/dfx/)
-   WebAssembly target for Rust:

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

# issue canister
dfx canister create bitcoin-metaprotocols-canister-backend

# [optiona] Build
dfx build

# Deploy
dfx deploy

# Test
dfx canister call --update bitcoin-metaprotocols-canister-backend get_address_inscriptions

# inputs:
# $MAESTRO_API_KEY
# bc1qg2jx2uw33th6p78v638nd3pyd0g8d3wvu2et7p
# 10
```

### Roadmap

-   [ ] Inscriptions by address with floor price
-   [ ] Cleanup
