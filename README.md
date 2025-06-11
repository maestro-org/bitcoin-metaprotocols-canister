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

## Running Locally

### Start Local Subnet

Start the local subnet in a dedicated terminal:

```bash
dfx start --clean
```

### Generate DID (Optional)

```bash
dfx generate
```

### Canister Management

#### Create Canister

```bash
dfx canister create bitcoin-metaprotocols-canister-backend
```

#### Build Canister (Optional)

```bash
dfx build
```

### Generate Candid (Optional)
make generate_did

#### Deploy Canister

```bash
make generate_did
dfx deploy
```

#### Get Canister Info

```bash
dfx canister info bitcoin-metaprotocols-canister-backend
```

#### Update Canister Settings

Set the canister principal:

```bash
dfx canister update-settings bitcoin-metaprotocols-canister-backend --set-controller <id>
```

### API Key Management

Set the API key:

```bash
dfx canister call --update bitcoin-metaprotocols-canister-backend set_api_key '("maestro_api_key")'
```

### Testing

#### Test Address Inscriptions

```bash
dfx canister call --update bitcoin-metaprotocols-canister-backend get_address_inscriptions '("bc1pa2lw8d6u3kkexzqn9hqgzultkzjjc9rxtveldes68ryfdq8tmslqwfuccl", "10")'
```

#### Test UTXO Inscriptions

```bash
dfx canister call --update bitcoin-metaprotocols-canister-backend get_utxo_inscriptions '("604abd1c0ff2ce5a89b004a0601a75280ed3b76384af37b0a46a23471e9288e7", "1")'
```

## TODOs

-   [ ] Inscriptions by address with floor price
