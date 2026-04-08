# Megron

Megron is an open-source payment infrastructure that lets anyone send and receive money across borders using the Stellar network. recipients can convert funds directly to their local currency.

Built for freelancers, gig workers, and businesses operating across borders, Megron ships three core features:

- **Personal Finance Dashboard** — a real-time view of every transaction in your Stellar wallet, automatically categorized so you always know where your money is going.
- **Trustless Subscription Billing** — recurring payments enforced by Soroban smart contracts, with no intermediary, no chargebacks, and full on-chain transparency.
- **Gig Worker Payments & Escrow** — milestone-based escrow that releases funds the moment work is confirmed, settling in seconds for fractions of a cent.

---

## The Problem

Sending money across borders is still slow, expensive, and exclusionary. Payment processors charge high fees, enforce geographic restrictions, and require banking infrastructure that over 1.4 billion adults simply don't have access to. Freelancers wait weeks for wire transfers. Gig workers can't get paid by international clients. Small businesses lose a significant cut of every transaction to intermediaries.

Megron removes all of that friction.

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

See [CONTRIBUTING.md](./CONTRIBUTING.md) for setup instructions, environment configuration, and how to deploy the smart contracts locally.

---


## Why This Project Deserves Funding

Financial infrastructure should be open, permissionless, and globally accessible. Megron is building tooling that does not exist on Stellar today in a cohesive form — a full-stack application that makes on-chain money management as intuitive as a consumer banking app, while removing every dependency on traditional financial intermediaries.

The gig economy is global. Subscribers and creators are global. Megron's target users are not just crypto-native early adopters — they are the freelancer in Southeast Asia waiting three weeks for a wire transfer, the independent contractor who can't get a business bank account, and the small business that wants to bill customers without surrendering a percentage to a payment processor. Stellar's architecture makes serving all of them possible. Megron makes it practical.

Open-source development and community grants are how infrastructure like this gets built. Funding through Drips directly enables continued development of the smart contracts, frontend application, and documentation that make Megron usable by real people.

---

## Contributing

Megron is open source and welcomes contributions. Please open an issue before submitting a pull request for significant changes.

See [CONTRIBUTING.md](./CONTRIBUTING.md) for detailed guidelines.


