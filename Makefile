# TODO: this can be rewritten as build.rs in the sandbox crate,
# but it might introduce unnecessary friction and longer build times.
# Moreover, any project using sandbox as dependency won't build unless
# there's Foundry installed on the machine.

.PHONY: artifacts cairo-lang local-contracts starkgate-contracts

CAIRO_LANG_COMMIT_HASH="8e11b8cc65ae1d0959328b1b4a40b92df8b58595"
STARKGATE_CONTRACTS_COMMIT_HASH="45941888479663ac93e898cd7f8504fa9066c54c"

artifacts:
	mkdir crates/starknet-proxy-client/src/artifacts || true
	mkdir crates/starknet-core-contract-client/src/artifacts || true
	forge build
	cp out/UnsafeProxy.sol/UnsafeProxy.json crates/starknet-proxy-client/src/artifacts/
	cp out/StarknetSovereign.sol/Starknet.json crates/starknet-core-contract-client/src/artifacts/
	(cd lib/piltover && scarb build && cp target/dev/* ../../crates/l3/appchain-core-contract-client/artifacts)

cairo-lang:
	cp build-artifacts/foundry.toml lib/cairo-lang/foundry.toml
	cd lib/cairo-lang && \
 	git checkout $(CAIRO_LANG_COMMIT_HASH) && \
 	forge build
	# Copying Contracts :
	mkdir -p artifacts/cairo-lang
	cp lib/cairo-lang/out/Starknet.sol/Starknet.json artifacts/cairo-lang/Starknet.json

local-contracts:
	forge build
	# Copying Contracts :
	cp out/StarknetOverride.sol/Starknet.json artifacts/StarknetOverride.json
	cp out/UnsafeProxy.sol/UnsafeProxy.json artifacts/UnsafeProxy.json

starkgate-contracts:
	cd lib/starkgate-contracts && \
	git checkout $(STARKGATE_CONTRACTS_COMMIT_HASH) && \
	./scripts/setup.sh && \
	FILES=$$(cat src/solidity/files_to_compile.txt) && \
 	solc $$FILES --allow-paths .=., --optimize --optimize-runs 200 --overwrite --combined-json abi,bin -o artifacts && \
 	./scripts/extract_artifacts.py
	# Copying Contracts :
	mkdir -p artifacts/starkgate-contracts
	cp lib/starkgate-contracts/artifacts/LegacyBridge.json artifacts/starkgate-contracts/LegacyBridge.json
	cp lib/starkgate-contracts/artifacts/StarkgateManager.json artifacts/starkgate-contracts/StarkgateManager.json
	cp lib/starkgate-contracts/artifacts/StarkgateRegistry.json artifacts/starkgate-contracts/StarkgateRegistry.json
	cp lib/starkgate-contracts/artifacts/Proxy.json artifacts/starkgate-contracts/Proxy_5_0_0.json
	cp lib/starkgate-contracts/artifacts/StarknetTokenBridge.json artifacts/starkgate-contracts/StarknetTokenBridge.json
	cp lib/starkgate-contracts/artifacts/ERC20.json artifacts/starkgate-contracts/ERC20.json
