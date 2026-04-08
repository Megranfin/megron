# Megron

**On-chain personal finance, subscription billing, and gig worker payments — built on Stellar.**

Megron is an open-source financial infrastructure layer that brings trustless money management to anyone with a Stellar wallet. It combines a real-time spending dashboard, Soroban-powered recurring billing, and instant escrow payments into a single, non-custodial application — no bank account required.

---

## The Problem

Over 1.4 billion adults worldwide remain unbanked, yet the global gig economy continues to grow. Freelancers and independent workers face a fundamental contradiction: they generate real economic value but are routinely excluded from the financial tools needed to manage it. Traditional subscription billing relies on centralized payment processors that charge high fees, enforce geographic restrictions, and require banking infrastructure that millions simply don't have. At the same time, crypto-native users lack a coherent way to track and categorize on-chain spending — the equivalent of a bank statement doesn't exist for most blockchain wallets.

Megron solves all three problems at once.

---

## What Megron Builds

### 1. Personal Finance Dashboard

A real-time, categorized view of every transaction flowing through a user's Stellar wallet. Megron indexes wallet activity via Horizon, automatically classifies transactions by type (payments received, subscriptions paid, DEX swaps, asset transfers), and surfaces them in a clean analytics interface. Users finally get the financial clarity of a modern banking app — without the bank.

### 2. Trustless Subscription Billing

Businesses and creators can deploy recurring billing agreements as Soroban smart contracts. Payment schedules are enforced on-chain — no intermediary, no chargebacks, no geographic restrictions. Subscribers retain full control: they can audit the contract terms, pause payments, or cancel at any time, directly from their wallet. This is subscription infrastructure that works for a SaaS company in San Francisco and a content creator in Lagos equally well.

### 3. Gig Worker Payments & Escrow

Megron provides a milestone-based escrow system for freelancers and gig workers. Clients fund an on-chain escrow contract in USDC or XLM; funds are released to the worker the moment a deliverable is marked complete — confirmed in seconds, settled for fractions of a cent. No wire transfers. No payment processor holds. No bank account required on either side.

---

## Why Stellar

Megron is built on Stellar because Stellar is the most payment-optimized public blockchain in production today.

- **3–5 second finality** — Payments confirm faster than a credit card swipe.
- **~0.00001 XLM per operation** — Transaction costs are negligible at any scale.
- **Native USDC** — Circle's USDC is a first-class asset on Stellar, enabling stable-value payments without volatility risk.
- **Soroban** — Stellar's smart contract platform (built on Rust/WASM) enables the trustless subscription and escrow logic at the core of Megron.
- **Built-in DEX** — Asset swaps happen natively on-chain; no bridge or external protocol required.
- **Global reach** — Stellar is explicitly designed for financial inclusion. Its architecture aligns directly with Megron's mission.

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────┐
│                    Megron Frontend                       │
│              React + TypeScript + Freighter              │
└────────────────────────┬────────────────────────────────┘
                         │
          ┌──────────────┼──────────────┐
          │              │              │
    ┌─────▼──────┐ ┌─────▼──────┐ ┌────▼───────────┐
    │  Horizon   │ │  Soroban   │ │  Stellar DEX   │
    │    API     │ │    RPC     │ │   (SDEX/AMM)   │
    │ (tx index) │ │(contracts) │ │  (asset swaps) │
    └─────┬──────┘ └─────┬──────┘ └────────────────┘
          │              │
    ┌─────▼──────────────▼───────┐
    │        Stellar Network      │
    │   Mainnet / Testnet         │
    └────────────────────────────┘
