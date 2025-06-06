# Bitcoin Metaprotocols Canister

[ICP canister](https://internetcomputer.org/docs/building-apps/essentials/canisters) for Bitcoin metaprotocols. Born out of a collaboration between Maestro and Liquidium.

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

# Get info about deployed canister
dfx canister info bitcoin-metaprotocols-canister-backend

# Set canister principal
dfx canister update-settings bitcoin-metaprotocols-canister-backend --set-controller <id>

# Set API key
dfx canister call --update bitcoin-metaprotocols-canister-backend set_api_key '("maestro_api_key")'

# Test
dfx canister call --update bitcoin-metaprotocols-canister-backend get_address_inscriptions '("bc1pa2lw8d6u3kkexzqn9hqgzultkzjjc9rxtveldes68ryfdq8tmslqwfuccl", "10")'

dfx canister call --update bitcoin-metaprotocols-canister-backend get_utxo_inscriptions

# inputs:
# $MAESTRO_API_KEY
# bc1qg2jx2uw33th6p78v638nd3pyd0g8d3wvu2et7p
# 10
```

### TODOs

-   [ ] Inscriptions by address with floor price
-   [ ] Store Liquidium's api key inside the canister with a gated setter function that can only be used by the controller
-   [ ] Whitelist Liquidium consumer canister ID
