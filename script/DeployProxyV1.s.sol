pragma solidity ^0.8.17;

import "forge-std/Script.sol";

import {MevWalletV1Factory} from "../contracts/MevWalletV1Factory.sol";

contract DeployProxyV1 is Script {
    MevWalletV1Factory factory = MevWalletV1Factory(0x9248B5e672e1880af34068C0FaE18D30c26D05Fb);

    event proxy(address);

    function run(bytes32 salt) public {
        vm.broadcast();
        emit proxy(factory.createWallet(salt));
    }
}
