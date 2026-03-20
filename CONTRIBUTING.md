# Contributing

## Setup
```bash
git clone https://github.com/JRNYcrypto/Wick.fun.git
cd Wick.fun
anchor build
```

## Structure
- `programs/wick/` — onchain Solana program (vault + atomic buy/sell)
- `keeper/` — offchain crank that fires wicks on schedule
- `sdk/` — client library for interacting with vaults

## Pull Requests
1. Fork
2. Branch
3. Make changes
4. Test: `anchor test`
5. Open PR
