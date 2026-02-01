# VDAO - Solana Mint & Burn Program

VDAO ek Solana blockchain par bana hua smart contract program hai jo **Anchor Framework** ke saath develop kiya gaya hai. Yeh program SPL tokens ko **mint** (create) aur **burn** (destroy) karne ki functionality provide karta hai. Iska sabse khaas feature yeh hai ke jab bhi tokens mint hote hain, automatically **10% tokens team ko allocate** ho jaate hain.

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

VDAO ek token management system hai jo Solana blockchain par run hota hai. Iska main kaam hai:

1. **Token Minting** - Naye tokens create karna users ke liye
2. **Team Allocation** - Har mint par 10% tokens automatically team ko dena
3. **Admin Burn** - Authority (admin) kisi bhi user ke tokens burn kar sakta hai
4. **User Burn** - Users apne tokens khud burn kar sakte hain

Program **PDA (Program Derived Address)** use karta hai as mint authority, matlab program khud tokens mint/burn kar sakta hai bina kisi private key ke.

---

## Features

| Feature | Description |
|---------|-------------|
| Token Minting | SPL tokens mint karta hai user ke account mein |
| 10% Team Fee | Har mint par automatically 10% extra tokens team wallet mein jaate hain |
| Admin Burn | Authority kisi bhi user ke tokens burn kar sakta hai (PDA signer se) |
| User Self-Burn | User apne tokens khud burn kar sakta hai apni signature se |
| PDA Authority | Program Derived Address use hota hai as mint/burn authority |
| Config Account | Single config PDA mein saara program state store hota hai |

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
│           └── lib.rs              # Main program code (saara logic yahan hai)
│
├── tests/                          # Test files
│   └── vdao.ts                     # TypeScript test suite
│
├── migrations/                     # Deployment scripts
│   └── deploy.ts                   # Deploy script (template)
│
├── app/                            # Frontend application (abhi empty hai)
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

Program ek PDA use karta hai jo `"config"` seed se derive hota hai:

- **Seed:** `b"config"`
- **Purpose:** Yeh PDA program ki mint authority hai
- **Benefit:** Kisi private key ki zaroorat nahi - program khud sign kar sakta hai

```
PDA = findProgramAddress([b"config"], program_id)
```

---

## Instructions (Functions)

### 1. `initialize`

Program ko pehli baar setup karne ke liye use hota hai. Yeh sirf **ek baar** call hota hai.

**Kya karta hai:**
- Config PDA account create karta hai
- Authority (admin) set karta hai
- Token mint address store karta hai
- Team token account address store karta hai
- PDA bump save karta hai

**Required Accounts:**

| Account | Type | Description |
|---------|------|-------------|
| `config` | `Account<Config>` | PDA - program config store karta hai (init) |
| `mint` | `Account<Mint>` | Token ka mint address |
| `team_token_account` | `Account<TokenAccount>` | Team ka token account |
| `program_authority` | `UncheckedAccount` | PDA authority |
| `authority` | `Signer` | Admin jo program setup kar raha hai |
| `system_program` | `Program<System>` | Solana system program |
| `token_program` | `Program<Token>` | SPL Token program |
| `rent` | `Sysvar<Rent>` | Rent sysvar |

**Parameters:**
| Param | Type | Description |
|-------|------|-------------|
| `_bump` | `u8` | PDA bump (internally calculated) |

---

### 2. `mint_to_user`

User ko tokens mint karta hai aur saath mein **10% team ko** bhi mint karta hai.

**Kya karta hai:**
- User ke account mein `amount` tokens mint karta hai
- Team ke account mein `amount / 10` tokens mint karta hai
- PDA signer use karta hai (program authority)

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
| `config` | `Account<Config>` | PDA config (has_one = mint check) |
| `mint` | `Account<Mint>` | Token mint (mutable) |
| `program_authority` | `UncheckedAccount` | PDA signer |
| `recipient_token_account` | `Account<TokenAccount>` | User ka token account |
| `team_token_account` | `Account<TokenAccount>` | Team ka token account |
| `authority` | `Signer` | Admin signature |
| `token_program` | `Program<Token>` | SPL Token program |

**Parameters:**
| Param | Type | Description |
|-------|------|-------------|
| `amount` | `u64` | Kitne tokens user ko mint karne hain |

---

### 3. `burn_from_user`

Admin kisi bhi user ke tokens burn kar sakta hai. Yeh **admin-controlled** burn hai.

**Kya karta hai:**
- User ke account se `amount` tokens burn karta hai
- PDA signer authority use karta hai
- Sirf authority (admin) call kar sakta hai

**Required Accounts:**

| Account | Type | Description |
|---------|------|-------------|
| `config` | `Account<Config>` | PDA config (has_one = mint check) |
| `mint` | `Account<Mint>` | Token mint (mutable) |
| `program_authority` | `UncheckedAccount` | PDA signer |
| `user_token_account` | `Account<TokenAccount>` | User ka token account (burn hoga) |
| `authority` | `Signer` | Admin signature |
| `token_program` | `Program<Token>` | SPL Token program |

**Parameters:**
| Param | Type | Description |
|-------|------|-------------|
| `amount` | `u64` | Kitne tokens burn karne hain |

---

### 4. `user_burn`

User apne tokens **khud** burn kar sakta hai. Iske liye admin ki zaroorat nahi.

**Kya karta hai:**
- User ke account se `amount` tokens burn karta hai
- User ki apni signature lagti hai
- Token account ka owner user hona chahiye (constraint check)

