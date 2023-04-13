// SPDX-License-Identifier: Apache-2.0
pragma solidity 0.8.16;

import "forge-std/Test.sol";
import "forge-std/Utils.sol";
import "starknet-token/solidity/StarkNetToken.sol";

contract STARKTest is Test {
    Utils internal utils;
    address payable[] internal users;

    address internal alice;
    address internal bob;

    function setUp() public {
        utils = new Utils();
        users = utils.createUsers(2);

        alice = users[0];
        vm.label(alice, "Alice");
        bob = users[1];
        vm.label(bob, "Bob");
        
        constructor() ERC20

    }
}
