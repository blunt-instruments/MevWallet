// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "forge-std/Script.sol";

import {MevWalletV0} from "../contracts/MevWalletV0.sol";

contract DeployImplV0 is Script {
    // 5 leading 5 total
    // 0x00000000007Dcbd85Fc67915ad4bE7DAE266e268
    bytes32 constant salt = 0x13061f1c1bbc52beabac07a60520c45cf18845c93d3e19d8273d48c0755820ca;

    event h(bytes32);

    function setUp() public {}

    function run() public {
        vm.broadcast();
        new MevWalletV0{salt: salt}();
        emit h(keccak256(type(MevWalletV0).creationCode));
    }
}
