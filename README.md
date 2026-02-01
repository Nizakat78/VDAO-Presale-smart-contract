# VDAO - Solana Mint & Burn Program

A Solana smart contract built with Anchor Framework that implements token minting and burning functionality with an automatic 10% team allocation on every mint.

## Overview

VDAO is a Solana program that provides:

- **Token Minting** with automatic 10% team fee allocation
- **Admin Burn** - authority can burn tokens from any user account
- **User Burn** - users can burn their own tokens voluntarily
- **PDA-based Authority** - program uses Program Derived Address for secure signing

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Blockchain | Solana |
| Framework | Anchor v0.31.0 |
| Smart Contract | Rust (Edition 2021) |
| Testing | TypeScript, Mocha, Chai |
| Package Manager | Yarn |

## Project Structure

```
vdao/
├── programs/
│   └── vdao/
│       ├── Cargo.toml            # Rust dependencies
│       └── src/
│           └── lib.rs            # Main smart contract
├── tests/
│   └── vdao.ts                   # Test suite
├── migrations/
│   └── deploy.ts                 # Deployment script
├── Anchor.toml                   # Anchor configuration
├── Cargo.toml                    # Rust workspace config
├── package.json                  # Node.js dependencies
└── tsconfig.json                 # TypeScript config
```

## Program Instructions

### 1. `initialize`
Initializes the program configuration account (PDA).

- Sets the authority, mint address, and team token account
- Creates a PDA with seed `"config"`

### 2. `mint_to_user`
Mints tokens to a user with automatic team allocation.

- Mints the requested `amount` to the recipient
- Mints `amount / 10` (10%) to the team token account
- Requires authority signature

### 3. `burn_from_user`
Admin-controlled burn from any user's token account.

- Burns specified `amount` from the user's account
- Uses PDA signer authority
- Requires authority signature

### 4. `user_burn`
Allows users to voluntarily burn their own tokens.

- Burns specified `amount` from the user's token account
- User must be the token account owner
- Requires user's own signature

## Account Structure

### Config (PDA)
| Field | Type | Description |
|-------|------|-------------|
| `authority` | `Pubkey` | Admin who controls the program |
| `mint` | `Pubkey` | Token mint address |
| `team_token_account` | `Pubkey` | Team's token account for 10% allocation |
| `bump` | `u8` | PDA bump seed |

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor CLI](https://www.anchor-lang.com/docs/installation) (v0.31.0+)
- [Node.js](https://nodejs.org/) and [Yarn](https://yarnpkg.com/)

## Setup & Installation

```bash
# Clone the repository
git clone <repository-url>
cd vdao

# Install JS dependencies
yarn install

# Build the program
anchor build

# Get the program ID
solana address -k target/deploy/vdao-keypair.json

# Update the program ID in:
#   - programs/vdao/src/lib.rs (declare_id!)
#   - Anchor.toml ([programs.localnet])
```

## Build

```bash
anchor build
```

## Testing

```bash
# Start local validator (in a separate terminal)
solana-test-validator

# Run tests
anchor test
```

## Deployment

```bash
# Deploy to localnet
anchor deploy

# Deploy to devnet
anchor deploy --provider.cluster devnet

# Deploy to mainnet
anchor deploy --provider.cluster mainnet
```

## Token Flow

```
User requests mint of N tokens
        │
        ▼
┌───────────────────┐
│   mint_to_user()  │
│                   │
│  User  ← N tokens│
│  Team  ← N/10    │
│         tokens    │
└───────────────────┘

Total minted = N + (N/10)
```

## Dependencies

### Rust
- `anchor-lang` v0.31.0 (with `init-if-needed` feature)
- `anchor-spl` v0.31.0

### JavaScript/TypeScript
- `@coral-xyz/anchor` ^0.31.1
- `mocha` ^9.0.3
- `chai` ^4.3.4
- `typescript` ^5.7.3

## License

ISC
