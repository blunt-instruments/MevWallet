// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "forge-std/Script.sol";

import {MevWalletV0} from "../../contracts/V0/MevWalletV0.sol";

contract DeployImplV0 is Script {
    // 5 leading 5 total
    // 0x00000000008EaBBE9A46Fa87F0d1e41e62A96d50
    bytes32 constant salt = 0x13061f1c1bbc52beabac07a60520c45cf18845c9b76fc1e181df83404270a4da;

    event h(bytes32);

    function setUp() public {}

    function run() public {
        vm.broadcast();
        new MevWalletV0{salt: salt}();
        emit h(keccak256(type(MevWalletV0).creationCode));
    }
}
