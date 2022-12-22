a simple smart contract demo with near for learning

# contract -

## usage

- Build 
```sh 
    ./build.sh
```

- Create sub account

```sh
near create-account password.jwt.testnet --masterAccount  mehrdadparsa.testnet --initialBalance 1
```
- deploy
```
near deploy password.jwt.testnet --wasmFile res/contract.wasm \ --initFunction 'new' \ --initArgs '"{"solution"}":"1234"'
```