pragma solidity ^0.8.17;

import "forge-std/Script.sol";

import {MevWalletV0Factory} from "../contracts/MevWalletV0Factory.sol";

contract DeployProxyV0 is Script {
    // TODO: fix
    MevWalletV0Factory factory = MevWalletV0Factory(0x385D9e104941e53fa73BBa3Ec3e9bAA4D1C5ad39);

    event proxy(address);

    function run(bytes32 salt) public {
        vm.broadcast();
        emit proxy(factory.createWallet(salt));
    }
}
