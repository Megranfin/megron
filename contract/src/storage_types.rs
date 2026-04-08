use soroban_sdk::{contracttype, Address};

/// Top-level storage keys
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Subscription,
    Escrow,
}

/// Recurring billing agreement
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct Subscription {
    pub subscriber: Address,
    pub recipient: Address,
    pub token: Address,
    pub amount: i128,
    pub interval: u64,    // seconds between payments
    pub next_payment: u64, // ledger timestamp of next due payment
    pub active: bool,
}

/// Milestone-based escrow
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum EscrowStatus {
    Funded,
    Released,
    Refunded,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct Escrow {
    pub client: Address,
    pub worker: Address,
    pub token: Address,
    pub amount: i128,
    pub status: EscrowStatus,
}
