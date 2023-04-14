// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.19;

import "forge-std/Test.sol";
import "../src/Bookie.sol";

contract BookieTest is Test {
    Bookie bookie;

    address[] stakers;

    function setUp() public {
        for (uint i = 1; i <= 101; i++) {
            stakers.push(vm.addr(i));
        }
        console.logAddress(stakers[0]);
        address[] memory addrs;
        bookie = new Bookie(addrs);
    }

    function testRegisterStaker() public {
        (bool f1,) = bookie.findStaker(stakers[0]);
        assertFalse(f1);
        
        vm.prank(stakers[0]);
        bookie.registerStaker();

        (bool f2,) = bookie.findStaker(stakers[0]);
        assertTrue(f2);
    }

    function testBuildEpoch() public {
        vm.roll(2);
        vm.warp(block.timestamp + 1);
        vm.difficulty(12345);
        for (uint i = 0; i <= stakers.length-2; i++) {
            vm.prank(stakers[i]);
            bookie.registerStaker();
        }

        vm.prank(stakers[0]);
        bookie.buildEpoch();
        Slot[] memory epoch = bookie.getCurrentEpoch();
        assertEq(epoch.length, SLOTS_PER_EPOCH);

        Slot memory slot1 = bookie.getCurrentSlot();
        assertEq(block.number, slot1.block_number);

        vm.roll(4);
        vm.warp(block.timestamp + 1);
        Slot memory slot2 = bookie.getCurrentSlot();
        assertEq(block.number, slot2.block_number);

        assertNotEq(slot1.block_number, slot2.block_number);
        assertNotEq(slot1.leader, slot2.leader);
        assertNotEq(slot1.randomness, slot2.randomness);

        vm.roll(6);
        vm.warp(block.timestamp + SECS_PER_EPOCH);
        vm.prank(stakers[0]);
        bookie.buildEpoch();
    }
}
