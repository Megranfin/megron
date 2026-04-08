# Contributing to Megron

Megron is open source and welcomes contributions. Please open an issue before submitting a pull request for significant changes.

## Getting Started

```bash
git clone https://github.com/Megron1/megron.git
cd megron
```

## Frontend Setup

```bash
cd frontend
npm install
```

## Environment Configuration

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

## Run Locally

```bash
npm run dev
```

## Build & Deploy Smart Contracts

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

## Submitting Changes

```bash
git checkout -b feature/your-feature-name
git commit -m "feat: describe your change"
git push origin feature/your-feature-name
```

Then open a pull request against `main`. Please keep PRs focused — one feature or fix per PR.

## Prerequisites

- Node.js v18+
- Rust toolchain (for Soroban contract development)
- [Freighter Wallet](https://freighter.app) browser extension
- Stellar Testnet account via [Stellar Laboratory](https://laboratory.stellar.org)
