// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import {Mevitize} from "mev-weth/Mevitize.sol";
import {IERC20} from "./IERC20.sol";

// Helper contract for MevWallet draining
contract Drain {
    function drain(address to) public {
        payable(to).transfer(address(this).balance);
    }

    function approveAll(address asset, address to) public {
        IERC20(asset).approve(to, type(uint256).max);
    }

    function drainAndApprove(address to) public {
        drain(to);
        approveAll(0x00000000008C43efC014746c230049e330039Cb3, to);
    }
}