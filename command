near create-account password.jwt.testnet --masterAccount  mehrdadparsa.testnet --initialBalance 1

near deploy password.jwt.testnet --wasmFile contract.wasm \ --initFunction 'new' \ --initArgs '"{"solution"}":"1234"'


near view password.jwt.testnet  get_password_number

near view password.jwt.testnet  set_solution '{"solution":"master@123"}' --accountId jwt.testnet

near delete password.jwt.testnet jwt.testnet