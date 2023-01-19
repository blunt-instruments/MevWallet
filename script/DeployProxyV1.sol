pragma solidity ^0.8.17;

import "forge-std/Script.sol";

import {MevWalletV1Factory} from "../contracts/MevWalletV1Factory.sol";

contract DeployProxyV1 is Script {
    MevWalletV1Factory factory = MevWalletV1Factory(0x49496DD21760ED9235aFE43871983869CC0eFC61);

    event proxy(address);

    function run(bytes32 salt) public {
        vm.broadcast();
        emit proxy(factory.createWallet(salt));
    }
}
