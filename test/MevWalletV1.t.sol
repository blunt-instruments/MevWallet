// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import {MevWalletV1} from "../contracts/MevWalletV1.sol";
import {MevWalletV1Factory} from "../contracts/MevWalletV1Factory.sol";
import {IERC20} from "../contracts/utils/IERC20.sol";
import {MWB} from "./MWB.sol";
import {IMevWeth} from "mev-weth/IMevWeth.sol";

contract MevWalletTest is Test {
    MevWalletV1Factory factory;
    MevWalletV1 mtx;

    uint256 priv = 1337;
    address payable user = payable(vm.addr(priv));

    address payable searcher = payable(vm.addr(31337));
    address friend = vm.addr(31333337);

    address payable constant mwethAddress = payable(0x00000000008C43efC014746c230049e330039Cb3);
    address payable constant mtxImplAddress = payable(0x00000000004096437C84E1B0927D5ED44F45F6b3);

    IERC20 weth = IERC20(mwethAddress);
    IMevWeth mev = IMevWeth(mwethAddress);

    function setUp() public {
        new MWB().etch(mwethAddress);
        // this leaves the impl uninitialized, which is AOK in tests :)
        vm.etch(mtxImplAddress, type(MevWalletV1).runtimeCode);

        factory = new MevWalletV1Factory();
        mtx = MevWalletV1(payable(factory.createWallet(keccak256("hello"), user)));

        bool success;
        (success,) = mwethAddress.call{value: 2 ether}("");
        searcher.transfer(1 ether);
        user.transfer(1 ether);
        weth.transfer(user, 1 ether);
        weth.transfer(address(mtx), 1 ether);

        // // TODO: re-enable this when we can set tx.gasprice
        vm.fee(1 gwei);
    }

    function buildMevTx(address to, bytes memory data, int256 value, int256 tip) internal view returns (bytes memory) {
        uint256 timing = 0;
        uint256 n = mtx.nonce();
        uint256 maxBaseFee = 0;

        uint8 v;
        bytes32 r;
        bytes32 s;

        bytes32 hashStruct =
            keccak256(abi.encode(mtx.TX_TYPEHASH(), to, keccak256(data), value, false, tip, maxBaseFee, timing, n));
        bytes32 h = keccak256(abi.encodePacked("\x19\x01", mtx._DOMAIN_SEPARATOR(), hashStruct));

        (v,r,s) = vm.sign(priv, h);

        return abi.encodeWithSelector(
            mtx.mevTx.selector,
            to, data, value, false, tip, maxBaseFee, timing, n, v, r, s
        );
    }

    function sendMevTx(bytes memory mevTx, uint256 value) internal {
        vm.prank(searcher);
        bool suc;
        (suc,) = address(mtx).call{value: value}(mevTx);
    }

    function getMev() internal {
        vm.prank(searcher);
        mev.getMev();
    }

    function test_setup() public {
        assertEq(weth.balanceOf(user), 1 ether);
        assertEq(mtx.owner(), user);
        assertEq(block.basefee, 1 gwei, "bf != 1gwei");
        assertEq(tx.gasprice, block.basefee, "bf != gp");
    }

    function test_typehash() public {
        assertEq(mtx.TX_TYPEHASH(), hex"fd4c9c9ceea85482c3671626f7b1c65bc2230325b86dbcb0f971327c3062c26a");
    }

    function test_sendEthSearcherValue() public {
        uint256 toSend = 0.5 ether;
        uint256 tip = 500 gwei;

        bytes memory t = buildMevTx(friend, "", int256(toSend), int256(tip));
        sendMevTx(t, toSend);

        assertEq(friend.balance, toSend, "friend recieved");
        assertGe(mev.mev(), tip + toSend, "mev amnt");

        getMev();
        assertGe(weth.balanceOf(searcher), tip + toSend);
    }

    function test_sendEthWalletValue() public {
        vm.deal(address(mtx), 15 ether);

        uint256 toSend = 0.5 ether;
        uint256 tip = 500 gwei;

        bytes memory t = buildMevTx(friend, "", int256(toSend), int256(tip));
        sendMevTx(t, 0);

        emit log_uint(friend.balance);

        assertEq(friend.balance, toSend, "friend recieved");
        assertGe(mev.mev(), tip, "mev amnt");

        getMev();
        assertGe(weth.balanceOf(searcher), tip);
    }

    function test_sendEthMixedVallue() public {
        uint256 startBal = 0.2 ether;
        vm.deal(address(mtx), startBal);

        uint256 toSend = 0.5 ether;
        uint256 tip = 500 gwei;

        bytes memory t = buildMevTx(friend, "", int256(toSend), int256(tip));
        sendMevTx(t, toSend - startBal);

        emit log_uint(friend.balance);

        assertEq(friend.balance, toSend, "friend recieved");
        assertGe(mev.mev(), tip, "mev amnt");

        getMev();
        assertGe(weth.balanceOf(searcher), tip + toSend - startBal);
    }
}
