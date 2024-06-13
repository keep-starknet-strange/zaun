# TODO: this can be rewritten as build.rs in the sandbox crate,
# but it might introduce unnecessary friction and longer build times.
# Moreover, any project using sandbox as dependency won't build unless
# there's Foundry installed on the machine.

.PHONY: artifacts

artifacts:
	mkdir crates/starknet-proxy-client/src/artifacts || true
	mkdir crates/starknet-core-contract-client/src/artifacts || true
	forge build
	cp out/UnsafeProxy.sol/UnsafeProxy.json crates/starknet-proxy-client/src/artifacts/
	cp out/StarknetSovereign.sol/Starknet.json crates/starknet-core-contract-client/src/artifacts/
	(cd lib/piltover && scarb build && cp target/dev/* ../../crates/l3/core-contract-client/artifacts)