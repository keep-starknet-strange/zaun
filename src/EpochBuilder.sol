// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.19;

uint256 constant EPOCH = 5 minutes;
uint256 constant SLOT = 15 seconds;
uint256 constant SLOTS_PER_EPOCH = EPOCH / SLOT;
uint256 constant SECS_PER_EPOCH = EPOCH * SLOT;

struct Slot {
    address leader;
    uint256 block_number;
    uint256 randomness;
}

abstract contract EpochBuilder {
    mapping(uint256 => Slot[]) public epochs;
    uint256 public currentEpoch;

    function getCurrentEpoch() public view virtual returns (Slot[] memory) {
        return epochs[currentEpoch];
    }

    function getCurrentSlot() public view virtual returns (Slot memory) {
        Slot memory _slot;
        for (uint i = 0; i < SLOTS_PER_EPOCH; ++i) {
            Slot memory s = epochs[currentEpoch][i];
            if (s.block_number == block.number) {
                _slot = s;
                break;
            }
        }
        return _slot;
    }

    function getCurrentTimestamp() public view virtual returns (uint256) {
        return currentEpoch;
    }
    
    function getCurrentLeader() public view virtual returns (address) {
        return getCurrentSlot().leader;
    }
    
    function getCurrentRandomness() public view virtual returns (uint256) {
        return getCurrentSlot().randomness;
    }

    function _buildEpoch(address[] memory stakers) public virtual {
        uint256 ts = block.timestamp;
        uint256 block_num = block.number;
        uint256 rand = block.prevrandao;

        currentEpoch = ts;

        for (uint i = 0; i < SLOTS_PER_EPOCH; ++i) {
            // TODO: remove once anvil supports difficulty OR randao
            if (rand == 0) {
                rand = ts;
            }
            uint256 innerRand = rand * i / 2 ** 3;
            address l = stakers[innerRand % stakers.length];
            epochs[ts].push(Slot(l, block_num + i, innerRand));
        }
    }
}
