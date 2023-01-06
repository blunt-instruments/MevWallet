// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

contract MevWalletV0Factory {
    error CreationFailed(); // 0xd786d393
    error InitFailed(bytes); // 0x225d0a58

    function createWallet(bytes32 salt, address owner) public returns (address) {
        // we commit to the owner so that deployment txns cannot be frontrun
        // (or rather, a frontrun will create _exactly_ the same) state
        salt = keccak256(abi.encode(salt, owner));
        address p;
        assembly {
            let buf := mload(0x40)
            mstore(buf, 0x3d602880600a3d3981f3363d3d373d3d3d363d6e682f8e82d3B3b37200C830E8)
            mstore(add(buf, 0x20), 0x6D2Ef95af43d82803e903d91602657fd5bf30000000000000000000000000000)

            p := create2(0, buf, 0x32, salt)
        }
        if (p == address(0)) revert CreationFailed();

        bytes memory data = abi.encodeWithSignature(
            "initialize(address)",
            [owner]
        );
        bool s;
        (s, data) = p.call(data);
        if (!s) revert InitFailed(data);
        return p;
    }

    function createWallet(bytes32 salt) public returns (address) {
        return createWallet(salt, msg.sender);
    }
}

// 3d602d80600a3d3981f3363d3d373d3d3d363d
// 6e 682f8e82d3B3b37200C830E86D2Ef9
// 5af43d82803e903d91602b57fd5bf3
