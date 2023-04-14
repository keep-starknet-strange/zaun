// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.19;

import "./EpochBuilder.sol";
import "./IBookie.sol";
    
uint8 constant MAX_STAKERS = 100;

contract Bookie is IBookie, EpochBuilder {
    address[] public stakers;

    constructor(address[] memory addrs) {
        stakers = addrs;
    }

    function findStaker(address check) public view returns (bool, uint256) {
        for (uint i = 0; i < stakers.length; i++) {
            if (stakers[i] == check) {
                return (true, i);
            }
        }
        return (false, 0);
    }

    function registerStaker() public {
        require(stakers.length < MAX_STAKERS, "max stakers reached");

        stakers.push(msg.sender);

        emit RegisteredStaker(msg.sender, stakers.length);
    }

    function buildEpoch() public {
        (bool found,) = findStaker(msg.sender);
        require(found, "epoch builder must be staker");
        
        _buildEpoch(stakers);
    }
}
