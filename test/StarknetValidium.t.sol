// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.6.12;
pragma experimental ABIEncoderV2;

import "forge-std/Test.sol";
import "src/StarknetValidium.sol";

contract StarknetTest is Test {
    Starknet sn;
    address governor;

    function setUp() public {
        governor = vm.addr(0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80);
        vm.prank(governor);
        sn = new Starknet();

        // proxy = new Proxy(0);
        // proxy.addImplementation(address(sn), "0x", false);
        // proxy.upgradeTo(address(sn), "0x", false);
    }

    function testDefaults() public {
        assertEq(sn.stateRoot(), 0);
    }

    function testIsOperator() public {
        console.logAddress(governor);
        // assertEq(sn.starknetIsGovernor(deployer), true);
    }
}
