// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "forge-std/Script.sol";

import {MevWalletV1} from "../contracts/MevWalletV1.sol";

contract DeployImplV1 is Script {
    // 0x0000000000c08718718B974D644B098C19bd0064
    bytes32 constant salt = 0x13061f1c1bbc52beabac07a60520c45cf18845c97844375d0e2d4023552cf4ac;

    event h(bytes32);

    function setUp() public {}

    function run() public {
        vm.broadcast();
        new MevWalletV1{salt: salt}();
        emit h(keccak256(type(MevWalletV1).creationCode));
    }
}
