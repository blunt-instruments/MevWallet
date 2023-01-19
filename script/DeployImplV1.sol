// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "forge-std/Script.sol";

import {MevWalletV1} from "../contracts/MevWalletV1.sol";

contract DeployImplV1 is Script {
    // 0x00000000004096437C84E1B0927D5ED44F45F6b3
    bytes32 constant salt = 0x13061f1c1bbc52beabac07a60520c45cf18845c95a4b8b4292b718e26456d249;

    event h(bytes32);

    function setUp() public {}

    function run() public {
        vm.broadcast();
        new MevWalletV1{salt: salt}();
        emit h(keccak256(type(MevWalletV1).creationCode));
    }
}
