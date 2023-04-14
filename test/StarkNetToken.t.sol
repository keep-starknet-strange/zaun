// SPDX-License-Identifier: Apache-2.0
pragma solidity 0.8.16;

import "forge-std/Test.sol";
import "starknet-token/solidity/StarkNetToken.sol";

uint256 constant SUPPLY = 18e8;
bytes32 constant MINTER_ROLE = keccak256("MINTER_ROLE");

contract StarkNetTokenTest is Test {
    StarkNetToken token;

    address alice = vm.addr(0x1);
    address bob = vm.addr(0x2);
    address mary = vm.addr(0x3);

    function setUp() public {
        vm.startPrank(alice);
        token = new StarkNetToken();
        token.grantRole(MINTER_ROLE, alice);
        vm.stopPrank();
    }

    function testMetadata() public {
        assertEq(token.name(), "StarkNet Token");
        assertEq(token.symbol(), "STRK");
        assertEq(token.totalSupply(), 0);
        assertEq(token.decimals(), 18);
    }

    function testDefaultRoles() public {
        bytes32 defRole = token.DEFAULT_ADMIN_ROLE();
        assertEq(defRole, 0);
        assertTrue(token.hasRole(defRole, alice));
        assertFalse(token.hasRole(defRole, bob));
    }

    function testMintingRoles() public {
        vm.prank(bob);
        vm.expectRevert(
            abi.encodePacked(
                "AccessControl: account ",
                Strings.toHexString(uint160(bob), 20),
                " is missing role ",
                Strings.toHexString(uint256(MINTER_ROLE), 32)
            )
        );
        token.mint(bob, SUPPLY);

        vm.prank(alice);
        token.grantRole(MINTER_ROLE, bob);
        assertTrue(token.hasRole(MINTER_ROLE, bob));
        assertFalse(token.hasRole(MINTER_ROLE, mary));
        
        vm.prank(bob);
        token.mint(mary, SUPPLY);
        assertEq(token.totalSupply(), SUPPLY);
        assertEq(token.balanceOf(mary), SUPPLY);
    }

    function testFuzzApprove(address to, uint256 amount) public {
        vm.assume(to != address(0));
        vm.assume(amount < SUPPLY/2);
        vm.startPrank(alice);
        token.mint(alice, SUPPLY);
        token.approve(to, amount);
        vm.stopPrank();
        assertEq(token.allowance(alice, to), amount);
    }
}
