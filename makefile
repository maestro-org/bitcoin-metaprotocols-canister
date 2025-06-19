DFX := dfx
CANISTER := bitcoin-metaprotocols-canister
WASM_PATH := target/wasm32-unknown-unknown/release/bitcoin_metaprotocols_canister.wasm
DID_PATH := src/bitcoin-metaprotocols-canister/$(CANISTER).did

EXTRACT_DID := candid-extractor $(WASM_PATH) > $(DID_PATH)

generate_did:
	@echo "🔧 Building WASM..."
	cargo build --target wasm32-unknown-unknown --release -p $(CANISTER) --locked
	@echo "📄 Extracting .did file..."
	$(EXTRACT_DID)
	@echo "🔗 Embedding .did metadata..."
	ic-wasm $(WASM_PATH) -o $(WASM_PATH) metadata candid:service -f $(DID_PATH) -v public
	@echo "✅ Build and DID generation complete."
