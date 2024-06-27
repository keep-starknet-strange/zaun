// SPDX-License-Identifier: Apache-2.0.

//! This contract used as a proxy for the Starknet core contract.
//! Starknet core contract can only be initialized indirectly
//! if ProxySupport interface is inherited.
//!
//! Since it's a delegate proxy - it holds the state, thus you need to use
//! the proxy address everywhere instead if the actual core contract address.
//!
//! This implementation IS FOR TESTING PURPOSES ONLY
//! and MUST NOT BE USED IN PRODUCTION.

pragma solidity ^0.8.12;

contract UnsafeProxy {
    // Address of the Starknet core contract deployment
    address delegate;

    constructor(address _delegate) {
        delegate = _delegate;
    }

    fallback() external payable {
        address target = delegate;
        bytes memory data = msg.data;
        assembly {
            let result := delegatecall(gas(), target, add(data,0x20), mload(data), 0, 0)
            let size := returndatasize()
            let ptr := mload(0x40)
            returndatacopy(ptr,0,size)
            switch result
            case 0 {revert(ptr,size)}
            default {return(ptr,size)}
        }
    }
}