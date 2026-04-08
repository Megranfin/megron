#![no_std]

mod errors;
mod storage_types;
pub mod escrow;
pub mod subscription;

use soroban_sdk::{contract, contractimpl, Address, Env};

use errors::MegronError;
use storage_types::{Escrow, Subscription};

#[contract]
pub struct MegronContract;

#[contractimpl]
impl MegronContract {
    // ── Subscription ──────────────────────────────────────────────────────────

    pub fn subscribe(
        env: Env,
        subscriber: Address,
        recipient: Address,
        token: Address,
        amount: i128,
        interval: u64,
    ) -> Result<Subscription, MegronError> {
        subscription::subscribe(&env, subscriber, recipient, token, amount, interval)
    }

    pub fn charge(env: Env) -> Result<i128, MegronError> {
        subscription::charge(&env)
    }

    pub fn cancel(env: Env) -> Result<(), MegronError> {
        subscription::cancel(&env)
    }

    pub fn get_subscription(env: Env) -> Result<Subscription, MegronError> {
        subscription::get_subscription(&env)
    }

    // ── Escrow ────────────────────────────────────────────────────────────────

    pub fn fund(
        env: Env,
        client: Address,
        worker: Address,
        token: Address,
        amount: i128,
    ) -> Result<Escrow, MegronError> {
        escrow::fund(&env, client, worker, token, amount)
    }

    pub fn release(env: Env) -> Result<i128, MegronError> {
        escrow::release(&env)
    }

    pub fn refund(env: Env) -> Result<i128, MegronError> {
        escrow::refund(&env)
    }

    pub fn get_escrow(env: Env) -> Result<Escrow, MegronError> {
        escrow::get_escrow(&env)
    }
}
