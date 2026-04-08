#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, token, Address, Env, Symbol,
};

// ── Storage keys ──────────────────────────────────────────────────────────────

const ESC: Symbol = symbol_short!("ESC");

// ── Data types ────────────────────────────────────────────────────────────────

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

// ── Contract ──────────────────────────────────────────────────────────────────

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {
    /// Client funds the escrow. Tokens are transferred from client to contract.
    pub fn fund(
        env: Env,
        client: Address,
        worker: Address,
        token: Address,
        amount: i128,
    ) -> Escrow {
        client.require_auth();
        assert!(amount > 0, "amount must be positive");

        let contract_address = env.current_contract_address();
        let token_client = token::Client::new(&env, &token);
        token_client.transfer(&client, &contract_address, &amount);

        let escrow = Escrow {
            client,
            worker,
            token,
            amount,
            status: EscrowStatus::Funded,
        };

        env.storage().persistent().set(&ESC, &escrow);
        escrow
    }

    /// Client marks the milestone complete — releases funds to the worker.
    pub fn release(env: Env) -> i128 {
        let mut escrow: Escrow = env.storage().persistent().get(&ESC).expect("no escrow");
        escrow.client.require_auth();

        assert!(
            escrow.status == EscrowStatus::Funded,
            "escrow is not in funded state"
        );

        let contract_address = env.current_contract_address();
        let token_client = token::Client::new(&env, &escrow.token);
        token_client.transfer(&contract_address, &escrow.worker, &escrow.amount);

        escrow.status = EscrowStatus::Released;
        env.storage().persistent().set(&ESC, &escrow);

        escrow.amount
    }

    /// Client cancels and reclaims funds before release.
    pub fn refund(env: Env) -> i128 {
        let mut escrow: Escrow = env.storage().persistent().get(&ESC).expect("no escrow");
        escrow.client.require_auth();

        assert!(
            escrow.status == EscrowStatus::Funded,
            "escrow is not in funded state"
        );

        let contract_address = env.current_contract_address();
        let token_client = token::Client::new(&env, &escrow.token);
        token_client.transfer(&contract_address, &escrow.client, &escrow.amount);

        escrow.status = EscrowStatus::Refunded;
        env.storage().persistent().set(&ESC, &escrow);

        escrow.amount
    }

    /// Read current escrow state.
    pub fn get(env: Env) -> Escrow {
        env.storage().persistent().get(&ESC).expect("no escrow")
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{
        testutils::Address as _,
        token::{Client as TokenClient, StellarAssetClient},
        Env,
    };

    fn setup() -> (Env, EscrowContractClient<'static>, Address, Address, Address) {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(EscrowContract, ());
        let client_contract = EscrowContractClient::new(&env, &contract_id);

        let client = Address::generate(&env);
        let worker = Address::generate(&env);

        let token_admin = Address::generate(&env);
        let token_id = env.register_stellar_asset_contract_v2(token_admin.clone());
        let token_address = token_id.address();
        let asset_client = StellarAssetClient::new(&env, &token_address);
        asset_client.mint(&client, &10_000);

        (env, client_contract, client, worker, token_address)
    }

    #[test]
    fn test_fund_transfers_to_contract() {
        let (env, contract, client, worker, token) = setup();

        contract.fund(&client, &worker, &token, &1_000);

        let token_client = TokenClient::new(&env, &token);
        // client balance reduced
        assert_eq!(token_client.balance(&client), 9_000);
        // contract holds the funds
        let escrow = contract.get();
        assert_eq!(escrow.amount, 1_000);
        assert_eq!(escrow.status, EscrowStatus::Funded);
    }

    #[test]
    fn test_release_pays_worker() {
        let (env, contract, client, worker, token) = setup();

        contract.fund(&client, &worker, &token, &1_000);
        let released = contract.release();

        assert_eq!(released, 1_000);

        let token_client = TokenClient::new(&env, &token);
        assert_eq!(token_client.balance(&worker), 1_000);

        let escrow = contract.get();
        assert_eq!(escrow.status, EscrowStatus::Released);
    }

    #[test]
    fn test_refund_returns_to_client() {
        let (env, contract, client, worker, token) = setup();

        contract.fund(&client, &worker, &token, &1_000);
        let refunded = contract.refund();

        assert_eq!(refunded, 1_000);

        let token_client = TokenClient::new(&env, &token);
        // client gets their money back
        assert_eq!(token_client.balance(&client), 10_000);

        let escrow = contract.get();
        assert_eq!(escrow.status, EscrowStatus::Refunded);
    }
}
