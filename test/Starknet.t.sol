// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.6.12;
pragma experimental ABIEncoderV2;

import "forge-std/Test.sol";
import "starknet-cc/Starknet.sol";

contract StarknetTest is Test {
    Starknet sn;
    function setUp() public {
        sn = new Starknet();
    }

    function testDefaultState() public {
        assertEq(sn.stateRoot(), 0);
    }
}
