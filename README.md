# BlockFrost HTTP Client
Minimal client

## API
### Health
- [ ] Root endpoint
- [ ] Backend health status
- [ ] Current backend time

### Metric 
- [ ] Blockfrost usage metrics
- [ ] Blockfrost endpoint usage metrics

## Cardano
### Accounts 
- [ ] Specific account address
- [ ] Account reward history
- [ ] Account history
- [ ] Account delegation history
- [ ] Account registration history
- [ ] Account withdrawal history
- [ ] Account MIR history
- [x] Account associated addresses
- [ ] Assets associated with the account addresses
- [x] Detailed information about account associated addresses

### Addresses 
- [x] Specific address
- [ ] Extended information of a specific address
- [ ] Address details
- [x] Address UTXOs
- [ ] Address UTXOs of a given asset
- [ ] Address transactions

### Assets
- [ ] Assets
- [ ] Specific asset
- [ ] Asset history
- [ ] Asset transactions 
- [ ] Asset addresses
- [ ] Assets of a specific policy

### Blocks 
- [ ] Latest block
- [ ] Latest block transactions
- [ ] Specific block
- [ ] Listing of next blocks
- [ ] Listing of previous blocks
- [ ] Specific block in a slot
- [ ] Specific block in a slot in an epoch
- [ ] Block transactions
- [ ] Addresses affected in a specific block

### Epochs 
- [ ] Latest epoch
- [x] Latest epoch protocol parameters
- [ ] Specific epoch
- [ ] Listing of next epochs
- [ ] Listing of previous epochs
- [ ] Stake distribution
- [ ] Stake distribution by pool
- [ ] Block distribution
- [ ] Block distribution by pool
- [ ] Protocol parameters

### Ledger 
- [x] Blockchain genesis

### Metadata 
- [ ] Transaction metadata labels
- [ ] Transaction metadata content in JSON
- [ ] Transaction metadata content in CBOR

### Network
- [ ] Network information

### Pools 
- [ ] List of stake pools
- [ ] List of stake pools with additional information
- [ ] List of retired stake pools
- [ ] List of retiring stake pools
- [ ] Specific stake pool
- [ ] Stake pool history
- [ ] Stake pool metadata
- [ ] Stake pool relays
- [ ] Stake pool delegators
- [ ] Stake pool blocks
- [ ] Stake pool updates

### Scripts 
- [ ] Scripts
- [ ] Specific script
- [ ] Script JSON
- [ ] Script CBOR
- [ ] Redeemers of a specific script
- [x] Datum value
- [ ] Datum CBOR value

### Transactions 
- [ ] Specific transaction
- [ ] Transaction UTXOs
- [ ] Transaction stake addresses certificates
- [ ] Transaction delegation certificates
- [ ] Transaction withdrawal
- [ ] Transaction MIRs
- [ ] Transaction stake pool registration and update certificates
- [ ] Transaction stake pool retirement certificates
- [ ] Transaction metadata
- [ ] Transaction metadata in CBOR
- [ ] Transaction redeemers
- [x] Submit a transaction

### Utilities
- [ ] Derive an address
- [x] Submit a transaction for execution units evaluation

## Tests
⚠️Only use test phrase! ️⚠️

This is still hacky, and the code needs to be cleaned up, but they give a good example on how to use the methods.

The tests are all ignored because they actually reach out the the API. This isn't a long-term solution.
To run the tests manually, use the `.blockfrost.toml.template` to create your own config file `.blockfrost.toml`
(that file will be ignored by git).
The transaction  tests will actually build transactions with your key, so ⚠️use at your own risk!
