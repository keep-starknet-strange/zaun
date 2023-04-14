# StarkNet Consensus Protocol Contracts

PoC solidity implementation of the following Starknet Decentralized Protocol proposal:
- [I - Introduction](https://community.starknet.io/t/starknet-decentralized-protocol-i-introduction/2671/1)
- [II - Candidate for Leader Elections](https://community.starknet.io/t/starknet-decentralized-protocol-ii-candidate-for-leader-elections/4751)
- [III - Consensus](https://community.starknet.io/t/starknet-decentralized-protocol-iii-consensus/5386)
- [IV - Proofs in the Protocol](https://community.starknet.io/t/starknet-decentralized-protocol-iv-proofs-in-the-protocol/6030)
- [V - Checkpoints for Fast Finality](https://community.starknet.io/t/starknet-decentralized-protocol-v-checkpoints-for-fast-finality/6032)
- [VI - The Buffer Problem](https://community.starknet.io/t/starknet-decentralized-protocol-vi-the-buffer-problem/7098)
- [VII - Chained Proof Protocols & Braiding](https://community.starknet.io/t/starknet-decentralized-protocol-vii-chained-proof-protocols-braiding/18831)

### Addresses:
Mainnet
- $STARK - 0xCa14007Eff0dB1f8135f4C25B34De49AB0d42766
- Core Contracts - 0x739A654271c565839F0408546706bBea2F1FfE42

### Setup:

```bash
curl -L https://foundry.paradigm.xyz | bash
foundryup
```

### Testing:

Unit
```bash
forge test -vv
```

Anvil
```bash
./script/local.sh

cast call <BOOKIE_ADDR> "getCurrentEpoch()((address,uint256,uint256)[])"

cast call <BOOKIE_ADDR> "getCurrentSlot()((address,uint256,uint256))"
```
