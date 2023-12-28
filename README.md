# Namada-Ping Middleware

(Incomplete)
This app is intended to run alongside a Namada full node to provide compatibility with the Ping-pub [explorer](https://github.com/ping-pub/explorer) which relies on the Cosmos-sdk REST endpoints; eg: `http://localhost:1317/cosmos/slashing/v1beta1/params`.  When a Cosmos-sdk endpoint is requested, the middleware will query the Namada fullnode for the corresponding info and map it as best as possible into a Cosmos-sdk style Json response.

A Namada full node is required; you can specify its url in the .env file at the root of the project.  

### Known issues
- many endpoints not implemented yet or still serving placeholder data
- Ping.pub explorer will sometimes try to convert `tnam` addresses into `cosmosvaloper` addresses, resulting in Bech32 errors; this can cause the explorer to hang.
- Namada and Cosmos designs differ enough that it's not always obvious how to map one to the other