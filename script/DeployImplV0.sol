// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "forge-std/Script.sol";

import {MevWalletV0} from "../contracts/MevWalletV0.sol";

contract DeployImplV0 is Script {

    // produces 5 leading 6 total
    // 0x0000000000682f8e82d3B3b37200C830E86D2Ef9
    bytes32 constant salt = 0x13061f1c1bbc52beabac07a60520c45cf18845c92906e4b57c40260bf9ec1676;

    event h(bytes32);

    function setUp() public {}

    function run() public {
        vm.broadcast();
        new MevWalletV0{salt: salt}();
    }
}
