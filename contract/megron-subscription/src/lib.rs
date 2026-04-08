#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, token, Address, Env, Symbol,
};

// ── Storage keys ──────────────────────────────────────────────────────────────

const SUB: Symbol = symbol_short!("SUB");

// ── Data types ────────────────────────────────────────────────────────────────

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct Subscription {
    pub subscriber: Address,
    pub recipient: Address,
    pub token: Address,
    pub amount: i128,
    pub interval: u64,  // seconds between payments
    pub next_payment: u64, // ledger timestamp of next due payment
    pub active: bool,
}

// ── Contract ──────────────────────────────────────────────────────────────────

#[contract]
pub struct SubscriptionContract;

#[contractimpl]
impl SubscriptionContract {
    /// Create a new recurring subscription.
    /// `interval` is in seconds (e.g. 2_592_000 = 30 days).
    pub fn subscribe(
        env: Env,
        subscriber: Address,
        recipient: Address,
        token: Address,
        amount: i128,
        interval: u64,
    ) -> Subscription {
        subscriber.require_auth();

        assert!(amount > 0, "amount must be positive");
        assert!(interval > 0, "interval must be positive");

        let now = env.ledger().timestamp();
        let sub = Subscription {
            subscriber: subscriber.clone(),
            recipient,
            token,
            amount,
            interval,
            next_payment: now + interval,
            active: true,
        };

        env.storage().persistent().set(&SUB, &sub);
        sub
    }

    /// Execute the next payment if it is due.
    /// Anyone can call this — the contract enforces the schedule.
    pub fn charge(env: Env) -> i128 {
        let mut sub: Subscription = env.storage().persistent().get(&SUB).expect("no subscription");

        assert!(sub.active, "subscription is not active");

        let now = env.ledger().timestamp();
        assert!(now >= sub.next_payment, "payment not yet due");

        let client = token::Client::new(&env, &sub.token);
        client.transfer(&sub.subscriber, &sub.recipient, &sub.amount);

        sub.next_payment += sub.interval;
        env.storage().persistent().set(&SUB, &sub);

        sub.amount
    }

    /// Cancel the subscription. Only the subscriber can cancel.
    pub fn cancel(env: Env) {
        let mut sub: Subscription = env.storage().persistent().get(&SUB).expect("no subscription");
        sub.subscriber.require_auth();

        sub.active = false;
        env.storage().persistent().set(&SUB, &sub);
    }

    /// Read the current subscription state.
    pub fn get(env: Env) -> Subscription {
        env.storage().persistent().get(&SUB).expect("no subscription")
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{
        testutils::{Address as _, Ledger, LedgerInfo},
        token::{Client as TokenClient, StellarAssetClient},
        Env,
    };

    fn setup() -> (Env, SubscriptionContractClient<'static>, Address, Address, Address) {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(SubscriptionContract, ());
        let client = SubscriptionContractClient::new(&env, &contract_id);

        let subscriber = Address::generate(&env);
        let recipient = Address::generate(&env);

        // Deploy a test token and mint to subscriber
        let token_admin = Address::generate(&env);
        let token_id = env.register_stellar_asset_contract_v2(token_admin.clone());
        let token_address = token_id.address();
        let asset_client = StellarAssetClient::new(&env, &token_address);
        asset_client.mint(&subscriber, &10_000);

        (env, client, subscriber, recipient, token_address)
    }

    #[test]
    fn test_subscribe_creates_subscription() {
        let (env, client, subscriber, recipient, token) = setup();

        let sub = client.subscribe(&subscriber, &recipient, &token, &500, &2_592_000);

        assert_eq!(sub.subscriber, subscriber);
        assert_eq!(sub.amount, 500);
        assert!(sub.active);
        assert_eq!(sub.interval, 2_592_000);
    }

    #[test]
    fn test_charge_transfers_and_advances_schedule() {
        let (env, client, subscriber, recipient, token) = setup();

        let interval = 2_592_000_u64;
        client.subscribe(&subscriber, &recipient, &token, &500, &interval);

        // Advance ledger time past the first payment due date
        let now = env.ledger().timestamp();
        env.ledger().set(LedgerInfo {
            timestamp: now + interval + 1,
            protocol_version: 22,
            sequence_number: env.ledger().sequence(),
            network_id: Default::default(),
            base_reserve: 10,
            min_temp_entry_ttl: 1,
            min_persistent_entry_ttl: 1,
            max_entry_ttl: 6_312_000,
        });

        let charged = client.charge();
        assert_eq!(charged, 500);

        // Recipient should have received the funds
        let token_client = TokenClient::new(&env, &token);
        assert_eq!(token_client.balance(&recipient), 500);

        // next_payment should have advanced by one interval
        let sub = client.get();
        assert_eq!(sub.next_payment, now + interval + interval);
    }

    #[test]
    fn test_cancel_deactivates_subscription() {
        let (env, client, subscriber, recipient, token) = setup();

        client.subscribe(&subscriber, &recipient, &token, &500, &2_592_000);
        client.cancel();

        let sub = client.get();
        assert!(!sub.active);
    }
}
