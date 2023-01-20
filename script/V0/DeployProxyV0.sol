pragma solidity ^0.8.17;

import "forge-std/Script.sol";

import {MevWalletV0Factory} from "../../contracts/V0/MevWalletV0Factory.sol";

contract DeployProxyV0 is Script {
    // TODO: fix
    MevWalletV0Factory factory = MevWalletV0Factory(0x444544a54b5193Ba6D1A3Cf9c83Ee12422b6A824);

    event proxy(address);

    function run(bytes32 /*salt*/ ) public pure {
        revert("MevWalletV0 use not recommended");
        // vm.broadcast();
        // emit proxy(factory.createWallet(salt));
    }
}
