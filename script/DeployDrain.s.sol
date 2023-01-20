// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "forge-std/Script.sol";

import {Drain} from "../contracts/utils/Drain.sol";
import {MevWalletV1} from "../contracts/MevWalletV1.sol";

contract DeployDrain is Script {
    event MevTx(
        address to,
        bytes data,
        int256 value,
        bool delegate,
        int256 tip,
        uint256 maxBaseFee,
        uint256 timing,
        uint256 nonce
    );
    event ToSign(bytes32 h);

    address constant drainAddr = 0x25B6f6FDc8ee71B3571dDBAC309c3c1194761d8D;

    function run() public {
        vm.broadcast();
        new Drain{salt: keccak256("")}();
    }

    function drain(MevWalletV1 mevWallet, int256 tip, address to) public {
        bytes32 hashStruct = keccak256(
            abi.encode(
                mevWallet.TX_TYPEHASH(),
                drainAddr,
                keccak256(abi.encodeWithSelector(Drain.drainAndApprove.selector)),
                0,
                true,
                tip,
                0,
                0,
                mevWallet.nonce()
            )
        );
        bytes32 h = keccak256(abi.encodePacked("\x19\x01", mevWallet._DOMAIN_SEPARATOR(), hashStruct));
        emit MevTx(
            drainAddr,
                abi.encodeWithSelector(Drain.drainAndApprove.selector, to),
                0,
                true,
                tip,
                0,
                0,
                mevWallet.nonce()
        );
        emit ToSign(h);
    }
}
