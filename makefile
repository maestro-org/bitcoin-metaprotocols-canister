DFX := dfx
EXTRACT_DID := candid-extractor target/wasm32-unknown-unknown/release/bitcoin_metaprotocols_canister_backend.wasm > src/bitcoin-metaprotocols-canister-backend/bitcoin-metaprotocols-canister-backend.did

generate_did:
	@echo "Building ..."
	cargo build --target wasm32-unknown-unknown --release -p bitcoin-metaprotocols-canister-backend --locked
	@echo "Adding .did file to binary..."
	$(EXTRACT_DID)
	ic-wasm ./target/wasm32-unknown-unknown/release/bitcoin_metaprotocols_canister_backend.wasm -o ./target/wasm32-unknown-unknown/release/bitcoin_metaprotocols_canister_backend.wasm metadata candid:service -f ./src/bitcoin-metaprotocols-canister-backend/bitcoin-metaprotocols-canister-backend.did -v public
	@echo "Build completed."
