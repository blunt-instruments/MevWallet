// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "forge-std/Script.sol";

import {MevWalletV0Factory} from "../../contracts/V0/MevWalletV0Factory.sol";

contract DeployFactoryV0 is Script {
    bytes32 constant salt = keccak256("MevWalletFactory");

    function setUp() public {}

    function run() public {
        vm.broadcast();
        new MevWalletV0Factory{salt: salt}();
    }
}
