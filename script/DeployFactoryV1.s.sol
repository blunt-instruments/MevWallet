// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "forge-std/Script.sol";

import {MevWalletV1Factory} from "../contracts/MevWalletV1Factory.sol";

contract DeployFactoryV1 is Script {
    bytes32 constant salt = keccak256("MevWalletFactory");

    function setUp() public {}

    function run() public {
        vm.broadcast();
        new MevWalletV1Factory{salt: salt}();
    }
}
