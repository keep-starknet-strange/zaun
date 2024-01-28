# TODO: this can be rewritten as build.rs in the sandbox crate,
# but it might introduce unnecessary friction and longer build times.
# Moreover, any project using sandbox as dependency won't build unless
# there's Foundry installed on the machine.

.PHONY: artifacts

artifacts:
	mkdir crates/sandbox/artifacts || true
	forge build
	cp out/UnsafeProxy.sol/UnsafeProxy.json crates/sandbox/artifacts/
	cp out/StarknetSovereign.sol/Starknet.json crates/sandbox/artifacts/