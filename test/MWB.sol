// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Base.sol";

// DO NOT EDIT
// this is a hacky way of deploying a non-0.8 contract by etching its bytecode

contract MWB is CommonBase {
    function etch(address at) public {
        vm.etch(at, vm.envBytes("MWB"));
    }
}
