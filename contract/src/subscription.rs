use soroban_sdk::{token, Address, Env};

use crate::errors::MegronError;
use crate::storage_types::{DataKey, Subscription};

/// Create a new recurring subscription.
/// `interval` is in seconds (e.g. 2_592_000 = 30 days).
/// The subscriber must pre-approve an allowance to this contract via
/// `token.approve(subscriber, contract, amount, expiry)`.
pub fn subscribe(
    env: &Env,
    subscriber: Address,
    recipient: Address,
    token: Address,
    amount: i128,
    interval: u64,
) -> Result<Subscription, MegronError> {
    subscriber.require_auth();

    if amount <= 0 {
        return Err(MegronError::InvalidAmount);
    }

    let now = env.ledger().timestamp();
    let sub = Subscription {
        subscriber,
        recipient,
        token,
        amount,
        interval,
        next_payment: now + interval,
        active: true,
    };

    env.storage().persistent().set(&DataKey::Subscription, &sub);
    Ok(sub)
}

/// Execute the next payment if it is due.
/// Anyone can call this — the schedule is enforced by the contract.
/// Pulls from the subscriber's pre-approved token allowance.
pub fn charge(env: &Env) -> Result<i128, MegronError> {
    let mut sub: Subscription = env
        .storage()
        .persistent()
        .get(&DataKey::Subscription)
        .ok_or(MegronError::SubscriptionNotFound)?;

    if !sub.active {
        return Err(MegronError::SubscriptionInactive);
    }

    let now = env.ledger().timestamp();
    if now < sub.next_payment {
        return Err(MegronError::PaymentNotDue);
    }

    let contract_address = env.current_contract_address();
    let token_client = token::Client::new(env, &sub.token);
    token_client.transfer_from(&contract_address, &sub.subscriber, &sub.recipient, &sub.amount);

    sub.next_payment += sub.interval;
    env.storage().persistent().set(&DataKey::Subscription, &sub);

    Ok(sub.amount)
}

/// Cancel the subscription. Only the subscriber can cancel.
pub fn cancel(env: &Env) -> Result<(), MegronError> {
    let mut sub: Subscription = env
        .storage()
        .persistent()
        .get(&DataKey::Subscription)
        .ok_or(MegronError::SubscriptionNotFound)?;

    sub.subscriber.require_auth();
    sub.active = false;
    env.storage().persistent().set(&DataKey::Subscription, &sub);
    Ok(())
}

/// Read the current subscription state.
pub fn get_subscription(env: &Env) -> Result<Subscription, MegronError> {
    env.storage()
        .persistent()
        .get(&DataKey::Subscription)
        .ok_or(MegronError::SubscriptionNotFound)
}
