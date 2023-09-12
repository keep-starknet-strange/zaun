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
        uint256 conf = sn.configHash();
        int256 snBlock = sn.stateBlockNumber();

        uint256[] memory arr = new uint256[](9);
        arr[0] = 21;
        arr[1] = 1;
        arr[2] = uint256(snBlock + 1);
        arr[3] = conf; // must be 0 to match the HEADER offset conf
        arr[4] = 100;
        arr[5] = 200;
        arr[6] = 1;
        arr[7] = 1351148242645005540004162531550805076995747746087542030095186557536641755046;
        arr[8] = 558404273560404778508455254030458021013656352466216690688595011803280448032;
        sn.updateState(arr);
    }
}
