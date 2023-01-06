pragma solidity ^0.8.17;

import "forge-std/Script.sol";

import {MevWalletV0Factory} from "../contracts/MevWalletV0Factory.sol";

contract DeployProxyV0 is Script {
    // TODO: fix
    MevWalletV0Factory factory = MevWalletV0Factory(0x4D9B7DEFfd09bE5cAAbC6ADc976030A45d0A6D31);

    event proxy(address);

    function run(bytes32 salt) public {
        vm.broadcast();
        emit proxy(factory.createWallet(salt));
    }
}