# Testnet Deployment

| Field | Value |
|-------|-------|
| Network | Testnet |
| Contract ID | `CCNILERRLSDPK2XNTFM6ZCE3QZ2OVLZ3D5JSJXJDDBGFAMNIUGZDEAGV` |
| Deployer | `GBBNDG3XN7RIQ7FJOGGL3UCZDRR22HIKW6Y7PKD5VRFNVVCFTEWSFRJS` |
| WASM hash | `b3d60403dce798e034e3ea4058fa6c3b4e720972211e61fc41f17720bad7b7f8` |
| Deploy tx | https://stellar.expert/explorer/testnet/tx/93dae347daa6c52745ccae01a6d6be0f6234989dd309d0d858649823bbdc5505 |
| Explorer | https://lab.stellar.org/r/testnet/contract/CCNILERRLSDPK2XNTFM6ZCE3QZ2OVLZ3D5JSJXJDDBGFAMNIUGZDEAGV |

Constructor minted **1,000,000** tokens to the deployer address (verified via `balance`).

## Redeploy

```bash
export PATH="/path/to/stellar:$PATH"
cd sep41_token
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
