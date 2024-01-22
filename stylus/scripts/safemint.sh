#!/bin/bash

RPC="http://localhost:8547"
MY_TOKEN_ADDRESS="0xC0F1193d1e8475fcBFBBa764ab1c29179F13fDC1"
PRIVATE_KEY="524903677da4ea8aac48b06bae3c4b286ec41cd53d86d8eee8a353aa1345db1e"

echo "minting NFT"
cast send --rpc-url $RPC --private-key $PRIVATE_KEY $MY_TOKEN_ADDRESS "mint()" 1000000 --gas-limit 1000000