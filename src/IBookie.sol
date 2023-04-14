// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.19;

interface IBookie {
    event RegisteredStaker(address indexed account, uint indexed numStakers);

    function findStaker(address check) external view returns (bool, uint256);

    function registerStaker() external;

    function buildEpoch() external;
}
