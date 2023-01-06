// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import {MevWalletV0} from "../contracts/MevWalletV0.sol";

contract MevWalletTest is Test {
    MevWalletV0 mtx;

    function setUp() public {
        mtx = new MevWalletV0();
    }

    function test_typehash() public {
        assertEq(mtx.TX_TYPEHASH(), hex"11b76fe4fff9c6956dc19a5147c221f72c465e18af029ae4f656a20699a0f0ce");
    }
}
