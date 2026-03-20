# Wick

Atomic volume infrastructure for Solana tokens.

Wick turns trading fees into green wicks. Fees from $WICK trading accumulate in a vault. On a timer, the vault fires an atomic buy+sell transaction into the chart. The buy outweighs the sell so the chart only goes up. Because it's one atomic transaction, it can't be sandwiched.

## how it works

Every Wick vault operates on a three-state cycle:

- **charging** — fees accumulate, countdown runs
- **firing** — atomic buy+sell executes in a single Solana transaction
- **cooldown** — brief rest period before the next cycle

The buy is always larger than the sell. A configurable retention ratio determines how much stays. If retention is 20%, the vault buys tokens with all its SOL, then sells 80% back, keeping 20% as permanent buy pressure.

Because both instructions execute in the same transaction, MEV bots can't sandwich it. It's the same atomic guarantee that makes flash loans work on Solana.

## the keeper

The keeper is an offchain process that monitors the vault and fires wicks on schedule. It runs on a configurable interval (default: 15 minutes). The keeper:

1. Checks vault SOL balance
2. If balance exceeds minimum threshold, builds the atomic tx
3. Submits the transaction with priority fees
4. Logs the wick to history
5. Resets the timer

Anyone can run a keeper. The program is permissionless. If the official keeper goes down, anyone can fire the vault.

---

## as a service

Wick is open source. Any token can deploy their own vault and keeper:

1. Create a vault for your token
2. Direct your creator fees to the vault address
3. Run a keeper (or use the hosted service)
4. Your chart gets automatic green wicks from your own trading fees

```bash
# create a vault for your token
wick init --mint <TOKEN_MINT> --interval 900 --retention 0.8

# check vault status
wick status --mint <TOKEN_MINT>

# manually fire a wick
wick fire --mint <TOKEN_MINT>
```

---

## architecture

```
wick/
├── programs/wick/src/
│   ├── lib.rs              # program entrypoint
│   ├── state.rs            # vault state accounts
│   ├── instructions/
│   │   ├── mod.rs
│   │   ├── initialize.rs   # create a new vault
│   │   ├── fire.rs         # execute atomic buy+sell
│   │   ├── configure.rs    # update vault settings
│   │   └── withdraw.rs     # emergency withdraw
│   └── errors.rs           # custom errors
├── keeper/src/
│   ├── main.rs             # keeper entrypoint
│   ├── monitor.rs          # vault balance monitoring
│   ├── executor.rs         # transaction builder + submitter
│   ├── config.rs           # keeper configuration
│   └── logger.rs           # wick history logging
├── sdk/src/
│   ├── lib.rs              # SDK entrypoint
│   ├── client.rs           # RPC client wrapper
│   ├── vault.rs            # vault interaction helpers
│   └── preview.rs          # wick size preview
├── tests/
│   ├── test_initialize.rs
│   ├── test_fire.rs
│   └── test_configure.rs
├── Anchor.toml
├── Cargo.toml
├── config.yaml
├── .env.example
└── README.md
```

---

## configuration

```yaml
# config.yaml
vault:
  mint: "<TOKEN_MINT_ADDRESS>"
  authority: "Wick67yV6tpCRVaPPXzXFi4HBjmkAYugdG9ffFJu2uv"
  interval_seconds: 900        # 15 minutes between wicks
  min_balance_sol: 0.1         # minimum vault balance to fire
  retention_ratio: 0.2         # keep 20% of tokens, sell 80%
  max_slippage_bps: 300        # 3% max slippage

keeper:
  rpc_url: "https://api.mainnet-beta.solana.com"
  priority_fee_lamports: 50000
  retry_attempts: 3

logging:
  log_to_file: true
  log_path: "wick_history.log"
```

---

## run

```bash
git clone https://github.com/JRNYcrypto/Wick.fun.git
cd Wick.fun

# build the program
anchor build

# run the keeper
cargo run --bin keeper

# or use the CLI
cargo run --bin wick -- status --mint <TOKEN_MINT>
```

---

## links

- **site:** [wickdot.fun](https://wickdot.fun)
- **x:** [@wickdotfun](https://x.com/wickdotfun)
- **vault:** [Wick67yV6tpCRVaPPXzXFi4HBjmkAYugdG9ffFJu2uv](https://solscan.io/account/Wick67yV6tpCRVaPPXzXFi4HBjmkAYugdG9ffFJu2uv)
