# Cross-Contract-Flipper

Examples of cross-contract calls in ink!

## Get Started

```
git clone https://github.com/brunopgalvao/cross-contract-flipper
cd cross-contract-flipper
pop build
```

1. Deploy other-contract

```
cd other-contract
pop build
pop up contract --args false
```
Now that other-contract is deployed, look up the code_hash of the deployed contract using ui.use.ink or PolkadotJs Apps.
Save the contract code_hash to use in the next step.

2. Deploy cross-contract-flipper
```
cd ..
pop build
// input the other-contract code_hash as the parameter
pop up contract -args 0x35fe6bd568fe247d08f87b860e412df6cdb5b60279243aebe4f5d6651ee2fecf
// input the cross-contract-flipper contract's onchain account id
pop call contract --contract 5FZ7j1GyJTRtdD6FKXwBpUwBcV7sjNuSjrsbfEMXVwtd9TMZ --message flip_using_builder --execute
```
