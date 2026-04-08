use soroban_sdk::{token, Address, Env};

use crate::errors::MegronError;
use crate::storage_types::{DataKey, Escrow, EscrowStatus};

/// Client funds the escrow. Tokens are transferred from client to contract.
pub fn fund(
    env: &Env,
    client: Address,
    worker: Address,
    token: Address,
    amount: i128,
) -> Result<Escrow, MegronError> {
    client.require_auth();

    if amount <= 0 {
        return Err(MegronError::InvalidAmount);
    }

    let contract_address = env.current_contract_address();
    let token_client = token::Client::new(env, &token);
    token_client.transfer(&client, &contract_address, &amount);

    let escrow = Escrow {
        client,
        worker,
        token,
        amount,
        status: EscrowStatus::Funded,
    };

    env.storage().persistent().set(&DataKey::Escrow, &escrow);
    Ok(escrow)
}

/// Client marks the milestone complete — releases funds to the worker.
pub fn release(env: &Env) -> Result<i128, MegronError> {
    let mut escrow: Escrow = env
        .storage()
        .persistent()
        .get(&DataKey::Escrow)
        .ok_or(MegronError::EscrowNotFound)?;

    escrow.client.require_auth();

    if escrow.status != EscrowStatus::Funded {
        return Err(MegronError::EscrowNotFunded);
    }

    let contract_address = env.current_contract_address();
    let token_client = token::Client::new(env, &escrow.token);
    token_client.transfer(&contract_address, &escrow.worker, &escrow.amount);

    escrow.status = EscrowStatus::Released;
    env.storage().persistent().set(&DataKey::Escrow, &escrow);

    Ok(escrow.amount)
}

/// Client cancels and reclaims funds before release.
pub fn refund(env: &Env) -> Result<i128, MegronError> {
    let mut escrow: Escrow = env
        .storage()
        .persistent()
        .get(&DataKey::Escrow)
        .ok_or(MegronError::EscrowNotFound)?;

    escrow.client.require_auth();

    if escrow.status != EscrowStatus::Funded {
        return Err(MegronError::EscrowNotFunded);
    }

    let contract_address = env.current_contract_address();
    let token_client = token::Client::new(env, &escrow.token);
    token_client.transfer(&contract_address, &escrow.client, &escrow.amount);

    escrow.status = EscrowStatus::Refunded;
    env.storage().persistent().set(&DataKey::Escrow, &escrow);

    Ok(escrow.amount)
}

/// Read current escrow state.
pub fn get_escrow(env: &Env) -> Result<Escrow, MegronError> {
    env.storage()
        .persistent()
        .get(&DataKey::Escrow)
        .ok_or(MegronError::EscrowNotFound)
}