```

Core smart contracts (Soroban):

- `megron-subscription` — Recurring billing with configurable intervals, grace periods, and on-chain cancellation
- `megron-escrow` — Milestone-based payment release with dispute resolution hooks

---

## Monorepo Structure

```
megron/
├── frontend/        # Next.js app — wallet UI, dashboard, subscription & escrow flows
├── contract/        # Soroban smart contracts (Rust)
└── README.md
```

---

## Tech Stack

| Layer           | Technology                                    |
| --------------- | --------------------------------------------- |
| Blockchain      | Stellar (Mainnet & Testnet)                   |
| Smart Contracts | Soroban (Rust / WASM)                         |
| Stable Payments | USDC on Stellar (Circle)                      |
| Blockchain SDK  | `@stellar/stellar-sdk`                        |
| Frontend        | React 18, TypeScript, Next.js                 |
| Wallet          | Freighter (Stellar's standard browser wallet) |
| Data Indexing   | Stellar Horizon API                           |

---

## Getting Started

### Prerequisites

- Node.js v18+
- Rust toolchain (for Soroban contract development)
- [Freighter Wallet](https://freighter.app) browser extension
- Stellar Testnet account via [Stellar Laboratory](https://laboratory.stellar.org)

### Installation

```bash
git clone https://github.com/Megron1/megron.git
cd megron
```

### Frontend

```bash
cd frontend
npm install
```

### Environment Configuration

```bash
cp .env.example .env
```

```env
VITE_STELLAR_NETWORK=testnet
VITE_HORIZON_URL=https://horizon-testnet.stellar.org
VITE_SOROBAN_RPC_URL=https://soroban-testnet.stellar.org
VITE_USDC_ASSET_CODE=USDC
VITE_USDC_ISSUER=GBBD47IF6LWK7P7MDEVSCWR7DPUWV3NY3DTQEVFL4NAT4AQH3ZLLFLA5
```

### Run Locally

```bash
npm run dev
```

### Build & Deploy Smart Contracts

```bash
cd contract

# Build
cargo build --target wasm32-unknown-unknown --release

# Deploy subscription contract to testnet
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/megron_subscription.wasm \
  --network testnet \
  --source <YOUR_ACCOUNT>

# Deploy escrow contract to testnet
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/megron_escrow.wasm \
  --network testnet \
  --source <YOUR_ACCOUNT>
```

---

## Project Status & Roadmap

Megron is in active development. The following reflects current progress and planned milestones:

| Milestone                                 | Status         |
| ----------------------------------------- | -------------- |
| Wallet connection & Freighter integration | ✅ Complete    |
| Transaction history & Horizon indexing    | ✅ Complete    |
| Spending categorization engine            | ✅ Complete    |
| Personal finance dashboard UI             | ✅ Complete    |
| Soroban subscription contract             | 🔄 In Progress |
| Gig worker escrow contract                | 🔄 In Progress |
| Subscription management UI                | 🔜 Planned     |
| Escrow payment flow UI                    | 🔜 Planned     |
| Stellar DEX swap integration              | 🔜 Planned     |
| Mobile app (React Native)                 | 🔜 Planned     |
| Mainnet launch                            | 🔜 Planned     |

---

## Why This Project Deserves Funding

Financial infrastructure should be open, permissionless, and globally accessible. Megron is building tooling that does not exist on Stellar today in a cohesive form — a full-stack application that makes on-chain money management as intuitive as a consumer banking app, while removing every dependency on traditional financial intermediaries.

The gig economy is global. Subscribers and creators are global. Megron's target users are not just crypto-native early adopters — they are the freelancer in Southeast Asia waiting three weeks for a wire transfer, the independent contractor who can't get a business bank account, and the small business that wants to bill customers without surrendering a percentage to a payment processor. Stellar's architecture makes serving all of them possible. Megron makes it practical.

Open-source development and community grants are how infrastructure like this gets built. Funding through Drips directly enables continued development of the smart contracts, frontend application, and documentation that make Megron usable by real people.

---

## Contributing

Megron is open source and welcomes contributions. Please open an issue before submitting a pull request for significant changes.

```bash
git checkout -b feature/your-feature-name
git commit -m "feat: describe your change"
git push origin feature/your-feature-name
```

See [CONTRIBUTING.md](./CONTRIBUTING.md) for detailed guidelines.

---

## License

MIT © 2025 Megron

> Megron is an independent open-source project. It is not affiliated with the Stellar Development Foundation.
