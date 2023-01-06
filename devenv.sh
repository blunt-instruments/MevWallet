#!/bin/sh

RPC_ARG="--rpc-url http://127.0.0.1:8545"
FROM_ARG="--from 0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266"
SENDER_ARG="--sender 0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266"
MEV_WETH="0x00000000008C43efC014746c230049e330039Cb3"

trap "kill 0" SIGINT SIGTERM EXIT

set -e

anvil &

sleep .2

# Funds the create2factory deployer
cast send $RPC_ARG $FROM_ARG --value 1ether 0x3fab184622dc19b6109349b94811493bf2a45362 > /dev/null

# Deploys the create2factory
cast publish $RPC_ARG 0xf8a58085174876e800830186a08080b853604580600e600039806000f350fe7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe03601600081602082378035828234f58015156039578182fd5b8082525050506014600cf31ba02222222222222222222222222222222222222222222222222222222222222222a02222222222222222222222222222222222222222222222222222222222222222 > /dev/null

cd ./lib/mev-weth

# Deploys MevWeth
forge script $RPC_ARG $SENDER_ARG --broadcast --unlocked DeployMevWeth -vvvvvv &> /dev/null

cd ../..

# mints 1000 MevWeth to the from
cast send $RPC_ARG $FROM_ARG --value 1000ether $MEV_WETH > /dev/null

forge script $RPC_ARG $SENDER_ARG --broadcast --unlocked DeployImplV0 -vvvvvv &> /dev/null

sleep .2

forge script $RPC_ARG $SENDER_ARG --broadcast --unlocked DeployFactoryV0 &> /dev/null

sleep .2

forge script $RPC_ARG $SENDER_ARG --broadcast --unlocked DeployProxyV0 --sig "run(bytes32)" 0x5af43d82803e903d91602b57fd5bf30000000000000001189998819991197253 -vvvvvv &> /dev/null

wait