**Required Accounts:**

| Account | Type | Description |
|---------|------|-------------|
| `config` | `Account<Config>` | PDA config (has_one = mint check) |
| `mint` | `Account<Mint>` | Token mint (mutable) |
| `user_token_account` | `Account<TokenAccount>` | User ka token account (owner = user) |
| `user` | `Signer` | User ki signature |
| `token_program` | `Program<Token>` | SPL Token program |

**Parameters:**
| Param | Type | Description |
|-------|------|-------------|
| `amount` | `u64` | Kitne tokens burn karne hain |

---

## Account Structures

### Config Account (PDA)

Yeh program ka main state account hai. Ek hi baar create hota hai aur saari information store karta hai.

| Field | Type | Size (bytes) | Description |
|-------|------|-------------|-------------|
| `authority` | `Pubkey` | 32 | Admin/authority ka wallet address |
| `mint` | `Pubkey` | 32 | Token mint ka address |
| `team_token_account` | `Pubkey` | 32 | Team ke token account ka address |
| `bump` | `u8` | 1 | PDA bump seed |
| **Discriminator** | - | 8 | Anchor ka account discriminator |
| **Total** | - | **105** | Total space used |

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
     ├── User signs as authority (apni signature)
     └── 200 tokens burned from User's account

Result: User loses 200 tokens, Total Supply -= 200
```

---

## Security

### Account Validations

| Check | Where | Description |
|-------|-------|-------------|
| `has_one = mint` | MintToUser, BurnFromUser, UserBurn | Config mein stored mint match hona chahiye |
| `address = config.team_token_account` | MintToUser | Team account address verify |
| `constraint = user_token_account.owner == *user.key` | UserBurn | User apne hi account se burn kare |
| `seeds = [b"config"], bump` | Initialize, MintToUser, BurnFromUser | PDA verification |
| `Signer` | All instructions | Authority/User signature required |

### Key Security Points

- **PDA Authority:** Program khud mint/burn authority hai, koi external private key nahi
- **Admin Control:** `mint_to_user` aur `burn_from_user` sirf authority call kar sakta hai
- **User Protection:** `user_burn` mein user apni hi token account se burn karta hai (owner check)
- **Config Immutability:** Config sirf `initialize` mein set hota hai, baad mein change nahi hota

---

## Prerequisites

In tools ka installed hona zaroori hai:

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
# 1. Repository clone karein
git clone <repository-url>
cd vdao

# 2. JavaScript dependencies install karein
yarn install

# 3. Program build karein
anchor build

# 4. Program ID generate karein
solana address -k target/deploy/vdao-keypair.json
```

Build ke baad program ID ko **2 jagah update** karna hoga:

1. **`programs/vdao/src/lib.rs`** - `declare_id!()` macro mein
2. **`Anchor.toml`** - `[programs.localnet]` section mein

```rust
// lib.rs mein update karein
declare_id!("YOUR_ACTUAL_PROGRAM_ID_HERE");
```

```toml
# Anchor.toml mein update karein
[programs.localnet]
vdao = "YOUR_ACTUAL_PROGRAM_ID_HERE"
```

---

## Build

```bash
# Program build karein
anchor build

# Build verify karein
ls target/deploy/
# Output: vdao-keypair.json  vdao.so
```

### Build Optimizations (Cargo.toml)

Release build mein yeh optimizations enabled hain:

| Setting | Value | Purpose |
|---------|-------|---------|
| `overflow-checks` | `true` | Integer overflow detect karta hai |
| `lto` | `"fat"` | Link-Time Optimization - smaller binary |
| `codegen-units` | `1` | Maximum optimization (slow build) |
| `opt-level` | `3` | Build override mein highest optimization |

---

## Testing

```bash
# Option 1: Anchor test (automatically starts local validator)
anchor test

# Option 2: Manual testing
# Terminal 1 - Local validator start karein
solana-test-validator

# Terminal 2 - Tests run karein
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

Abhi sirf basic `initialize` instruction ka test hai (`tests/vdao.ts`). Aur tests add karne ki zaroorat hai:

- `mint_to_user` - minting + team allocation verify
- `burn_from_user` - admin burn verify
- `user_burn` - user self-burn verify
- Error cases - unauthorized access, invalid accounts, etc.

---

## Deployment

```bash
# Localnet par deploy (default)
anchor deploy

# Devnet par deploy
anchor deploy --provider.cluster devnet

# Mainnet par deploy
anchor deploy --provider.cluster mainnet
```

### Deployment Checklist

- [ ] Program ID update kiya (`lib.rs` + `Anchor.toml`)
- [ ] Solana wallet mein SOL hai (deployment fee ke liye)
- [ ] Correct cluster selected hai (`Anchor.toml`)
- [ ] Program build successful hai
- [ ] Tests pass ho rahe hain

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

- **`init-if-needed`** feature: Agar account pehle se exist nahi karta toh initialize kar deta hai

### tsconfig.json

```json
{
  "compilerOptions": {
    "types": ["mocha", "chai"],     // Test type definitions
    "lib": ["es2015"],              // ES2015 library
    "module": "commonjs",           // CommonJS modules
    "target": "es6",                // ES6 target
    "esModuleInterop": true         // ES module compatibility
  }
}
```

---

## Dependencies

### Rust (Smart Contract)

| Package | Version | Purpose |
|---------|---------|---------|
| `anchor-lang` | 0.31.0 | Anchor framework core |
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
