# TODO: this can be rewritten as build.rs
# but it might introduce unnecessary friction when building the project.

.PHONY: artifacts

artifacts:
	mkdir artifacts || true
	forge build --root ../../../
	cp ../../../out/UnsafeProxy.sol/UnsafeProxy.json artifacts/
	cp ../../../out/StarknetSovereign.sol/Starknet.json artifacts/