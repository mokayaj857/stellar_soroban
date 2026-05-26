# SEP-41 Token (Soroban)

SEP-41 compliant fungible token on Stellar Soroban implementing:

- `mint` (admin-only; initial supply minted in `__constructor`)
- `transfer_from`
- `burn`
- `burn_from`
- Full `TokenInterface` trait (SEP-41)

## Build

```bash
cd sep41_token
cargo build --target wasm32v1-none --release
```

## Test

```bash
cargo test
```

## Deploy to Testnet

Deployed on testnet — see [DEPLOYMENT.md](./DEPLOYMENT.md) for contract ID and explorer links.

Requires [Stellar CLI](https://developers.stellar.org/docs/tools/cli) and a funded testnet account:

```bash
stellar network use testnet
stellar keys generate deployer --network testnet
# Fund via friendbot: curl "https://friendbot.stellar.org?addr=$(stellar keys address deployer)"
cargo build --target wasm32v1-none --release
ADMIN=$(stellar keys address deployer)
stellar contract deploy \
  --source-account deployer \
  --wasm target/wasm32v1-none/release/sep41_token.wasm \
  --network testnet \
  -- \
  --admin "$ADMIN" \
  --initial_holder "$ADMIN" \
  --initial_mint_amount 1000000 \
  --decimal 7 \
  --name "SEP41 Token" \
  --symbol "SEP41"
```
