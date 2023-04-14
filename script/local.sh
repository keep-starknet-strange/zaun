#!/bin/bash

anvil -b 5 --config-out script/anvil.json &
ANVIL_PID=$!

PRE_PRIVATE=$(jq -r '.private_keys[1]' script/anvil.json)
PRE_PUBLICS=$(jq -r '.available_accounts[]' script/anvil.json | tr '\n' ',' | sed 's/,$//')
forge create --private-key $PRE_PRIVATE lib/starknet-cc/Starknet.sol:Starknet
echo -e "\nstarknet core contracts deployed..."

forge create --private-key $PRE_PRIVATE lib/starknet-token/src/starkware/isd/solidity/StarkNetToken.sol:StarkNetToken
echo -e "\n\$STARK deployed..."

BOOKIE_ADDR=$(forge create --private-key $PRE_PRIVATE src/Bookie.sol:Bookie --constructor-args "[$PRE_PUBLICS]" | grep "Deployed to:" | awk '{print $3}')
echo -e "\nbookie deployed $BOOKIE_ADDR..."

echo "anvil($ANVIL_PID) running..."
echo "to stop - $ killall anvil"
while true; do
    cast send $BOOKIE_ADDR --private-key $PRE_PRIVATE "buildEpoch()"
    sleep 100
done
