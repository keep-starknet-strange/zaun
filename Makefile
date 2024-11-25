# TODO: this can be rewritten as build.rs in the sandbox crate,
# but it might introduce unnecessary friction and longer build times.
# Moreover, any project using sandbox as dependency won't build unless
# there's Foundry installed on the machine.

.PHONY: pilt-over cairo-lang local-contracts starkgate-contracts-latest starkgate-contracts-old l2-artifacts

CAIRO_LANG_COMMIT_HASH="8e11b8cc65ae1d0959328b1b4a40b92df8b58595"
STARKGATE_CONTRACTS_COMMIT_HASH="45941888479663ac93e898cd7f8504fa9066c54c"

pilt-over:
	mkdir crates/starknet-proxy-client/src/artifacts || true
	mkdir crates/starknet-core-contract-client/src/artifacts || true
	forge build
	cp out/UnsafeProxy.sol/UnsafeProxy.json crates/starknet-proxy-client/src/artifacts/
	cp out/StarknetSovereign.sol/Starknet.json crates/starknet-core-contract-client/src/artifacts/
	(cd lib/piltover && scarb build && cp target/dev/* ../../crates/l3/appchain-core-contract-client/artifacts)

cairo-lang:
	# Building
	cp build-artifacts/cairo-lang/foundry.toml lib/cairo-lang/foundry.toml
	cd lib/cairo-lang && \
 	git checkout $(CAIRO_LANG_COMMIT_HASH) && \
 	forge build
	# Copying Contracts :
	mkdir -p artifacts/cairo-lang
	cp lib/cairo-lang/out/Starknet.sol/Starknet.json artifacts/cairo-lang/Starknet.json

local-contracts:
	# Building
	forge build
	# Copying Contracts :
	cp out/StarknetOverride.sol/Starknet.json artifacts/StarknetOverride.json
	cp out/UnsafeProxy.sol/UnsafeProxy.json artifacts/UnsafeProxy.json

starkgate-contracts-latest:
	# Configure solidity version
	solc-select install 0.8.24 && solc-select use 0.8.24
	# Building
	cd lib/starkgate-contracts && \
	git checkout $(STARKGATE_CONTRACTS_COMMIT_HASH) && \
	./scripts/setup.sh && \
	FILES=$$(cat src/solidity/files_to_compile.txt) && \
 	solc $$FILES --allow-paths .=., --optimize --optimize-runs 200 --overwrite --combined-json abi,bin -o artifacts && \
 	./scripts/extract_artifacts.py
	# Copying Contracts :
	mkdir -p artifacts/starkgate-contracts
	cp lib/starkgate-contracts/artifacts/StarkgateManager.json artifacts/starkgate-contracts/StarkgateManager.json
	cp lib/starkgate-contracts/artifacts/StarkgateRegistry.json artifacts/starkgate-contracts/StarkgateRegistry.json
	cp lib/starkgate-contracts/artifacts/Proxy.json artifacts/starkgate-contracts/Proxy_5_0_0.json
	cp lib/starkgate-contracts/artifacts/StarknetTokenBridge.json artifacts/starkgate-contracts/StarknetTokenBridge.json
	cp lib/starkgate-contracts/artifacts/ERC20.json artifacts/starkgate-contracts/ERC20.json

starkgate-contracts-old:
	# Configure solidity version
	solc-select install 0.6.12 && solc-select use 0.6.12
	# Building
	cp build-artifacts/starkgate-contracts-0.9/foundry.toml lib/starkgate-contracts-0.9/src/starkware/solidity/foundry.toml
	cp build-artifacts/starkgate-contracts-0.9/foundry2.toml lib/starkgate-contracts-0.9/src/starkware/foundry.toml
	cd lib/starkgate-contracts-0.9/src/starkware/solidity && \
	forge build
	cd lib/starkgate-contracts-0.9/src/starkware && \
	forge build
	# Copying Contracts :
	mkdir -p artifacts/starkgate-contracts-0.9
	cp lib/starkgate-contracts-0.9/src/starkware/solidity/out/Proxy.sol/Proxy.json artifacts/starkgate-contracts-0.9/Proxy_3_0_2.json
	cp lib/starkgate-contracts-0.9/src/starkware/out/StarknetEthBridge.sol/StarknetEthBridge.json artifacts/starkgate-contracts-0.9/LegacyBridge.json

l2-artifacts:
	make cairo-lang
	make local-contracts
	make starkgate-contracts-latest
	make starkgate-contracts-old
	python3 build-artifacts/convert.py
	echo "L2 Artifacts built ✅"
