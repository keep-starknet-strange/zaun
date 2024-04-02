# Zaun

This repository contains interfaces and functions to interact with starknet solidity contracts.

## Setup

```bash
curl -L https://foundry.paradigm.xyz | bash
foundryup
```

## Testing

Unit

```bash
forge test -vv
```

### Devnet

Start Anvil as background process w/ Core Contracts and STARK token:

```bash
./script/sn-base-dev.sh
```

Kill Anvil deployment:

```bash
./script/sn-base-kill.sh
```

## Artifacts

Zaun sandbox crate can be used as dev dependency in external projects. If any changes made to the Solidity contracts, one has to re-generate Ethers bindings (this won't be done automatically).

```bash
make artifacts
```

## StarkNet Consensus Protocol Contracts

PoC solidity implementation of the following Starknet Decentralized Protocol proposal:

- [I - Introduction](https://community.starknet.io/t/starknet-decentralized-protocol-i-introduction/2671/1)
- [II - Candidate for Leader Elections](https://community.starknet.io/t/starknet-decentralized-protocol-ii-candidate-for-leader-elections/4751)
- [III - Consensus](https://community.starknet.io/t/starknet-decentralized-protocol-iii-consensus/5386)
- [IV - Proofs in the Protocol](https://community.starknet.io/t/starknet-decentralized-protocol-iv-proofs-in-the-protocol/6030)
- [V - Checkpoints for Fast Finality](https://community.starknet.io/t/starknet-decentralized-protocol-v-checkpoints-for-fast-finality/6032)
- [VI - The Buffer Problem](https://community.starknet.io/t/starknet-decentralized-protocol-vi-the-buffer-problem/7098)
- [VII - Chained Proof Protocols & Braiding](https://community.starknet.io/t/starknet-decentralized-protocol-vii-chained-proof-protocols-braiding/18831)
