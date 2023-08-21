#!/bin/bash

BUILD_DIR=$1

if [ -z $BUILD_DIR ]; then
    BUILD_DIR="$PWD/out"
fi

PRE_PRIVATE=$(jq -r '.private_keys[0]' $BUILD_DIR/anvil.json)
PRE_PUBLICS=$(jq -r '.available_accounts[]' $BUILD_DIR/anvil.json | tr '\n' ',' | sed 's/,$//')

BOOKIE_ADDR=$(forge create --private-key $PRE_PRIVATE src/Bookie.sol:Bookie --constructor-args "[$PRE_PUBLICS]" | grep "Deployed to:" | awk '{print $3}')
echo -e "\nbookie deployed $BOOKIE_ADDR..."

echo "anvil($(cat out/anvil_pid)) running..."
echo "to stop - $ killall anvil"
while true; do
    cast send $BOOKIE_ADDR --private-key $PRE_PRIVATE "buildEpoch()"
    sleep 100
done
