# EVM Wallet

This project is a wallet that works with EVM compatible chains. Currently supports sending transactions. New configurations can be added while the wallet is running.

## Configuration

The server starts up and loads configuration from `config.toml` in the root of the directory. While the server is running configuration can be added by using the endpoints, see the next section. Every time a EVM configuration is added it will add the new configuration to `config.toml`.

## Endpoints

Chain configurations can be added by sending a POST request to the `/config/add` endpoint.

```json
{
  "node_url": "https://rpc.sepolia.org",
  "denom": "eth",
  "currency": "Eth"
}
```

The endpoint in `/blockchain` can be used to send transactions or get the balance of the configured account. To send a transaction send a POST request to `/blockchain/send_tx`.

```json
{
  "to": "0xE50DB02A31D0A95b4B09E9Aaaea913F73a78Ef5e",
  "amount": 1000000000000000, // ETH in wei
  "currency": "Eth"
}
```

Balances can be retrieved for all configured chains by sending a GET request to `/blockchain/balance_all` and for single configured blockchains by sending a POST request to `/blockchain/balance`

```json
"Eth"
```
