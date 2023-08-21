#!/bin/bash

if ! command -v jq > /dev/null
then
    echo "please install jq"
    exit 1
fi

if ! command -v forge > /dev/null
then
    echo "please install foundry"
    exit 1
fi

BUILD_DIR=$1

if [ -z $BUILD_DIR ]; then
    BUILD_DIR="$PWD/out"
fi

rm $BUILD_DIR/anvil.log 2> /dev/null 
echo "starting anvil..."
anvil -b 5 --config-out $BUILD_DIR/anvil.json 2>&1  >> $BUILD_DIR/anvil.log &
echo $! > $BUILD_DIR/anvil_pid

sleep 2

PRE_PRIVATE=$(jq -r '.private_keys[0]' $BUILD_DIR/anvil.json)

STARKNET_CC=$(forge create --private-key $PRE_PRIVATE lib/starknet-cc/Starknet.sol:Starknet | grep -i 'deployed to' | awk '{print $3}')
jq -r '.starknet_cc += "'$STARKNET_CC'"' $BUILD_DIR/anvil.json > $BUILD_DIR/anvil-cc.json

STARK_TOKEN=$(forge create --private-key $PRE_PRIVATE lib/starknet-token/src/starkware/isd/solidity/StarkNetToken.sol:StarkNetToken | grep -i 'deployed to' | awk '{print $3}')
jq -r '.starknet_token += "'$STARK_TOKEN'"' $BUILD_DIR/anvil-cc.json > $BUILD_DIR/anvil.json

rm $BUILD_DIR/anvil-cc.json

echo "anvil started and contracts deployed..."
