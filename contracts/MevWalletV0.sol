// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import {Mevitize} from "mev-weth/Mevitize.sol";

contract MevWalletV0 is Mevitize {
    error ProvideValue(uint256); // 0x73883387
    error HighBaseFee(uint256); // 0x74878d58
    error WrongSigner(address); // 0x32c15fc2
    error Reverted(bytes); // 0xa8159920
    error NotBefore(uint64); // 0x08567e55
    error UsedNonce(uint256); // 0x6ac964b0
    error MissingNonce(uint256); // 0x299aa731

    error PermanentlyInvalid(); // 0xa04d981f

    // 0xbcf6a68a2f901be4a23a41b53acd7697893a7e34def4e28acba584da75283b67
    event Executed(uint256 indexed nonce);

    // 0x5679fb6ec38d3c67731b4def49181a8fbbb334cda5c263b0993e50cfe699d4e8
    bytes32 public constant TX_TYPEHASH = keccak256(
        "MevTx(address to,bytes data,int256 value,bool delegate,int256 tip,uint256 maxBaseFee,uint256 timing,uint256 nonce)"
    );
    bytes32 public _DOMAIN_SEPARATOR;

    address public owner;
    uint256 public nonce;

    fallback() external payable {}
    receive() external payable {}

    constructor() {
        owner = address(0xff); // factor that, jerks
    }

    /**
     * @notice initializes the owner and domain separator
     */
    function initialize(address newOwner) public {
        require(owner == address(0));
        // Enforced because contracts cannot produce signatures
        uint256 s;
        assembly {
            s := extcodesize(newOwner)
        }
        require(s == 0, "No contract owner");
        owner = newOwner;
        _DOMAIN_SEPARATOR = keccak256(
            abi.encode(
                keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)"),
                keccak256("MevTx"),
                keccak256(bytes("1")),
                block.chainid,
                address(this)
            )
        );
    }

    /**
     * @notice onlyOwner does what it says on the tin
     */
    modifier onlyOwner() {
        // we allow address(this) so that the wallet can be administered with
        // its own meta-tx
        require(msg.sender == owner || msg.sender == address(this));
        _;
    }

    /**
     * @notice transferOwnership does what it says on the tin
     */
    function transferOwnership(address newOwner) public onlyOwner {
        require(newOwner != address(0) && newOwner != address(this));
        uint256 s;
        // Enforced because contracts cannot produce signatures
        assembly {
            s := extcodesize(newOwner)
        }
        require(s == 0, "No contract owner");
        owner = newOwner;
    }

    /**
     * @notice checks the EIP-712 signsture
     */
    function check712(
        address to,
        bytes memory data,
        int256 value,
        bool delegate,
        int256 tip,
        uint256 maxBaseFee,
        uint256 timing,
        uint256 n,
        uint8 v,
        bytes32 r,
        bytes32 s
    ) internal view {
        bytes32 hashStruct =
            keccak256(abi.encode(TX_TYPEHASH, to, keccak256(data), value, delegate, tip, maxBaseFee, timing, n));
        bytes32 h = keccak256(abi.encodePacked("\x19\x01", _DOMAIN_SEPARATOR, hashStruct));
        address signer = ecrecover(h, v, r, s);
        // signature must be valid
        if (signer == address(0)) revert PermanentlyInvalid();
        // signature must be from owner
        if (signer != owner) revert WrongSigner(signer);
    }

    /**
     * @notice checks that the basefee is in user-acceptable range.
     */
    function checkBaseFee(uint256 maxBaseFee) internal view {
        // if there's a limit on the basefee, it cannot be over that limit
        if (maxBaseFee != 0 && block.basefee > maxBaseFee) revert HighBaseFee(maxBaseFee);
    }

    /**
     * @notice checks that the block timestamp is in user-acceptable range.
     */
    function checkTiming(uint256 timing) internal view {
        // Timing is encoded as `notBefore << 64 | notAfter`
        uint64 time = uint64(block.timestamp);
        uint64 notAfter = uint64(timing);
        uint64 notBefore = uint64(timing >> 64);
        // if notAfter is non-zero, timestamp cannot be after it
        if (notAfter != 0 && time > notAfter) {
            revert PermanentlyInvalid();
        }
        // if notBefore  is non-zero, timestamp cannot be before it
        if (notBefore != 0 && time < notBefore) {
            revert NotBefore(notBefore);
        }
    }

    /**
     * @notice checks that the value is as user specified.
     */
    function checkValue(bool delegate, int256 value) internal view {
        // value cannot be negative
        if (value < 0) revert PermanentlyInvalid();
        // delegate calls cannot have value
        if (delegate && value != 0) revert PermanentlyInvalid();
        // msg.value must be 0, or the committed amount
        if (value > 0 && msg.value != uint256(value)) revert ProvideValue(uint256(value));
    }

    /**
     * @notice checks that the nonce is correct
     */
    function checkNonce(uint256 n) internal view {
        uint256 _nonce = nonce;
        // Nonce cannot be
        if (n < _nonce) revert UsedNonce(_nonce);
        if (n > _nonce) revert MissingNonce(_nonce);
        // pass if equal
    }

    /**
     * @notice executes the meta-tx
     */
    function execute(address to, bytes memory data, bool delegate) internal {
        bool success;
        // overwrite data because we don't need it anymore
        if (delegate) {
            (success, data) = to.delegatecall(data);
        } else {
            // safe to use msg.value as it has been checked in checkValue
            (success, data) = to.call{value: msg.value}(data);
        }
        // okay this seems crazy but hear me out
        // MEV block builders already drop reverting txns.
        // This just makes it so they never get included at all
        // which is desirable for a metatx
        if (!success) {
            revert Reverted(data);
        }
    }

    /**
     * @notice execute a MEV-driven meta-transaction
     */
    function mevTx(
        address to,
        bytes memory data,
        int256 value,
        bool delegate,
        int256 tip,
        uint256 maxBaseFee,
        uint256 timing,
        uint256 n,
        uint8 v,
        bytes32 r,
        bytes32 s
    ) external payable subsidize(tip + value) {
        // check sig first, as this is most likely to produce reverts
        check712(to, data, value, delegate, tip, maxBaseFee, timing, n, v, r, s);

        // other condition of use checks
        if (to == address(0)) revert PermanentlyInvalid();
        checkBaseFee(maxBaseFee);
        checkTiming(timing);
        checkValue(delegate, value);
        checkNonce(n);

        // re-entrancy protection
        nonce = type(uint256).max;

        // execute the tx
        execute(to, data, delegate);

        // emit executed, and incement nonce
        nonce = n + 1;
        emit Executed(n);
    }
}
