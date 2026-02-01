# VDAO - Solana Mint & Burn Program

VDAO is a smart contract program built on the Solana blockchain using the **Anchor Framework**. It provides SPL token **minting** (creation) and **burning** (destruction) functionality. Its key feature is that every time tokens are minted, **10% is automatically allocated to the team wallet**.

---

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Tech Stack](#tech-stack)
- [Project Structure](#project-structure)
- [Program Architecture](#program-architecture)
- [Instructions (Functions)](#instructions-functions)
- [Account Structures](#account-structures)
- [Token Flow](#token-flow)
- [Security](#security)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Build](#build)
- [Testing](#testing)
- [Deployment](#deployment)
- [Configuration Files](#configuration-files)
- [Dependencies](#dependencies)
- [License](#license)

---

## Overview

VDAO is a token management system that runs on the Solana blockchain. Its core responsibilities are:

1. **Token Minting** - Create new tokens for users
2. **Team Allocation** - Automatically allocate 10% of minted tokens to the team
3. **Admin Burn** - The authority (admin) can burn tokens from any user's account
4. **User Burn** - Users can voluntarily burn their own tokens

The program uses a **PDA (Program Derived Address)** as the mint authority, meaning the program itself can mint and burn tokens without requiring an external private key.

---

## Features

| Feature | Description |
|---------|-------------|
| Token Minting | Mints SPL tokens directly into a user's token account |
| 10% Team Fee | Automatically mints 10% extra tokens to the team wallet on every mint |
| Admin Burn | Authority can burn tokens from any user's account using PDA signer |
| User Self-Burn | Users can burn their own tokens using their own signature |
| PDA Authority | Program Derived Address is used as the mint/burn authority |
| Config Account | All program state is stored in a single config PDA |

---

## Tech Stack

| Layer | Technology | Version |
|-------|-----------|---------|
| Blockchain | Solana | - |
| Smart Contract Framework | Anchor | v0.31.0 |
| Smart Contract Language | Rust | Edition 2021 |
| Client SDK | @coral-xyz/anchor | ^0.31.1 |
| Test Framework | Mocha | ^9.0.3 |
| Assertion Library | Chai | ^4.3.4 |
| Language | TypeScript | ^5.7.3 |
| Package Manager | Yarn | - |
| Code Formatter | Prettier | ^2.6.2 |

---

## Project Structure

```
vdao/
├── programs/                       # Solana programs (smart contracts)
│   └── vdao/
│       ├── Cargo.toml              # Rust crate dependencies
│       └── src/
│           └── lib.rs              # Main program code (all logic lives here)
│
├── tests/                          # Test files
│   └── vdao.ts                     # TypeScript test suite
│
├── migrations/                     # Deployment scripts
│   └── deploy.ts                   # Deploy script (template)
│
├── app/                            # Frontend application (currently empty)
│
├── Anchor.toml                     # Anchor framework configuration
├── Cargo.toml                      # Rust workspace configuration
├── Cargo.lock                      # Rust dependency lock file
├── package.json                    # Node.js dependencies & scripts
├── tsconfig.json                   # TypeScript compiler configuration
├── yarn.lock                       # Yarn dependency lock file
├── .gitignore                      # Git ignore rules
└── .prettierignore                 # Prettier ignore rules
```

---

## Program Architecture

### How the Program Works

```
                    ┌─────────────────────────┐
                    │      VDAO Program        │
                    │   (Solana Smart Contract) │
                    └────────────┬────────────┘
                                 │
              ┌──────────────────┼──────────────────┐
              │                  │                   │
              ▼                  ▼                   ▼
     ┌────────────┐    ┌────────────────┐   ┌──────────────┐
     │ Initialize │    │  Mint Tokens   │   │ Burn Tokens  │
     │  (1 time)  │    │ (with 10% fee) │   │ (admin/user) │
     └────────────┘    └────────────────┘   └──────────────┘
           │                   │                    │
           ▼                   ▼                    ▼
     ┌──────────┐     ┌───────────────┐     ┌────────────┐
     │  Config   │     │ User Account  │     │ Token Burn │
     │   PDA     │     │ + Team Account│     │  from User │
     └──────────┘     └───────────────┘     └────────────┘
```

### PDA (Program Derived Address) System

The program uses a PDA derived from the `"config"` seed:

- **Seed:** `b"config"`
- **Purpose:** This PDA serves as the program's mint authority
- **Benefit:** No external private key is needed - the program can sign transactions on its own

```
PDA = findProgramAddress([b"config"], program_id)
```

---

## Instructions (Functions)

### 1. `initialize`

Used to set up the program for the first time. This is called only **once**.

**What it does:**
- Creates the Config PDA account
- Sets the authority (admin)
- Stores the token mint address
- Stores the team token account address
- Saves the PDA bump

**Required Accounts:**

| Account | Type | Description |
|---------|------|-------------|
| `config` | `Account<Config>` | PDA that stores the program configuration (initialized here) |
| `mint` | `Account<Mint>` | The token mint address |
| `team_token_account` | `Account<TokenAccount>` | The team's token account |
| `program_authority` | `UncheckedAccount` | PDA authority |
| `authority` | `Signer` | The admin setting up the program |
| `system_program` | `Program<System>` | Solana system program |
| `token_program` | `Program<Token>` | SPL Token program |
| `rent` | `Sysvar<Rent>` | Rent sysvar |

**Parameters:**
| Param | Type | Description |
|-------|------|-------------|
| `_bump` | `u8` | PDA bump (internally calculated) |

---

### 2. `mint_to_user`

Mints tokens to a user and simultaneously mints **10% to the team**.

**What it does:**
- Mints `amount` tokens to the user's account
- Mints `amount / 10` tokens to the team's account
- Uses the PDA as the signer (program authority)

**Example:**
```
User requests: 1000 tokens
├── User receives:  1000 tokens
├── Team receives:   100 tokens (10%)
└── Total minted:   1100 tokens
```

**Required Accounts:**

| Account | Type | Description |
|---------|------|-------------|
| `config` | `Account<Config>` | PDA config (validated with has_one = mint) |
| `mint` | `Account<Mint>` | Token mint (mutable) |
| `program_authority` | `UncheckedAccount` | PDA signer |
| `recipient_token_account` | `Account<TokenAccount>` | The user's token account to receive tokens |
| `team_token_account` | `Account<TokenAccount>` | The team's token account for the 10% allocation |
| `authority` | `Signer` | Admin signature required |
| `token_program` | `Program<Token>` | SPL Token program |

**Parameters:**
| Param | Type | Description |
|-------|------|-------------|
| `amount` | `u64` | Number of tokens to mint to the user |

---

### 3. `burn_from_user`

Allows the admin to burn tokens from any user's account. This is an **admin-controlled** burn.

**What it does:**
- Burns `amount` tokens from the user's token account
- Uses the PDA as the signer authority
- Only the authority (admin) can call this instruction

**Required Accounts:**

| Account | Type | Description |
|---------|------|-------------|
| `config` | `Account<Config>` | PDA config (validated with has_one = mint) |
| `mint` | `Account<Mint>` | Token mint (mutable) |
| `program_authority` | `UncheckedAccount` | PDA signer |
| `user_token_account` | `Account<TokenAccount>` | The user's token account (tokens will be burned from here) |
| `authority` | `Signer` | Admin signature required |
| `token_program` | `Program<Token>` | SPL Token program |

**Parameters:**
| Param | Type | Description |
|-------|------|-------------|
| `amount` | `u64` | Number of tokens to burn |

---

### 4. `user_burn`

Allows users to **voluntarily burn their own tokens**. No admin involvement required.

**What it does:**
- Burns `amount` tokens from the user's token account
- The user provides their own signature
- The token account owner must match the signing user (enforced by constraint)

**Required Accounts:**

| Account | Type | Description |
|---------|------|-------------|
| `config` | `Account<Config>` | PDA config (validated with has_one = mint) |
| `mint` | `Account<Mint>` | Token mint (mutable) |
| `user_token_account` | `Account<TokenAccount>` | The user's token account (owner must equal user) |
| `user` | `Signer` | The user's signature |
| `token_program` | `Program<Token>` | SPL Token program |

**Parameters:**
| Param | Type | Description |
|-------|------|-------------|
| `amount` | `u64` | Number of tokens to burn |

---

## Account Structures

### Config Account (PDA)

This is the program's main state account. It is created once during initialization and stores all the program configuration.

| Field | Type | Size (bytes) | Description |
|-------|------|-------------|-------------|
| `authority` | `Pubkey` | 32 | The admin/authority wallet address |
| `mint` | `Pubkey` | 32 | The token mint address |
| `team_token_account` | `Pubkey` | 32 | The team's token account address |
| `bump` | `u8` | 1 | PDA bump seed |
| **Discriminator** | - | 8 | Anchor's account discriminator (auto-added) |
| **Total** | - | **105** | Total space allocated |

**PDA Seeds:** `[b"config"]`

---

## Token Flow

### Minting Flow

```
Authority (Admin) calls mint_to_user(amount: 1000)
│
├──► Step 1: Validate accounts
│    ├── Config PDA check
│    ├── Mint address match (has_one)
│    └── Team account address match
│
├──► Step 2: Mint to User
│    ├── CPI call to SPL Token Program
│    ├── PDA signs as mint authority
│    └── 1000 tokens → User's token account
│
└──► Step 3: Mint to Team (10%)
     ├── CPI call to SPL Token Program
     ├── PDA signs as mint authority
     └── 100 tokens → Team's token account

Result: User = 1000 tokens, Team = 100 tokens, Total Supply += 1100
```

### Admin Burn Flow

```
Authority (Admin) calls burn_from_user(amount: 500)
│
├──► Step 1: Validate accounts
│    ├── Config PDA check
│    └── Mint address match (has_one)
│
└──► Step 2: Burn from User
     ├── CPI call to SPL Token Program
     ├── PDA signs as authority
     └── 500 tokens burned from User's account

Result: User loses 500 tokens, Total Supply -= 500
```

### User Self-Burn Flow

```
User calls user_burn(amount: 200)
│
├──► Step 1: Validate accounts
│    ├── Config PDA check
│    ├── Mint address match (has_one)
│    └── Token account owner == User (constraint check)
│
└──► Step 2: Burn from User
     ├── CPI call to SPL Token Program
     ├── User signs as authority (own signature)
     └── 200 tokens burned from User's account

Result: User loses 200 tokens, Total Supply -= 200
```

---

## Security

### Account Validations

| Check | Where | Description |
|-------|-------|-------------|
| `has_one = mint` | MintToUser, BurnFromUser, UserBurn | Ensures the mint stored in config matches the provided mint |
| `address = config.team_token_account` | MintToUser | Verifies the team account address matches config |
| `constraint = user_token_account.owner == *user.key` | UserBurn | Ensures the user can only burn from their own account |
| `seeds = [b"config"], bump` | Initialize, MintToUser, BurnFromUser | PDA derivation verification |
| `Signer` | All instructions | Requires authority or user signature |

### Key Security Points

- **PDA Authority:** The program itself is the mint/burn authority - no external private key exists
- **Admin Control:** `mint_to_user` and `burn_from_user` can only be called by the authority
- **User Protection:** In `user_burn`, the user can only burn from their own token account (owner check enforced)
- **Config Immutability:** The config is only set during `initialize` and cannot be changed afterwards

---

## Prerequisites

The following tools must be installed:

| Tool | Purpose | Install Link |
|------|---------|-------------|
| Rust | Smart contract language | [rustup.rs](https://rustup.rs/) |
| Solana CLI | Blockchain interaction | [docs.solana.com](https://docs.solana.com/cli/install-solana-cli-tools) |
| Anchor CLI | Framework CLI (v0.31.0+) | [anchor-lang.com](https://www.anchor-lang.com/docs/installation) |
| Node.js | JavaScript runtime | [nodejs.org](https://nodejs.org/) |
| Yarn | Package manager | [yarnpkg.com](https://yarnpkg.com/) |

---

## Installation

```bash
# 1. Clone the repository
git clone <repository-url>
cd vdao

# 2. Install JavaScript dependencies
yarn install

# 3. Build the program
anchor build

# 4. Get the generated program ID
solana address -k target/deploy/vdao-keypair.json
```

After building, you need to update the program ID in **2 places**:

1. **`programs/vdao/src/lib.rs`** - Inside the `declare_id!()` macro
2. **`Anchor.toml`** - Under the `[programs.localnet]` section

```rust
// Update in lib.rs
declare_id!("YOUR_ACTUAL_PROGRAM_ID_HERE");
```

```toml
# Update in Anchor.toml
[programs.localnet]
vdao = "YOUR_ACTUAL_PROGRAM_ID_HERE"
```

---

## Build

```bash
# Build the program
anchor build

# Verify the build output
ls target/deploy/
# Output: vdao-keypair.json  vdao.so
```

### Build Optimizations (Cargo.toml)

The following optimizations are enabled for release builds:

| Setting | Value | Purpose |
|---------|-------|---------|
| `overflow-checks` | `true` | Detects integer overflow at runtime |
| `lto` | `"fat"` | Link-Time Optimization for a smaller binary |
| `codegen-units` | `1` | Maximum optimization (slower compilation) |
| `opt-level` | `3` | Highest optimization level in build override |

---

## Testing

```bash
# Option 1: Anchor test (automatically starts a local validator)
anchor test

# Option 2: Manual testing
# Terminal 1 - Start the local validator
solana-test-validator

# Terminal 2 - Run tests without starting a new validator
anchor test --skip-local-validator
```

### Test Configuration

| Setting | Value |
|---------|-------|
| Framework | Mocha + Chai |
| Language | TypeScript (ts-mocha) |
| Timeout | 1,000,000 ms |
| Config | tsconfig.json (ES6, CommonJS) |

### Current Test Coverage

Currently, only a basic `initialize` instruction test exists (`tests/vdao.ts`). Additional tests should be added for:

- `mint_to_user` - Verify minting and team allocation
- `burn_from_user` - Verify admin burn functionality
- `user_burn` - Verify user self-burn functionality
- Error cases - Unauthorized access, invalid accounts, etc.

---

## Deployment

```bash
# Deploy to localnet (default)
anchor deploy

# Deploy to devnet
anchor deploy --provider.cluster devnet

# Deploy to mainnet
anchor deploy --provider.cluster mainnet
```

### Deployment Checklist

- [ ] Program ID updated in both `lib.rs` and `Anchor.toml`
- [ ] Solana wallet has enough SOL for deployment fees
- [ ] Correct cluster is selected in `Anchor.toml`
- [ ] Program build completes successfully
- [ ] All tests are passing

---

## Configuration Files

### Anchor.toml

```toml
[toolchain]
package_manager = "yarn"          # Yarn as package manager

[features]
resolution = true                  # Dependency resolution enabled
skip-lint = false                  # Linting enabled

[programs.localnet]
vdao = "Gjx6kfs9ukdQx4CLTnWjjqZWJymjaWicHS6WzrWQpp9V"  # Program ID

[provider]
cluster = "localnet"               # Default cluster
wallet = "~/.config/solana/id.json"  # Wallet path
```

### Cargo.toml (Program)

```toml
[dependencies]
anchor-lang = { version = "0.31.0", features = ["init-if-needed"] }
anchor-spl = { version = "0.31.0" }
```

- **`init-if-needed`** feature: Automatically initializes an account if it doesn't already exist

### tsconfig.json

```json
{
  "compilerOptions": {
    "types": ["mocha", "chai"],     // Test type definitions
    "lib": ["es2015"],              // ES2015 library
    "module": "commonjs",           // CommonJS modules
    "target": "es6",                // ES6 target
    "esModuleInterop": true         // ES module interop compatibility
  }
}
```

---

## Dependencies

### Rust (Smart Contract)

| Package | Version | Purpose |
|---------|---------|---------|
| `anchor-lang` | 0.31.0 | Anchor framework core library |
| `anchor-spl` | 0.31.0 | SPL token operations (mint, burn, transfer) |

### JavaScript/TypeScript

| Package | Version | Type | Purpose |
|---------|---------|------|---------|
| `@coral-xyz/anchor` | ^0.31.1 | dependency | Anchor client SDK |
| `mocha` | ^9.0.3 | devDependency | Test framework |
| `chai` | ^4.3.4 | devDependency | Assertion library |
| `ts-mocha` | ^10.0.0 | devDependency | TypeScript test runner |
| `typescript` | ^5.7.3 | devDependency | TypeScript compiler |
| `prettier` | ^2.6.2 | devDependency | Code formatter |
| `@types/bn.js` | ^5.1.0 | devDependency | BigNumber type definitions |
| `@types/chai` | ^4.3.0 | devDependency | Chai type definitions |
| `@types/mocha` | ^9.0.0 | devDependency | Mocha type definitions |

---

## License

ISC
